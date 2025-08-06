// Copyright (c) 2025 Gobley Contributors.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use bindgen::RustTarget;
use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli {
        repo,
        tag,
        rust_target,
    } = Cli::parse();

    env_logger::init();

    let current_platform = Platform::current();
    log::info!("Downloading headers for {current_platform:?}...");

    let header_destination_dir = sys_manifest_dir().join(".bindgen");
    log::info!("Header destination: {}", header_destination_dir.display());

    if matches!(header_destination_dir.try_exists(), Ok(true) | Err(_)) {
        fs::remove_dir_all(&header_destination_dir)
            .with_context(|| format!("failed to clean {}", header_destination_dir.display()))?;
    }
    fs::create_dir_all(&header_destination_dir).with_context(|| {
        format!(
            "failed to create a new directory at {}",
            header_destination_dir.display()
        )
    })?;
    let main_header_destination = header_destination_dir.join("bindings.h");
    fs::write(&main_header_destination, include_str!("bindings.h"))
        .with_context(|| "failed to generate bindings.h")?;

    for header in Header::all() {
        let location = header.location(&repo, &tag, current_platform)?;

        log::info!("Downloading {location}...");
        let response = reqwest::get(location.clone())
            .await
            .with_context(|| format!("failed to send a request to {location}"))?;

        response
            .error_for_status_ref()
            .with_context(|| format!("{location} responded with {}", response.status()))?;

        let body = response
            .bytes()
            .await
            .with_context(|| format!("failed to download from {location}"))?;
        let header_destination = header.destination(&header_destination_dir);
        fs::write(&header_destination, body)
            .with_context(|| format!("failed to write to {}", header_destination.display()))?;
    }

    log::info!("Generating bindings...");

    let builder = bindgen::builder()
        .header(main_header_destination.display().to_string())
        .allowlist_recursively(false)
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("")
        .raw_line("use jni_sys::*;");

    let builder = match current_platform {
        Platform::Windows => {
            builder
                .raw_line("use windows_sys::Win32::Foundation::HWND;")
                .raw_line("use windows_sys::Win32::Graphics::Gdi::{HBITMAP, HDC, HPALETTE};")
                // To avoid jawt_Win32DrawingSurfaceInfo__bindgen_ty_1 being generated as
                // a bindgen-generated wrapper struct
                .allowlist_item("HWND")
                .allowlist_item("HBITMAP")
                .allowlist_item("HDC")
                .allowlist_item("HPALETTE")
        }
        Platform::MacOS => builder,
        Platform::Unix => {
            builder.raw_line("use x11_dl::xlib::{Colormap, Display, Drawable, VisualID};")
        }
    };

    let builder = builder
        .blocklist_file(
            Header::Jni
                .destination(&header_destination_dir)
                .display()
                .to_string(),
        )
        .blocklist_file(
            Header::JniPlatform
                .destination(&header_destination_dir)
                .display()
                .to_string(),
        )
        .allowlist_file(
            Header::Jawt
                .destination(&header_destination_dir)
                .display()
                .to_string(),
        )
        .allowlist_file(
            Header::JawtPlatform
                .destination(&header_destination_dir)
                .display()
                .to_string(),
        )
        .rust_target(rust_target)
        .clang_arg(format!("-I{}", header_destination_dir.display()));

    let mut bindings = builder
        .generate()
        .with_context(|| "binding generation failed")?
        .to_string();

    // Postprocessing
    if matches!(current_platform, Platform::Windows) {
        let regex = Regex::new("pub type (HWND|HBITMAP|HDC|HPALETTE) = .*;\n").unwrap();
        bindings = regex.replace_all(&bindings, "").into_owned();
    }

    let bindings_destination_dir = sys_manifest_dir().join("src");

    fs::write(
        bindings_destination_dir.join(match current_platform {
            Platform::Windows => "bindings_windows.rs",
            Platform::MacOS => "bindings_macos.rs",
            Platform::Unix => "bindings_unix.rs",
        }),
        bindings,
    )
    .with_context(|| "failed to write bindings to a file")?;

    log::info!("Done.");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Platform {
    Windows,
    MacOS,
    Unix,
}

impl Platform {
    fn current() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::MacOS
        } else {
            Self::Unix
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Header {
    Jni,
    JniPlatform,
    Jawt,
    JawtPlatform,
}

impl Header {
    fn all() -> &'static [Self] {
        &[
            Header::Jni,
            Header::JniPlatform,
            Header::Jawt,
            Header::JawtPlatform,
        ]
    }

    fn location(self, repo: &Url, tag: &str, platform: Platform) -> anyhow::Result<Url> {
        Self::construct_location(
            repo,
            tag,
            self.module(),
            self.platform(platform),
            self.filename(),
        )
    }

    fn destination(self, destination_dir: &Path) -> PathBuf {
        destination_dir.join(self.filename())
    }

    fn module(self) -> &'static str {
        match self {
            Self::Jni | Self::JniPlatform => "java.base",
            Self::Jawt | Self::JawtPlatform => "java.desktop",
        }
    }

    fn platform(self, platform: Platform) -> &'static str {
        match (self, platform) {
            (Self::Jni | Self::Jawt, _) => "share",
            (Self::JniPlatform, Platform::Windows) => "windows",
            (Self::JniPlatform, _) => "unix",
            (Self::JawtPlatform, Platform::Windows) => "windows",
            (Self::JawtPlatform, Platform::MacOS) => "macosx",
            (Self::JawtPlatform, Platform::Unix) => "unix",
        }
    }

    fn filename(self) -> &'static str {
        match self {
            Self::Jni => "jni.h",
            Self::JniPlatform => "jni_md.h",
            Self::Jawt => "jawt.h",
            Self::JawtPlatform => "jawt_md.h",
        }
    }

    fn construct_location(
        repo: &Url,
        tag: &str,
        module: &str,
        platform: &str,
        filename: &str,
    ) -> anyhow::Result<Url> {
        let mut new_url = repo.clone();
        new_url
            .path_segments_mut()
            .ok()
            .with_context(|| "given url cannot be a base URL")?
            .extend([
                "raw", "refs", "tags", tag, "src", module, platform, "native", "include", filename,
            ]);
        Ok(new_url)
    }
}
