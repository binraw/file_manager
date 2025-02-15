
use utils::lib::{preamble, routine};
mod utils;
use std::io;

fn main() 
{
    let mut param = String::new();
    loop
    {
        preamble();
        param.clear();
        io::stdin().read_line(&mut param).expect("Erreur de lecture");
        let param = param.trim();
        if routine(param) == 1
        {
            break;
        }
    }
}

