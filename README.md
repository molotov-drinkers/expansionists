# Expansionists

[![Godot Version](https://img.shields.io/badge/Godot-4.3-478cbf.svg?style=for-the-badge&logo=godot-engine)](https://godotengine.org)
[![Rust Version](https://img.shields.io/badge/Rust-stable-dea584.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

**Expansionists** is a strategy game of global conquest, built with the Godot Engine and Rust. Command your troops, expand your empire, and dominate the world.

This project is currently in (ON and OFF) development.

## 🌍 About The Project

Inspired by classic world domination board games, Expansionists brings the battle to a 3D globe. The core game logic is powered by Rust for performance and safety, integrated into Godot 4 via GDExtension.

- [Private Kanban Board](https://trello.com/b/5sQdN0i1/expansionists-%F0%9F%8C%8E)

![Godot](demo/panel.png)

### 🎮 Demo: Expanding and Occupying Lands
![europe_to_america](demo/europe_to_america.gif)
![south_america_to_south_pole](demo/south_america_to_south_pole.gif)

### 🎮 Demo: In Progress Combat
![north_america_to_north_asia](demo/north_america_to_north_asia.gif)

### Features (Planned/In-Progress)
*   3D globe for world map interaction.
*   A variety of land and sea units (Tanks, Cannons, Galleons, and more).
*   Territory-based resource and troop management.
*   Turn-based or real-time strategy combat.

## 🛠️ Built With

*   [Godot Engine v4.3](https://godotengine.org/)
*   [Rust](https://www.rust-lang.org/)
*   [godot-rust (gdext)](https://github.com/godot-rust/gdext)

## 🗺️ Development Roadmap

Here's a look at the current development status based on the project's Kanban board.

### 🚧 In Progress (Doing)
* **Combat System:** Core implementation of combat mechanics. (#12)
* **AI Pathfinding:** Creating AI for troop movement trajectories. (#99)

### 📝 To Do / Backlog
A categorized list of planned features and tasks.

#### Core Gameplay & AI
* Create a CPU opponent to play against. (#14)
* Implement a saving system for user profiles and troop data. (#21)
* Create troop fatality effects/animations. (#95)
* Add post-combat logic, like delaying troop spawns in a territory. (#96)
* Create a visual indicator (blinking light) for lands in conflict. (#98)

#### Multiplayer & Networking
* Build the core multiplayer system. (#15)
* Integrate GodotSteam for Steam social login. (#25)
* Research and set up a dedicated server solution (e.g., Nakama). (#27)
* Set up a Google Cloud Platform account for the game server. (#74)

#### UI / HUD
* Create the main menu with animated troop visuals. (#29)
* Add a Mercator projection thumbnail to the HUD. (#59)
* Create a timeline HUD. (#60)
* Design and implement a HUD for player and enemy statuses. (#61)
* Create a tutorial HUD. (#71)
* Develop a second version of the Troop HUD showing troop states. (#78)
* Create a splash screen. (#84)

#### Art & Sound
* Record base narrator voice lines. (#9)
* Implement the narrator's voice in-game. (#10)
* Add menu and in-game soundtracks. (#28)
* Enhance the planet's appearance with vegetation (trees, cacti, etc.). (#83)

#### Technical & Infrastructure
* Address commercial licensing requirements. (#62)
* Create profiling tools to debug performance. (#68)
* Localize game text for English, Portuguese, and Spanish (i18n). (#97)
* **Tech Debt/Bugs:**
    * Fix issues with dynamic camera speed. (#44)
    * Properly get spawner coordinates. (#50)
    * Fix idle occupation logic. (#89)
    * Refactor conquest logic to allow multiple players to attack a territory simultaneously. (#93)

## 🚀 Getting Started

To get a local copy up and running, follow these steps.

### Prerequisites

*   **Rust Toolchain**: Make sure you have `rustup` and `cargo` installed.
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   **Godot Engine**: Download Godot v4.3 or newer.
    *   Godot Website

### Installation & Running

1.  **Get to the codebase folder**
    ```sh
    cd expansionists/codebase
    ```

2.  **Build the Rust Library**
    Navigate to the `rust` directory and build the project. This compiles the Rust code into a dynamic library that Godot will use.
    ```sh
    cd rust
    cargo build
    ```
    The build script will automatically place the compiled library (`.dll`, `.so`, or `.dylib`) into the `godot/bin/` directory.

3.  **Open in Godot**
    *   Launch the Godot Engine editor.
    *   Choose **Import** or **Scan** and select the `codebase/godot` folder.
    *   Open the "Expansionists" project from the project manager.

4.  **Run the Game**
    *   With the project open in Godot, press `F5` or click the **Play** button in the top-right corner to start the game.

## 📂 Project Structure

The project is split into two main parts:

*   `codebase/godot/`: Contains all Godot-related files, including scenes (`.tscn`), resources (`.tres`), assets (models, textures), and the main `project.godot` file.
*   `codebase/rust/`: The Rust crate containing the core game logic, managed by Cargo. It's compiled as a GDExtension for use in Godot.

## 📧 Contact

Lucas - @luhalvesbr - hey@luhalvesbr.dev

Project Link: https://github.com/molotov-drinkers/expansionists/
