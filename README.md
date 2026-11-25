# 🦀 Syspeek

**Syspeek** est un utilitaire en ligne de commande (CLI) rapide et léger écrit en **Rust**, permettant d'afficher d'un coup d'œil les informations matérielles et système essentielles de votre ordinateur directement dans le terminal.

---

## ✨ Fonctionnalités

* **Identité & Système :** Nom de la machine, utilisateur actif, système d'exploitation, architecture processeur et temps d'activité (*uptime* formaté en jours, heures, minutes, secondes).
* **Processeur (CPU) :** Modèle exact de la puce, fréquence en MHz et nombre de cœurs logiques.
* **Mémoire vive (RAM) :** Mémoire utilisée et capacité totale en Go.
* **Stockage & Réseau :** Espace disque utilisé/libre avec alerte visuelle automatique sous 50 Go disponibles, et détection de l'adresse IPv4 locale.
* **Multiplateforme :** Compatible **macOS**, **Linux** et **Windows**.

---

## 🖥️ Exemple de rendu

```text
            -----------------
             --- Syspeek ---
            -----------------

 1° Identité & Système

Nom du PC        ===> MacBookAir.lan
Nom Utilisateur  ===> greg
System OS        ===> macos
Architecture     ===> aarch64
Temps d'allumage ===> 38J /0H /38m /50s.
------------------------------------------

 2° PROCESSEUR 

Modèle CPU       ===> Apple M1
Fréquence        ===> 3204 Mhz
coeurs CPU       ===> 8
------------------------------------------

 3° Mémoire vive (RAM) 

Mémoire RAM      ===> 6.07 Go / 8.00 Go
------------------------------------------

 4° Stockage & Reseau 

Disque (/)       ===> 201.51 Go / 228.27 Go (Libre: 26.77 Go)
⚠️ Disque bientôt saturé 

IP Locale (en0)  ===> 192.168.1.189
------------------------------------------
```

---

## 🚀 Installation et utilisation

### Prérequis
Avoir **Rust** et **Cargo** installés sur votre machine ([installer Rust](https://rustup.rs)).

### Compiler et lancer le projet

1. Clonez ce dépôt :
   ```bash
   git clone https://github.com
   cd Syspeek
   ```

2. Lancez le programme :
   ```bash
   cargo run --release
   ```

3. *(Optionnel)* Installez le binaire sur votre système pour pouvoir taper `Syspeek` depuis n'importe quel dossier :
   ```bash
   cargo install --path .
   ```

---

## 📁 Architecture du projet

```text
src/
├── main.rs        # Point d'entrée et orchestration
├── couleurs.rs    # Constantes de couleurs ANSI pour le terminal
├── systeme.rs     # Informations OS, hôte, utilisateur et uptime
├── cpu.rs         # Informations processeur
├── memoire.rs     # Informations mémoire vive (RAM)
└── reseau.rs      # Informations disques et interfaces réseau
```

---
<img width="455" height="439" alt="Capture d’écran 2026-11-25 à 16 13 41" src="https://github.com/user-attachments/assets/80d161c4-c44f-48fa-a086-f3c0b15e0a1f" />

## 🛠️ Technologies utilisées

* [Rust](https://rust-lang.org)
* [sysinfo](https://crates.io) — Récupération des métriques système et matérielles

