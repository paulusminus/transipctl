//! ```text
//! NAME
//!         build-man
//!
//! SYNOPSIS
//!         build-man
//!
//! DESCRIPTION
//!         Build the man pages for package `transipctl`.
//!         For more, read their doc comments.
//! ```

use std::io;
use std::path::PathBuf;
use std::process;
use std::process::Command;

const SOURCE_FILE: &str = "crates/transipctl/transipctl.man";
const DEST_PATH: &str = "crates/transipctl/docs";
const PKG_NAME: &str = "transipctl";

fn main() -> io::Result<()> {
    cwd_to_workspace_root()?;

    let src_paths = &[SOURCE_FILE.into()];
    let outs = [("md", DEST_PATH), ("txt", DEST_PATH), ("man", DEST_PATH)];

    build_man(PKG_NAME, src_paths, &outs)
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
fn build_man(pkg_name: &str, src_paths: &[PathBuf], outs: &[(&str, &str)]) -> io::Result<()> {
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
