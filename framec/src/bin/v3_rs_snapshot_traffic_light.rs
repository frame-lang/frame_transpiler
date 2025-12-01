use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

fn find_framec(root: &Path) -> PathBuf {
    if let Ok(override_bin) = std::env::var("FRAMEC_BIN") {
        return PathBuf::from(override_bin);
    }
    root.join("target").join("debug").join("framec")
}

fn find_tsc(root: &Path) -> PathBuf {
    let local_bin = root.join("node_modules").join(".bin").join("tsc");
    if local_bin.is_file() {
        return local_bin;
    }
    let local_direct = root
        .join("node_modules")
        .join("typescript")
        .join("bin")
        .join("tsc");
    if local_direct.is_file() {
        return local_direct;
    }
    PathBuf::from("tsc")
}

fn run_python_snapshot(root: &Path, framec: &Path) -> Result<String, String> {
    let frm = root
        .join("framec_tests")
        .join("language_specific")
        .join("python")
        .join("v3_persistence")
        .join("positive")
        .join("traffic_light_snapshot_dump.frm");
    if !frm.is_file() {
        return Err(format!("Python fixture not found at {}", frm.display()));
    }

    let out_dir = root
        .join("frame_build")
        .join("v3_rs_persist_py_snapshot_dump");
    fs::create_dir_all(&out_dir)
        .map_err(|e| format!("failed to create {}: {e}", out_dir.display()))?;

    let compile = Command::new(framec)
        .arg("compile")
        .arg("-l")
        .arg("python_3")
        .arg(&frm)
        .arg("-o")
        .arg(&out_dir)
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to spawn framec (python): {e}"))?;

    if !compile.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&compile.stdout));
        text.push_str(&String::from_utf8_lossy(&compile.stderr));
        return Err(format!("framec compile (python) failed:\n{text}"));
    }

    let py_path = fs::read_dir(&out_dir)
        .map_err(|e| format!("read_dir {} failed: {e}", out_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|s| s.to_str()) == Some("py"))
        .ok_or_else(|| "no .py generated for python snapshot dump".to_string())?;

    let module_name = py_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "invalid python module name".to_string())?;

    let root_str = root.to_string_lossy().to_string();
    let code = format!(
        "import sys; sys.path.insert(0, {root}); import {m}; {m}.main()",
        root = format!("{:?}", root_str),
        m = module_name
    );

    let run = Command::new("python3")
        .arg("-c")
        .arg(&code)
        .current_dir(&out_dir)
        .output()
        .map_err(|e| format!("failed to spawn python3: {e}"))?;

    if !run.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&run.stdout));
        text.push_str(&String::from_utf8_lossy(&run.stderr));
        return Err(format!("python snapshot runner failed:\n{text}"));
    }

    Ok(String::from_utf8_lossy(&run.stdout).trim().to_string())
}

fn run_typescript_snapshot(root: &Path, framec: &Path) -> Result<String, String> {
    let frm = root
        .join("framec_tests")
        .join("language_specific")
        .join("typescript")
        .join("v3_persistence")
        .join("positive")
        .join("traffic_light_snapshot_dump.frm");
    if !frm.is_file() {
        return Err(format!("TypeScript fixture not found at {}", frm.display()));
    }

    let out_dir = root
        .join("frame_build")
        .join("v3_rs_persist_ts_snapshot_dump");
    fs::create_dir_all(&out_dir)
        .map_err(|e| format!("failed to create {}: {e}", out_dir.display()))?;

    let compile = Command::new(framec)
        .arg("compile")
        .arg("-l")
        .arg("typescript")
        .arg(&frm)
        .arg("-o")
        .arg(&out_dir)
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to spawn framec (typescript): {e}"))?;

    if !compile.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&compile.stdout));
        text.push_str(&String::from_utf8_lossy(&compile.stderr));
        return Err(format!("framec compile (typescript) failed:\n{text}"));
    }

    let ts_path = fs::read_dir(&out_dir)
        .map_err(|e| format!("read_dir {} failed: {e}", out_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|s| s.to_str()) == Some("ts"))
        .ok_or_else(|| "no .ts generated for typescript snapshot dump".to_string())?;

    let tsc = find_tsc(root);
    let js_path = ts_path.with_extension("js");

    let cproc = Command::new(&tsc)
        .arg("--target")
        .arg("es5")
        .arg("--module")
        .arg("commonjs")
        .arg("--skipLibCheck")
        .arg("--lib")
        .arg("es2015,dom")
        .arg(&ts_path)
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to spawn tsc: {e}"))?;
    if !cproc.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&cproc.stdout));
        text.push_str(&String::from_utf8_lossy(&cproc.stderr));
        return Err(format!("tsc compilation failed for TS snapshot dump:\n{text}"));
    }

    let js_rel = js_path
        .strip_prefix(root)
        .unwrap_or(&js_path)
        .to_string_lossy()
        .to_string();

    let node_script = format!(
        r#"
const {{ TrafficLight }} = require("./{js_rel}");
const {{ snapshotSystem, snapshotToJson }} = require("./frame_persistence_ts");
const tl = new TrafficLight("red", "red", null);
tl.tick();
const snap = snapshotSystem(tl);
console.log(snapshotToJson(snap));
"#,
        js_rel = js_rel
    );

    // Ensure project-local modules are visible to Node.
    let mut env_map = std::env::vars().collect::<std::collections::HashMap<_, _>>();
    let root_str = root.to_string_lossy().to_string();
    env_map
        .entry("NODE_PATH".to_string())
        .and_modify(|v| *v = format!("{root}{sep}{v}", root = root_str, sep = std::path::MAIN_SEPARATOR))
        .or_insert(root_str);

    let run = Command::new("node")
        .arg("-e")
        .arg(&node_script)
        .current_dir(root)
        .envs(&env_map)
        .output()
        .map_err(|e| format!("failed to spawn node: {e}"))?;

    if !run.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&run.stdout));
        text.push_str(&String::from_utf8_lossy(&run.stderr));
        return Err(format!("node execution failed for TS snapshot dump:\n{text}"));
    }

    Ok(String::from_utf8_lossy(&run.stdout).trim().to_string())
}

fn main() {
    let root = repo_root();
    let framec = find_framec(&root);
    let py_json = match run_python_snapshot(&root, &framec) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("v3_rs_snapshot_traffic_light python leg failed: {e}");
            std::process::exit(1);
        }
    };
    println!("PY_SNAPSHOT: {}", py_json);

    let ts_json = match run_typescript_snapshot(&root, &framec) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("v3_rs_snapshot_traffic_light typescript leg failed: {e}");
            std::process::exit(1);
        }
    };
    println!("TS_SNAPSHOT: {}", ts_json);
    // Rust leg and structural comparison will be added in a subsequent step.
    std::process::exit(0);
}
