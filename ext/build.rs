use std::env;
use std::fs::{self};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

fn main() {
    const LIB_TYPE: &str = "shared_libs";

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = PathBuf::from(manifest_dir);

    let install_dir = project_root.join("ntgcalls");

    let target = env::var("TARGET").unwrap();

    // Detect platform //
    let (os, arch) = match target.as_str() {
        t if t.contains("linux") && t.contains("x86_64") => ("linux", "x86_64"),
        t if t.contains("linux") && t.contains("aarch64") => ("linux", "arm64"),
        t if t.contains("darwin") && t.contains("aarch64") => ("macos", "arm64"),
        t if t.contains("darwin") && t.contains("x86_64") => ("macos", "x86_64"),
        t if t.contains("windows") && t.contains("x86_64") => ("windows", "x86_64"),
        _ => panic!("Unsupported target architecture : {}", target),
    };

    let target_filename = format!("ntgcalls.{}-{}-{}.zip", os, arch, LIB_TYPE);

    // Direct Link Construction //
    if !install_dir.join("lib").exists() {
        let download_url = format!(
            "https://github.com/pytgcalls/ntgcalls/releases/latest/download/{}",
            target_filename
        );

        println!("cargo:warning=Downloading direct from : {}", download_url);
        download_file(&download_url, &install_dir);
    }

    // Link Logic //
    let lib_path = install_dir.join("lib");

    println!("cargo:rustc-link-search=native={}/", lib_path.display());
    println!("cargo:rustc-link-lib=ntgcalls");
    
    if target.contains("linux") {
        println!("cargo:rustc-link-arg-cdylib=-Wl,-rpath,$ORIGIN");
    } else if target.contains("darwin") {
        println!("cargo:rustc-link-arg-cdylib=-Wl,-rpath,@loader_path");
    }

    // OS Specific C++ Linking //
    if target.contains("apple") || target.contains("freebsd") {
        println!("cargo:rustc-link-lib=c++");
    } else if !target.contains("windows") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rerun-if-changed=ntgcalls");
}

fn download_file(url: &str, out_dir: &Path) {
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust-build-script")
        .build()
        .unwrap();

    let resp = client.get(url).send().expect("Failed to download ntgcalls");

    if !resp.status().is_success() {
        panic!("Download failed ! HTTP Status : {} & URL : {}", resp.status(), url);
    }

    let content = Cursor::new(resp.bytes().expect("Failed to read bytes"));
    let mut zip = zip::ZipArchive::new(content).expect("Failed to open zip archive");

    fs::create_dir_all(out_dir).expect("Failed to create output dir");

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = out_dir.join(file.mangled_name());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(&p).unwrap();
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}