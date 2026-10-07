//! Programme de démonstration : charge un catalogue et simule des emprunts.
//!
//! Utilisation : cargo run -- [chemin du catalogue]
//! Sans argument, le fichier donnees/catalogue.csv est utilisé.

use mediatheque::mediatheque::{charger_fichier, LIMITE_EMPRUNTS};
use std::env;
use std::process;

const ANNEE_COURANTE: u16 = 2026;

fn main() {
    let args: Vec<String> = env::args().collect();
    let chemin = if args.len() > 1 {
        args[1].clone() // [16] on ne peut pas déplacer une String hors d'un Vec
    } else {
        String::from("donnees/catalogue.csv")
    };

    // [17] mut : emprunter et rendre modifient la médiathèque
    let (mut media, erreurs) = match charger_fichier(&chemin) {
        Ok(resultat) => resultat,
        Err(e) => {
            // [15] le match doit traiter le cas d'erreur
            eprintln!("Erreur : {e}");
            process::exit(1);
        }
    };

    println!("=== Catalogue ({} livres) ===", media.livres.len());
    for livre in &media.livres {
        println!("{livre} — {} ans", livre.age(ANNEE_COURANTE));
    }
    println!("{} ligne(s) ignorée(s) :", erreurs.len());
    for e in &erreurs {
        println!("  - {e}");
    }

    if let Some(livre) = media.plus_ancien() {
        println!("Le plus ancien : {livre}");
    }

    println!("\n=== Emprunts (limite : {LIMITE_EMPRUNTS}) ===");
    for id in [1, 1, 3, 42, 4, 6] {
        match media.emprunter(id) {
            Ok(()) => println!("Emprunt du livre n°{id} : ok"),
            Err(e) => println!("Emprunt du livre n°{id} : refusé, {e}"),
        }
    }
    for id in [3, 3] {
        match media.rendre(id) {
            Ok(()) => println!("Retour du livre n°{id} : ok"),
            Err(e) => println!("Retour du livre n°{id} : refusé, {e}"),
        }
    }
    println!(
        "Disponibles : {} · Empruntés : {}",
        media.nb_disponibles(),
        media.nb_emprunts
    );
}
