# re-tools

Reverse-engineering helpers for `GameAssembly.dll`, mainly for finding the cheats' patches again after a game update. They read the DLL from disk, so the game doesn't need to be running.

To read a function as C instead of assembly, use Ghidra: see `tools/ghidra/README.md`.

## Setup: get the game's method names

The game is built with Unity IL2CPP: its C# code is compiled into `GameAssembly.dll`, and the names live separately in `global-metadata.dat`. [Il2CppDumper](https://github.com/Perfare/Il2CppDumper) combines the two. Run it again after every game update.

1. Download `Il2CppDumper-net6-win-v6.7.46.zip` from the [v6.7.46 release](https://github.com/Perfare/Il2CppDumper/releases/tag/v6.7.46). That's the first release that supports metadata version 31, which the game uses.
2. In its `config.json`, set `"RequireAnyKey": false`. Setting `"GenerateDummyDll": false` also saves time.
3. From the repository root, in PowerShell:

   ```powershell
   $game = "C:\Program Files (x86)\Steam\steamapps\common\PVZ Replanted"
   $env:DOTNET_ROLL_FORWARD = "Major"  # Il2CppDumper targets .NET 6; this lets a newer .NET run it
   <path to>\Il2CppDumper.exe "$game\GameAssembly.dll" "$game\Replanted_Data\il2cpp_data\Metadata\global-metadata.dat" il2cpp-dump
   ```

This writes `il2cpp-dump/`, which is gitignored. `re-tools` reads method names from `il2cpp-dump/script.json`. `il2cpp-dump/dump.cs` is the most readable file for people: every class with its fields' offsets and its methods' addresses.

## Commands

Run from the repository root with `cargo run -p re-tools --release -- <command>`. Addresses are hex RVAs, written as `6F10D6`, `0x6F10D6` or Cheat Engine's `GameAssembly.dll+6F10D6`.

| Command | What it does |
|---|---|
| `check` | Checks every cheat's patches against the game's DLLs and lists the ones that no longer match. Also checks that the global the entities are read from, `RootModel_TypeInfo`, is where `script.json` says. |
| `snapshot` | Copies the game's DLLs and metadata into `game-versions/<Steam build ID>/`, and downloads the engine's PDB into `symbols/` (both gitignored). Steam overwrites the game's files when it updates, so run it whenever you start working on a new build. |
| `find "83 AB 8C000000 04"` | Finds a byte pattern in the code. `??` matches any byte. Accepts `--from` and `--to` to limit the range. |
| `text "runInBackground"` | Finds a text string anywhere in the DLL. |
| `disasm 6F10D6` | Disassembles the code around an address. Calls to named functions are labeled. |
| `where 6F10D6` | Names the function that contains each address. |
| `callers 695DC0` | Lists the `call`s and `jmp`s that go to an address, such as a function's start. |
| `refs 1C61298` | Lists the code and pointers that refer to an address, such as a string or a global variable. |
| `names 'Coin$$'` | Lists the names that contain some text, with their addresses. These include IL2CPP's globals, like `Reloaded.Gameplay.Board_TypeInfo`. |
| `bytes 6F10D6 7` | Prints the bytes at an address. |

- `--dll` picks the DLL to inspect. It defaults to `GameAssembly.dll` in the standard Steam install, and `--dll ".../UnityPlayer.dll"` inspects the engine instead. `check` reads every DLL it needs from the same folder.
- Names come from two places:
  - `GameAssembly.dll`'s are the game's C# methods, like `Reloaded.Gameplay.Coin$$Update`, from the IL2CPP dump in `--dump` (default `il2cpp-dump`).
  - `UnityPlayer.dll`'s are the engine's C++ functions and globals, like `GetPlayerShouldRunInBackground`, from its PDB in `--symbols` (default `symbols`). `snapshot` downloads it from Unity's symbol server.
- Without names, everything except `where` and `names` still works, just showing addresses only.
- The engine's PDB only names public functions and globals, plus some private functions. For anything else, `where` shows the closest name before the address, which may belong to something else.

## Finding a cheat's patch after a game update

Each cheat's file in `src/pvz_sdk/toggleables/` names the method it patches and lists every patch's location, original bytes and patched bytes, with the instructions they encode.

1. **See what broke:** `check`. Every patch marked `BROKEN` needs a new offset. Patches marked `skip` are values in the running game's memory, which `check` can't read; they only change when the engine (`UnityPlayer.dll`) is updated.
2. **Search for the original bytes:** `find "<original bytes>"`. A single match inside the method the cheat names is the new offset.
3. **If there are several matches or none,** disassemble the method (`names`, then `disasm`) and look for the same logic. Things that commonly change:
   - The compiler can encode the same instruction differently. For example, `sub ebp,edi` is both `29 FD` and `2B EF`.
   - Short jumps can get a different distance, which changes their second byte.
   - The compiler can copy a small method into its callers. If `callers` finds nothing for a method, look for its code inside the methods that used to call it.
4. **Update the cheat:** the patch's location, its `original` bytes if the encoding changed, and the comments. Then run `check` again.

## Keeping the entities working after a game update

The entities (`src/pvz_sdk/entities/`) find each field by its C# name in the running game, so they don't break when an update moves fields. Two things can still go stale:

- **The global they start from.** `entities::ROOT_MODEL` (in `entities/board.rs`) is the address of `Tekly.DataModels.Models.RootModel_TypeInfo`, which changes with every build. After regenerating the IL2CPP dump, `check` prints its new address.
- **Renamed fields.** Reading an entity then fails with the class and field it can't find, like `Reloaded.Gameplay.Plant has no field named mPlantHealth`. Look for the field's new name in that class in `il2cpp-dump/dump.cs`, and change it in the entity's `class_layout!`.

`cargo run --bin testing` prints everything in the level being played, which shows whether the entities read correctly.

## Finding things in the engine (`UnityPlayer.dll`)

Pass `--dll ".../UnityPlayer.dll"`. With the engine's PDB, start from names: `names RunInBackground` finds `GetPlayerShouldRunInBackground`, and `callers` shows that the window procedure (`PlayerMainWndProc`) calls it before pausing the game.

The PDB doesn't name everything, but the engine's strings also lead into its code. No Pause's `runInBackground` patch needed the setting's field and the private global that points to it:

1. `text "runInBackground"` finds the name the engine's serializer uses when it loads that setting.
2. `refs` on that string finds the serializer code, which shows the field's offset in the settings object (`lea r9,[rsi+0x24F]` right next to the string).
3. `find` for instructions that read that offset (for example `0F B6 ?? 4F 02 00 00`, which is `movzx` from `+0x24F`) finds the code that uses the setting, and the global pointer it reads the object from (`mov rax,[0x1F15830]`).
