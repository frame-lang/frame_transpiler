//! This crate contains helper functions for compiling Frame as part of a Rust build process. It is
//! assumed that these functions will be called from a `build.rs` file. This module uses Frame as a
//! library, so `framec` does not need to be available on the path.

use anyhow::Result;
use framec::frame_c::compiler::Exe;
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;

// re-export `TargetLanguage` struct here since it's part of the `frame_build` interface
pub use framec::frame_c::compiler::TargetLanguage;

/// Configure the Frame build process.
pub struct Config {
    /// The output directory to store generated files.
    ///
    /// Default value is the default output directory for a `build.rs` script, which is obtained
    /// from the `OUT_DIR` environment variable set by Cargo. If this library is used outside of
    /// the context of a `build.rs` script, this value should be changed.
    pub output_dir: PathBuf,

    /// When looking for Frame files in the input directory, this is the maximum depth, where a
    /// a max depth of `Some(1)` means to search only the immediate contents of the input
    /// directory. For greater depths, subdirectories are traversed in a depth-first order.
    ///
    /// Default value is `None`, indicating that the entire tree of subdirectories will be
    /// searched.
    pub max_depth: Option<usize>,

    /// Whether or not to follow symbolic links when traversing subdirectories.
    ///
    /// Default value is `false`.
    pub follow_links: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    /// Construct a new default configuration.
    pub fn new() -> Self {
        Config {
            output_dir: PathBuf::from(env::var("OUT_DIR").unwrap()),
            max_depth: None,
            follow_links: false,
        }
    }

    /// Set the output directory. This method can be chained with the constructor to override the
    /// default configuration.
    pub fn output_dir(mut self, dir: &Path) -> Self {
        self.output_dir = dir.to_path_buf();
        self
    }

    /// Set a maximum depth of traversal. This method can be chained with the constructor to
    /// override the default configuration.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Set the flag indicating whether to follow symbolic links. This method can be chained with
    /// the constructor to override the default configuration.
    pub fn follow_links(mut self, follow: bool) -> Self {
        self.follow_links = follow;
        self
    }
}

/// Run Framec on potentially several Frame files as part of building a Rust package. Intended to
/// be called from a `build.rs` script. This function traverses a directory searching for `.frm`
/// files, translating each one into potentially several different target languages.
///
/// On success, returns a vector of paths to each of the files generated as a result of this
/// process.
///
/// # Arguments
/// * `frame_config` - Optional path to a `config.yaml` file to pass to Framec. If `None`, Framec
///   will look for the file in the current working directory.
/// * `build_config` - Used to configure aspects of this build function. See the documentation of
///   the [Config] struct for options and their default values.
/// * `input_dir` - The directory to search for `.frm` files.
/// * `targets` - The target languages to compile each `.frm` file to using Framec.
pub fn frame_build(
    frame_config: Option<PathBuf>,
    build_config: Config,
    input_dir: PathBuf,
    targets: Vec<TargetLanguage>,
) -> Result<Vec<PathBuf>> {
    let mut generated_files = Vec::new();
    let exe = Exe::new();

    for entry in WalkDir::new(&input_dir) {
        let entry = entry?;
        let input_path = entry.path();
        if input_path.extension().unwrap_or_default() == "frm" {
            println!("cargo:rerun-if-changed={:?}", &input_path);

            let local_path = input_path.strip_prefix(&input_dir)?;
            let output_path = build_config.output_dir.join(local_path);
            fs::create_dir_all(output_path.parent().unwrap())?;

            for target in &targets {
                let mut target_output_path = output_path.clone();
                target_output_path.set_extension(target.file_extension());
                let output_content = exe.run_file(&frame_config, input_path, Some(*target))?;
                fs::write(&target_output_path, output_content)?;
                generated_files.push(target_output_path);
            }
        }
    }

    Ok(generated_files)
}
