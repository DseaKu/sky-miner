# Sky Miner

> [!IMPORTANT] **🚧 Under Development:** This project is currently in
> development.

Sky Miner is a procedurally generated mining game built with **Godot
4.6** and **Rust** via **GDExtension**.

### Core Idea

Unlike traditional mining games, the core loop in Sky Miner focuses on **upward
progression**. While the most valuable resources are found at higher altitudes,
ascending presents increasing challenges. 


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
| **Left Hand Action**  | `Left Mouse Button`             |
| **Right Hand Action** | `Right Mouse Button`            |
| **Open Radial Menus** | `Q` (Left), `E` (Right)         |
| **Quick Switch**      | `1`, `2`, `3`, `4`, `5`         |
| **Use Binoculars**    | `X`                             |
| **Toggle Debug UI**   | `U`                             |

