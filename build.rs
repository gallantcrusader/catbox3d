use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{copy, BufWriter};
use std::path::Path;
use zip::ZipArchive;
use ureq::{Error, Agent, AgentBuilder};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = env::var("TARGET")?;
    if target.contains("pc-windows") {
        
        
        
        
        

        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
        
        //CREATING TEMP DIR FOR DOWNLOADS
        let mut temp_dir = manifest_dir.clone();
        temp_dir.push("/tmp");
        std::fs::create_dir(temp_dir.clone());
        
        let url_sdl = downloadFiles(temp_dir.clone(), "https://github.com/libsdl-org/SDL/releases/download/release-2.26.4/SDL2-devel-2.26.4-mingw.zip");
        let url_ttf = downloadFiles(temp_dir.clone(), "https://github.com/libsdl-org/SDL_ttf/releases/download/release-2.20.2/SDL2_ttf-devel-2.20.2-mingw.zip");
        let url_image = downloadFiles(temp_dir.clone(), "https://github.com/libsdl-org/SDL_image/releases/download/release-2.6.3/SDL2_image-devel-2.6.3-mingw.zip");

        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();


        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            
            dll_dir.push("gnu-mingw");
        }


        lib_dir.push("lib");
        dll_dir.push("dll");


        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        
        std::fs::create_dir_all(&lib_dir);


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

pub fn downloadFiles<P: AsRef<Path>>(path: P, url: &str) -> Result<File, Box<dyn std::error::Error>>
{
    let agent = Agent::new();
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
    let file = File::create(file_name)?;

    // Use a BufWriter to efficiently write the contents of the response to the file
    let mut writer = BufWriter::new(file);
    copy(&mut resp.into_reader(), &mut writer)?;


    Ok(writer.into_inner()?)
}
