use std::fs;
use std::path::Path;
fn main() {
    let dossier = "test";
    if !Path::new(dossier).exists()
    {
        fs::create_dir(dossier).expect("error creation dossier");
        println!("dossier créé");
    }
    else
    {
        println!("dossier existe");
    }

}

