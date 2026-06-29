use std::io::{self, Write};
mod library;
use library::{Livre, Statut, Affichable};

fn main() {
    let mut bibliotheque: Vec<Livre> = Vec::new();
    bibliotheque.push(Livre {
        titre: String::from("Le Petit Prince"),
        auteur: String::from("Antoine de Saint-Exupéry"),
        annee: 1943,
        statut: Statut::Disponible,
    });

    loop {
        afficher_menu();
        let choix = lire_entree("Votre choix : ");
        match choix.trim() {
            "1" => afficher_livres(&bibliotheque),
            "2" => ajouter_livre(&mut bibliotheque),
            "3" => rechercher_livre(&bibliotheque),
            "4" => modifier_statut(&mut bibliotheque),
            "5" => afficher_statistiques(&bibliotheque),
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide. Veuillez réessayer.\n"),
        }
    }
}

fn afficher_menu() {
    println!("=== Gestionnaire de bibliothèque ===");
    println!("1. Afficher tous les livres");
    println!("2. Ajouter un livre");
    println!("3. Rechercher un livre par titre");
    println!("4. Modifier le statut d’un livre");
    println!("5. Afficher les statistiques");
    println!("6. Quitter");
    println!("====================================");
}

fn lire_entree(consigne: &str) -> String {
    print!("{}", consigne);
    io::stdout().flush().unwrap();
    let mut tampon = String::new();
    io::stdin().read_line(&mut tampon).expect("Échec de la lecture");
    tampon.trim().to_string()
}

fn afficher_livres(bibliotheque: &Vec<Livre>) {
    if bibliotheque.is_empty() {
        println!("La bibliothèque est vide.\n");
        return;
    }
    println!("\n--- Liste des livres ---");
    for (index, livre) in bibliotheque.iter().enumerate() {
        println!("{}. {} par {} ({}) - [{}]", 
            index + 1, 
            livre.titre, 
            livre.auteur, 
            livre.annee, 
            livre.statut.vers_chaine()
        );
    }
    println!("");
}
