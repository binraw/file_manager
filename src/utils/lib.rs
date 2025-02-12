
use std::fs::{self};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum Extension 
{
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
{
    let path = "./";
    if let Ok(entries) = fs::read_dir(path)
    {
        for entry in entries
        {
            if let Ok(entry) = entry 
            {
                let catalog = entry.path();  
                let file = catalog.display().to_string(); 
                if let Some(extension_str) = catalog.extension().and_then(|e| e.to_str())
                {
                    if let Some(ext) = Extension::from_str(extension_str)
                    {
                        control_typefile(&file, ext);
                    } 
                    else 
                    {
                        println!("❌ Extension inconnue pour : {}", file);
                    }
                } 
                else
                {
                    println!("❌ Le fichier {} n'a pas d'extension", file);
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

impl Extension 
{
    
    fn from_str(ext: &str) -> Option<Extension> 
    {
        match ext.to_lowercase().as_str() 
        {
            "png" => Some(Extension::Png),
            "pdf" => Some(Extension::Pdf),
            "txt" => Some(Extension::Txt),
            "img" => Some(Extension::Img),
            _ => None,
        }
    }
}

impl Extension 
{
    pub fn all() -> Vec<Extension> 
    {
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

    match path.extension().and_then(|file_ext| file_ext.to_str()) 
    {
        Some(extension_str) => match Extension::from_str(extension_str) 
        {
            Some(file_extension) if file_extension == ext => 
            {
                match file_extension 
                {
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

pub fn show_size_file(repertory:&str)
{
    let path = Path::new(repertory);
    if let Ok(entries) = fs::read_dir(path)
    {
        for entry in entries
        {
            if let Ok(entry) = entry
            {
                let path_file = entry.path();
                if let Ok(metadata) = fs::metadata(&&path_file)
                {
                    let size = metadata.len();
                    if metadata.is_file()
                    {
                        println!("La taille du fichier : {} - {} octets", path_file.display(), size);
                    }
                    else if metadata.is_dir()
                    {
                        println!("La taille du dossier : {} - {} octets", path_file.display(), size);
                    }
                    else
                    {
                        println!(" Impossible de récupérer la taille de {}", path_file.display());
                    }
                }
            }
        }
    }
    else 
    {
        println!("❌ Impossible de lancer show_size_file.");
    }
}

pub fn remove_file_directory(file_directory:&str)
{
    let path = Path::new(file_directory);
    if path.exists()
    {
        if path.is_file()
        {
                match fs::remove_file(path)
                {
                    Ok(_) => println!("Fichier supprimé: {}", file_directory),
                    Err(e) => println!("Erreur de suppression du fichier {}: {}", file_directory, e),
                }
        }
        else if path.is_dir()  
        {
            match fs::remove_dir_all(path)
            {
                Ok(_) => println!("Dossier supprimé: {}", file_directory),
                Err(e) => println!("Erreur de suppression du dossier {}: {}", file_directory, e),
            }
        }
        else  
        {
            println!("Ce n'est ni un dossier ni un fichier.");    
        }
    }
    else 
    {
        println!("Ce chemin de file/dossier n'existe pas.");
    }
}

//  fct pas encore test mais normalement ok.
pub fn copy_file(file:&str, dest:&str) 
{
    let src_path = Path::new(file);
    let dest_path = Path::new(dest);

    if src_path.is_file()
    {
        match fs::copy(src_path, dest_path) 
        {
            Ok(_) => println!("Copie effectuer avec succes."),
            Err(e) => println!("Erreur lors de la copie: {}", e),
        }
    }
    else 
    {
        println!("Le fichier source {} n'existe pas ou n'est pas un fichier.", file);    
    }
}

pub fn copy_repertory(source: &str, destination: &str) 
{
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if source_path.is_dir() 
    {
        if let Err(e) = fs::create_dir_all(destination_path) 
        {
            println!("❌ Impossible de créer le dossier {}: {}", destination, e);
            return;
        }

        for entry in fs::read_dir(source_path).expect("Impossible de lire le dossier") 
        {
            if let Ok(entry) = entry {
                let source_file = entry.path();
                let dest_file = destination_path.join(entry.file_name());

                if source_file.is_file() 
                {
                    copy_file(source_file.to_str().unwrap(), dest_file.to_str().unwrap());
                } 
                else if source_file.is_dir() 
                {
                    copy_repertory(source_file.to_str().unwrap(), dest_file.to_str().unwrap());
                }
            }
        }

        println!("✅ Dossier copié de {} vers {}", source, destination);
    } 
    else 
    {
        println!("⚠️ Le dossier source {} n'existe pas.", source);
    }
}
