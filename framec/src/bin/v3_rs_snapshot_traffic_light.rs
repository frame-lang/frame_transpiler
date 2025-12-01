use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

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

fn main() {
    let root = repo_root();
    let framec = find_framec(&root);
    match run_python_snapshot(&root, &framec) {
        Ok(json) => {
            println!("PY_SNAPSHOT: {}", json);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("v3_rs_snapshot_traffic_light python leg failed: {e}");
            std::process::exit(1);
        }
    }
}
