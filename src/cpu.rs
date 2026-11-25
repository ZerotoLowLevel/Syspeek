use crate::couleurs::{CYAN_VIF, GRAS, RESET, SOULIGNE, VERT_VIF};
use sysinfo::System;

pub fn afficher() {
    //2° PROCESSEUR
    //----------------------------------------------------------------------------
    println!("{}{}{} 2° PROCESSEUR {}\n", CYAN_VIF, SOULIGNE, GRAS, RESET);
    //----------------------------------------------------------------------------
    //
    // On crée une instance vide et on scan uniquement le CPU
    let mut sys = System::new().unwrap();
    sys.refresh_cpu_all();

    if let Some(puce) = sys.cpus().first() {
        println!(
            "Modèle CPU       ===> {}{}{}",
            VERT_VIF,
            puce.brand(),
            RESET
        );
        println!(
            "Fréquence        ===> {}{} Mhz{}",
            VERT_VIF,
            puce.frequency(),
            RESET
        );
    }
    println!(
        "coeurs CPU       ===> {}{}{}",
        VERT_VIF,
        sys.cpus().len(),
        RESET
    );
}
