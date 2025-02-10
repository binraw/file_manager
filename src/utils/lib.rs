
use std::fs::{self};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum Extension {
    Png,
    Pdf,
    Txt,
    Img,

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
{ // ca ne fonctionne pas ca boucle sur les extension et quand ca trouve la bonne ca stop le programme
    // donc fonctionne sur un seul file pour l'instand
    let path = "./";

    if let Ok(entries) = fs::read_dir(path) 
    {
        let mut file_count = 0;
        for entry in entries 
        {
            if let Ok(entry) = entry 
            {
                let catalog = entry.path();  
                let file = catalog.display().to_string();
                file_count += 1;
                println!("{}", file_count); 
                for ext in Extension::all() 
                {
                    print!("{}",file);
                    control_typefile(&file, ext);
                }
            }
        }
    }
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
        return;
    }
    else 
    {
        return;
    }
}

impl Extension {
    
    fn from_str(ext: &str) -> Option<Extension> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Extension::Png),
            "pdf" => Some(Extension::Pdf),
            "txt" => Some(Extension::Txt),
            "img" => Some(Extension::Img),
            _ => None,
        }
    }
}

impl Extension {
    pub fn all() -> Vec<Extension> {
        vec![
            Extension::Png,
            Extension::Pdf,
            Extension::Txt,
            Extension::Img,
        ]
    }
}

pub fn control_extension(file: &str, ext: Extension)
{
    let path = Path::new(file);

    match path.extension().and_then(|file_ext| file_ext.to_str()) {
        Some(extension_str) => match Extension::from_str(extension_str) {
            Some(file_extension) if file_extension == ext => {
                match file_extension {
                    Extension::Png => move_file(file, "PNG_Collection"),
                    Extension::Pdf => move_file(file, "PDF_Collection"),
                    Extension::Txt => move_file(file, "TXT_Collection"),
                    Extension::Img => move_file(file, "IMG_Collection"),
                }
            }
            Some(_) => println!("L'extension ne correspond pas à ce que l'on attend ❌"),
            None => println!("Extension inconnue ❌"),
        },
        None => println!("Le fichier n'a pas d'extension ❌"),
    }
}
