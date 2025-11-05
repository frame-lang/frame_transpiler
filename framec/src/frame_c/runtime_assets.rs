use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FRAME_RUNTIME_PY_INIT: &str = include_str!("../../../frame_runtime_py/__init__.py");
const FRAME_RUNTIME_PY_SOCKET: &str = include_str!("../../../frame_runtime_py/socket.py");
const FRAME_RUNTIME_TS_INDEX: &str = include_str!("../../../frame_runtime_ts/index.ts");

/// Get the embedded TypeScript runtime source as a &str for inlining
pub fn typescript_runtime_source() -> &'static str {
    FRAME_RUNTIME_TS_INDEX
}

/// Ensure the shared Python runtime package is available in the provided
/// output directory. Returns the path to the emitted package.
pub fn emit_python_runtime_package(output_dir: &Path) -> io::Result<PathBuf> {
    let package_dir = output_dir.join("frame_runtime_py");
    fs::create_dir_all(&package_dir)?;
    let init_path = package_dir.join("__init__.py");
    let socket_path = package_dir.join("socket.py");

    let needs_write = match fs::read_to_string(&init_path) {
        Ok(existing) => existing != FRAME_RUNTIME_PY_INIT,
        Err(_) => true,
    };

    if needs_write {
        fs::write(&init_path, FRAME_RUNTIME_PY_INIT)?;
    }

    let socket_needs_write = match fs::read_to_string(&socket_path) {
        Ok(existing) => existing != FRAME_RUNTIME_PY_SOCKET,
        Err(_) => true,
    };

    if socket_needs_write {
        fs::write(&socket_path, FRAME_RUNTIME_PY_SOCKET)?;
    }

    Ok(package_dir)
}

/// Ensure the shared TypeScript runtime module is available in the provided
/// output directory. Returns the path to the emitted module directory.
pub fn emit_typescript_runtime_package(output_dir: &Path) -> io::Result<PathBuf> {
    let package_dir = output_dir.join("frame_runtime_ts");
    fs::create_dir_all(&package_dir)?;
    let index_path = package_dir.join("index.ts");

    let needs_write = match fs::read_to_string(&index_path) {
        Ok(existing) => existing != FRAME_RUNTIME_TS_INDEX,
        Err(_) => true,
    };

    if needs_write {
        fs::write(&index_path, FRAME_RUNTIME_TS_INDEX)?;
    }

    Ok(package_dir)
}
