use chrono::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct ErrorLogger;

impl ErrorLogger {
    pub fn write(file_name: &str, error: String) {
        let path = format!("logs/{}.log", file_name).to_owned();
        let file = OpenOptions::new().write(true).append(true).open(&path);

        match file {
            Ok(mut f) => {
                if let Err(e) = writeln!(f, "{}: {}", Utc::now().date(), error) {
                    println!(
                        "Une erreur est survenue durant l'écriture dans le fichier log \"{}\":",
                        path
                    );
                    println!("\t{}", e.to_string());
                }
            }
            // Le fichier n'existe pas, on le créer
            Err(_) => {
                match File::create(&path) {
                    Ok(mut f) => {
                        if let Err(e) = writeln!(f, "{}: {}", Utc::now().date(), error) {
                            println!("Une erreur est survenue durant l'écriture dans le fichier log \"{}\":", path);
                            println!("\t{}", e.to_string());
                        }
                    }
                    Err(e) => {
                        println!(
                            "Une erreur est survenue durant la création du fichier log \"{}\":",
                            path
                        );
                        println!("\t{}", e.to_string());
                    }
                }
            }
        }
    }
}
