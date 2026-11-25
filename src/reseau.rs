use crate::couleurs::{CYAN_VIF, GRAS, RESET, ROUGE_VIF, SOULIGNE, VERT_VIF};
use sysinfo::Disks;
use sysinfo::Networks;

pub fn afficher() {
    //2° Stockage & Réseau
    //----------------------------------------------------------------------------
    println!(
        "{}{}{} 4° Stockage & Reseau {}\n",
        CYAN_VIF, SOULIGNE, GRAS, RESET
    );
    //----------------------------------------------------------------------------
    //
    if let Ok(disks) = Disks::new_with_refreshed_list() {
        for disk in &disks {
            // On ignore le volume interne Data de macOS pour éviter le doublon
            if disk.mount_point().to_str() == Some("/System/Volumes/Data") {
                continue;
            }

            let libre = disk.available_space() as f64 / 1_073_741_824.0;
            let total = disk.total_space() as f64 / 1_073_741_824.0;
            let utilise = total - libre;

            println!(
                "Disque ({})       ===> {}{:.2} Go / {:.2} Go{} (Libre: {:.2} Go)",
                disk.mount_point().display(),
                VERT_VIF,
                utilise,
                total,
                RESET,
                libre
            );
            if libre < 50.0 {
                println!("{}⚠️ Disque bientôt saturé {}\n", ROUGE_VIF, RESET);
            }
        }
    }
    //---------------------------------------------------------------------------------
    //
    if let Ok(networks) = Networks::new_with_refreshed_list() {
        for (interface, data) in &networks {
            // Sur Mac, "en0" est l'interface Wi-Fi principale
            if interface == "en0" {
                for ip in data.ip_networks() {
                    if ip.addr.is_ipv4() {
                        println!(
                            "IP Locale ({})  ===> {}{}{}",
                            interface.display(),
                            VERT_VIF,
                            ip.addr,
                            RESET
                        );
                    }
                }
            }
        }
    }
}
