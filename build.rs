use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    
    if target.contains("m68k") && target.contains("amiga") {
        // Use GNU ar (included in most m68k cross-toolchains)
        // VBCC users should ensure 'ar' is available in PATH
        let ar = env::var("AR").unwrap_or_else(|_| "ar".to_string());
        let vc = env::var("VC").unwrap_or_else(|_| "vc".to_string());
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        
        let sources = [
            "c/amiga_dos_c.c",
            "c/amiga_graphics_c.c",
            "c/amiga_audio_c.c",
            "c/amiga_intuition_c.c",
            "c/amiga_exceptions_c.c",
        ];
        
        let mut objects = Vec::new();
        
        // Compile each C source to an object file
        for src in &sources {
            let obj_name = format!("{}.o", 
                src.strip_prefix("c/").unwrap().strip_suffix(".c").unwrap());
            let obj_path = out_dir.join(&obj_name);
            
            let status = Command::new(&vc)
                .arg("-cpu=68000")
                .arg("-O")
                .arg("-c")
                .arg("-Ic/")
                .arg(src)
                .arg("-o")
                .arg(&obj_path)
                .status()
                .expect("Failed to invoke VBCC");
            
            if !status.success() {
                panic!("VBCC compilation failed for {}", src);
            }
            objects.push(obj_path);
        }
        
        // Archive objects into a static library
        // GNU ar syntax: ar rcs archive.a obj1.o obj2.o
        // Flags MUST come before archive path
        let lib_path = out_dir.join("libamiga-bindings.a");
        
        let status = Command::new(&ar)
            .arg("rcs")
            .arg(&lib_path)
            .args(&objects)
            .status()
            .expect("Failed to invoke archiver");
        
        if !status.success() {
            panic!("Failed to create static library");
        }
        
        // Tell cargo where to find the library and what to link
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=amiga-bindings");
        println!("cargo:rerun-if-changed=c/");
    }
}