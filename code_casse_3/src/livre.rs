//! Un livre et son genre.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Genre {
    Roman,
    Essai,
    BandeDessinee,
}

impl Genre {
    /// Convertit un texte en genre, sans tenir compte des majuscules ni des espaces
    /// autour : « roman », « essai », « bd ». Tout autre texte donne None.
    pub fn depuis_texte(texte: &str) -> Option<Genre> {
        match texte {
            "roman" => Genre::Roman,
            "essai" => Genre::Essai,
            "bd" => Genre::BandeDessinee,
            _ => None,
        }
    }

    /// Libellé affiché : « Roman », « Essai », « Bande dessinée ».
    pub fn libelle(&self) -> &'static str {
        match self {
            Genre::Roman => "Roman",
            Genre::Essai => "Essai",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Livre {
    pub id: u32,
    pub titre: String,
    pub auteur: String,
    pub annee: u16,
    pub genre: Genre,
    pub disponible: bool,
}

impl Livre {
    /// Crée un livre, disponible par défaut.
    pub fn new(id: u32, titre: &str, auteur: &str, annee: u16, genre: Genre) -> Livre {
        Livre {
            id,
            titre: titre.to_string(),
            auteur: auteur.to_string(),
            annee,
            genre,
            disponible: true,
        }
    }

    /// Vrai si le livre est disponible.
    pub fn est_disponible() -> bool {
        self.disponible
    }

    /// Âge du livre en années, par rapport à `annee_courante`
    /// (0 si l'année de parution est postérieure).
    pub fn age(&self, annee_courante: u16) -> u16 {
        annee_courante - self.annee
    }
}

/// Affichage : « n°3 « Dune » de Frank Herbert (1965, Roman) ».
impl fmt::Display for Livre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "n°{} « {} » de {} ({}, {})",
            self.id,
            self.auteur,
            self.titre,
            self.annee,
            self.genre.libelle()
        );
    }
}
