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

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
// use std::process;
// use std::process::Command;

const SOURCE_FILE: &str = "crates/transipctl/docs/transipctl.md";
const DESTINATION_FILE: &str = "crates/transipctl/docs/transipctl.1";
// const PKG_NAME: &str = "transipctl";

fn main() -> anyhow::Result<()> {
    cwd_to_workspace_root()?;

    let output = mdman::convert(
        Path::new(SOURCE_FILE),
        mdman::Format::Man,
        None,
        mdman::ManMap::new(),
    )?;
    writer_to_file(DESTINATION_FILE)
        .and_then(|w| dump(output.as_bytes(), w))
        .map_err(Into::into)
}

fn writer_to_file<P>(p: P) -> std::io::Result<impl Write>
where
    P: AsRef<Path>,
{
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)
}

fn dump<W>(s: &[u8], w: W) -> std::io::Result<()>
where
    W: Write,
{
    let mut buf_writer = BufWriter::new(w);
    buf_writer.write_all(s)?;
    buf_writer.flush()?;
    Ok(())
}

/// Change to workspace root.
///
/// Assumed this xtask is located in `[WORKSPACE]/crates/xtask-build-man`.
fn cwd_to_workspace_root() -> std::io::Result<()> {
    let pkg_root = std::env!("CARGO_MANIFEST_DIR");
    let ws_root = format!("{pkg_root}/../..");
    std::env::set_current_dir(ws_root)
}

// Builds the man pages.
// fn build_man(pkg_name: &str, src_paths: &[PathBuf], outs: &[(&str, &str)]) -> io::Result<()> {
//     for (format, dst_path) in outs {
//         eprintln!("Start converting `{format}` for package `{pkg_name}`...");
//         let mut cmd = Command::new(std::env!("CARGO"));
//         cmd.args(["run", "--package", "mdman", "--"])
//             .args(["-t", format, "-o", dst_path])
//             .args(src_paths);
//
//         let status = cmd.status()?;
//         if !status.success() {
//             eprintln!("failed to build the man pages for package `{pkg_name}`");
//             eprintln!("failed command: `{cmd:?}`");
//             process::exit(status.code().unwrap_or(1));
//         }
//     }
//
//     Ok(())
// }
