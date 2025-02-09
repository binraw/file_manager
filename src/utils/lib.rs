use std::fs::{self, metadata};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum Extension {
    png,
    pdf,
    txt,
    img,

}

pub fn move_file(file: &str, dossier: &str)
{
    if !Path::new(dossier).exists()
    {
        fs::create_dir(dossier).expect("error creation dossier");
        println!("dossier créé");
    }
    let file_destination = PathBuf::from(dossier).join(file);
    fs::rename(file, &file_destination).expect("Impossible");
}

pub fn algo_range()
{

}

pub fn control_typefile(file: &str, ext:Extension)
{
    let metadata = fs::metadata(file).expect("Unable to retrieve metadata.");
    let file_type = metadata.file_type();
    if file_type.is_file()
    {
        control_extension(file, ext);
    }
    else if file_type.is_dir()
    {

    }
    else 
    {
        
    }
}

impl Extension {
    
    fn from_str(ext: &str) -> Option<Extension> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Extension::png),
            "pdf" => Some(Extension::pdf),
            "txt" => Some(Extension::txt),
            "img" => Some(Extension::img),
            _ => None,
        }
    }
}

pub fn control_extension(file: &str, ext: Extension)
{
    let path = Path::new(file);

    match path.extension().and_then(|file_ext| file_ext.to_str()) {
        Some(extension_str) => match Extension::from_str(extension_str) {
            Some(file_extension) if file_extension == ext => {
                match file_extension {
                    Extension::png => move_file(file, "PNG_Collection"),
                    Extension::pdf => move_file(file, "PDF_Collection"),
                    Extension::txt => move_file(file, "TXT_Collection"),
                    Extension::img => move_file(file, "IMG_Collection"),
                }
            }
            Some(_) => println!("L'extension ne correspond pas à ce que l'on attend ❌"),
            None => println!("Extension inconnue ❌"),
        },
        None => println!("Le fichier n'a pas d'extension ❌"),
    }
}
