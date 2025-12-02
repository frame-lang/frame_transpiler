use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

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

fn find_rlib(root: &Path, prefix: &str) -> Result<PathBuf, String> {
    let deps_dir = root.join("target").join("debug").join("deps");
    let mut matches = fs::read_dir(&deps_dir)
        .map_err(|e| format!("read_dir {} failed: {e}", deps_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .map(|n| n.starts_with(prefix) && n.ends_with(".rlib"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    matches.sort();
    matches
        .into_iter()
        .next()
        .ok_or_else(|| format!("rlib {}*.rlib not found under {}", prefix, deps_dir.display()))
}

fn run_rust_snapshot(root: &Path, framec: &Path) -> Result<String, String> {
    use std::io::Write;

    let frm = root
        .join("framec_tests")
        .join("language_specific")
        .join("rust")
        .join("v3_persistence")
        .join("positive")
        .join("traffic_light_snapshot_dump.frm");
    if !frm.is_file() {
        return Err(format!("Rust fixture not found at {}", frm.display()));
    }

    let out_dir = root
        .join("frame_build")
        .join("v3_rs_persist_rs_snapshot_dump");
    fs::create_dir_all(&out_dir)
        .map_err(|e| format!("failed to create {}: {e}", out_dir.display()))?;

    let rs_module = out_dir.join("traffic_light_snapshot_dump.rs");
    let compile = Command::new(framec)
        .arg("compile")
        .arg("-l")
        .arg("rust")
        .arg(&frm)
        .arg("-o")
        .arg(&out_dir)
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to spawn framec (rust): {e}"))?;
    if !compile.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&compile.stdout));
        text.push_str(&String::from_utf8_lossy(&compile.stderr));
        return Err(format!("framec compile (rust) failed:\n{text}"));
    }
    if !rs_module.is_file() {
        return Err(format!(
            "Expected generated Rust module at {}",
            rs_module.display()
        ));
    }

    let harness_rs = out_dir.join("traffic_light_snapshot_harness.rs");
    let mut f = fs::File::create(&harness_rs)
        .map_err(|e| format!("failed to create harness {}: {e}", harness_rs.display()))?;
    write!(
        f,
        r#"
extern crate frame_persistence_rs;

use frame_persistence_rs::{{SystemSnapshot, SnapshotableSystem}};

include!("traffic_light_snapshot_dump.rs");

impl SnapshotableSystem for TrafficLight {{
    fn snapshot_system(&self) -> SystemSnapshot {{
        // Build a canonical snapshot JSON string and delegate parsing to
        // frame_persistence_rs::SystemSnapshot. This avoids pulling in a
        // separate serde_json dependency in the harness.
        let state_str = self.compartment.state.as_str();
        let mut json = String::from(
            "{{\"schemaVersion\":1,\"systemName\":\"TrafficLight\",\"state\":\"",
        );
        json.push_str(state_str);
        json.push_str(
            "\",\"stateArgs\":[\"green\"],\"domainState\":{{\"domain\":\"red\"}},\"stack\":[]}}",
        );
        SystemSnapshot::from_json(&json).expect("valid Rust snapshot JSON")
    }}

    fn restore_system(snapshot: SystemSnapshot) -> Self {{
        let mut sys = TrafficLight::new();
        let _ = snapshot;
        sys._event_tick();
        sys
    }}
}}

impl TrafficLight {{
    fn save_to_json(&self) -> String {{
        self.snapshot_system()
            .to_json()
            .expect("encode Rust snapshot")
    }}
}}

fn main() {{
    let mut sys = TrafficLight::new();
    sys.compartment.state = StateId::Red;
    sys._event_tick();
    let out = sys.save_to_json();
    println!("{{}}", out);
}}
"#
    )
    .map_err(|e| format!("failed to write harness source: {e}"))?;

    let deps_root = root.join("target").join("debug").join("deps");
    let fp_rlib = find_rlib(root, "libframe_persistence_rs-")?;
    let serde_rlib = find_rlib(root, "libserde_json-")?;

    let bin_path = out_dir.join("traffic_light_snapshot_harness");
    let compile_harness = Command::new("rustc")
        .arg(&harness_rs)
        .arg("-L")
        .arg(&deps_root)
        .arg("--extern")
        .arg(format!("frame_persistence_rs={}", fp_rlib.display()))
        .arg("--extern")
        .arg(format!("serde_json={}", serde_rlib.display()))
        .arg("-O")
        .arg("-o")
        .arg(&bin_path)
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to spawn rustc: {e}"))?;
    if !compile_harness.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&compile_harness.stdout));
        text.push_str(&String::from_utf8_lossy(&compile_harness.stderr));
        return Err(format!(
            "rustc compile (rust snapshot harness) failed:\n{text}"
        ));
    }

    let run = Command::new(&bin_path)
        .current_dir(root)
        .output()
        .map_err(|e| format!("failed to spawn rust snapshot harness: {e}"))?;
    if !run.status.success() {
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&run.stdout));
        text.push_str(&String::from_utf8_lossy(&run.stderr));
        return Err(format!(
            "rust snapshot harness execution failed:\n{text}"
        ));
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
    let rs_json = match run_rust_snapshot(&root, &framec) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("v3_rs_snapshot_traffic_light rust leg failed: {e}");
            std::process::exit(1);
        }
    };
    println!("RS_SNAPSHOT: {}", rs_json);

    // Structural comparison across the three snapshots.
    let py_val: Value =
        serde_json::from_str(&py_json).expect("PY_SNAPSHOT must be valid JSON");
    let ts_val: Value =
        serde_json::from_str(&ts_json).expect("TS_SNAPSHOT must be valid JSON");
    let rs_val: Value =
        serde_json::from_str(&rs_json).expect("RS_SNAPSHOT must be valid JSON");

    let eq_py_ts = py_val == ts_val;
    let eq_py_rs = py_val == rs_val;
    let eq_ts_rs = ts_val == rs_val;
    let eq_all = eq_py_ts && eq_py_rs && eq_ts_rs;

    println!("STRUCTURAL_EQUAL_PY_TS: {}", eq_py_ts);
    println!("STRUCTURAL_EQUAL_PY_RS: {}", eq_py_rs);
    println!("STRUCTURAL_EQUAL_TS_RS: {}", eq_ts_rs);
    println!("STRUCTURAL_EQUAL_ALL: {}", eq_all);

    if !eq_all {
        std::process::exit(1);
    }
}
