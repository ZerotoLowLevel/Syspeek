use crate::couleurs::{CYAN_VIF, GRAS, RESET, SOULIGNE, VERT_VIF};
use std::env;
use std::env::consts::{ARCH, OS};
use sysinfo::System;
//------------------------------------------------------------------------------------------
// Fonction qui sera appelé dans le main
pub fn identite() {
    //1° Identité et Système
    println!(
        "{}{}{} 1° Identité & Système{}\n",
        CYAN_VIF, SOULIGNE, GRAS, RESET
    );
    //--------------------------------------------------------------------------------------
    //
    /*let machine crée la variable avec System::host_name() dedans
    .unwrap_or_else = si tu trouve pas mets Inconnu à la place
     */
    let machine = System::host_name().unwrap_or_else(|_| "inconnu".to_string());
    println!("Nom du PC        ===> {}{}{}", VERT_VIF, machine, RESET);
    //-------------------------------------------------------------------------------------
    //
    /* Récupération du nom d'utilisateur de la machine
    env::var::("USER") chercher la variable USER
    .or_else (|_| en::var::("USERNAME")) si tu la trouve pas cherche dans USERNAME(Window)
    unwrap_or_else etc... = si tu trouve rien mets inconnue à la place
    */
    let utilisateur = env::var("USER") // pour Linux et Mac
        .or_else(|_| env::var("USERNAME")) // pour window
        .unwrap_or_else(|_| "Inconnu".to_string()); // sinon mets inconnu
                                                    //--------------------------------------------------------------------------------------
                                                    //
    println!("Nom Utilisateur  ===> {}{}{}", VERT_VIF, utilisateur, RESET,);
    println!("System OS        ===> {}{}{}", VERT_VIF, OS, RESET);
    println!("Architecture     ===> {}{}{}", VERT_VIF, ARCH, RESET);
    //
    //-------------------------------------------------------------------------------------
    // On récupère le temps d'allumage du pc
    let secondes = System::uptime().unwrap_or(0);
    let heures = (secondes / 86400) / 3600;
    let minutes = (secondes % 3600) / 60;
    let jours = secondes / 86400;
    let secondes_restantes = secondes % 60;
    println!(
        "Temps d'allumage ===> {}{}J /{}H /{}m /{}s.",
        VERT_VIF, jours, heures, minutes, secondes_restantes
    );
}
