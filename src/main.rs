mod couleurs;
mod cpu;
mod memoire;
mod reseau;
mod systeme;

fn main() {
    // Nom du programme
    println!(
        "{}            -----------------{}",
        couleurs::JAUNE_VIF,
        couleurs::RESET
    );
    println!(
        "{}{}             --- Syspeek ---{}",
        couleurs::JAUNE_VIF,
        couleurs::GRAS,
        couleurs::RESET
    );
    println!(
        "{}            -----------------{}\n",
        couleurs::JAUNE_VIF,
        couleurs::RESET
    );
    //appel de la fonction identité se trouvant dans système
    systeme::identite();
    println!("------------------------------------------\n");
    //
    // On appel la fonction afficher se trouvant dans cpu
    cpu::afficher();
    println!(
        "{}------------------------------------------{}\n",
        couleurs::VERT_VIF,
        couleurs::RESET
    );
    //
    // On appel la fonction afficher se trouvant dans memoire
    memoire::afficher();
    println!(
        "{}------------------------------------------{}\n",
        couleurs::VERT_VIF,
        couleurs::RESET
    );
    reseau::afficher();
    println!(
        "{}------------------------------------------{}\n",
        couleurs::VERT_VIF,
        couleurs::RESET
    );
}
