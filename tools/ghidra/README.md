# Ghidra

`pvz_ghidra.py` runs [Ghidra](https://ghidra-sre.org/) from the command line, to read the game's code as C instead of assembly. It uses Ghidra's Python API (PyGhidra), and [uv](https://docs.astral.sh/uv/) installs the Python packages it needs.

## Setup

1. Download Ghidra 12.1 and unzip it. Ghidra needs a JDK, version 21 or newer.
2. Set `GHIDRA_INSTALL_DIR` to Ghidra's folder, or pass `--ghidra <folder>` before the command.
3. From the repository root:
   1. `cargo run -p re-tools --release -- snapshot` copies the game's files into `game-versions/<Steam build ID>/`.
   2. Generate `il2cpp-dump/` from the same build, as described in `tools/re-tools/README.md`.
   3. `uv run tools/ghidra/pvz_ghidra.py import` imports the newest snapshot into `ghidra-project/`. It names everything with Il2CppDumper's output and parses the game's C types from `il2cpp.h`. It takes about 5 minutes, once per game build, and the project takes about 1 GB.

For the engine, `uv run tools/ghidra/pvz_ghidra.py import --engine` imports `UnityPlayer.dll` with the names and types from Unity's PDB for it, in about 2 minutes. `re-tools snapshot` downloads that PDB into `symbols/`. Besides the PDB, the import runs Ghidra's Microsoft demangler, which turns C++ names like `?GetPlayerShouldRunInBackground@@YA_NXZ` into `bool GetPlayerShouldRunInBackground(void)`.

`game-versions/`, `symbols/` and `ghidra-project/` are gitignored, since they're generated from the game's own files.

## Decompiling

```powershell
uv run tools/ghidra/pvz_ghidra.py decompile 69A5F0 GameAssembly.dll+6F10D6
```

This prints the C code of the function that contains each address, in about 10 seconds. First it applies the C signatures from Il2CppDumper to that function and the functions it calls, so parameters and fields show up with their types and names (`__this->fields.mIsBeingCollected`). The signatures are saved in the project, so each function only needs this once.

`decompile --engine <address>` decompiles the engine instead, such as `decompile --engine 5775F0` for `GetPlayerShouldRunInBackground`.

`--build <ID>` picks a different snapshot than the newest one.

The project also opens in Ghidra's own window (`ghidra-project/Replanted.gpr`), but not while a command is using it.

## How the import works

- It doesn't run Ghidra's automatic analysis, which is very slow on an 87 MB DLL. Instead, it disassembles every method Il2CppDumper lists and every function in the DLL's exception table, and makes each one a function. The decompiler doesn't need more than that.
- The exception table also lists pieces of methods that the compiler moved out of line. In the `il2cpp` section, where IL2CPP's generated code is, anything Il2CppDumper doesn't list as a method is such a piece, so it's merged into its method instead of becoming a function.
- It ports Il2CppDumper's own Ghidra scripts (`ghidra_with_struct.py` and `il2cpp_header_to_ghidra.py`) to Python 3. Their originals are Python 2 and expect Ghidra's window.
- Applying all 332,000 method signatures up front would make the import much slower, so `import` saves them to `ghidra-project/signatures-<build>.json` for `decompile` to apply as needed.
