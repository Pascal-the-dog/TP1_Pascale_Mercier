use std::io::{self, Write};

mod library;
use library::{Livre, Statut, Description};

fn main() {
    let mut bibliotheque: Vec<Livre> = Vec::new();

    // Ajout de deux livres
    bibliotheque.push(Livre {
        titre: String::from("Le Petit Prince"),
        auteur: String::from("Antoine de Saint-Exupéry"),
        annee: 1943,
        statut: Statut::Disponible,
    });
    bibliotheque.push(Livre {
        titre: String::from("1984"),
        auteur: String::from("George Orwell"),
        annee: 1949,
        statut: Statut::Emprunte,
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
                println!("Fermeture de l'application. Au revoir !");
                break;
            }
            _ => println!("Choix invalide. Veuillez entrer un nombre entre 1 et 6.\n"),
        }
    }
}

fn afficher_menu() {
    println!("\n=== Gestionnaire de bibliothèque ===");
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
        println!("La bibliothèque ne contient aucun livre.");
        return;
    }

    println!("\n--- Liste des livres ---");
    for (index, livre) in bibliotheque.iter().enumerate() {
        println!("{}. {}", index + 1, livre.obtenir_resume());
    }
}

fn ajouter_livre(bibliotheque: &mut Vec<Livre>) {
    println!("\n--- Ajouter un nouveau livre ---");
    let titre = lire_entree("Titre : ");
    let auteur = lire_entree("Auteur : ");
    let annee_str = lire_entree("Année de publication : ");

    let annee: u32 = annee_str.parse().unwrap_or(0);

    let nouveau_livre = Livre {
        titre,
        auteur,
        annee,
        statut: Statut::Disponible,
    };

    bibliotheque.push(nouveau_livre);
    println!("Livre ajouté avec succès !");
}

fn rechercher_livre(bibliotheque: &Vec<Livre>) {
    println!("\n--- Rechercher un livre ---");
    let recherche = lire_entree("Entrez le titre (ou partie du titre) : ").to_lowercase();
    let mut trouve = false;

    for livre in bibliotheque {
        if livre.titre.to_lowercase().contains(&recherche) {
            println!("[Trouvé] {}", livre.obtenir_resume());
            trouve = true;
        }
    }

    if !trouve {
        println!("Aucun livre ne correspond à votre recherche.");
    }
}

fn modifier_statut(bibliotheque: &mut Vec<Livre>) {
    println!("\n--- Modifier le statut d'un livre ---");
    afficher_livres(bibliotheque);
    
    if bibliotheque.is_empty() {
        return;
    }

    let index_str = lire_entree("Entrez le numéro du livre à modifier : ");
    let index: usize = index_str.parse().unwrap_or(0);

    if index > 0 && index <= bibliotheque.len() {
        let livre = &mut bibliotheque[index - 1];
        
        livre.statut = match livre.statut {
            Statut::Disponible => Statut::Emprunte,
            Statut::Emprunte => Statut::Disponible,
        };
        
        println!("Le statut du livre a été mis à jour !");
    } else {
        println!("Numéro de livre invalide.");
    }
}

fn afficher_statistiques(bibliotheque: &Vec<Livre>) {
    println!("\n--- Statistiques de la bibliothèque ---");
    
    let total = bibliotheque.len();
    let mut dispos = 0;
    let mut empruntes = 0;

    for livre in bibliotheque {
        match livre.statut {
            Statut::Disponible => dispos += 1,
            Statut::Emprunte => empruntes += 1,
        }
    }

    let stats_tuple: (usize, u32, u32) = (total, dispos, empruntes);

    println!("Nombre total de livres : {}", stats_tuple.0);
    println!("Livres disponibles     : {}", stats_tuple.1);
    println!("Livres empruntes       : {}", stats_tuple.2);
}

