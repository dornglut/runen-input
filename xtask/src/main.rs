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
    "rust-toolchain.toml",
    "src/input.rs",
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
    require_contains("src/lib.rs", &library, "pub use input::*;")?;
    require_absent("src/lib.rs", &library, "bootstrap shell")?;

    Ok(())
}

fn validate_standalone_boundary(root: &Path) -> Result<(), String> {
    let cargo = read_text(root, "Cargo.toml")?;
    for forbidden in ["runenwerk", "winit", "runen_ecs", "runen-ui", "runen_ui"] {
        require_absent("Cargo.toml", &cargo, forbidden)?;
    }

    let input = read_text(root, "src/input.rs")?;
    require_contains("src/input.rs", &input, "pub struct InputState")?;
    require_contains("src/input.rs", &input, "Keyboard(KeyboardInput)")?;
    require_contains(
        "src/input.rs",
        &input,
        "PointerButton(PointerButtonInput)",
    )?;
    require_absent("src/input.rs", &input, "pub struct ControlId")?;
    require_absent("src/input.rs", &input, "pub enum DigitalTransition")?;
    require_absent("src/input.rs", &input, "DigitalControl {")?;
    require_absent("src/input.rs", &input, "NeutralInputAuthority")?;

    for forbidden in ["runenwerk::", "winit::", "runen_ecs", "runen_ui"] {
        require_absent("src/input.rs", &input, forbidden)?;
    }

    let downstream = read_text(root, "conformance/downstream/Cargo.toml")?;
    require_contains(
        "downstream manifest",
        &downstream,
        "runen-input = { package = \"runen-input\", path = \"../..\" }",
    )?;
    for forbidden in ["workspace = true", "runenwerk", "winit", "runen_ecs", "runen-ui"] {
        require_absent("downstream manifest", &downstream, forbidden)?;
    }

    if root.join(".gitmodules").exists() {
        return Err("standalone repository must not contain a submodule".to_owned());
    }

    Ok(())
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
