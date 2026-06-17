# Sky Miner

> [!IMPORTANT] **🚧 Under Development:** This project is currently in
> development.

Sky Miner is a procedurally generated mining adventure game built with **Godot
4.6** and **Rust** via **GDExtension**.

### 🏔️ The Vertical Challenge

Unlike traditional mining games, the core loop in Sky Miner focuses on **upward
progression**. While the most valuable resources are found at higher altitudes,
ascending presents increasing challenges. Players must balance the pursuit of
rare materials with the growing environmental penalties.

It features multi-threaded terrain generation and a variety of tools for
exploration and resource gathering.

## 🚀 Features

- **Procedural Terrain:** Infinite world generation using noise-based
  algorithms, optimized with parallel processing (Rayon).
- **Advanced Player Controller:** Physics-based movement with states for
  walking, jumping, falling, gliding, and flying.
- **Equipment System:** Interactive tools including a Pickaxe, Binoculars, and
  Harpoon(in development).
- **Dynamic UI:** Intuitive radial menus for tool selection and a comprehensive
  debug interface.
- **GDExtension Architecture:** High-performance core logic written in Rust,
  seamlessly integrated with Godot's engine.
- **Save/Load System:** Persistent chunk data storage using Bincode
  serialization.

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

- **Godot Engine 4.6+** (Standard or .NET version)
- **Rust Toolchain** (latest stable)
- **Make** (optional, for using the provided Makefile)

## 📥 Installation & Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/sky-miner.git
   cd sky-miner
   ```

2. **Build the Rust library:**

   ```bash
   make build
   ```

   _This compiles the Rust code into a dynamic library. The
   `godot/bin/mine_world.gdextension` file is configured to point directly to
   the `rust/target/` directory._

3. **Open the project in Godot:**
   - Launch Godot and import the `godot/` folder as a project.
   - Alternatively, use the Makefile command:
     ```bash
     make run
     ```

## 🎮 Controls

| Action                | Key/Button                      |
| --------------------- | ------------------------------- |
| **Movement**          | `W`, `A`, `S`, `D` / Arrow Keys |
| **Jump**              | `Space`                         |
| **Boost**             | `Shift`                         |
| **Left Hand Action**  | `Left Mouse Button`             |
| **Right Hand Action** | `Right Mouse Button`            |
| **Open Radial Menus** | `Q` (Left), `E` (Right)         |
| **Quick Switch**      | `1`, `2`, `3`, `4`, `5`         |
| **Use Binoculars**    | `X`                             |
| **Toggle Debug UI**   | `U`                             |

## 🏗️ Project Structure

- `godot/`: The Godot project directory containing scenes, assets, and
  GDExtension configuration.
  - `entities/`: Player and terrain scene files and GDScript components.
  - `ui/`: UI components like the radial menu and debug overlay.
  - `bin/`: Compiled Rust binaries and `.gdextension` bridge file.
- `rust/`: The Rust source code for the game logic.
  - `src/core/`: Internal utilities, logging, and common macros.
  - `src/entities/`: Player state machine and physics implementation.
  - `src/terrain/`: Procedural generation, chunking logic, and I/O handlers.
  - `src/ui/`: Rust-side UI logic and data structures.
- `docs/`: Project documentation and task lists.

## 🔧 Development Commands

The root `Makefile` provides several helper commands:

- `make check`: Run `cargo check` on the Rust codebase.
- `make clippy`: Run Clippy for linting.
- `make build`: Build the Rust library in debug mode.
- `make run`: Launch the Godot project.
- `make clean-conf`: Clear the Godot application user data (configs/saves).
