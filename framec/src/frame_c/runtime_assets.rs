use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FRAME_RUNTIME_PY_INIT: &str = include_str!("../../../frame_runtime_py/__init__.py");

/// Ensure the shared Python runtime package is available in the provided
/// output directory. Returns the path to the emitted package.
pub fn emit_python_runtime_package(output_dir: &Path) -> io::Result<PathBuf> {
    let package_dir = output_dir.join("frame_runtime_py");
    fs::create_dir_all(&package_dir)?;
    let init_path = package_dir.join("__init__.py");

    let needs_write = match fs::read_to_string(&init_path) {
        Ok(existing) => existing != FRAME_RUNTIME_PY_INIT,
        Err(_) => true,
    };

    if needs_write {
        fs::write(&init_path, FRAME_RUNTIME_PY_INIT)?;
    }

    Ok(package_dir)
}
