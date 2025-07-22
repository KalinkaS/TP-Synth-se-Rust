use std::io;

struct CompteBancaire {
    id: u32,
    nom: String,
    solde: f64,
}

fn main() {
    let mut comptes: Vec<CompteBancaire> = Vec::new();
    let mut prochain_id = 1;

    loop {
        println!("\n--- Menu ---");
        println!("1. Créer un compte");
        println!("2. Afficher les comptes");
        println!("3. Déposer de l'argent");
        println!("4. Retirer de l'argent");
        println!("5. Fermer un compte");
        println!("6. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        if choix == "1" {
            println!("Entrez le nom du titulaire :");
            let mut nom = String::new();
            io::stdin().read_line(&mut nom).unwrap();
            let nom = nom.trim().to_string();

            let compte = CompteBancaire {
                id: prochain_id,
                nom,
                solde: 0.0,
            };
            comptes.push(compte);
            println!("Compte créé avec l'ID {}", prochain_id);
            prochain_id += 1;

        } else if choix == "2" {
            for compte in &comptes {
                println!("ID: {}, Nom: {}, Solde: {:.2}€", compte.id, compte.nom, compte.solde);
            }

        } else if choix == "3" {
            println!("Entrez l'ID du compte :");
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();

            for compte in &mut comptes {
                if compte.id == id {
                    println!("Montant à déposer :");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).unwrap();
                    let montant: f64 = montant.trim().parse().unwrap();
                    compte.solde += montant;
                    println!("Dépôt effectué !");
                }
            }

        } else if choix == "4" {
            println!("Entrez l'ID du compte :");
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();

            for compte in &mut comptes {
                if compte.id == id {
                    println!("Montant à retirer :");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).unwrap();
                    let montant: f64 = montant.trim().parse().unwrap();

                    if compte.solde >= montant {
                        compte.solde -= montant;
                        println!("Retrait effectué !");
                    } else {
                        println!("Fonds insuffisants !");
                    }
                }
            }

        } else if choix == "5" {
            println!("Entrez l'ID du compte à fermer :");
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();

            let mut index = 0;
            let mut trouvé = false;

            while index < comptes.len() {
                if comptes[index].id == id {
                    comptes.remove(index);
                    println!("Compte supprimé !");
                    trouvé = true;
                    break;
                }
                index += 1;
            }

            if !trouvé {
                println!("Compte non trouvé !");
            }

        } else if choix == "6" {
            println!("Au revoir !");
            break;

        } else {
            println!("Choix invalide.");
        }
    }
}
