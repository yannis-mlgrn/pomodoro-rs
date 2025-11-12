# Pomodoro Rust

Un simple minuteur Pomodoro développé en Rust avec la bibliothèque `egui`.

## Aperçu

Cette application fournit une interface graphique minimaliste pour gérer vos sessions de travail et de pause en utilisant la technique Pomodoro. Elle affiche un minuteur et vous permet de passer d'un mode à l'autre.

L'interface utilisateur est simple et intuitive :
- Un grand affichage du temps restant.
- Des titres qui changent en fonction de l'état : "Productivity Mode 🚀" pour le travail et "Coffee Break ☕" pour les pauses.
- Des boutons pour démarrer, réinitialiser ou passer à la session suivante.

## Fonctionnalités

- **Minuteur Pomodoro :** alterne entre des sessions de travail (par défaut 25 minutes) et des pauses courtes (par défaut 5 minutes).
- **Contrôles simples :**
    - **Démarrer :** Lance une nouvelle session de travail.
    - **Reset :** Arrête le minuteur et le réinitialise.
    - **Passer :** Passe immédiatement à l'état suivant (du travail à la pause, ou de la pause au travail).
- **Interface graphique claire :** Construite avec `eframe` et `egui` pour une expérience utilisateur légère.

## Comment l'utiliser

### Prérequis

Assurez-vous d'avoir installé [Rust et Cargo](https://www.rust-lang.org/tools/install).

### Installation et Lancement

1.  Clonez ce dépôt (si ce n'est pas déjà fait) :
    ```bash
    git clone <URL_DU_REPO>
    cd pomodoro
    ```

2.  Exécutez l'application :
    ```bash
    cargo run --release
    ```
    L'application se compilera et se lancera.

## Technologies utilisées

- **Langage :** [Rust](https://www.rust-lang.org/)
- **Bibliothèque GUI :** [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe) / [`egui`](https://github.com/emilk/egui)
