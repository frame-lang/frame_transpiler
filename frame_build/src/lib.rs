//! This crate contains helper functions for compiling Frame as part of a Rust build process. It is
//! assumed that these functions will be called from a `build.rs` file. This module uses Frame as a
//! library, so `framec` does not need to be available on the path.

use anyhow::{Error, Result};
use framec::frame_c::compiler::Exe;
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;

// re-export `TargetLanguage` struct here since it's part of the `frame_build` interface
pub use framec::frame_c::compiler::TargetLanguage;

/// Configure the Frame build process. The methods associated with this struct are designed to be
/// chained to override the default configuration.
pub struct Config {
    frame_config: Option<PathBuf>,
    input_dir: PathBuf,
    output_dir: PathBuf,
    targets: Vec<TargetLanguage>,
    input_filter: Box<dyn Fn(&Path) -> bool>,
    max_depth: usize,
    min_depth: usize,
    follow_links: bool,
    continue_on_error: bool,
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
            frame_config: None,
            input_dir: PathBuf::from("src"),
            output_dir: PathBuf::from(env::var("OUT_DIR").unwrap()),
            targets: vec![TargetLanguage::Rust],
            input_filter: Box::new(|_| true),
            max_depth: ::std::usize::MAX,
            min_depth: 0,
            follow_links: false,
            continue_on_error: false,
        }
    }

    /// Set the path to a Frame `config.yaml` file to pass to Framec.
    ///
    /// If unset, Framec will look for the file in the current working directory.
    pub fn frame_config(mut self, path: &Path) -> Self {
        self.frame_config = Some(path.to_path_buf());
        self
    }

    /// Set the directory to traverse, searching for `.frm` files.
    ///
    /// If unset, [frame_build] will search the project's `src` directory.
    pub fn input_dir(mut self, path: &Path) -> Self {
        self.input_dir = path.to_path_buf();
        self
    }

    /// Set the root output directory to store generated files. Each generated file will be stored
    /// at a relative path within this directory corresponding to the file's location in the input
    /// directory.
    ///
    /// If unset, the build process will store generated files in the default output directory for
    /// a `build.rs` script, which is obtained from the `OUT_DIR` environment variable set by
    /// Cargo.
    ///
    /// If this library is used outside of the context of a `build.rs` script, this value should be
    /// changed.
    pub fn output_dir(mut self, path: &Path) -> Self {
        self.output_dir = path.to_path_buf();
        self
    }

    /// Set the list of target languages to compile each Frame file to using Framec.
    ///
    /// By default, the build process compiles each Frame file to only Rust.
    pub fn set_targets(mut self, targets: &[TargetLanguage]) -> Self {
        self.targets = targets.to_vec();
        self
    }

    /// Add an additional target language to compile to.
    ///
    /// By default, the build process compiles each Frame file to Rust. If Rust output is not wanted,
    /// use [Config::set_targets] to override the list of target languages rather than add to it.
    pub fn add_target(mut self, target: TargetLanguage) -> Self {
        self.targets.push(target);
        self
    }

    /// Set a function that filters the Frame files found in the input directory based on their
    /// paths. For the path of each Frame file found, if this function returns `true`, the file is
    /// compiled into all of the target languages. If the function returns `false`, it is skipped.
    pub fn include_only_if(mut self, filter: impl Fn(&Path) -> bool + 'static) -> Self {
        self.input_filter = Box::new(filter);
        self
    }

    /// Set a maximum depth to search for Frame files in the input directory. A max depth of `1`
    /// means to search only the immediate contents of the input directory. For greater depths,
    /// subdirectories are traversed in a depth-first order.
    ///
    /// By default, the max depth is effectively unbounded.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Set a minimum depth to search for Frame files in the input directory. A min depth of `1`
    /// would include the contents of the input directory, whereas a min depth of `2` would begin
    /// with the contents of the input directory's sub-directories.
    ///
    /// By default, the min depth is `0`.
    pub fn min_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// By default, the traversal of the input directory does not follow symbolic links. Calling
    /// this method sets a flag indicating that symbolic links *should* be followed.
    pub fn follow_links(mut self) -> Self {
        self.follow_links = true;
        self
    }

    /// By default, the build process halts if Framec panics or returns an error. Calling this
    /// method sets a flag that changes this behavior to instead print an error message to `stderr`
    /// and continue searching for and translating Frame files. This can be useful, for example,
    /// when some Frame files are known to not translate to all target languages, and the failing
    /// ones are not needed, or in test suites where some translations are expected to fail.
    ///
    /// Rather than setting this flag, consider also invoking [frame_build] multiple times with
    /// different target languages and different configurations of [Self::include_only_if].
    ///
    /// Non-Frame errors (e.g. file I/O errors) will halt the build regardless of this setting.
    pub fn continue_on_error(mut self) -> Self {
        self.continue_on_error = true;
        self
    }
}

/// Run Framec on potentially several Frame files as part of building a Rust package. This function
/// is intended to be called from a `build.rs` script. This function traverses a directory
/// searching for `.frm` files, translating each one into potentially several different target
/// languages.
///
/// The generated files are stored in an output directory at a relative position corresponding to
/// where the files were found in the input directory. For example, a Frame specification found at
/// `input_dir/a/b/sm.frm` might be translated into a Rust file `output_dir/a/b/sm.rs` and a
/// PlantUML file `output_dir/a/b/sm.puml`.
///
/// On success, this function returns a vector of paths to each of the generated files.
///
/// The build process is highly configurable. See the documentation for the [Config] struct.
pub fn frame_build(config: Config) -> Result<Vec<PathBuf>> {
    let mut generated_files = Vec::new();

    let walk_dir = WalkDir::new(&config.input_dir)
        .max_depth(config.max_depth)
        .min_depth(config.min_depth)
        .follow_links(config.follow_links);

    for entry in walk_dir {
        let entry = entry?;
        let input_path = entry.path();
        if input_path.extension().unwrap_or_default() == "frm" && (&config.input_filter)(input_path)
        {
            // tell Cargo this is a source file
            println!("cargo:rerun-if-changed={:?}", &input_path);

            let local_path = input_path.strip_prefix(&config.input_dir)?;
            let output_path = config.output_dir.join(local_path);
            fs::create_dir_all(output_path.parent().unwrap())?;

            for target in &config.targets {
                let mut target_output_path = output_path.clone();
                target_output_path.set_extension(target.file_extension());

                let frame_config = &config.frame_config;
                let framec_result = std::panic::catch_unwind(move || {
                    Exe::new().run_file(frame_config, input_path, Some(*target))
                });

                match framec_result {
                    Ok(Ok(output_content)) => {
                        // success, write the file
                        fs::write(&target_output_path, output_content)?;
                        generated_files.push(target_output_path);
                    }
                    Ok(Err(err)) => {
                        // framec returned an error
                        let msg = format!(
                            "Framec errored while generating {:?}: {:?}",
                            target_output_path, err
                        );
                        if config.continue_on_error {
                            eprintln!("{}", msg);
                        } else {
                            return Err(Error::msg(msg));
                        }
                    }
                    Err(err) => {
                        // framec panicked
                        let msg = format!(
                            "Framec panicked while generating {:?}: {:?}",
                            target_output_path, err
                        );
                        if config.continue_on_error {
                            eprintln!("{}", msg);
                        } else {
                            return Err(Error::msg(msg));
                        }
                    }
                }
            }
        }
    }

    Ok(generated_files)
}
