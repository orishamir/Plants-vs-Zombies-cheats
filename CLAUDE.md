# CLAUDE.md

Cheats for Plants vs. Zombies **Replanted**, written in Rust. It's a hobby project, and it's also how the owner is learning Rust.

## Number one rule: the repository must be maintainable by a human

Every change should leave the code easy for a person to read, understand and extend. The rules below make sure of that too.

### a. Write idiomatic Rust

Learning Rust is as much the goal as the cheats, so use Rust's methods and idioms instead of C-style code in Rust syntax. That means traits, enums that carry data, iterators, `Option`/`Result` with `?`, `From`/`TryFrom` conversions, newtypes and the standard library's existing methods. When there's an idiomatic way and a quick way, take the idiomatic way. Spending time on this is expected.

### b. Always consider the architecture

The goal is not only to make it work, but just as much to do it in a good way. Before adding code, decide where it belongs and whether the existing abstractions fit.

Implementing a new cheat must be easy. A cheat should describe *what* it changes, and shared code should handle *how*: writing bytes, toggling, and checking whether the cheat is active. If a change needs code copied from another cheat, an abstraction is missing. Fix the abstraction instead of copying the code.

### c. Keep reverse-engineering work self-contained

Everything needed to understand or redo a piece of reverse engineering lives in this repository, not in someone's head or a temp folder.

- Each cheat documents what it patches: the game method, what the code there does, and the instruction behind each patch's bytes, as in `src/pvz_sdk/toggleables/`. After a game update, someone should be able to find the patch again from the cheat's file alone.
- Reverse-engineering tooling and artifacts (scanners, scripts, Ghidra headers, ReClass.NET files) live in their own clearly separated place. They don't leak into the SDK or the cheats.
- Don't commit the game's own files (DLLs, `global-metadata.dat`, full dumps). The work should be reproducible from a game install.

### d. Low-level constraints must not shape the design

The game's memory layout is an implementation detail. Don't mirror it in the types the rest of the code uses. For example, the project started with `#[repr(C, packed)]` structs full of `_pad: [u8; N]` fields that matched the bytes in memory; don't go back to that.

Build layers instead, from low-level memory reading up to convenient structs and traits:

1. **Memory access:** reading and writing bytes at an address in the game process.
2. **Layout:** where each field lives and how it's encoded, such as offsets, sizes, pointer width, and enums stored as integers.
3. **Domain:** easy-to-use Rust types and traits (`Plant`, `Zombie`, `PlantType`, cheats) that the rest of the code and the GUI work with.

Padding, pointer width and raw integer values belong only in the lower layers.

## Project context

- The game is 64-bit Unity with IL2CPP. Its logic lives in `GameAssembly.dll` and the engine in `UnityPlayer.dll`. A `Patch` has a `Location`: either an offset into one of those DLLs (`Dll::GameAssembly.at(0x665DE0)`), or a field of an object reached through a pointer stored in one.
- Game updates move code, so offsets go stale. A patch refuses to write when the game's bytes aren't the ones it expects, and `re-tools check` lists the patches that no longer match. `tools/re-tools/README.md` describes how to find a patch again.
- Entities don't use offsets: `il2cpp/` reads IL2CPP's own class metadata in the running game, so fields are found by their C# names (`Plant.mPlantHealth`), as Il2CppDumper's `dump.cs` shows them. They keep working across game updates unless the game renames a field, and then reading fails with the field's name. The one address they need is `entities::ROOT_MODEL`, which `re-tools check` checks against the dump.

## Layout

- `src/pvz_sdk/` is the library crate `pvz_sdk`:
  - `game.rs`: `Popcapgame` attaches to `Replanted.exe` and reads and writes its memory (layer 1).
  - `il2cpp/`: reads the game's C# objects, finding classes and fields by name (layer 2). `class_layout!` declares which fields to read from a class, and `runtime.rs` has the only hard-coded offsets, those of IL2CPP's own structs.
  - `location.rs`: `Dll` and `Location`, which say where something is in the game's memory independently of where Windows loaded the DLLs.
  - `toggleables/`: the cheats, one per file. Each is a `PatchCheat` constant (a name and its `Patch`es) listed in `toggleables::ALL`, which the GUI and `re-tools check` read. `patch.rs` holds the shared logic: checking, applying and reverting patches.
  - `entities/`: the game's objects as Rust types (layer 3), one file each: the domain type at the top, and the class's fields (its `class_layout!`) and the conversion between them at the bottom. `Board::current` is where reading starts, and an `Entity<T>` writes back only the fields that changed.
- `src/bin/gui/`: the egui window that lists the cheats (the default binary).
- `src/bin/testing/`: prints the entities read from the running game.
- `tools/re-tools/`: a command-line tool (a separate crate in the workspace) for finding and inspecting patches in the game's DLLs. Its README covers the reverse-engineering workflow.
- `tools/ghidra/`: `pvz_ghidra.py`, a uv script that imports the game into Ghidra and decompiles functions to C from the command line. Its README covers setup.
- Gitignored, generated from the game's files:
  - `il2cpp-dump/`: Il2CppDumper's output, with the game's class, field and method names. Regenerate it as described in the `re-tools` README.
  - `game-versions/<Steam build ID>/`: copies of the game's DLLs and metadata, made by `re-tools snapshot`.
  - `symbols/`: the engine's PDB from Unity's symbol server, downloaded by `re-tools snapshot` and laid out like a symbol server (`<name>/<GUID+age>/<name>`).
  - `ghidra-project/`: the Ghidra project made by `pvz_ghidra.py import`.

## Commands

- `cargo run`: start the GUI. The game must already be running.
- `cargo run --bin testing`: print the game's entities.
- `cargo run -p re-tools --release -- <command>`: the reverse-engineering tool.
- `uv run tools/ghidra/pvz_ghidra.py decompile <address>...`: print a function's C code (needs Ghidra and an imported build; see `tools/ghidra/README.md`).
- `cargo clippy --workspace --all-targets`: changes shouldn't add warnings.
- `cargo test --workspace`: run the tests.
