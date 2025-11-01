use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use serde_json::Value;

use crate::frame_c::ast::{
    NativeFunctionDeclNode, NativeFunctionParameterNode, NativeModuleDeclNode, NativeModuleItem,
    NativeTypeDeclNode,
};

use super::{DeclarationImportContext, DeclarationImporter, DeclarationSourceConfig};

#[derive(Debug)]
pub struct TypeScriptTypedocImporter;

impl DeclarationImporter for TypeScriptTypedocImporter {
    fn name(&self) -> &'static str {
        "typescript-typedoc"
    }

    fn import(
        &self,
        source: &DeclarationSourceConfig,
        context: &DeclarationImportContext,
    ) -> Result<Vec<NativeModuleDeclNode>, String> {
        let input_path = source.input_path(&context.config_dir);
        let mut options = TypeScriptImporterOptions::from_source(source)
            .map_err(|err| format!("Invalid TypeScript importer options: {}", err))?;
        options.resolve_paths(&context.config_dir, &input_path)?;

        if context.verbose {
            eprintln!("[decl import] running typedoc on {}", input_path.display());
        }

        let reflection = load_typedoc_reflection(&input_path, &options, context)?;
        let module_path = resolve_module_path(source, &input_path);

        let mut builder = ModuleBuilder::new(&options);
        if let Some(children) = reflection.get("children").and_then(|v| v.as_array()) {
            for child in children {
                builder.consume_top_level(child);
            }
        }

        if builder.items.is_empty() {
            if context.verbose {
                eprintln!(
                    "[decl import] no declarations discovered in {}",
                    input_path.display()
                );
            }
            return Ok(Vec::new());
        }

        let module = NativeModuleDeclNode::new(module_path, 1, 1, builder.items);
        Ok(vec![module])
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TypeScriptImporterOptions {
    #[serde(default)]
    tsconfig: Option<PathBuf>,
    #[serde(default)]
    typedoc_options: Option<PathBuf>,
    #[serde(default)]
    entry_points: Vec<PathBuf>,
    #[serde(default)]
    json_cache: Option<PathBuf>,
    #[serde(default)]
    extra_args: Vec<String>,
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

impl TypeScriptImporterOptions {
    fn from_source(source: &DeclarationSourceConfig) -> Result<Self, serde_json::Error> {
        match &source.options {
            Some(value) => serde_json::from_value(value.clone()),
            None => Ok(TypeScriptImporterOptions::default()),
        }
    }

    fn resolve_paths(&mut self, base_dir: &Path, input_path: &Path) -> Result<(), String> {
        if let Some(tsconfig) = &self.tsconfig {
            self.tsconfig = Some(resolve_path(base_dir, tsconfig));
        }
        if let Some(typedoc) = &self.typedoc_options {
            self.typedoc_options = Some(resolve_path(base_dir, typedoc));
        }
        if let Some(json_cache) = &self.json_cache {
            self.json_cache = Some(resolve_path(base_dir, json_cache));
        }
        if self.entry_points.is_empty() {
            self.entry_points.push(input_path.to_path_buf());
        } else {
            self.entry_points = self
                .entry_points
                .iter()
                .map(|p| resolve_path(base_dir, p))
                .collect();
        }
        self.include = self.include.iter().map(|s| s.to_lowercase()).collect();
        self.exclude = self.exclude.iter().map(|s| s.to_lowercase()).collect();
        Ok(())
    }

    fn symbol_included(&self, name: &str) -> bool {
        let lowered = name.to_lowercase();
        if !self.include.is_empty() && !self.include.iter().any(|n| n == &lowered) {
            return false;
        }
        !self.exclude.iter().any(|n| n == &lowered)
    }
}

struct ModuleBuilder<'a> {
    items: Vec<NativeModuleItem>,
    seen_types: HashSet<String>,
    seen_functions: HashSet<String>,
    options: &'a TypeScriptImporterOptions,
}

impl<'a> ModuleBuilder<'a> {
    fn new(options: &'a TypeScriptImporterOptions) -> Self {
        ModuleBuilder {
            items: Vec::new(),
            seen_types: HashSet::new(),
            seen_functions: HashSet::new(),
            options,
        }
    }

    fn consume_top_level(&mut self, node: &Value) {
        let Some(kind) = node_kind(node) else {
            return;
        };
        let name = node_name(node);
        if let Some(name_str) = name.as_deref() {
            if !self.options.symbol_included(name_str) {
                return;
            }
        }

        match kind {
            KIND_CLASS => self.consume_class(node),
            KIND_FUNCTION => self.consume_function(node, None),
            KIND_VARIABLE => self.consume_variable(node),
            KIND_INTERFACE => self.consume_interface(node),
            KIND_NAMESPACE | KIND_MODULE => {
                if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
                    for child in children {
                        self.consume_top_level(child);
                    }
                }
            }
            _ => {}
        }
    }

    fn consume_class(&mut self, node: &Value) {
        let class_name = match node_name(node) {
            Some(name) => name,
            None => return,
        };
        let type_name = sanitize_identifier(&class_name);
        if self.seen_types.insert(type_name.clone()) {
            self.items
                .push(NativeModuleItem::Type(NativeTypeDeclNode::new(
                    type_name.clone(),
                    None,
                    1,
                    1,
                )));
        }

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                if node_kind(child) == Some(KIND_METHOD) {
                    let is_static = child
                        .get("flags")
                        .and_then(|flags| flags.get("isStatic"))
                        .and_then(|flag| flag.as_bool())
                        .unwrap_or(false);
                    self.consume_function(
                        child,
                        Some(ClassContext {
                            class_name: class_name.clone(),
                            is_static,
                        }),
                    );
                }
            }
        }
    }

    fn consume_function(&mut self, node: &Value, class: Option<ClassContext>) {
        let name = match node_name(node) {
            Some(name) => name,
            None => return,
        };

        let signatures = node.get("signatures").and_then(|v| v.as_array());
        let signature = match signatures.and_then(|arr| arr.first()) {
            Some(sig) => sig,
            None => return,
        };

        let (return_type, is_async) = self.parse_return_type(signature.get("type"));
        let mut params = Vec::new();

        if let Some(class_ctx) = &class {
            if !class_ctx.is_static {
                params.push(NativeFunctionParameterNode::new(
                    "instance".to_string(),
                    Some(sanitize_identifier(&class_ctx.class_name)),
                    1,
                    1,
                ));
            }
        }

        if let Some(parameters) = signature.get("parameters").and_then(|v| v.as_array()) {
            for param in parameters {
                let param_name = param
                    .get("name")
                    .and_then(|n| n.as_str())
                    .map(|s| sanitize_identifier(s))
                    .unwrap_or_else(|| "arg".to_string());
                let mut type_annotation = param
                    .get("type")
                    .and_then(|t| self.type_to_string(t).ok())
                    .filter(|s| !s.is_empty());
                let is_optional = param
                    .get("flags")
                    .and_then(|flags| flags.get("isOptional"))
                    .and_then(|flag| flag.as_bool())
                    .unwrap_or(false);
                if is_optional {
                    type_annotation = Some(match type_annotation {
                        Some(inner) => format!("{} | undefined", inner),
                        None => "undefined".to_string(),
                    });
                }
                params.push(NativeFunctionParameterNode::new(
                    param_name,
                    type_annotation,
                    1,
                    1,
                ));
            }
        }

        let function_name = match &class {
            Some(class_ctx) => format!(
                "{}_{}",
                to_snake_case(&class_ctx.class_name),
                to_snake_case(&name)
            ),
            None => to_snake_case(&name),
        };

        if !self.seen_functions.insert(function_name.clone()) {
            return;
        }

        self.items
            .push(NativeModuleItem::Function(NativeFunctionDeclNode::new(
                function_name,
                params,
                return_type,
                is_async,
                1,
                1,
            )));
    }

    fn consume_variable(&mut self, node: &Value) {
        let name = match node_name(node) {
            Some(name) => name,
            None => return,
        };
        let sanitized = sanitize_identifier(&name);
        if !self.seen_types.insert(sanitized.clone()) {
            return;
        }

        let alias = node
            .get("type")
            .and_then(|t| self.type_to_string(t).ok())
            .filter(|s| !s.is_empty());

        self.items
            .push(NativeModuleItem::Type(NativeTypeDeclNode::new(
                sanitized, alias, 1, 1,
            )));
    }

    fn consume_interface(&mut self, node: &Value) {
        let name = match node_name(node) {
            Some(name) => name,
            None => return,
        };
        let sanitized = sanitize_identifier(&name);
        if !self.seen_types.insert(sanitized.clone()) {
            return;
        }

        let alias = node
            .get("type")
            .and_then(|t| self.type_to_string(t).ok())
            .or_else(|| {
                node.get("children").and_then(|children| {
                    let children_array = match children.as_array() {
                        Some(array) => array,
                        None => return None,
                    };
                    let mut fields = Vec::new();
                    for child in children_array {
                        if node_kind(child) == Some(KIND_PROPERTY) {
                            if let Some(field_type) = child.get("type") {
                                if let Ok(field_str) = self.type_to_string(field_type) {
                                    let field_name = node_name(child).unwrap_or_default();
                                    fields.push(format!(
                                        "{}: {}",
                                        sanitize_identifier(&field_name),
                                        field_str
                                    ));
                                }
                            }
                        }
                    }
                    if fields.is_empty() {
                        None
                    } else {
                        Some(format!("{{ {} }}", fields.join(", ")))
                    }
                })
            });

        self.items
            .push(NativeModuleItem::Type(NativeTypeDeclNode::new(
                sanitized, alias, 1, 1,
            )));
    }

    fn ensure_type_alias(&mut self, name: &str, aliased_type: Option<&str>) {
        let sanitized = sanitize_identifier(name);
        if self.seen_types.insert(sanitized.clone()) {
            self.items
                .push(NativeModuleItem::Type(NativeTypeDeclNode::new(
                    sanitized,
                    aliased_type.map(|s| s.to_string()),
                    1,
                    1,
                )));
        }
    }

    fn parse_return_type(&mut self, value: Option<&Value>) -> (Option<String>, bool) {
        let Some(type_value) = value else {
            return (None, false);
        };

        if let Some(obj) = type_value.as_object() {
            if obj.get("type") == Some(&Value::String("reference".into())) {
                if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                    if name == "Promise" {
                        let inner = obj
                            .get("typeArguments")
                            .and_then(|args| args.as_array())
                            .and_then(|arr| arr.first())
                            .and_then(|inner| self.type_to_string(inner).ok());
                        return (inner, true);
                    }
                }
            }
        }

        (self.type_to_string(type_value).ok(), false)
    }

    fn type_to_string(&mut self, value: &Value) -> Result<String, String> {
        let type_name = value
            .get("type")
            .and_then(|t| t.as_str())
            .ok_or_else(|| "Type node missing 'type' field".to_string())?;

        match type_name {
            "intrinsic" => Ok(value
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("any")
                .to_string()),
            "reference" => {
                let name = value.get("name").and_then(|n| n.as_str());
                let target = value.get("target");

                if let Some(mapped) = self.map_external_reference(name, target) {
                    return Ok(mapped);
                }

                let mut base = name
                    .or_else(|| {
                        target
                            .and_then(|target| target.get("qualifiedName"))
                            .and_then(|qualified| qualified.as_str())
                    })
                    .unwrap_or("any")
                    .to_string();

                if let Some(args) = value.get("typeArguments").and_then(|v| v.as_array()) {
                    if !args.is_empty() {
                        let rendered: Result<Vec<_>, _> =
                            args.iter().map(|arg| self.type_to_string(arg)).collect();
                        base.push('<');
                        base.push_str(&rendered?.join(", "));
                        base.push('>');
                    }
                }

                Ok(base)
            }
            "array" => {
                let element = value
                    .get("elementType")
                    .ok_or_else(|| "Array type missing elementType".to_string())?;
                let rendered = self.type_to_string(element)?;
                Ok(format!("{}[]", rendered))
            }
            "union" => {
                let elements = value
                    .get("types")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Union missing types array".to_string())?;
                let rendered: Result<Vec<_>, _> = elements
                    .iter()
                    .map(|element| self.type_to_string(element))
                    .collect();
                Ok(rendered?.join(" | "))
            }
            "intersection" => {
                let elements = value
                    .get("types")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Intersection missing types array".to_string())?;
                let rendered: Result<Vec<_>, _> = elements
                    .iter()
                    .map(|element| self.type_to_string(element))
                    .collect();
                Ok(rendered?.join(" & "))
            }
            "literal" | "stringLiteral" => {
                if let Some(value_str) = value.get("value") {
                    Ok(value_str.to_string())
                } else {
                    Ok("null".to_string())
                }
            }
            "reflection" => {
                if let Some(declaration) = value.get("declaration") {
                    if let Some(children) = declaration.get("children").and_then(|v| v.as_array()) {
                        let mut fields = Vec::new();
                        for child in children {
                            if node_kind(child) == Some(KIND_PROPERTY) {
                                if let Some(field_type) = child.get("type") {
                                    if let Ok(field_rendered) = self.type_to_string(field_type) {
                                        let field_name = node_name(child).unwrap_or_default();
                                        fields.push(format!(
                                            "{}: {}",
                                            sanitize_identifier(&field_name),
                                            field_rendered
                                        ));
                                    }
                                }
                            }
                        }
                        if !fields.is_empty() {
                            return Ok(format!("{{ {} }}", fields.join(", ")));
                        }
                    }
                }
                Ok("object".to_string())
            }
            "tuple" => {
                let elements = value
                    .get("elements")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Tuple missing elements".to_string())?;
                let rendered: Result<Vec<_>, _> = elements
                    .iter()
                    .map(|element| self.type_to_string(element))
                    .collect();
                Ok(format!("[{}]", rendered?.join(", ")))
            }
            other => Err(format!("Unsupported TypeDoc type '{}'", other)),
        }
    }

    fn map_external_reference(
        &mut self,
        name: Option<&str>,
        target: Option<&Value>,
    ) -> Option<String> {
        let qualified = target
            .and_then(|t| t.get("qualifiedName"))
            .and_then(|q| q.as_str());

        for mapping in NODE_REFERENCE_MAPPINGS {
            let qualified_match = qualified == Some(mapping.qualified);
            let name_match = name == mapping.name;

            if qualified_match || name_match {
                self.ensure_type_alias(mapping.alias, Some(mapping.aliased_type));
                return Some(mapping.alias.to_string());
            }
        }

        None
    }
}

#[derive(Clone)]
struct ClassContext {
    class_name: String,
    is_static: bool,
}

fn resolve_path(base_dir: &Path, path: &Path) -> PathBuf {
    if path.is_relative() {
        base_dir.join(path)
    } else {
        path.to_path_buf()
    }
}

fn resolve_module_path(source: &DeclarationSourceConfig, input_path: &Path) -> Vec<String> {
    if let Some(module) = &source.module {
        return module_segments(module);
    }

    if let Some(stem) = input_path.file_stem().and_then(|s| s.to_str()) {
        if stem == "index" {
            if let Some(parent) = input_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
            {
                return vec![sanitize_identifier(parent)];
            }
        }
        return vec![sanitize_identifier(stem)];
    }

    vec!["typescript_runtime".to_string()]
}

fn load_typedoc_reflection(
    input_path: &Path,
    options: &TypeScriptImporterOptions,
    _context: &DeclarationImportContext,
) -> Result<Value, String> {
    if let Some(cache) = &options.json_cache {
        if cache.exists() {
            let contents = fs::read_to_string(cache).map_err(|err| {
                format!(
                    "Unable to read cached TypeDoc JSON '{}': {}",
                    cache.display(),
                    err
                )
            })?;
            return serde_json::from_str(&contents)
                .map_err(|err| format!("Invalid TypeDoc JSON in '{}': {}", cache.display(), err));
        }
    }

    let json_output = options
        .json_cache
        .clone()
        .unwrap_or_else(|| temp_json_path());

    let mut cmd = Command::new("npx");
    cmd.arg("typedoc");

    if let Some(typedoc_options) = &options.typedoc_options {
        cmd.arg("--options").arg(typedoc_options);
    }
    if let Some(tsconfig) = &options.tsconfig {
        cmd.arg("--tsconfig").arg(tsconfig);
    }

    cmd.arg("--json").arg(&json_output);

    for extra in &options.extra_args {
        cmd.arg(extra);
    }

    for entry in &options.entry_points {
        cmd.arg(entry);
    }

    let output = cmd.output().map_err(|err| {
        format!(
            "Failed to execute TypeDoc for '{}': {}",
            input_path.display(),
            err
        )
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "TypeDoc invocation failed for '{}': {}",
            input_path.display(),
            stderr
        ));
    }

    let contents = fs::read_to_string(&json_output).map_err(|err| {
        format!(
            "TypeDoc did not produce JSON at '{}': {}",
            json_output.display(),
            err
        )
    })?;

    if options.json_cache.is_none() {
        let _ = fs::remove_file(&json_output);
    }

    serde_json::from_str(&contents).map_err(|err| {
        format!(
            "Unable to parse TypeDoc output for '{}': {}",
            input_path.display(),
            err
        )
    })
}

fn temp_json_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    std::env::temp_dir().join(format!(
        "framec_typedoc_{}_{}.json",
        std::process::id(),
        timestamp
    ))
}

const KIND_MODULE: u32 = 2;
const KIND_NAMESPACE: u32 = 4;
const KIND_VARIABLE: u32 = 32;
const KIND_FUNCTION: u32 = 64;
const KIND_CLASS: u32 = 128;
const KIND_INTERFACE: u32 = 256;
const KIND_METHOD: u32 = 2048;
const KIND_PROPERTY: u32 = 1024;

fn node_kind(node: &Value) -> Option<u32> {
    node.get("kind").and_then(|k| k.as_u64()).map(|k| k as u32)
}

fn node_name(node: &Value) -> Option<String> {
    node.get("name")
        .and_then(|n| n.as_str())
        .map(|s| s.to_string())
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

fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut prev_lowercase = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() {
                if prev_lowercase {
                    result.push('_');
                }
                result.extend(ch.to_ascii_lowercase().to_string().chars());
                prev_lowercase = false;
            } else {
                prev_lowercase = true;
                result.push(ch);
            }
        } else if !result.ends_with('_') {
            result.push('_');
            prev_lowercase = false;
        }
    }
    result.trim_matches('_').to_string()
}

struct NodeReferenceMapping {
    qualified: &'static str,
    name: Option<&'static str>,
    alias: &'static str,
    aliased_type: &'static str,
}

const NODE_REFERENCE_MAPPINGS: &[NodeReferenceMapping] = &[
    NodeReferenceMapping {
        qualified: "net.Socket",
        name: Some("Socket"),
        alias: "NodeSocketHandle",
        aliased_type: "any",
    },
    NodeReferenceMapping {
        qualified: "fs.promises.FileHandle",
        name: Some("FileHandle"),
        alias: "NodeFileHandle",
        aliased_type: "any",
    },
    NodeReferenceMapping {
        qualified: "Buffer",
        name: Some("Buffer"),
        alias: "NodeBuffer",
        aliased_type: "any",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::PathBuf;

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../framec_tests/fixtures/native_decl_generation/typescript")
    }

    #[test]
    fn imports_typedoc_json_from_cache() {
        let fixtures = fixtures_dir();
        let json_cache = fixtures.join("typedoc_runtime_socket.json");

        let source = DeclarationSourceConfig {
            adapter: "typescript".into(),
            input: PathBuf::from("frame_runtime_ts/index.ts"),
            target: None,
            module: Some("runtime/socket".into()),
            options: Some(json!({
                "jsonCache": json_cache,
                "include": [
                    "framesocketclient",
                    "frame_socket_client_connect",
                    "frame_socket_client_read_line",
                    "frame_socket_client_write_line",
                    "frame_socket_client_close",
                    "frame_socket_client_from_node_socket",
                    "frame_file_handle"
                ]
            })),
        };

        let context = DeclarationImportContext {
            config_dir: fixtures,
            verbose: false,
        };

        let importer = TypeScriptTypedocImporter;
        let modules = importer.import(&source, &context).expect("import succeeds");

        assert_eq!(modules.len(), 1);
        let module = &modules[0];
        assert_eq!(module.path(), "runtime/socket");
        let mut type_names = Vec::new();
        let mut function_names = Vec::new();

        for item in &module.items {
            match item {
                NativeModuleItem::Type(type_decl) => {
                    type_names.push((type_decl.name.clone(), type_decl.aliased_type.clone()));
                }
                NativeModuleItem::Function(func) => {
                    function_names.push(func.name.clone());
                }
            }
        }

        type_names.sort();
        function_names.sort();

        assert_eq!(
            type_names,
            vec![
                (
                    "FRAME_FILE_HANDLE".to_string(),
                    Some("NodeFileHandle".to_string())
                ),
                ("FrameSocketClient".to_string(), None),
                ("NodeFileHandle".to_string(), Some("any".to_string())),
                ("NodeSocketHandle".to_string(), Some("any".to_string()))
            ]
        );

        assert_eq!(
            function_names,
            vec![
                "frame_socket_client_close".to_string(),
                "frame_socket_client_connect".to_string(),
                "frame_socket_client_from_node_socket".to_string(),
                "frame_socket_client_read_line".to_string(),
                "frame_socket_client_write_line".to_string()
            ]
        );
    }
}
