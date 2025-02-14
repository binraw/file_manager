
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::io;

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

pub fn algo_range(path:&str) 
{
    // let path = "./";
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


pub fn create_dir(filename:&str)
{
    let path = Path::new(filename);

    if !path.exists()
    {
        match fs::create_dir(path)
        {
            Ok(_) => println!("Dossier {} cree avec succes ", filename),
            Err(e) => println!("Erreur a la creation {}", e),
        }
    }
    else 
    {
        println!("Le Dossier {} existe deja.", filename);
    }
}

pub fn preamble()
{
    println!("Veuillez choisir l'action voulu:");
    println!("Pour ranger tout les fichiers grace a leurs extensions tapez: 1 ");
    println!("Pour supprimer un dossier ou un fichier tapez: 2");
    println!("Pour voir la taille de tout les fichiers/dossiers tapez: 3");
    println!("Pour copier un fichier tapez: 4");
    println!("Pour copier un dossier tapez: 5");
    println!("Pour supprimer un dossier ou un fichier tapez: 6");
    println!("Pour créé un dossier tapez: 7");
}

pub fn choose_one()
{
    println!("Vous venez de choisir de ranger tout les fichiers pour validez tapez de nouveau: 1");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "1"
    {
        println!("indiquez l'endroit dans votre ordinateur que vous voulez ranger (sachant que si vous voulez rangez le dossier actuel indiquez : './' ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        algo_range(action);
    }
    else {
        return;
    }
}

pub fn choose_two()
{
    println!("Vous venez de choisir l'action qui permet de supprimez un dossier/fichier pour validez tapez de nouveau: 2");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "2"
    {
        println!("indiquez le nom du dossier ou fichier a supprimer (son chemin): ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        remove_file_directory(action);
    }
    else {
        return;
    }
}

pub fn choose_three()
{
    println!("Vous venez de choisir l'action qui permet de voir toutes les infos des fichiers/dossiers pour validez tapez de nouveau: 3");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "3"
    {
        println!("indiquez le nom du dossier dans lequel vous voulez toutes les infos: ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        show_size_file(action);
    }
    else {
        return;
    }
}

pub fn choose_four()
{
    println!("Vous venez de choisir l'action qui permet de copier un fichier pour validez tapez de nouveau: 4");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "4"
    {
        println!("Premierement indiquez le nom du fichier a copier: ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        println!("Maintenant indiquez le lieu (chemin) ou vous voulez le copier:");
        let mut path = String::new();
        io::stdin().read_line(&mut path).expect("Erreur de lecture");
        let path = path.trim();
        copy_file(action, path);
    }
    else {
        return;
    }
}

pub fn choose_five()
{
    println!("Vous venez de choisir l'action qui permet de copier un dossier pour validez tapez de nouveau: 5");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "5"
    {
        println!("Premierement indiquez le nom du dossier a copier: ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        println!("Maintenant indiquez le lieu (chemin) ou vous voulez le copier:");
        let mut path = String::new();
        io::stdin().read_line(&mut path).expect("Erreur de lecture");
        let path = path.trim();
        copy_repertory(action, path);
    }
    else {
        return;
    }
}

pub fn choose_six()
{
    println!("Vous venez de choisir l'action qui permet de SUPPRIMER un FICHIER ou un DOSSIER pour validez tapez de nouveau: 6");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "6"
    {
        println!("Premierement indiquez le nom du dossier ou fichier a supprimer: ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        remove_file_directory(action);
    }
    else {
        return;
    }
}

pub fn choose_seven()
{
    println!("Vous venez de choisir l'action qui permet de creer un dossier pour validez tapez de nouveau: 7");
    println!("Si ce n'est pas l'action voulu pour revenir en arrière tapez sur la touche de votre choix:");
    let mut valid = String::new();
    io::stdin().read_line(&mut valid).expect("Erreur de lecture");
    let valid = valid.trim();
    if valid == "6"
    {
        println!("Premierement indiquez le nom du dossier a creer: ");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erreur de lecture");
        let action = action.trim();
        create_dir(action);
    }
    else {
        return;
    }
}

pub fn routine(param: &str) -> u32
{
    if param == "1"
    {
        choose_one();
        return 0;
    }
    else if param == "2"
    {
        choose_two();
        return 0;
    }
    else if param == "3" 
    {
        choose_three();
        return 0;
    }
    else if  param == "4"
    {
        choose_four();
        return 0;
    }
    else if param == "5" 
    {
        choose_five();
        return 0;
    }
    else if param == "6" 
    {
        choose_six();
        return 0;
    }
    else if param == "7" 
    {
        choose_seven();
        return 0;
    }
    else 
    {
        return 1;    
    }
}