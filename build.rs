// build.rs
extern crate embed_manifest;

fn main() {
	
	#[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.compile().unwrap();
    }
	
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        embed_manifest::embed_manifest_file("manifest.xml")
            .expect("unable to embed manifest file");
    }
    
    println!("cargo:rerun-if-changed=manifest.xml");
}