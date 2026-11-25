use crate::couleurs::{CYAN_VIF, GRAS, RESET, SOULIGNE, VERT_VIF};
use sysinfo::System;

pub fn afficher() {
    //2° MEMOIRE
    //----------------------------------------------------------------------------
    println!(
        "{}{}{} 3° Mémoire vive (RAM) {}\n",
        CYAN_VIF, SOULIGNE, GRAS, RESET
    );
    //----------------------------------------------------------------------------
    //
    if let Ok(mut sys) = System::new() {
        sys.refresh_memory();
        // On convertit les octets en Gigaoctets (Go)
        let ram_totale = sys.total_memory() as f64 / 1_073_741_824.0;
        let ram_utilisee = sys.used_memory() as f64 / 1_073_741_824.0;

        println!(
            "Mémoire RAM      ===> {}{:.2} Go / {:.2} Go{}",
            VERT_VIF, ram_utilisee, ram_totale, RESET
        );
    }
}
