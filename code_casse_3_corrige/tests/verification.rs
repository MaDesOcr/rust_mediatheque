//! Tests de vérification. NE PAS MODIFIER CE FICHIER.
//! Lancer : cargo test · Un seul test : cargo test t07

use mediatheque::erreur::ErreurMediatheque;
use mediatheque::livre::{Genre, Livre};
use mediatheque::mediatheque::*;

fn dune() -> Livre {
    Livre::new(1, "Dune", "Frank Herbert", 1965, Genre::Roman)
}

/// Médiathèque de test : 4 livres, ids 1 à 4.
fn jeu() -> Mediatheque {
    let mut m = Mediatheque::new();
    m.ajouter(dune());
    m.ajouter(Livre::new(2, "Sapiens", "Y. N. Harari", 2011, Genre::Essai));
    m.ajouter(Livre::new(
        3,
        "Persepolis",
        "M. Satrapi",
        2000,
        Genre::BandeDessinee,
    ));
    m.ajouter(Livre::new(
        4,
        "Le Petit Prince",
        "A. de Saint-Exupéry",
        1943,
        Genre::Roman,
    ));
    m
}

#[test]
fn t01_genre_depuis_texte() {
    assert_eq!(Genre::depuis_texte("roman"), Some(Genre::Roman));
    assert_eq!(Genre::depuis_texte("bd"), Some(Genre::BandeDessinee));
    assert_eq!(Genre::depuis_texte("poésie"), None);
}

#[test]
fn t02_genre_majuscules_et_espaces() {
    assert_eq!(
        Genre::depuis_texte(" Essai "),
        Some(Genre::Essai),
        "les espaces sont ignorés"
    );
    assert_eq!(
        Genre::depuis_texte("BD"),
        Some(Genre::BandeDessinee),
        "les majuscules sont ignorées"
    );
}

#[test]
fn t03_libelles() {
    assert_eq!(Genre::Roman.libelle(), "Roman");
    assert_eq!(Genre::BandeDessinee.libelle(), "Bande dessinée");
}

#[test]
fn t04_nouveau_livre() {
    let l = dune();
    assert!(l.est_disponible(), "un nouveau livre est disponible");
    assert_eq!(l.titre, "Dune");
}

#[test]
fn t05_age() {
    assert_eq!(dune().age(2026), 61);
    assert_eq!(
        dune().age(1900),
        0,
        "une parution postérieure donne un âge de 0"
    );
}

#[test]
fn t06_affichage_livre() {
    assert_eq!(
        dune().to_string(),
        "n°1 « Dune » de Frank Herbert (1965, Roman)"
    );
}

#[test]
fn t07_affichage_erreurs() {
    assert_eq!(
        ErreurMediatheque::LivreIntrouvable(7).to_string(),
        "aucun livre n°7"
    );
    assert_eq!(
        ErreurMediatheque::LimiteAtteinte.to_string(),
        "limite d'emprunts atteinte"
    );
    assert_eq!(
        ErreurMediatheque::DejaEmprunte("Dune".into()).to_string(),
        "« Dune » est déjà emprunté"
    );
}

#[test]
fn t08_ajouter_et_trouver() {
    let mut m = Mediatheque::new();
    assert_eq!(m.ajouter(dune()), 1);
    assert_eq!(m.trouver(1).map(|l| l.titre.as_str()), Some("Dune"));
    assert_eq!(m.trouver(99), None);
}

#[test]
fn t09_nb_disponibles() {
    let mut m = jeu();
    assert_eq!(m.nb_disponibles(), 4);
    m.emprunter(2).unwrap();
    assert_eq!(m.nb_disponibles(), 3);
}

#[test]
fn t10_emprunter() {
    let mut m = jeu();
    assert_eq!(m.emprunter(1), Ok(()));
    assert!(!m.trouver(1).unwrap().est_disponible());
    assert_eq!(m.nb_emprunts, 1);
}

#[test]
fn t11_emprunter_erreurs() {
    let mut m = jeu();
    assert_eq!(
        m.emprunter(42),
        Err(ErreurMediatheque::LivreIntrouvable(42))
    );
    m.emprunter(1).unwrap();
    assert_eq!(
        m.emprunter(1),
        Err(ErreurMediatheque::DejaEmprunte("Dune".into()))
    );
    assert_eq!(m.nb_emprunts, 1, "un emprunt refusé ne compte pas");
}

#[test]
fn t12_limite_emprunts() {
    let mut m = jeu();
    m.emprunter(1).unwrap();
    m.emprunter(2).unwrap();
    m.emprunter(3).unwrap();
    assert_eq!(
        m.emprunter(4),
        Err(ErreurMediatheque::LimiteAtteinte),
        "LIMITE_EMPRUNTS = 3 : le 4e emprunt doit être refusé"
    );
}

#[test]
fn t13_rendre() {
    let mut m = jeu();
    m.emprunter(1).unwrap();
    assert_eq!(m.rendre(1), Ok(()));
    assert!(m.trouver(1).unwrap().est_disponible());
    assert_eq!(m.nb_emprunts, 0);
}

#[test]
fn t14_rendre_un_livre_non_emprunte() {
    let mut m = jeu();
    assert_eq!(
        m.rendre(2),
        Err(ErreurMediatheque::PasEmprunte("Sapiens".into()))
    );
    assert_eq!(m.rendre(42), Err(ErreurMediatheque::LivreIntrouvable(42)));
}

#[test]
fn t15_plus_ancien() {
    assert_eq!(jeu().plus_ancien().unwrap().id, 4);
    assert_eq!(Mediatheque::new().plus_ancien(), None);
}

#[test]
fn t16_analyser_ligne() {
    assert_eq!(
        analyser_ligne("1;Dune;Frank Herbert;1965;Roman"),
        Ok(dune())
    );
    let l = analyser_ligne(" 2 ; Sapiens ; Y. N. Harari ; 2011 ; essai ").unwrap();
    assert_eq!((l.id, l.titre.as_str(), l.annee), (2, "Sapiens", 2011));
}

#[test]
fn t17_analyser_ligne_invalide() {
    let invalide = |l: &str| Err(ErreurMediatheque::FormatInvalide(l.to_string()));
    assert_eq!(
        analyser_ligne("1;Dune;Frank Herbert;MCMLXV;Roman"),
        invalide("1;Dune;Frank Herbert;MCMLXV;Roman")
    );
    assert_eq!(
        analyser_ligne("1;Dune;Frank Herbert;1965;Poésie"),
        invalide("1;Dune;Frank Herbert;1965;Poésie")
    );
    assert_eq!(
        analyser_ligne("1;Dune"),
        invalide("1;Dune"),
        "une ligne trop courte est invalide"
    );
    assert_eq!(
        analyser_ligne("1;a;b;1965;Roman;en trop"),
        invalide("1;a;b;1965;Roman;en trop")
    );
}

#[test]
fn t18_charger_texte() {
    let contenu = "id;titre;auteur;annee;genre\n1;Dune;Frank Herbert;1965;Roman\n\n2;Sapiens;Harari;2011;Essai\n3;Erreur;X;?;Roman\n";
    let (m, erreurs) = charger_texte(contenu);
    assert_eq!(
        m.livres.len(),
        2,
        "en-tête et ligne vide ignorés, ligne invalide écartée"
    );
    assert_eq!(
        erreurs,
        vec![ErreurMediatheque::FormatInvalide(
            "3;Erreur;X;?;Roman".into()
        )]
    );
}

#[test]
fn t19_charger_fichier() {
    let (m, erreurs) = charger_fichier("donnees/catalogue.csv").unwrap();
    assert_eq!(m.livres.len(), 5);
    assert_eq!(erreurs.len(), 3);
    assert_eq!(
        charger_fichier("absent.csv").unwrap_err(),
        ErreurMediatheque::Fichier("absent.csv".into())
    );
}
