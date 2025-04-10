# GPU Path Tracer

A real-time GPU-accelerated path tracer implemented in Rust using WGPU.

![GPU Path Tracer](./img/gif.gif)

## Features

- Real-time path tracing on the GPU using compute shaders
- Progressive rendering with temporal accumulation
- 3D model loading (.obj format)
- Camera controls for orbit, pan, and zoom
- Physically based lighting simulation

## Requirements

- Rust (2021 edition)
- A GPU with Vulkan, Metal, or DirectX 12 support

## Installation

Clone the repository and build using Cargo:

```bash
git clone https://github.com/yourusername/gpu-path-tracing.git
cd gpu-path-tracing
cargo build --release
```

## Usage

Run the application:

```bash
cargo run --release
```

### Controls

- **Left mouse button + drag**: Orbit the camera around the scene
- **Right mouse button + drag**: Pan the camera
- **Mouse wheel**: Zoom in/out

## Technical Details

This path tracer is implemented using:

- **WGPU** for cross-platform GPU access
- **Winit** for windowing and event handling
- **WGSL** shaders for the path tracing algorithm
- **Monte Carlo integration** for light transport simulation

The renderer uses a progressive accumulation technique, where each frame adds more samples to reduce noise over time. Camera movement resets the accumulation to provide responsive feedback.

## Project Structure

- `src/main.rs`: Application entry point and event handling
- `src/render.rs`: The path tracer implementation
- `src/camera.rs`: Camera controls and projection
- `src/algebra.rs`: Vector math utilities
- `src/load.rs`: Model loading
- `src/types.rs`: Common data structures
- `src/shaders.wgsl`: GPU shader code for the path tracer
- `models/`: Contains 3D models in .obj format

## License

CC-BY-4.0

---

# Traceur de chemins GPU (Path Tracer)

Un traceur de chemins en temps réel accéléré par GPU, implémenté en Rust avec WGPU.

![Traceur de chemins GPU](https://example.com/screenshot.png)

## Fonctionnalités

- Tracé de chemins en temps réel sur GPU utilisant des shaders de calcul
- Rendu progressif avec accumulation temporelle
- Chargement de modèles 3D (format .obj)
- Contrôles de caméra pour orbiter, panoramiquer et zoomer
- Simulation d'éclairage basée sur la physique

## Prérequis

- Rust (édition 2021)
- Un GPU compatible avec Vulkan, Metal ou DirectX 12

## Installation

Clonez le dépôt et compilez avec Cargo :

```bash
git clone https://github.com/votrepseudo/gpu-path-tracing.git
cd gpu-path-tracing
cargo build --release
```

## Utilisation

Exécutez l'application :

```bash
cargo run --release
```

### Contrôles

- **Bouton gauche de la souris + glisser** : Faire orbiter la caméra autour de la scène
- **Bouton droit de la souris + glisser** : Déplacer la caméra (panoramique)
- **Molette de la souris** : Zoomer/dézoomer

## Détails techniques

Ce traceur de chemins est implémenté avec :

- **WGPU** pour l'accès multiplateforme au GPU
- **Winit** pour la gestion des fenêtres et des événements
- **Shaders WGSL** pour l'algorithme de tracé de chemins
- **Intégration de Monte Carlo** pour la simulation du transport de lumière

Le moteur de rendu utilise une technique d'accumulation progressive, où chaque image ajoute plus d'échantillons pour réduire le bruit au fil du temps. Les mouvements de caméra réinitialisent l'accumulation pour fournir un retour réactif.

## Structure du projet

- `src/main.rs` : Point d'entrée de l'application et gestion des événements
- `src/render.rs` : Implémentation du traceur de chemins
- `src/camera.rs` : Contrôles de caméra et projection
- `src/algebra.rs` : Utilitaires de mathématiques vectorielles
- `src/load.rs` : Chargement de modèles
- `src/types.rs` : Structures de données communes
- `src/shaders.wgsl` : Code de shader GPU pour le traceur de chemins
- `models/` : Contient des modèles 3D au format .obj

## Licence

CC-BY-4.0 