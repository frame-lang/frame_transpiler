use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use crate::frame_c::ast::{
    NativeFunctionDeclNode, NativeFunctionParameterNode, NativeModuleDeclNode, NativeModuleItem,
    NativeTypeDeclNode,
};

use super::{DeclarationImportContext, DeclarationImporter, DeclarationSourceConfig};

#[derive(Debug)]
pub struct PythonRuntimeImporter;

impl DeclarationImporter for PythonRuntimeImporter {
    fn name(&self) -> &'static str {
        "python-runtime"
    }

    fn import(
        &self,
        source: &DeclarationSourceConfig,
        context: &DeclarationImportContext,
    ) -> Result<Vec<NativeModuleDeclNode>, String> {
        let mut options = PythonImporterOptions::from_source(source)
            .map_err(|err| format!("Invalid Python importer options: {}", err))?;
        options.resolve_paths(&context.config_dir)?;

        let module_name = options
            .module_name
            .clone()
            .ok_or_else(|| "python importer requires 'moduleName' option".to_string())?;

        let module_path = resolve_module_path(source, &module_name);

        let mut cmd = Command::new(options.python_executable());
        cmd.arg("-c").arg(build_inspection_script(
            &module_name,
            &options.include,
            &options.exclude,
        ));

        if !options.python_path.is_empty() {
            let mut combined_paths: Vec<PathBuf> = Vec::new();
            if let Some(existing) = env::var_os("PYTHONPATH") {
                combined_paths.extend(env::split_paths(&existing));
            }
            combined_paths.extend(options.python_path.iter().cloned());
            let joined = env::join_paths(&combined_paths)
                .map_err(|err| format!("Unable to build PYTHONPATH: {}", err))?;
            cmd.env("PYTHONPATH", joined);
        }

        if context.verbose {
            eprintln!(
                "[decl import] executing python runtime importer for module {}",
                module_name
            );
        }

        let output = cmd.output().map_err(|err| {
            format!(
                "Failed to run python importer for '{}': {}",
                module_name, err
            )
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "Python importer script failed for '{}': {}",
                module_name, stderr
            ));
        }

        let json = String::from_utf8(output.stdout).map_err(|err| {
            format!(
                "Python importer produced invalid UTF-8 output for '{}': {}",
                module_name, err
            )
        })?;

        let members: Vec<PythonMember> = serde_json::from_str(&json).map_err(|err| {
            format!(
                "Unable to parse python importer output for '{}': {}",
                module_name, err
            )
        })?;

        if members.is_empty() {
            return Ok(Vec::new());
        }

        let mut builder = PythonModuleBuilder::new(&options);
        for member in members {
            builder.consume(member);
        }

        if builder.items.is_empty() {
            return Ok(Vec::new());
        }

        let module = NativeModuleDeclNode::new(module_path, 1, 1, builder.items);
        Ok(vec![module])
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PythonImporterOptions {
    #[serde(default)]
    module_name: Option<String>,
    #[serde(default)]
    python_path: Vec<PathBuf>,
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default = "default_python_executable")]
    python: String,
}

impl PythonImporterOptions {
    fn from_source(source: &DeclarationSourceConfig) -> Result<Self, serde_json::Error> {
        match &source.options {
            Some(value) => serde_json::from_value(value.clone()),
            None => Ok(PythonImporterOptions::default()),
        }
    }

    fn resolve_paths(&mut self, base_dir: &Path) -> Result<(), String> {
        self.python_path = self
            .python_path
            .iter()
            .map(|path| resolve_path(base_dir, path))
            .collect();
        self.include = self.include.iter().map(|s| s.to_lowercase()).collect();
        self.exclude = self.exclude.iter().map(|s| s.to_lowercase()).collect();
        Ok(())
    }

    fn python_executable(&self) -> &str {
        &self.python
    }
}

fn default_python_executable() -> String {
    "python3".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
enum PythonMember {
    #[serde(rename = "function")]
    Function {
        name: String,
        is_async: bool,
        parameters: Vec<PythonParameter>,
        #[serde(default)]
        return_type: Option<String>,
    },
    #[serde(rename = "class")]
    Class { name: String },
}

#[derive(Debug, Deserialize)]
struct PythonParameter {
    name: String,
    #[serde(default)]
    annotation: Option<String>,
    #[serde(default, rename = "optional")]
    _optional: bool,
}

struct PythonModuleBuilder<'a> {
    items: Vec<NativeModuleItem>,
    seen_types: HashSet<String>,
    seen_functions: HashSet<String>,
    options: &'a PythonImporterOptions,
}

impl<'a> PythonModuleBuilder<'a> {
    fn new(options: &'a PythonImporterOptions) -> Self {
        Self {
            items: Vec::new(),
            seen_types: HashSet::new(),
            seen_functions: HashSet::new(),
            options,
        }
    }

    fn consume(&mut self, member: PythonMember) {
        match member {
            PythonMember::Class { name } => self.add_type(name),
            PythonMember::Function {
                name,
                is_async,
                parameters,
                return_type,
            } => self.add_function(name, is_async, parameters, return_type),
        }
    }

    fn add_type(&mut self, name: String) {
        let sanitized = sanitize_identifier(&name);
        if self.seen_types.insert(sanitized.clone()) {
            self.items
                .push(NativeModuleItem::Type(NativeTypeDeclNode::new(
                    sanitized, None, 1, 1,
                )));
        }
    }

    fn add_function(
        &mut self,
        name: String,
        is_async: bool,
        parameters: Vec<PythonParameter>,
        return_type: Option<String>,
    ) {
        if !self.should_include(&name) {
            return;
        }

        let sanitized = sanitize_identifier(&name);
        if !self.seen_functions.insert(sanitized.clone()) {
            return;
        }

        let params = parameters
            .into_iter()
            .map(|param| {
                NativeFunctionParameterNode::new(
                    sanitize_identifier(&param.name),
                    param.annotation.clone(),
                    1,
                    1,
                )
            })
            .collect();

        let function = NativeFunctionDeclNode::new(sanitized, params, return_type, is_async, 1, 1);

        self.items.push(NativeModuleItem::Function(function));
    }

    fn should_include(&self, name: &str) -> bool {
        let lowered = name.to_lowercase();
        if !self.options.include.is_empty() && !self.options.include.iter().any(|n| n == &lowered) {
            return false;
        }
        if self.options.exclude.iter().any(|n| n == &lowered) {
            return false;
        }
        true
    }
}

fn resolve_path(base_dir: &Path, path: &Path) -> PathBuf {
    if path.is_relative() {
        base_dir.join(path)
    } else {
        path.to_path_buf()
    }
}

fn resolve_module_path(source: &DeclarationSourceConfig, module_name: &str) -> Vec<String> {
    if let Some(module) = &source.module {
        return module_segments(module);
    }

    module_name
        .split('.')
        .map(|segment| sanitize_identifier(segment))
        .collect()
}

fn module_segments(path: &str) -> Vec<String> {
    path.split(|c| c == '/' || c == '.' || c == ':')
        .filter(|segment| !segment.is_empty())
        .map(|segment| sanitize_identifier(segment))
        .collect()
}

fn sanitize_identifier(name: &str) -> String {
    let mut result = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            result.push(c);
        } else if c == '-' || c == ' ' {
            result.push('_');
        }
    }
    if result.is_empty() {
        "symbol".into()
    } else {
        result
    }
}

fn build_inspection_script(module_name: &str, include: &[String], exclude: &[String]) -> String {
    let include_json = serde_json::to_string(include).expect("include serializable");
    let exclude_json = serde_json::to_string(exclude).expect("exclude serializable");
    format!(
        r#"
import importlib
import inspect
import json
import typing

module_name = {module}
module = importlib.import_module(module_name)

includes = {include}
excludes = {exclude}

def should_include(name: str) -> bool:
    lowered = name.lower()
    if includes and lowered not in includes:
        return False
    if lowered in excludes:
        return False
    return True

def annotation_to_string(annotation):
    if annotation is inspect._empty:
        return None
    if annotation is type(None):
        return "None"
    try:
        return annotation.__name__
    except AttributeError:
        if hasattr(annotation, "__qualname__"):
            return annotation.__qualname__
        return str(annotation)

def parameter_optional(param, hints):
    if param.default is not inspect._empty:
        return True
    annotation = hints.get(param.name)
    if annotation is None:
        return False
    origin = getattr(annotation, "__origin__", None)
    args = getattr(annotation, "__args__", ())
    if origin is typing.Union and type(None) in args:
        return True
    return False

members = []

for name in dir(module):
    if name.startswith("_"):
        continue
    attr = getattr(module, name)
    if inspect.isclass(attr):
        if should_include(name):
            members.append({{"kind": "class", "name": name}})
        continue
    if inspect.isfunction(attr):
        if not should_include(name):
            continue
        try:
            hints = typing.get_type_hints(attr)
        except Exception:
            hints = {{}}
        signature = inspect.signature(attr)
        params = []
        for param in signature.parameters.values():
            if param.kind in (inspect.Parameter.VAR_POSITIONAL, inspect.Parameter.VAR_KEYWORD):
                continue
            params.append({{
                "name": param.name,
                "annotation": annotation_to_string(hints.get(param.name, param.annotation)),
                "optional": parameter_optional(param, hints),
            }})
        members.append({{
            "kind": "function",
            "name": name,
            "is_async": inspect.iscoroutinefunction(attr),
            "parameters": params,
            "return_type": annotation_to_string(hints.get("return", signature.return_annotation)),
        }})

print(json.dumps(members))
"#,
        module = serde_json::to_string(module_name).expect("module name serializable"),
        include = include_json,
        exclude = exclude_json,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../framec_tests/fixtures/native_decl_generation/python")
    }

    fn project_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    #[test]
    fn imports_python_runtime_socket() {
        let config_dir = fixtures_dir();
        let source = DeclarationSourceConfig {
            adapter: "python".into(),
            input: PathBuf::from("../../frame_runtime_py/socket.py"),
            target: None,
            module: Some("runtime/socket".into()),
            options: Some(json!({
                "moduleName": "frame_runtime_py.socket",
                "pythonPath": [project_root()],
                "include": [
                    "framesocketclient",
                    "frame_socket_client_connect",
                    "frame_socket_client_read_line",
                    "frame_socket_client_write_line",
                    "frame_socket_client_close"
                ]
            })),
        };

        let context = DeclarationImportContext {
            config_dir,
            verbose: false,
        };

        let importer = PythonRuntimeImporter;
        let modules = importer.import(&source, &context).expect("import succeeds");
        assert_eq!(modules.len(), 1);
        let module = &modules[0];
        assert_eq!(module.path(), "runtime/socket");

        let mut types = Vec::new();
        let mut functions = Vec::new();
        for item in &module.items {
            match item {
                NativeModuleItem::Type(ty) => types.push(ty.name.clone()),
                NativeModuleItem::Function(func) => functions.push((
                    func.name.clone(),
                    func.is_async,
                    func.return_type.clone(),
                    func.parameters
                        .iter()
                        .map(|p| (p.name.clone(), p.type_annotation.clone()))
                        .collect::<Vec<_>>(),
                )),
            }
        }

        types.sort();
        functions.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(types, vec!["FrameSocketClient".to_string()]);

        let expected = vec![
            (
                "frame_socket_client_close".to_string(),
                false,
                Some("None".to_string()),
                vec![("client".to_string(), Some("FrameSocketClient".to_string()))],
            ),
            (
                "frame_socket_client_connect".to_string(),
                true,
                Some("FrameSocketClient".to_string()),
                vec![
                    ("host".to_string(), Some("str".to_string())),
                    ("port".to_string(), Some("int".to_string())),
                ],
            ),
            (
                "frame_socket_client_read_line".to_string(),
                true,
                Some("str".to_string()),
                vec![("client".to_string(), Some("FrameSocketClient".to_string()))],
            ),
            (
                "frame_socket_client_write_line".to_string(),
                true,
                Some("None".to_string()),
                vec![
                    ("client".to_string(), Some("FrameSocketClient".to_string())),
                    ("line".to_string(), Some("str".to_string())),
                ],
            ),
        ];

        assert_eq!(functions, expected);
    }
}
