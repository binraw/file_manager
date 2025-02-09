use std::fs;
use std::path::{Path, PathBuf};
mod utils;

fn main() {


}

fn move_file(file: &str, dossier: &str)
{
    if !Path::new(dossier).exists()
    {
        fs::create_dir(dossier).expect("error creation dossier");
        println!("dossier créé");
        let file_destination = PathBuf::from(dossier).join(file);
        fs::rename(file, &file_destination).expect("Impossible de déplacer le fichier");
    }
}