use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const REQUIRED_FILES: &[&str] = &[
    ".cargo/config.toml",
    ".github/workflows/validation.yml",
    ".gitignore",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "BOOTSTRAP.md",
    "Cargo.lock",
    "Cargo.toml",
    "LICENSE",
    "LICENSING.md",
    "README.md",
    "TESTING.md",
    "conformance/downstream/Cargo.lock",
    "conformance/downstream/Cargo.toml",
    "conformance/downstream/src/lib.rs",
    "examples/basic_state.rs",
    "rust-toolchain.toml",
    "src/lib.rs",
    "tests/public_contract.rs",
    "xtask/Cargo.toml",
    "xtask/src/main.rs",
];

fn main() {
    let result = match env::args().nth(1).as_deref() {
        Some("validate") => validate(),
        _ => Err("usage: cargo xtask validate".to_owned()),
    };

    if let Err(error) = result {
        eprintln!("validation failed: {error}");
        std::process::exit(1);
    }
}

fn validate() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest must have a workspace root")
        .to_path_buf();

    validate_required_files(&root)?;
    validate_product_identity(&root)?;
    validate_standalone_boundary(&root)?;

    let initial_state = git_status(&root)?;
    if !initial_state.is_empty() {
        return Err(format!(
            "repository must be clean before validation:\n{initial_state}"
        ));
    }

    run(&root, "cargo", &["fmt", "--all", "--", "--check"])?;
    run(
        &root,
        "cargo",
        &[
            "fmt",
            "--manifest-path",
            "conformance/downstream/Cargo.toml",
            "--",
            "--check",
        ],
    )?;
    run(&root, "cargo", &["test", "--workspace", "--locked"])?;
    run(
        &root,
        "cargo",
        &[
            "test",
            "--manifest-path",
            "conformance/downstream/Cargo.toml",
            "--locked",
        ],
    )?;
    run(
        &root,
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run(
        &root,
        "cargo",
        &[
            "clippy",
            "--manifest-path",
            "conformance/downstream/Cargo.toml",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_with_env(
        &root,
        "cargo",
        &["doc", "--workspace", "--no-deps", "--locked"],
        &[("RUSTDOCFLAGS", "-D warnings")],
    )?;
    run(
        &root,
        "cargo",
        &["+1.93.0", "check", "--workspace", "--locked"],
    )?;
    run(
        &root,
        "cargo",
        &[
            "+1.93.0",
            "check",
            "--manifest-path",
            "conformance/downstream/Cargo.toml",
            "--locked",
        ],
    )?;
    run(&root, "git", &["diff", "--check"])?;
    run(&root, "git", &["diff", "--cached", "--check"])?;

    let final_state = git_status(&root)?;
    if final_state != initial_state {
        return Err(format!(
            "validation changed repository state:\nbefore:\n{initial_state}after:\n{final_state}"
        ));
    }

    Ok(())
}

fn validate_required_files(root: &Path) -> Result<(), String> {
    for relative_path in REQUIRED_FILES {
        let path = root.join(relative_path);
        if !path.is_file() {
            return Err(format!("required file is missing: {relative_path}"));
        }
    }

    Ok(())
}

fn validate_product_identity(root: &Path) -> Result<(), String> {
    let cargo = read_text(root, "Cargo.toml")?;
    require_contains("Cargo.toml", &cargo, "name = \"runen-input\"")?;
    require_contains("Cargo.toml", &cargo, "version = \"0.1.0\"")?;
    require_contains(
        "Cargo.toml",
        &cargo,
        "repository = \"https://github.com/dornglut/runen-input\"",
    )?;
    require_contains("Cargo.toml", &cargo, "license = \"GPL-3.0-only\"")?;
    require_absent("Cargo.toml", &cargo, "rust-framework-template")?;
    validate_std_only_product_manifest(&cargo)?;

    let lock = read_text(root, "Cargo.lock")?;
    require_contains("Cargo.lock", &lock, "name = \"runen-input\"")?;
    require_contains("Cargo.lock", &lock, "version = \"0.1.0\"")?;
    require_absent("Cargo.lock", &lock, "rust-framework-template")?;

    let readme = read_text(root, "README.md")?;
    require_contains("README.md", &readme, "# RunenInput")?;
    require_contains("README.md", &readme, "GPL-3.0-only")?;
    require_contains("README.md", &readme, "InputState")?;

    let license = read_text(root, "LICENSE")?;
    require_contains("LICENSE", &license, "GNU GENERAL PUBLIC LICENSE")?;
    require_contains("LICENSE", &license, "Version 3, 29 June 2007")?;

    let licensing = read_text(root, "LICENSING.md")?;
    require_contains("LICENSING.md", &licensing, "GPL-3.0-only")?;
    require_contains(
        "LICENSING.md",
        &licensing,
        "separately negotiated commercial license",
    )?;

    let workflow = read_text(root, ".github/workflows/validation.yml")?;
    require_contains(
        "validation workflow",
        &workflow,
        "name: RunenInput Validation",
    )?;
    require_contains(
        "validation workflow",
        &workflow,
        "name: Validate RunenInput",
    )?;

    let library = read_text(root, "src/lib.rs")?;
    validate_root_facade(&library)?;
    require_absent("src/lib.rs", &library, "bootstrap shell")?;

    Ok(())
}

fn validate_std_only_product_manifest(cargo: &str) -> Result<(), String> {
    for line in cargo.lines() {
        let section = line.split('#').next().unwrap_or_default().trim();
        let direct_dependency_section =
            matches!(section, "[dependencies]" | "[build-dependencies]")
                || section.starts_with("[dependencies.")
                || section.starts_with("[build-dependencies.");
        let target_dependency_section = section.starts_with("[target.")
            && (section.contains(".dependencies]")
                || section.contains(".dependencies.")
                || section.contains(".build-dependencies]")
                || section.contains(".build-dependencies."));
        if direct_dependency_section || target_dependency_section {
            return Err(format!(
                "Cargo.toml product semantic core must remain std-only; dependency section is not allowed: {section}"
            ));
        }
    }

    Ok(())
}

fn validate_root_facade(library: &str) -> Result<(), String> {
    require_absent("src/lib.rs", library, "pub mod ")?;

    if library.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with("pub use ") && line.contains("::*")
    }) {
        return Err("src/lib.rs must use explicit crate-root re-exports".to_owned());
    }

    Ok(())
}

fn validate_standalone_boundary(root: &Path) -> Result<(), String> {
    let cargo = read_text(root, "Cargo.toml")?;
    for forbidden in ["runenwerk", "winit", "runen_ecs", "runen-ui", "runen_ui"] {
        require_absent("Cargo.toml", &cargo, forbidden)?;
    }

    validate_product_sources(root)?;
    validate_conformance_sources(root)?;

    let downstream = read_text(root, "conformance/downstream/Cargo.toml")?;
    require_contains(
        "downstream manifest",
        &downstream,
        "runen-input = { package = \"runen-input\", path = \"../..\" }",
    )?;
    for forbidden in [
        "workspace = true",
        "runenwerk",
        "winit",
        "runen_ecs",
        "runen-ui",
    ] {
        require_absent("downstream manifest", &downstream, forbidden)?;
    }

    if root.join(".gitmodules").exists() {
        return Err("standalone repository must not contain a submodule".to_owned());
    }

    Ok(())
}

fn validate_product_sources(root: &Path) -> Result<(), String> {
    for path in rust_source_files(root, "src", "product", &[])? {
        let label = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {label}: {error}"))?;
        validate_source_file(&label, &source)?;
    }

    Ok(())
}

fn validate_conformance_sources(root: &Path) -> Result<(), String> {
    for path in rust_source_files(root, "conformance/downstream", "conformance", &["target"])? {
        let label = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {label}: {error}"))?;
        validate_source_file(&label, &source)?;
    }

    Ok(())
}

fn rust_source_files(
    root: &Path,
    relative_root: &str,
    closure_label: &str,
    ignored_top_level_directories: &[&str],
) -> Result<Vec<PathBuf>, String> {
    let source_root = root.join(relative_root);
    if !source_root.is_dir() {
        return Err(format!(
            "{closure_label} source root is missing: {relative_root}"
        ));
    }

    let mut paths = Vec::new();
    collect_rust_source_files(
        &source_root,
        &source_root,
        ignored_top_level_directories,
        &mut paths,
    )?;
    paths.sort();

    if paths.is_empty() {
        return Err(format!(
            "{closure_label} source closure contains no Rust files"
        ));
    }

    Ok(paths)
}

fn collect_rust_source_files(
    source_root: &Path,
    current_root: &Path,
    ignored_top_level_directories: &[&str],
    paths: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(current_root)
        .map_err(|error| format!("failed to read {}: {error}", current_root.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to read source entry: {error}"))?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        let ignored_top_level_subtree = current_root == source_root
            && entry
                .file_name()
                .to_str()
                .is_some_and(|name| ignored_top_level_directories.contains(&name));
        if ignored_top_level_subtree {
            continue;
        }

        let file_type = entry
            .file_type()
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;

        if file_type.is_symlink() {
            return Err(format!(
                "Rust source closure must not contain symlinks: {}",
                path.display()
            ));
        }
        if file_type.is_dir() {
            collect_rust_source_files(source_root, &path, ignored_top_level_directories, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            paths.push(path);
        }
    }

    Ok(())
}

fn validate_source_file(label: &str, source: &str) -> Result<(), String> {
    let code = without_line_comments(source);

    for forbidden in [
        "runenwerk",
        "winit::",
        "runen_ecs",
        "runen_ui",
        "include!(",
        "#[path =",
        "#[path=",
    ] {
        require_absent(label, &code, forbidden)?;
    }

    for forbidden in [
        "pub struct ControlId",
        "pub(crate) struct ControlId",
        "pub(super) struct ControlId",
        "pub enum DigitalTransition",
        "pub(crate) enum DigitalTransition",
        "pub(super) enum DigitalTransition",
        "DigitalControl {",
        "NeutralInputAuthority",
    ] {
        require_absent(label, &code, forbidden)?;
    }

    Ok(())
}

fn without_line_comments(contents: &str) -> String {
    contents
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("//") && !trimmed.starts_with('*')
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_text(root: &Path, relative_path: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative_path))
        .map_err(|error| format!("failed to read {relative_path}: {error}"))
}

fn require_contains(label: &str, text: &str, needle: &str) -> Result<(), String> {
    if text.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} must contain {needle:?}"))
    }
}

fn require_absent(label: &str, text: &str, needle: &str) -> Result<(), String> {
    if text.contains(needle) {
        Err(format!("{label} must not contain {needle:?}"))
    } else {
        Ok(())
    }
}

fn git_status(root: &Path) -> Result<String, String> {
    output(
        root,
        "git",
        &["status", "--porcelain", "--untracked-files=all"],
    )
}

fn run(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    run_with_env(root, program, args, &[])
}

fn run_with_env(
    root: &Path,
    program: &str,
    args: &[&str],
    environment: &[(&str, &str)],
) -> Result<(), String> {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(root)
        .envs(environment.iter().copied())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = command
        .status()
        .map_err(|error| format!("failed to execute {program}: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} {} exited with {status}", args.join(" ")))
    }
}

fn output(root: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let result = Command::new(program)
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("failed to execute {program}: {error}"))?;

    if !result.status.success() {
        return Err(format!(
            "{program} {} exited with {}:\n{}",
            args.join(" "),
            result.status,
            String::from_utf8_lossy(&result.stderr)
        ));
    }

    String::from_utf8(result.stdout)
        .map_err(|error| format!("{program} produced invalid UTF-8: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEMP_ROOT: AtomicUsize = AtomicUsize::new(0);

    fn temporary_source_root(label: &str) -> PathBuf {
        let serial = NEXT_TEMP_ROOT.fetch_add(1, Ordering::Relaxed);
        let root = env::temp_dir().join(format!(
            "runen-input-xtask-{label}-{}-{serial}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale temporary source root should be removable");
        }
        fs::create_dir_all(root.join("src/nested"))
            .expect("temporary nested source root should be creatable");
        root
    }

    #[test]
    fn std_only_product_manifest_rejects_runtime_and_build_dependencies() {
        for manifest in [
            "[package]\nname = \"runen-input\"\n[dependencies]\nserde = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[dependencies.serde]\nversion = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[build-dependencies]\ncc = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[build-dependencies.cc]\nversion = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[target.'cfg(unix)'.dependencies]\nlibc = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[target.'cfg(unix)'.dependencies.libc]\nversion = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[target.'cfg(unix)'.build-dependencies.cc]\nversion = \"1\"\n",
        ] {
            let error = validate_std_only_product_manifest(manifest)
                .expect_err("product dependency section must violate the std-only boundary");
            assert!(error.contains("std-only"));
        }

        for manifest in [
            "[package]\nname = \"runen-input\"\n[dev-dependencies]\nproptest = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[dev-dependencies.proptest]\nversion = \"1\"\n",
            "[package]\nname = \"runen-input\"\n[target.'cfg(unix)'.dev-dependencies]\nproptest = \"1\"\n",
        ] {
            validate_std_only_product_manifest(manifest)
                .expect("dev-only test dependencies do not change the product semantic core");
        }
    }

    #[test]
    fn nested_conformance_source_cannot_escape_standalone_boundary() {
        let root = temporary_source_root("conformance-boundary");
        let conformance = root.join("conformance/downstream/src/nested");
        fs::create_dir_all(&conformance)
            .expect("temporary conformance source root should be creatable");
        fs::write(
            root.join("conformance/downstream/src/lib.rs"),
            "// fixture\n",
        )
        .expect("temporary conformance crate root should be writable");
        fs::write(
            conformance.join("escape.rs"),
            "use runenwerk::runtime::Host;\n",
        )
        .expect("temporary nested conformance source should be writable");

        let error = validate_conformance_sources(&root)
            .expect_err("nested forbidden coupling must fail the conformance-source scan");

        assert!(error.contains("conformance/downstream/src/nested/escape.rs"));
        assert!(error.contains("runenwerk"));

        fs::remove_dir_all(root).expect("temporary source root should be removable");
    }

    #[test]
    fn conformance_test_target_cannot_escape_standalone_boundary() {
        let root = temporary_source_root("conformance-test-boundary");
        let source = root.join("conformance/downstream/src");
        let test_target = root.join("conformance/downstream/tests/nested");
        fs::create_dir_all(&source).expect("temporary conformance source root should be creatable");
        fs::create_dir_all(&test_target)
            .expect("temporary conformance test target should be creatable");
        fs::write(source.join("lib.rs"), "// fixture\n")
            .expect("temporary conformance crate root should be writable");
        fs::write(
            test_target.join("escape.rs"),
            "include!(\"../../../../src/state.rs\");\n",
        )
        .expect("temporary conformance test target should be writable");

        let error = validate_conformance_sources(&root)
            .expect_err("non-src private-source reach-through must fail conformance scanning");

        assert!(error.contains("conformance/downstream/tests/nested/escape.rs"));
        assert!(error.contains("include!("));

        fs::remove_dir_all(root).expect("temporary source root should be removable");
    }

    #[test]
    fn conformance_cargo_target_output_is_not_authored_source() {
        let root = temporary_source_root("conformance-target-output");
        let source = root.join("conformance/downstream/src");
        let target = root.join("conformance/downstream/target/debug/build");
        fs::create_dir_all(&source).expect("temporary conformance source root should be creatable");
        fs::create_dir_all(&target).expect("temporary Cargo target output should be creatable");
        fs::write(source.join("lib.rs"), "// fixture\n")
            .expect("temporary conformance crate root should be writable");
        fs::write(
            target.join("generated.rs"),
            "use runenwerk::runtime::Host;\n",
        )
        .expect("temporary Cargo target output should be writable");

        validate_conformance_sources(&root)
            .expect("Cargo target build output must not be treated as authored conformance source");

        fs::remove_dir_all(root).expect("temporary source root should be removable");
    }

    #[test]
    fn nested_product_source_cannot_escape_standalone_boundary() {
        let root = temporary_source_root("nested-boundary");
        fs::write(root.join("src/lib.rs"), "// fixture\n")
            .expect("temporary crate root should be writable");
        fs::write(
            root.join("src/nested/escape.rs"),
            "use winit::event::WindowEvent;\n",
        )
        .expect("temporary nested source should be writable");

        let error = validate_product_sources(&root)
            .expect_err("nested forbidden coupling must fail the product-source scan");

        assert!(error.contains("src/nested/escape.rs"));
        assert!(error.contains("winit::"));

        fs::remove_dir_all(root).expect("temporary source root should be removable");
    }
}
