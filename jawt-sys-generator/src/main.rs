// Copyright (c) 2025 Gobley Contributors.

use std::fs;
use std::path::{Path, PathBuf};

use bindgen::RustTarget;
use clap::Parser;
use once_cell::sync::Lazy;
use url::Url;

#[derive(Parser)]
#[clap(name = clap::crate_name!())]
#[clap(version = clap::crate_version!())]
#[clap(propagate_version = true)]
/// Generates raw bindings for jawt-sys.
struct Cli {
    #[clap(long, default_value("https://github.com/openjdk/jdk"))]
    /// The URL of the OpenJDK GitHub repository containing the required headers.
    repo: Url,
    #[clap(long, default_value("jdk-17+35"))]
    /// The name of the tag to extract the required headers from.
    tag: String,
    #[clap(long, default_value(sys_rust_target()))]
    /// The target Rust version.
    rust_target: RustTarget,
}

fn sys_manifest_dir() -> &'static Path {
    static SYS_MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("jawt-sys")
            .canonicalize()
            .expect("cannot canonicalize the path to jawt-sys")
    });
    &SYS_MANIFEST_DIR
}

fn sys_rust_target() -> &'static str {
    static SYS_RUST_TARGET: Lazy<String> = Lazy::new(|| {
        let content = fs::read(sys_manifest_dir().join("Cargo.toml"))
            .expect("failed to read jawt-sys's manifest");
        let content: toml::Value =
            toml::from_slice(&content).expect("jawt-sys's manifest contains an invalid toml");

        content
            .get("package")
            .and_then(|p| p.get("rust-version"))
            .and_then(|v| v.as_str())
            .expect("jawt-sys's manifest doesn't contain the package.rust-version field")
            .to_string()
    });
    &SYS_RUST_TARGET
}

fn main() -> anyhow::Result<()> {
    let _ = Cli::parse();
    Ok(())
}
