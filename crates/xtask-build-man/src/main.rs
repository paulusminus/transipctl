//! ```text
//! NAME
//!         build-man
//!
//! SYNOPSIS
//!         build-man
//!
//! DESCRIPTION
//!         Build the man pages for packages `mdman` and `cargo`.
//!         For more, read their doc comments.
//! ```

use std::io;
use std::path::PathBuf;
use std::process;
use std::process::Command;

fn main() -> io::Result<()> {
    // build_mdman()?;
    build_transipctl()
}

fn build_transipctl() -> io::Result<()> {
    cwd_to_workspace_root()?;

    let src_paths = &["crates/transipctl/transipctl.man".into()];
    let dst_path = "crates/transipctl/docs";
    let outs = [("md", dst_path), ("txt", dst_path), ("man", dst_path)];

    build_man("mdman", src_paths, &outs)
}

/// Change to workspace root.
///
/// Assumed this xtask is located in `[WORKSPACE]/crates/xtask-build-man`.
fn cwd_to_workspace_root() -> io::Result<()> {
    let pkg_root = std::env!("CARGO_MANIFEST_DIR");
    let ws_root = format!("{pkg_root}/../..");
    std::env::set_current_dir(ws_root)
}

/// Builds the man pages.
fn build_man(
    pkg_name: &str,
    src_paths: &[PathBuf],
    outs: &[(&str, &str)],
) -> io::Result<()> {
    for (format, dst_path) in outs {
        eprintln!("Start converting `{format}` for package `{pkg_name}`...");
        let mut cmd = Command::new(std::env!("CARGO"));
        cmd.args(["run", "--package", "mdman", "--"])
            .args(["-t", format, "-o", dst_path])
            .args(src_paths);

        let status = cmd.status()?;
        if !status.success() {
            eprintln!("failed to build the man pages for package `{pkg_name}`");
            eprintln!("failed command: `{cmd:?}`");
            process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(())
}