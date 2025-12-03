use crate::frame_c::compiler::{Exe, TargetLanguage};
use crate::frame_c::config::FrameConfig;
use clap::{Arg, Command};
use std::convert::TryFrom;
use std::path::PathBuf;

pub struct Cli {
    stdin_flag: bool,
    path: Option<PathBuf>,
    language: Option<String>,
    multifile: bool,
    output_dir: Option<PathBuf>,
    debug_output: bool,
    /// Validate-only mode
    validate_only: bool,
    /// Validate (structural) and continue
    validate: bool,
    /// Enable strict/native validation (facade mode)
    validate_native: bool,
    /// Emit debug trailers (errors-json, frame-map, visitor-map, debug-manifest)
    emit_debug: bool,
    command: CliCommand,
}

#[derive(Debug, Clone)]
pub enum CliCommand {
    None,
    Init,
    CompileProject { language: String, dir: PathBuf, output_dir: PathBuf, recursive: bool },
    Compile { language: String, file: PathBuf },
    ProjectBuild { language: String, output_dir: PathBuf, recursive: bool },
    FidImport { target: String, input: PathBuf, cache_root: Option<PathBuf> },
    Test {
        language: String,
        category: String,
        framec_bin: Option<PathBuf>,
        compare_python: bool,
        exec_smoke: bool,
        exec_curated: bool,
    },
}

impl Cli {
    pub fn new() -> Cli {
        let matches = Command::new("framec")
            .version(env!("FRAME_VERSION"))
            .about("Frame language transpiler (V3 architecture rebuild in progress)")
            .subcommand_required(false)
            .arg_required_else_help(false)
            .subcommand_precedence_over_arg(true)
            .subcommand(Command::new("init").about("Initialize a new Frame project with frame.toml").arg(Arg::new("name").help("Project name").value_name("NAME").index(1)))
            .subcommand(
                Command::new("project")
                    .about("Project-level operations (optional; PRT-first)")
                    .subcommand(
                        Command::new("build")
                            .about("Build a Frame project using frame.toml")
                            .arg(Arg::new("language").long("language").short('l').value_name("LANG").required(true))
                            .arg(Arg::new("output-dir").long("output-dir").short('o').value_name("DIR").required(true))
                            .arg(Arg::new("recursive").long("recursive").short('r').action(clap::ArgAction::SetTrue))
                    )
            )
            .subcommand(
                Command::new("fid")
                    .about("FID cache operations (optional; PRT-only Phase A)")
                    .subcommand(
                        Command::new("import")
                            .about("Import a FID JSON file into the cache")
                            .arg(Arg::new("target").long("target").short('t').value_name("TARGET").required(true))
                            .arg(Arg::new("cache-root").long("cache-root").value_name("DIR"))
                            .arg(Arg::new("input").value_name("FID_JSON").required(true))
                    )
            )
            .subcommand(
                Command::new("compile")
                    .about("Compile a full V3 module file (non-demo)")
                    .arg(Arg::new("language").long("language").short('l').value_name("LANG").required(true))
                    .arg(Arg::new("file").value_name("FILE").required(true))
            )
            .subcommand(
                Command::new("compile-project")
                    .about("Compile all V3 module files in a directory (non-demo)")
                    .arg(Arg::new("language").long("language").short('l').value_name("LANG").required(true))
                    .arg(Arg::new("output-dir").long("output-dir").short('o').value_name("DIR").required(true))
                    .arg(Arg::new("recursive").long("recursive").short('r').action(clap::ArgAction::SetTrue))
                    .arg(Arg::new("dir").value_name("DIR").required(true))
            )
            .subcommand(
                Command::new("test")
                    .about("Run V3 PRT tests via the Rust harness (validation / exec prototype)")
                    .arg(Arg::new("language").long("language").short('l').value_name("LANG").required(true))
                    .arg(Arg::new("category").long("category").short('c').value_name("CATEGORY").required(true))
                    .arg(Arg::new("framec-bin").long("framec-bin").value_name("PATH"))
                    .arg(
                        Arg::new("compare-python")
                            .long("compare-python")
                            .help("Also run the Python V3 runner for this slice and compare validation results")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("exec-smoke")
                            .long("exec-smoke")
                            .help("Run exec-smoke harness for this slice instead of validation-only")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("exec-curated")
                            .long("exec-curated")
                            .help("Run curated exec harness for this slice instead of validation-only")
                            .action(clap::ArgAction::SetTrue),
                    )
            )
            .arg(Arg::new("FILE-PATH").help("File path").value_name("FILE").index(1))
            .arg(Arg::new("language").value_name("LANG").long("language").short('l').help("Target language (python_3, typescript, graphviz, llvm)").num_args(1))
            .arg(Arg::new("multifile").long("multifile").short('m').help("Enable multi-file project compilation").action(clap::ArgAction::SetTrue))
            .arg(Arg::new("output-dir").long("output-dir").short('o').help("Output directory for generated files (compile/multi-file)").value_name("DIR").num_args(1).global(true))
            .arg(Arg::new("debug-output").long("debug-output").help("Generate JSON output with transpiled code and source map").action(clap::ArgAction::SetTrue).global(true))
            .arg(Arg::new("validate").long("validate").help("Run V3 validation before transpile").action(clap::ArgAction::SetTrue).global(true))
            .arg(Arg::new("validate-syntax").long("validate-syntax").help("Alias for --validate (compat) ").action(clap::ArgAction::SetTrue).global(true))
            .arg(Arg::new("validation-only").long("validation-only").help("Run validation only and exit with status").action(clap::ArgAction::SetTrue).global(true))
            .arg(Arg::new("validation-level").long("validation-level").help("Validation level (compat)").num_args(1).global(true))
            .arg(Arg::new("validate-native").long("validate-native").help("Enable strict/native validation (facade mode)").action(clap::ArgAction::SetTrue).global(true))
            .arg(Arg::new("validation-format").long("validation-format").help("Validation output format (compat)").num_args(1).global(true))
            .arg(Arg::new("emit-debug").long("emit-debug").help("Emit debug trailers: errors-json, frame-map, visitor-map (module), debug-manifest").action(clap::ArgAction::SetTrue).global(true))
            
            .get_matches();

        let mut has_subcommand = false;
        let command = match matches.subcommand() {
            Some((name, sub)) => {
                has_subcommand = true;
                match name {
                    "init" => CliCommand::Init,
                    "compile-project" => {
                        let lang = sub.get_one::<String>("language").expect("language required").to_string();
                        let dir = sub.get_one::<String>("dir").map(|s| PathBuf::from(s)).expect("dir required");
                        let out = sub.get_one::<String>("output-dir").map(|s| PathBuf::from(s)).expect("output-dir required");
                        let recursive = sub.get_flag("recursive");
                        CliCommand::CompileProject { language: lang, dir, output_dir: out, recursive }
                    }
                    "compile" => {
                        let lang = sub.get_one::<String>("language").expect("language required").to_string();
                        let file = sub.get_one::<String>("file").map(|s| PathBuf::from(s)).expect("file required");
                        CliCommand::Compile { language: lang, file }
                    }
                    "project" => {
                        match sub.subcommand() {
                            Some(("build", sb)) => {
                                let lang = sb.get_one::<String>("language").expect("language required").to_string();
                                let out = sb.get_one::<String>("output-dir").map(|s| PathBuf::from(s)).expect("output-dir required");
                                let recursive = sb.get_flag("recursive");
                                CliCommand::ProjectBuild { language: lang, output_dir: out, recursive }
                            }
                            _ => CliCommand::None,
                        }
                    }
                    "fid" => {
                        match sub.subcommand() {
                            Some(("import", sb)) => {
                                let target = sb.get_one::<String>("target").expect("target required").to_string();
                                let input = sb.get_one::<String>("input").map(|s| PathBuf::from(s)).expect("FID_JSON required");
                                let cache_root = sb.get_one::<String>("cache-root").map(|s| PathBuf::from(s));
                                CliCommand::FidImport { target, input, cache_root }
                            }
                            _ => CliCommand::None,
                        }
                    }
                    "test" => {
                        let lang = sub
                            .get_one::<String>("language")
                            .expect("language required")
                            .to_string();
                        let cat = sub
                            .get_one::<String>("category")
                            .expect("category required")
                            .to_string();
                        let framec_bin = sub
                            .get_one::<String>("framec-bin")
                            .map(|s| PathBuf::from(s));
                        let compare_python = sub.get_flag("compare-python");
                        let exec_smoke = sub.get_flag("exec-smoke");
                        let exec_curated = sub.get_flag("exec-curated");
                        if compare_python && (exec_smoke || exec_curated) {
                            eprintln!("--compare-python is only supported in validation mode (no --exec-* flags)");
                            std::process::exit(exitcode::USAGE);
                        }
                        if exec_smoke && exec_curated {
                            eprintln!("--exec-smoke and --exec-curated are mutually exclusive");
                            std::process::exit(exitcode::USAGE);
                        }
                        CliCommand::Test {
                            language: lang,
                            category: cat,
                            framec_bin,
                            compare_python,
                            exec_smoke,
                            exec_curated,
                        }
                    }
                    _ => CliCommand::None,
                }
            }
            None => CliCommand::None,
        };

        let mut stdin = false;
        let mut path_opt = None;
        if !has_subcommand && matches.contains_id("FILE-PATH") {
            let file_path = matches.get_one::<String>("FILE-PATH");
            path_opt = file_path.map(|file_path| PathBuf::from(file_path.clone()));
        } else if !has_subcommand {
            stdin = true;
        }

        let language_opt = matches.get_one::<String>("language").map(|s| s.clone());
        let multifile = matches.get_flag("multifile");
        let output_dir_opt = matches.get_one::<String>("output-dir").map(|s| PathBuf::from(s.clone()));
        let debug_output = matches.get_flag("debug-output");
        let validate_only = matches.get_flag("validation-only");
        let validate = matches.get_flag("validate") || matches.get_flag("validate-syntax");
        let validate_native = matches.get_flag("validate-native");
        let emit_debug = matches.get_flag("emit-debug");

        Cli {
            stdin_flag: stdin,
            path: path_opt,
            language: language_opt,
            multifile,
            output_dir: output_dir_opt,
            debug_output,
            validate_only,
            validate,
            validate_native,
            emit_debug,
            command,
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}

pub fn run() {
    run_with(Cli::new());
}

pub fn run_with(args: Cli) {
    match args.command {
        CliCommand::Init => {
            handle_init_command();
            return;
        }
        CliCommand::Test {
            language,
            category,
            framec_bin,
            compare_python,
            exec_smoke,
            exec_curated,
        } => {
            // Minimal Rust-based V3 test entry point:
            // - validation-only for a single `<language>/<category>` pair
            // - optional `--compare-python` to run the Python runner for the
            //   same slice and compare validation outcomes.
            // - `--exec-smoke` / `--exec-curated` to run exec harnesses instead.
            let harness_lang = match language.as_str() {
                "python_3" | "py" | "python" => "python",
                "typescript" | "ts" => "typescript",
                "rust" | "rs" => "rust",
                other => other,
            }
            .to_string();
            let framec_path = framec_bin.unwrap_or_else(|| {
                std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("framec"))
            });
            let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .to_path_buf();

            // Exec-smoke mode
            if exec_smoke {
                let summary = match harness_lang.as_str() {
                    "rust" => crate::frame_c::v3::test_harness_rs::run_rust_exec_smoke(
                        &repo_root,
                        &framec_path,
                        &category,
                    ),
                    "python" => crate::frame_c::v3::test_harness_rs::run_python_exec_smoke(
                        &repo_root,
                        &framec_path,
                        &category,
                    ),
                    "typescript" => {
                        crate::frame_c::v3::test_harness_rs::run_typescript_exec_smoke(
                            &repo_root,
                            &framec_path,
                            &category,
                        )
                    }
                    other => {
                        eprintln!(
                            "framec test --exec-smoke currently supports languages: python_3, typescript, rust (got: {})",
                            other
                        );
                        std::process::exit(exitcode::USAGE);
                    }
                }
                .unwrap_or_else(|e| {
                    eprintln!("framec test exec-smoke error: {}", e);
                    std::process::exit(exitcode::IOERR);
                });

                println!(
                    "framec test exec-smoke: language={} category={} passed={} failed={}",
                    summary.language, summary.category, summary.passed, summary.failed
                );
                if summary.failed > 0 {
                    std::process::exit(exitcode::DATAERR);
                } else {
                    std::process::exit(0);
                }
            }

            // Exec-curated mode
            if exec_curated {
                let summary = match harness_lang.as_str() {
                    "rust" => crate::frame_c::v3::test_harness_rs::run_rust_curated_exec_for_category(
                        &repo_root,
                        &framec_path,
                        &category,
                    ),
                    "python" => crate::frame_c::v3::test_harness_rs::run_python_curated_exec_for_category(
                        &repo_root,
                        &framec_path,
                        &category,
                    ),
                    "typescript" => crate::frame_c::v3::test_harness_rs::run_typescript_curated_exec_for_category(
                        &repo_root,
                        &framec_path,
                        &category,
                    ),
                    other => {
                        eprintln!(
                            "framec test --exec-curated currently supports languages: python_3, typescript, rust (got: {})",
                            other
                        );
                        std::process::exit(exitcode::USAGE);
                    }
                }
                .unwrap_or_else(|e| {
                    eprintln!("framec test exec-curated error: {}", e);
                    std::process::exit(exitcode::IOERR);
                });

                println!(
                    "framec test exec-curated: language={} category={} passed={} failed={}",
                    summary.language, summary.category, summary.passed, summary.failed
                );
                if summary.failed > 0 {
                    std::process::exit(exitcode::DATAERR);
                } else {
                    std::process::exit(0);
                }
            }

            // Default: validation-only mode.
            let summary = match crate::frame_c::v3::test_harness_rs::run_validation_for_category(
                &repo_root,
                &framec_path,
                &harness_lang,
                &category,
            ) {
                Ok(summary) => summary,
                Err(e) => {
                    eprintln!("framec test error (Rust harness): {}", e);
                    std::process::exit(exitcode::IOERR);
                }
            };

            println!(
                "framec test: language={} category={} passed={} failed={}",
                summary.language, summary.category, summary.passed, summary.failed
            );

            if compare_python {
                // Mirror v3_rs_test_runner's compare mode for a single slice.
                let py_output = std::process::Command::new("python3")
                    .arg("framec_tests/runner/frame_test_runner.py")
                    .arg("--languages")
                    .arg(&harness_lang)
                    .arg("--categories")
                    .arg(&category)
                    .arg("--framec")
                    .arg(&framec_path)
                    .arg("--transpile-only")
                    .arg("--no-run")
                    .current_dir(&repo_root)
                    .output();

                let (py_ok, py_text) = match py_output {
                    Ok(out) => {
                        let ok = out.status.success();
                        let mut text = String::new();
                        text.push_str(&String::from_utf8_lossy(&out.stdout));
                        text.push_str(&String::from_utf8_lossy(&out.stderr));
                        (ok, text)
                    }
                    Err(e) => {
                        eprintln!("framec test compare-python: failed to spawn Python runner: {}", e);
                        std::process::exit(exitcode::IOERR);
                    }
                };

                let rust_ok = summary.failed == 0;
                println!(
                    "framec test compare-python: rust_ok={} python_ok={}",
                    rust_ok, py_ok
                );

                if !py_ok {
                    eprintln!("Python runner reported failure for this slice:");
                    for line in py_text.lines() {
                        eprintln!("    {}", line);
                    }
                }

                if !py_ok || !rust_ok {
                    std::process::exit(exitcode::DATAERR);
                } else {
                    std::process::exit(0);
                }
            } else if summary.failed > 0 {
                std::process::exit(exitcode::DATAERR);
            } else {
                std::process::exit(0);
            }
        }
        CliCommand::ProjectBuild { language, ref output_dir, recursive } => {
            // PRT-first, advisory project build:
            // - If a frame.toml is found, use its root and source dirs.
            // - Otherwise, delegate to compile-project over the current directory.
            //
            // Project builds should honour the same validation/debug flags as
            // direct compile-project invocations, so we thread those settings
            // through when we construct the inner Cli value.
            if let Some((config_path, cfg)) = FrameConfig::find_project_config() {
                let project_root = config_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let src_dirs: Vec<std::path::PathBuf> = if !cfg.build.source_dirs.is_empty() {
                    cfg.build
                        .source_dirs
                        .iter()
                        .map(|p| project_root.join(p))
                        .collect()
                } else if !cfg.paths.modules.is_empty() {
                    cfg.paths
                        .modules
                        .iter()
                        .map(|p| project_root.join(p))
                        .collect()
                } else {
                    vec![project_root.join("src")]
                };
                for dir in src_dirs {
                    let project_args = Cli {
                        stdin_flag: false,
                        path: None,
                        // The compile-project command carries the language/dir/output;
                        // we keep the top-level language field unused here.
                        language: None,
                        multifile: false,
                        output_dir: Some(output_dir.clone()),
                        debug_output: args.debug_output,
                        validate_only: args.validate_only,
                        validate: args.validate,
                        validate_native: args.validate_native,
                        emit_debug: args.emit_debug,
                        command: CliCommand::CompileProject {
                            language: language.clone(),
                            dir: dir.clone(),
                            output_dir: output_dir.clone(),
                            recursive,
                        },
                    };
                    run_with(project_args);
                }
            } else {
                let dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let project_args = Cli {
                    stdin_flag: false,
                    path: None,
                    language: None,
                    multifile: false,
                    output_dir: Some(output_dir.clone()),
                    debug_output: args.debug_output,
                    validate_only: args.validate_only,
                    validate: args.validate,
                    validate_native: args.validate_native,
                    emit_debug: args.emit_debug,
                    command: CliCommand::CompileProject {
                        language,
                        dir: dir.clone(),
                        output_dir: output_dir.clone(),
                        recursive,
                    },
                };
                run_with(project_args);
            }
            return;
        }
        CliCommand::FidImport { target, input, cache_root } => {
            // Phase A: simple file copy into the FID cache layout. This does not
            // invoke external tools; it only organizes existing JSON into the
            // expected `.frame/cache/fid/<target>/` directory.
            let target_dir = match target.as_str() {
                "python" | "python_3" => "python",
                "typescript" | "ts" => "typescript",
                "rust" | "rs" => "rust",
                other => {
                    eprintln!(
                        "Unsupported FID target '{}'; expected one of python, python_3, typescript, ts, rust, rs",
                        other
                    );
                    std::process::exit(exitcode::USAGE);
                }
            };
            let root = cache_root.unwrap_or_else(|| std::path::PathBuf::from(".frame/cache/fid"));
            let dest_dir = root.join(target_dir);
            if let Err(e) = std::fs::create_dir_all(&dest_dir) {
                eprintln!("Failed to create FID cache directory {:?}: {}", dest_dir, e);
                std::process::exit(exitcode::IOERR);
            }
            let file_name = input
                .file_name()
                .map(|s| s.to_owned())
                .unwrap_or_else(|| std::ffi::OsStr::new("fid.json").to_owned());
            let dest_path = dest_dir.join(&file_name);
            match std::fs::copy(&input, &dest_path) {
                Ok(_) => {
                    println!("Imported FID: {}", dest_path.display());
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Failed to import FID from {:?} to {:?}: {}", input, dest_path, e);
                    std::process::exit(exitcode::IOERR);
                }
            }
        }
        CliCommand::CompileProject { language, dir, output_dir, recursive } => {
            let lang = match TargetLanguage::try_from(language) { Ok(l) => l, Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); } };
            let allowed_targets: std::collections::HashSet<&str> = match lang {
                TargetLanguage::Python3 => ["python_3", "python"].into_iter().collect(),
                TargetLanguage::TypeScript => ["typescript", "ts"].into_iter().collect(),
                TargetLanguage::Rust => ["rust", "rs"].into_iter().collect(),
                TargetLanguage::CSharp => ["csharp"].into_iter().collect(),
                TargetLanguage::C => ["c"].into_iter().collect(),
                TargetLanguage::Cpp => ["cpp", "c++"].into_iter().collect(),
                TargetLanguage::Java => ["java"].into_iter().collect(),
                _ => ["python_3"].into_iter().collect(), // default/fallback
            };
            fn detect_target(content: &str) -> Option<String> {
                for line in content.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.starts_with("@target") {
                        let rest = trimmed["@target".len()..].trim();
                        let token = rest.split_whitespace().next()?;
                        let clean = token.trim_matches(|c| c == '"' || c == '\'').to_lowercase();
                        return Some(clean);
                    }
                }
                None
            }
            // Walk directory, compile module files (@target present), write outputs to output_dir
            fn iter(dir: &std::path::Path, recursive: bool) -> std::io::Result<Vec<std::path::PathBuf>> {
                let mut out = Vec::new();
                fn walk(acc: &mut Vec<std::path::PathBuf>, p: &std::path::Path, recursive: bool) -> std::io::Result<()> {
                    for entry in std::fs::read_dir(p)? {
                        let entry = entry?; let path = entry.path();
                        if path.is_dir() { if recursive { walk(acc, &path, recursive)?; } }
                        else if path.is_file() { acc.push(path); }
                    }
                    Ok(())
                }
                walk(&mut out, dir, recursive)?; Ok(out)
            }
            let files = match iter(&dir, recursive) { Ok(v) => v, Err(e) => { eprintln!("walk error: {}", e); std::process::exit(exitcode::IOERR); } };
            // Respect debug/map flags for trailers
            if args.debug_output { std::env::set_var("FRAME_ERROR_JSON", "1"); }
            if args.emit_debug {
                std::env::set_var("FRAME_ERROR_JSON", "1");
                std::env::set_var("FRAME_MAP_TRAILER", "1");
                std::env::set_var("FRAME_DEBUG_MANIFEST", "1");
            }
            if let Err(e) = std::fs::create_dir_all(&output_dir) { eprintln!("cannot create output dir: {}", e); std::process::exit(exitcode::IOERR); }
            let mut compiled: Vec<String> = Vec::new();
            let mut had_errors = false;
            let mut errors_count: usize = 0;
            let mut validated_count: usize = 0;
            let mut missing_target: Vec<std::path::PathBuf> = Vec::new();
            let mut mismatched_target: Vec<(std::path::PathBuf, String)> = Vec::new();
            for f in files {
                let Ok(content) = std::fs::read_to_string(&f) else { continue };
                let target_decl = detect_target(&content);
                if target_decl.is_none() {
                    missing_target.push(f.clone());
                    had_errors = true;
                    continue;
                }
                let target_decl = target_decl.unwrap();
                if !allowed_targets.contains(target_decl.as_str()) {
                    mismatched_target.push((f.clone(), target_decl));
                    had_errors = true;
                    continue;
                }
                // Check for duplicate system names across modules (best-effort)
                if let Some(sys_name) = crate::frame_c::v3::find_system_name(content.as_bytes(), 0) {
                    let entry = dup_systems.entry(sys_name).or_insert_with(Vec::new);
                    entry.push(f.clone());
                }
                if args.validate || args.validate_only {
                    match crate::frame_c::v3::validate_module_demo_with_mode(&content, lang, args.validate_native) {
                        Ok(res) => {
                            let mut had_any = false;
                            for issue in &res.issues { eprintln!("{}: validation: {}", f.display(), issue.message); had_any = true; }
                            if had_any { had_errors = true; }
                            errors_count += res.issues.len();
                            validated_count += 1;
                            if args.validate_only && !res.ok { /* defer exit to post-loop */ }
                            if args.validate_native && !res.ok { /* continue; we'll still compile but print issues */ }
                        }
                        Err(e) => { eprintln!("{}: validation error: {}", f.display(), e.error); if args.validate_only || args.validate_native { std::process::exit(e.code); } }
                    }
                }
                if args.validate_only { continue; }
                match crate::frame_c::v3::compile_module(&content, lang) {
                    Ok(code) => {
                        let ext = match lang { TargetLanguage::Python3 => ".py", TargetLanguage::TypeScript => ".ts", TargetLanguage::CSharp => ".cs", TargetLanguage::C => ".c", TargetLanguage::Cpp => ".cpp", TargetLanguage::Java => ".java", TargetLanguage::Rust => ".rs", _ => ".txt" };
                        let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
                        let outp = output_dir.join(format!("{}{}", stem, ext));
                        if let Err(e) = std::fs::write(&outp, code) { eprintln!("write error: {}", e); std::process::exit(exitcode::IOERR); }
                        compiled.push(outp.display().to_string());
                    }
                    Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
                }
            }
            if !missing_target.is_empty() {
                for p in &missing_target {
                    eprintln!("{}: missing @target declaration (compile-project requires explicit @target per module)", p.display());
                }
            }
            if !mismatched_target.is_empty() {
                for (p, t) in &mismatched_target {
                    eprintln!("{}: @target '{}' does not match requested project target", p.display(), t);
                }
            }
            for (sys, paths) in &dup_systems {
                if paths.len() > 1 {
                    eprintln!("Duplicate system '{}' across modules:", sys);
                    for p in paths {
                        eprintln!("  - {}", p.display());
                    }
                }
            }
            if args.validate_only {
                println!("[compile-project] summary: validated={} errors={}", validated_count, errors_count);
                // Fail if no modules were validated or if any had errors
                if validated_count == 0 || had_errors { std::process::exit(exitcode::DATAERR); }
                else { std::process::exit(0); }
            }
            if !missing_target.is_empty() || !mismatched_target.is_empty() || dup_systems.values().any(|v| v.len() > 1) {
                std::process::exit(exitcode::DATAERR);
            }
            // Print a simple manifest for now
            println!("Compiled {} module(s)", compiled.len());
            for p in &compiled { println!("{}", p); }
            // For Python projects, copy frame_runtime_py once to the output directory root
            if matches!(lang, TargetLanguage::Python3) {
                if let Some(outdir) = args.output_dir.as_ref() {
                    let env_override = std::env::var("FRAME_RUNTIME_PY_DIR").ok().map(std::path::PathBuf::from);
                    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()));
                    let repo_guess = exe_dir.as_ref()
                        .and_then(|d| d.parent().map(|d| d.to_path_buf()))
                        .and_then(|d| d.parent().map(|d| d.to_path_buf()))
                        .map(|d| d.join("frame_runtime_py"));
                    let target_guess = exe_dir.as_ref()
                        .and_then(|d| d.parent().map(|d| d.to_path_buf()))
                        .map(|d| d.join("frame_runtime_py"));
                    let cwd_guess = Some(std::path::PathBuf::from("frame_runtime_py"));
                    let runtime_src = env_override
                        .filter(|p| p.exists())
                        .or(repo_guess.filter(|p| p.exists()))
                        .or(target_guess.filter(|p| p.exists()))
                        .or(cwd_guess.filter(|p| p.exists()))
                        .unwrap_or_else(|| std::path::PathBuf::from("frame_runtime_py"));
                    let dst_dir = outdir.join("frame_runtime_py");
                    if runtime_src.exists() {
                        fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                            if !dst.exists() { std::fs::create_dir_all(dst)?; }
                            for entry in std::fs::read_dir(src)? {
                                let entry = entry?; let p = entry.path();
                                let name = entry.file_name(); let to = dst.join(name);
                                if p.is_dir() { copy_dir(&p, &to)?; } else if p.is_file() { std::fs::copy(&p, &to)?; }
                            }
                            Ok(())
                        }
                        if let Err(e) = copy_dir(&runtime_src, &dst_dir) {
                            eprintln!("warning: failed to copy frame_runtime_py: {}", e);
                        }
                    } else {
                        eprintln!("warning: frame_runtime_py not found at {:?}; set FRAME_RUNTIME_PY_DIR to override", runtime_src);
                    }
                }
            }
            return;
        }
        CliCommand::Compile { language, file } => {
            let lang = match TargetLanguage::try_from(language) { Ok(l) => l, Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); } };
            match std::fs::read_to_string(&file) {
                Ok(content) => {
                    if args.debug_output { std::env::set_var("FRAME_ERROR_JSON", "1"); }
                    if args.emit_debug {
                        std::env::set_var("FRAME_ERROR_JSON", "1");
                        std::env::set_var("FRAME_MAP_TRAILER", "1");
                        std::env::set_var("FRAME_DEBUG_MANIFEST", "1");
                    }
                    // Optional validation
                    if args.validate || args.validate_only {
                        match crate::frame_c::v3::validate_module_demo_with_mode(&content, lang, args.validate_native) {
                            Ok(res) => {
                                for issue in res.issues { eprintln!("validation: {}", issue.message); }
                                if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                                if args.validate_native && !res.ok { std::process::exit(exitcode::DATAERR); }
                            }
                            Err(e) => { eprintln!("validation error: {}", e.error); if args.validate_only || args.validate_native { std::process::exit(e.code); } }
                        }
                    }
                    match crate::frame_c::v3::compile_module(&content, lang) {
                        Ok(code) => {
                            if let Some(dir) = args.output_dir.as_ref() {
                                if let Err(e) = std::fs::create_dir_all(dir) { eprintln!("cannot create output dir: {}", e); std::process::exit(exitcode::IOERR); }
                                let ext = match lang { TargetLanguage::Python3 => ".py", TargetLanguage::TypeScript => ".ts", TargetLanguage::CSharp => ".cs", TargetLanguage::C => ".c", TargetLanguage::Cpp => ".cpp", TargetLanguage::Java => ".java", TargetLanguage::Rust => ".rs", _ => ".txt" };
                                let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
                                let out_path = dir.join(format!("{}{}", stem, ext));
                                if let Err(e) = std::fs::write(&out_path, code) { eprintln!("write error: {}", e); std::process::exit(exitcode::IOERR); }
                                // Emit Python runtime package next to outputs when compiling Python modules
                                if matches!(lang, TargetLanguage::Python3) {
                                    // Resolve runtime source directory robustly for compile -o
                                    let env_override = std::env::var("FRAME_RUNTIME_PY_DIR").ok().map(std::path::PathBuf::from);
                                    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()));
                                    let repo_guess = exe_dir.as_ref()
                                        .and_then(|d| d.parent().map(|d| d.to_path_buf()))
                                        .and_then(|d| d.parent().map(|d| d.to_path_buf()))
                                        .map(|d| d.join("frame_runtime_py"));
                                    let target_guess = exe_dir.as_ref()
                                        .and_then(|d| d.parent().map(|d| d.to_path_buf()))
                                        .map(|d| d.join("frame_runtime_py"));
                                    let cwd_guess = Some(std::path::PathBuf::from("frame_runtime_py"));
                                    let runtime_src = env_override
                                        .filter(|p| p.exists())
                                        .or(repo_guess.filter(|p| p.exists()))
                                        .or(target_guess.filter(|p| p.exists()))
                                        .or(cwd_guess.filter(|p| p.exists()))
                                        .unwrap_or_else(|| std::path::PathBuf::from("frame_runtime_py"));
                                    let dst_dir = dir.join("frame_runtime_py");
                                    if runtime_src.exists() {
                                        // Recursively copy (create dirs as needed)
                                        fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                                            if !dst.exists() { std::fs::create_dir_all(dst)?; }
                                            for entry in std::fs::read_dir(src)? {
                                                let entry = entry?; let p = entry.path();
                                                let name = entry.file_name(); let to = dst.join(name);
                                                if p.is_dir() {
                                                    copy_dir(&p, &to)?;
                                                } else if p.is_file() {
                                                    std::fs::copy(&p, &to)?; // overwrite if exists
                                                }
                                            }
                                            Ok(())
                                        }
                                        if let Err(e) = copy_dir(&runtime_src, &dst_dir) { eprintln!("warning: failed to copy frame_runtime_py: {}", e); }
                                    } else {
                                        eprintln!("warning: frame_runtime_py not found at {:?}; set FRAME_RUNTIME_PY_DIR to override", runtime_src);
                                    }
                                }
                                println!("{}", out_path.display());
                            } else {
                                println!("{}", code);
                            }
                        }
                        Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
                    }
                }
                Err(e) => { eprintln!("Failed to read {}: {}", file.display(), e); std::process::exit(exitcode::NOINPUT); }
            }
            return;
        }
        
        /* Removed legacy demo-project */
        /*
        CliCommand::DemoProject { language, dir, recursive } => {
            let lang = match TargetLanguage::try_from(language) {
                Ok(l) => l,
                Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); }
            };
            if args.validate || args.validate_only {
                // Validate each eligible single-body file
                fn iter_files(dir: &std::path::Path, recursive: bool) -> std::io::Result<Vec<std::path::PathBuf>> {
                    let mut out = Vec::new();
                    fn walk(acc: &mut Vec<std::path::PathBuf>, p: &std::path::Path, recursive: bool) -> std::io::Result<()> {
                        for entry in std::fs::read_dir(p)? { let entry = entry?; let path = entry.path(); if path.is_dir() { if recursive { walk(acc, &path, recursive)?; } } else if path.is_file() { acc.push(path); } }
                        Ok(())
                    }
                    walk(&mut out, dir, recursive)?; Ok(out)
                }
                let mut had_errors = false;
                let mut validated_count: usize = 0;
                let mut errors_count: usize = 0;
                match iter_files(&dir, recursive) {
                    Ok(files) => {
                        // Build a project-level arcanum from module files (@target present)
                        let mut module_files: Vec<(std::path::PathBuf, String)> = Vec::new();
                        for f in &files {
                            if let Ok(content) = std::fs::read_to_string(f) {
                                if content.contains("@target ") { module_files.push((f.clone(), content)); }
                            }
                        }
                        // Build Arcanum by merging AST-based symbol tables from each module file
                        let mut arc = crate::frame_c::v3::arcanum::Arcanum::new();
                        for (_p, content) in &module_files {
                            let bytes = content.as_bytes();
                            let module_ast = crate::frame_c::v3::system_parser::SystemParserV3::parse_module(bytes, lang);
                            let a = crate::frame_c::v3::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
                            // merge into arc (simple overlay keyed by system name)
                            for (k, v) in a.systems.into_iter() { arc.systems.entry(k).or_insert(v); }
                        }
                        for f in files {
                            if let Ok(content) = std::fs::read_to_string(&f) {
                                if content.contains("@target ") {
                                    match super::v3::validate_module_with_arcanum(&content, lang, &arc, false) {
                                        Ok(res) => {
                                            let mut had_any = false;
                                            for issue in &res.issues { eprintln!("{}: validation: {}", f.display(), issue.message); had_any = true; }
                                            if had_any { had_errors = true; }
                                            errors_count += res.issues.len();
                                            validated_count += 1;
                                            if args.validate_only && !res.ok { /* defer exit to post-loop */ }
                                        }
                                        Err(e) => { eprintln!("{}: validation error: {}", f.display(), e.error); if args.validate_only { std::process::exit(e.code); } }
                                    }
                                } else {
                                    let bytes = content.as_bytes(); if bytes.first().copied() != Some(b'{') { continue; }
                                    match super::v3::validate_single_body(&content, Some(lang)) {
                                        Ok(res) => {
                                            let mut had_any = false;
                                            for issue in &res.issues { eprintln!("{}: validation: {}", f.display(), issue.message); had_any = true; }
                                            if had_any { had_errors = true; }
                                            errors_count += res.issues.len();
                                            validated_count += 1;
                                            if args.validate_only && !res.ok { /* defer exit to post-loop */ }
                                        }
                                        Err(e) => { eprintln!("{}: validation error: {}", f.display(), e.error); if args.validate_only { std::process::exit(e.code); } }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => { eprintln!("walk error: {}", e); if args.validate_only { std::process::exit(exitcode::IOERR); } }
                }
                if args.validate_only {
                    println!("[demo-project] summary: validated={} errors={}", validated_count, errors_count);
                    if validated_count == 0 || had_errors { std::process::exit(exitcode::DATAERR); } else { std::process::exit(0); }
                }
            }
            match crate::frame_c::v3::multifile_demo::compile_directory_demo(&dir, lang, recursive) {
                Ok(outputs) => {
                    for (path, code) in outputs {
                        println!("=== file: {} ===\n{}", path.display(), code);
                    }
                }
                Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
            }
            return;
        }
        */
        /* Removed legacy demo-frame
        CliCommand::DemoFrame { language, file } => {
            let lang = match TargetLanguage::try_from(language) {
                Ok(l) => l,
                Err(e) => { eprintln!("Invalid target language: {}", e); std::process::exit(exitcode::USAGE); }
            };
            match std::fs::read_to_string(&file) {
                Ok(content) => {
                    // Set optional demo emission flags
                    if args.emit_body_only { std::env::set_var("FRAME_EMIT_BODY_ONLY", "1"); }
                    if args.emit_exec { std::env::set_var("FRAME_EMIT_EXEC", "1"); }
                    if args.debug_output { std::env::set_var("FRAME_ERROR_JSON", "1"); }
                    if args.emit_map { std::env::set_var("FRAME_MAP_TRAILER", "1"); }
                    if args.emit_debug {
                        std::env::set_var("FRAME_ERROR_JSON", "1");
                        std::env::set_var("FRAME_MAP_TRAILER", "1");
                        std::env::set_var("FRAME_DEBUG_MANIFEST", "1");
                    }
                    if args.validate || args.validate_only {
                        match crate::frame_c::v3::validate_module_demo_with_mode(&content, lang, args.validate_native) {
                            Ok(res) => {
                                for issue in res.issues { eprintln!("{}: validation: {}", file.display(), issue.message); }
                                if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                                // When native validation is requested, fail fast on validation errors (used by facade-smoke tests)
                                if args.validate_native && !res.ok { std::process::exit(exitcode::DATAERR); }
                            }
                            Err(e) => { eprintln!("{}: validation error: {}", file.display(), e.error); if args.validate_only || args.validate_native { std::process::exit(e.code); } }
                        }
                    }
                    match crate::frame_c::v3::compile_module(&content, lang) {
                        Ok(code) => { println!("{}", code); }
                        Err(e) => { eprintln!("{}", e.error); std::process::exit(e.code); }
                    }
                }
                Err(e) => { eprintln!("Failed to read {}: {}", file.display(), e); std::process::exit(exitcode::NOINPUT); }
            }
            return;
        }
        */
        CliCommand::None => {}
    }

    let exe = Exe::new();
    // Validation-only pathway (V3 demo; single-body input only)
    let target_language = match &args.language {
        Some(lang_str) => match TargetLanguage::try_from(lang_str.clone()) {
            Ok(lang) => Some(lang),
            Err(err) => {
                eprintln!("Invalid target language: {}", err);
                std::process::exit(exitcode::USAGE);
            }
        },
        None => None,
    };
    if args.validate_only || args.validate {
        let path = args.path.clone().expect("file path required");
        if let Ok(content) = std::fs::read_to_string(&path) {
            // If this appears to be a module file (@target present), run module validation; otherwise single-body demo
            let is_module = content.contains("@target ");
            if is_module {
                // Require target language
                let lang = target_language.unwrap_or(TargetLanguage::Python3);
                match super::v3::validate_module_demo_with_mode(&content, lang, args.validate_native) {
                    Ok(res) => {
                        for issue in res.issues { eprintln!("validation: {}", issue.message); }
                        if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                    }
                    Err(e) => {
                        eprintln!("validation error: {}", e.error);
                        if args.validate_only { std::process::exit(e.code); }
                    }
                }
            } else {
                match super::v3::validate_single_body(&content, target_language) {
                    Ok(res) => {
                        for issue in res.issues { eprintln!("validation: {}", issue.message); }
                        if args.validate_only { std::process::exit(if res.ok { 0 } else { exitcode::DATAERR }); }
                    }
                    Err(e) => {
                        eprintln!("validation error: {}", e.error);
                        if args.validate_only { std::process::exit(e.code); }
                    }
                }
            }
        }
    }

    if args.stdin_flag {
        match exe.run_stdin(target_language) {
            Ok(code) => println!("{}", code),
            Err(err) => {
                eprintln!("{}", err.error);
                std::process::exit(err.code);
            }
        }
        } else {
            let path = args.path.unwrap();
            let result = if args.debug_output {
                exe.run_file_debug(&path, target_language)
            } else if args.multifile {
                exe.run_multifile(&path, target_language, args.output_dir)
            } else {
                if args.emit_debug {
                    std::env::set_var("FRAME_ERROR_JSON", "1");
                    std::env::set_var("FRAME_MAP_TRAILER", "1");
                    std::env::set_var("FRAME_DEBUG_MANIFEST", "1");
                }
                exe.run_file(&path, target_language)
            };

        match result {
            Ok(code) => println!("{}", code),
            Err(err) => {
                eprintln!("{}", err.error);
                std::process::exit(err.code);
            }
        }
    }
}

fn handle_init_command() {
    use std::env;
    use std::fs;

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("frame.toml");

    if config_path.exists() {
        eprintln!("frame.toml already exists in this directory");
        std::process::exit(exitcode::CANTCREAT);
    }

    let project_name = current_dir.file_name().and_then(|n| n.to_str()).map(|s| s.to_string());

    match FrameConfig::create_default(&config_path, project_name.as_deref()) {
        Ok(_) => {
            println!("Created frame.toml");
            let src_dir = current_dir.join("src");
            if !src_dir.exists() {
                fs::create_dir(&src_dir).expect("Failed to create src directory");
                println!("Created src/");
                let main_file = src_dir.join("main.frm");
                let main_content = r#"# Main entry point for Frame project

fn main() {
    print("Hello from Frame!")
}
"#;
                fs::write(&main_file, main_content).expect("Failed to create main.frm");
                println!("Created src/main.frm");
            }
            println!("\nFrame project initialized successfully!");
        }
        Err(e) => {
            eprintln!("Failed to create frame.toml: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }
}
