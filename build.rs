use native_tls;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{copy, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use ureq::AgentBuilder;
use zip_extract::extract;

use tempfile::tempdir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = env::var("TARGET")?;
    if target.contains("pc-windows-gnu") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
        let temp = tempdir()?;
        let temp_dir = temp.path();

        //GETTING LIBRARY DIRECTORIES GIVEN THE BUILD TARGET
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();

        let mut zip_extract = manifest_dir.clone();
        zip_extract.push("gnu-mingw");
        lib_dir.push("gnu-mingw");
        dll_dir.push("gnu-mingw");

        let mut part = String::new();
        if target.contains("x86_64") {
            part += "x86_64-w64-mingw32";
            lib_dir.push("x86_64-w64-mingw32");
            dll_dir.push("x86_64-w64-mingw32");
        } else {
            part += "x86_64-w64-mingw32";
            lib_dir.push("i686-w64-mingw32");
            dll_dir.push("i686-w64-mingw32");
        }
        lib_dir.push("lib");
        dll_dir.push("bin");
        println!("DEBUG: Managed Dirs!");

        if !zip_extract.exists() {
            std::fs::create_dir_all(&zip_extract)?;
        }

        println!("DEBUG: Created Dirs!");

        if !lib_dir.exists() {
            //NOW THAT WE HAVE THE OUTPUT DIRECTORIES, WE NEED TO EXTRACT THE ZIP FILES INTO THE
            //CORRECT DIRECTORIES
            //returns zip files
            let url_sdl = download_files(temp_dir,"https://github.com/libsdl-org/SDL/releases/download/release-2.26.4/SDL2-devel-2.26.4-mingw.zip")?;
            url_sdl.sync_all()?;
            let url_ttf = download_files(temp_dir,"https://github.com/libsdl-org/SDL_ttf/releases/download/release-2.20.2/SDL2_ttf-devel-2.20.2-mingw.zip")?;
            url_ttf.sync_all()?;
            let url_image = download_files(temp_dir,"https://github.com/libsdl-org/SDL_image/releases/download/release-2.6.3/SDL2_image-devel-2.6.3-mingw.zip")?;
            url_image.sync_all()?;

            println!("DEBUG: Downloaded Files!");

            let zip_vec = vec![&url_sdl, &url_ttf, &url_image];
            for file in zip_vec {
                extract(file, &zip_extract, true)?;
                println!("DEBUG: Extracted 'a' File");
            }
            temp.close()?;
        }

        //SEARCHES AND LINKS LIBRARIES WITH CARGO
        println!("cargo:rustc-link-search=all={}", lib_dir.display());

        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
    Ok(())
}

pub fn download_files(path: &Path,url: &str) -> Result<File, Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build();
    let resp = agent.get(url).call()?;

    let content_disposition = resp.header("content-disposition").unwrap();
    let file_name = content_disposition
        .split("; ")
        .find(|s| s.starts_with("filename="))
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .trim_matches('"');

    // Create a new File object to store the downloaded zip file

    let mut path = path.to_path_buf();
    path.push(&file_name);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    // Use a BufWriter to efficiently write the contents of the response to the file
    let mut writer = BufWriter::new(file);
    copy(&mut resp.into_reader(), &mut writer)?;

    Ok(writer.into_inner()?)
}
