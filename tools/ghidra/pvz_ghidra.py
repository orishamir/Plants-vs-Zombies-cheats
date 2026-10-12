# /// script
# requires-python = ">=3.10,<3.14"
# dependencies = ["pyghidra==3.1.0"]
# ///
"""Ghidra from the command line, for Plants vs. Zombies Replanted.

`import` loads a snapshot of GameAssembly.dll (made by `re-tools snapshot`) into a Ghidra
project, and names everything in it with Il2CppDumper's output: functions, strings, type info,
and the C types of the game's classes. `import --engine` does the same for the engine,
UnityPlayer.dll, with the names and types from Unity's PDB for it.

`decompile` prints the C code of the functions that contain some addresses.

    uv run tools/ghidra/pvz_ghidra.py import
    uv run tools/ghidra/pvz_ghidra.py decompile 69A5F0 GameAssembly.dll+6F10D6
    uv run tools/ghidra/pvz_ghidra.py decompile --engine 5775F0

Ghidra is found through --ghidra or the GHIDRA_INSTALL_DIR environment variable. The pyghidra
version above must match Ghidra's: Ghidra 12.1 ships pyghidra 3.1.0.
"""

import argparse
import json
import re
import time
from pathlib import Path, PureWindowsPath

REPO = Path(__file__).resolve().parents[2]
SNAPSHOTS = REPO / "game-versions"
DUMP = REPO / "il2cpp-dump"
SYMBOLS = REPO / "symbols"
PROJECT_DIR = REPO / "ghidra-project"
# The game's code, and the engine.
GAME = "GameAssembly"
ENGINE = "UnityPlayer"
PROJECT_NAME = "Replanted"
# GameAssembly.dll is big, so Ghidra needs more memory than Java gives it by default.
JAVA_HEAP = "-Xmx16g"
DECOMPILE_TIMEOUT_SECONDS = 120


def main() -> None:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--ghidra", type=Path, help="Ghidra's folder (default: GHIDRA_INSTALL_DIR)")
    parser.add_argument("--build", help="the game build to use (default: the newest snapshot)")
    commands = parser.add_subparsers(dest="command", required=True)
    import_parser = commands.add_parser(
        "import",
        help="import a snapshot with its names and types (a few minutes, once per build)",
    )
    decompile_parser = commands.add_parser(
        "decompile", help="print the C code of the functions containing some addresses"
    )
    decompile_parser.add_argument(
        "addresses",
        nargs="+",
        type=parse_rva,
        help="hex offsets into the DLL: 69A5F0, 0x69A5F0 or GameAssembly.dll+69A5F0",
    )
    for command in (import_parser, decompile_parser):
        command.add_argument(
            "--engine", action="store_true", help=f"use {ENGINE}.dll instead of {GAME}.dll"
        )
    args = parser.parse_args()

    build = args.build or newest_snapshot()
    start_ghidra(args.ghidra)
    if args.command == "import":
        if args.engine:
            import_engine(build)
        else:
            import_game(build)
    else:
        decompile(build, args.addresses, ENGINE if args.engine else GAME)


def parse_rva(text: str) -> int:
    """Parses hex, with or without `0x`, or Cheat Engine's `GameAssembly.dll+69A5F0`."""
    offset = text.rsplit("+", 1)[-1]
    return int(offset.removeprefix("0x").removeprefix("0X"), 16)


def newest_snapshot() -> str:
    builds = [path.name for path in SNAPSHOTS.glob("*") if path.name.isdigit()]
    if not builds:
        raise SystemExit(f"No snapshots in {SNAPSHOTS}. Run `re-tools snapshot` first.")
    return max(builds, key=int)


def start_ghidra(install_dir: Path | None) -> None:
    from pyghidra.launcher import HeadlessPyGhidraLauncher

    launcher = HeadlessPyGhidraLauncher(install_dir=install_dir)
    launcher.add_vmargs(JAVA_HEAP)
    launcher.start()


def program_name(build: str, dll: str) -> str:
    return f"{dll}-{build}.dll"


def signatures_path(build: str) -> Path:
    return PROJECT_DIR / f"signatures-{build}.json"


def open_project(create: bool = False):
    from ghidra.base.project import GhidraProject

    if (PROJECT_DIR / f"{PROJECT_NAME}.gpr").exists():
        return GhidraProject.openProject(str(PROJECT_DIR), PROJECT_NAME, False)
    if not create:
        raise SystemExit(f"No Ghidra project in {PROJECT_DIR}. Run `import` first.")
    PROJECT_DIR.mkdir(exist_ok=True)
    return GhidraProject.createProject(str(PROJECT_DIR), PROJECT_NAME, False)


def progress(message: str) -> None:
    print(f"[{time.strftime('%H:%M:%S')}] {message}", flush=True)


# ---------------------------------------------------------------------------------------------
# import


def import_game(build: str) -> None:
    import pyghidra
    from java.io import File

    dll = SNAPSHOTS / build / f"{GAME}.dll"
    progress(f"Reading {DUMP / 'script.json'}")
    script = json.loads((DUMP / "script.json").read_text(encoding="utf-8"))

    project = open_project(create=True)
    try:
        progress(f"Importing {dll}")
        program = project.importProgram(File(str(dll)))
        with pyghidra.transaction(program, "Il2CppDumper"):
            parse_types(program)
            name_methods(program, script["ScriptMethod"])
            create_functions(program, script["Addresses"])
            name_strings(program, script["ScriptString"])
            name_metadata(program, script["ScriptMetadata"], script["ScriptMetadataMethod"])
        progress(f"Saving {program_name(build, GAME)}")
        project.saveAs(program, "/", program_name(build, GAME), True)
    finally:
        project.close()

    save_signatures(build, script["ScriptMethod"])
    progress("Done")


def import_engine(build: str) -> None:
    """Imports UnityPlayer.dll with the names and types from Unity's PDB for it."""
    import pyghidra
    from java.io import File

    dll = SNAPSHOTS / build / f"{ENGINE}.dll"
    project = open_project(create=True)
    try:
        progress(f"Importing {dll}")
        program = project.importProgram(File(str(dll)))
        apply_pdb(program, find_pdb(program))
        with pyghidra.transaction(program, "Functions"):
            create_functions(program, [])
        progress(f"Saving {program_name(build, ENGINE)}")
        project.saveAs(program, "/", program_name(build, ENGINE), True)
    finally:
        project.close()
    progress("Done")


def apply_pdb(program, pdb: Path) -> None:
    """Applies a PDB's names and types. Ghidra only applies PDBs during analysis, so this runs
    analysis with only two analyzers on: the PDB one, and the one that turns C++ mangled names
    like `?SetPlayerPause@@YAXW4PlayerPause@@_N@Z` into names and types."""
    import pyghidra
    from ghidra.app.plugin.core.analysis import PdbUniversalAnalyzer
    from ghidra.framework.options import OptionType
    from java.io import File

    analyzers = {"PDB Universal", "Demangler Microsoft"}
    with pyghidra.transaction(program, "PDB options"):
        PdbUniversalAnalyzer.setPdbFileOption(program, File(str(pdb)))
        options = pyghidra.analysis_properties(program)
        for name in options.getOptionNames():
            # Each analyzer has an on/off option named after it; its settings have dotted names.
            if "." not in name and options.getType(name) == OptionType.BOOLEAN_TYPE:
                options.setBoolean(name, name in analyzers)

    progress(f"Applying {pdb.name} (this takes a while)")
    log = pyghidra.analyze(program)
    if "error" in log.lower():
        print(log)


def find_pdb(program) -> Path:
    """The PDB for `program` in the symbol store that `re-tools snapshot` fills. Symbol stores
    file a PDB under its name and the GUID and age that the DLL records for it."""
    from ghidra.app.util.pdb import PdbProgramAttributes

    attributes = PdbProgramAttributes(program)
    name = PureWindowsPath(str(attributes.getPdbFile())).name
    key = str(attributes.getPdbGuid()).replace("-", "").upper() + f"{attributes.getPdbAgeAsInt():X}"
    pdb = SYMBOLS / name / key / name
    if not pdb.exists():
        raise SystemExit(f"{pdb} is missing. Run `re-tools snapshot` to download it.")
    return pdb


def parse_types(program) -> None:
    """Parses Il2CppDumper's il2cpp.h into the program's data types, so the decompiler can show
    fields by name, like `__this->fields.mIsBeingCollected`."""
    from ghidra.app.util.cparser.C import CParserUtils
    from ghidra.program.model.data import DataTypeManager
    from ghidra.util.task import TaskMonitor
    from jpype import JArray, JString

    header = PROJECT_DIR / "il2cpp_ghidra.h"
    progress(f"Converting il2cpp.h for Ghidra into {header}")
    header.write_text(
        ghidra_header((DUMP / "il2cpp.h").read_text(encoding="utf-8")), encoding="utf-8"
    )

    progress("Parsing the C types (this takes a while)")
    results = CParserUtils.parseHeaderFiles(
        JArray(DataTypeManager)([]),
        JArray(JString)([str(header)]),
        JArray(JString)([]),
        JArray(JString)([]),
        program.getDataTypeManager(),
        TaskMonitor.DUMMY,
    )
    if not results.successful():
        print(results.getFormattedParseMessage(None))
        raise SystemExit("Ghidra couldn't parse the C types")


def ghidra_header(il2cpp_header: str) -> str:
    """Il2CppDumper's il2cpp_header_to_ghidra.py: Ghidra's C parser needs the integer types
    defined, and doesn't understand C++ inheritance (`struct A : B {`), so the base type
    becomes a `super` field at the start of the struct."""
    integer_types = (
        "typedef unsigned __int8 uint8_t;\n"
        "typedef unsigned __int16 uint16_t;\n"
        "typedef unsigned __int32 uint32_t;\n"
        "typedef unsigned __int64 uint64_t;\n"
        "typedef __int8 int8_t;\n"
        "typedef __int16 int16_t;\n"
        "typedef __int32 int32_t;\n"
        "typedef __int64 int64_t;\n"
        "typedef __int64 intptr_t;\n"
        "typedef __int64 uintptr_t;\n"
        "typedef unsigned __int64 size_t;\n"
        "typedef _Bool bool;\n"
    )
    return integer_types + re.sub(r": (\w+) {", r"{\n \1 super;", il2cpp_header)


def name_methods(program, methods: list[dict]) -> None:
    progress(f"Naming {len(methods)} methods")
    for method in methods:
        label(program, method["Address"], method["Name"])


def create_functions(program, addresses: list[int]) -> None:
    """Gives every function its code. Without full analysis, this is what tells Ghidra where
    the code is. `addresses` are Il2CppDumper's method addresses, if the DLL has them; the last
    one only marks where the code ends.

    Ghidra's PE loader already puts an empty placeholder function at every entry of the DLL's
    exception table, but disassembles nothing. In GameAssembly.dll's `il2cpp` section, which
    holds the code IL2CPP generated from C#, Il2CppDumper lists every method, so the other
    entries there are pieces of methods that the compiler moved out of line; their placeholders
    are removed so the pieces join their methods."""
    from ghidra.app.cmd.function import CreateFunctionCmd
    from ghidra.program.flatapi import FlatProgramAPI
    from ghidra.util.task import TaskMonitor

    api = FlatProgramAPI(program)
    functions = program.getFunctionManager()
    space = program.getImageBase().getAddressSpace()
    generated_code = program.getMemory().getBlock("il2cpp")

    starts = {program.getImageBase().add(rva).getOffset() for rva in addresses[:-1]}
    for function in list(functions.getFunctions(True)):
        entry = function.getEntryPoint()
        if entry.getOffset() in starts:
            continue
        if generated_code is not None and generated_code.contains(entry):
            functions.removeFunction(entry)
        else:
            starts.add(entry.getOffset())
    entries = [space.getAddress(offset) for offset in sorted(starts)]

    progress(f"Disassembling {len(entries)} functions (this takes a while)")
    for entry in entries:
        if api.getInstructionAt(entry) is None:
            api.disassemble(entry)

    progress("Finding each function's code")
    for entry in entries:
        function = functions.getFunctionAt(entry)
        if function is None:
            api.createFunction(entry, None)
        else:
            CreateFunctionCmd.fixupFunctionBody(program, function, TaskMonitor.DUMMY)
    with_code = sum(1 for f in functions.getFunctions(True) if f.getBody().getNumAddresses() > 1)
    progress(f"{with_code} functions have code")


def name_strings(program, strings: list[dict]) -> None:
    """Labels the slots that hold the game's string literals, with the text as a comment."""
    progress(f"Naming {len(strings)} strings")
    listing = program.getListing()
    base = program.getImageBase()
    for index, string in enumerate(strings, start=1):
        label(program, string["Address"], f"StringLiteral_{index}")
        listing.setComment(base.add(string["Address"]), eol_comment(), string["Value"])


def name_metadata(program, metadata: list[dict], method_metadata: list[dict]) -> None:
    """Labels IL2CPP's runtime data, like `Reloaded.Gameplay.Coin_TypeInfo`, and types it when
    il2cpp.h has the type."""
    from ghidra.program.flatapi import FlatProgramAPI
    from java.util import ArrayList

    progress(f"Naming {len(metadata) + len(method_metadata)} metadata entries")
    api = FlatProgramAPI(program)
    types = program.getDataTypeManager()
    base = program.getImageBase()
    for entry in metadata + method_metadata:
        label(program, entry["Address"], entry["Name"])
        signature = entry.get("Signature") or ""
        if not signature.endswith("*"):
            continue
        found = ArrayList()
        types.findDataTypes(signature.removesuffix("*").strip(), found)
        if found.size() == 1:
            try:
                api.createData(base.add(entry["Address"]), types.getPointer(found.get(0)))
            except Exception:
                pass  # something is already defined there


def label(program, rva: int, name: str) -> None:
    from ghidra.program.model.symbol import SourceType

    program.getSymbolTable().createLabel(
        program.getImageBase().add(rva), name.replace(" ", "-"), SourceType.USER_DEFINED
    )


def eol_comment():
    from ghidra.program.model.listing import CommentType

    return CommentType.EOL


def save_signatures(build: str, methods: list[dict]) -> None:
    """Keeps each method's C signature for `decompile`. Applying all of them during the import
    would take hours, so `decompile` applies the ones it needs."""
    signatures: dict[str, str] = {}
    for method in methods:
        signatures.setdefault(str(method["Address"]), method["Signature"])
    signatures_path(build).write_text(json.dumps(signatures), encoding="utf-8")


# ---------------------------------------------------------------------------------------------
# decompile


def decompile(build: str, rvas: list[int], dll: str) -> None:
    import pyghidra
    from ghidra.app.decompiler import DecompInterface
    from ghidra.util.task import TaskMonitor

    # Only the game's methods have signatures to apply; the engine's PDB already has its types.
    signatures = {}
    if dll == GAME:
        signatures = json.loads(signatures_path(build).read_text(encoding="utf-8"))
    project = open_project()
    try:
        program = project.openProgram("/", program_name(build, dll), False)
        base = program.getImageBase()
        functions = program.getFunctionManager()

        targets = []
        with pyghidra.transaction(program, "Apply signatures"):
            for rva in rvas:
                function = functions.getFunctionContaining(base.add(rva))
                if function is None:
                    print(f"// No function contains {dll}.dll+{rva:X}")
                    continue
                # The callees' signatures tell the decompiler what the calls pass and return.
                called = function.getCalledFunctions(TaskMonitor.DUMMY)
                for each in [function, *called]:
                    apply_signature(program, each, signatures)
                targets.append((rva, function))

        decompiler = DecompInterface()
        decompiler.openProgram(program)
        for rva, function in targets:
            entry = function.getEntryPoint().subtract(base)
            print(f"// {function.getName()} at {dll}.dll+{entry:X}, containing +{rva:X}")
            result = decompiler.decompileFunction(
                function, DECOMPILE_TIMEOUT_SECONDS, TaskMonitor.DUMMY
            )
            if result.decompileCompleted():
                print(result.getDecompiledFunction().getC())
            else:
                print(f"// Decompiling failed: {result.getErrorMessage()}")
        decompiler.dispose()
        # Keep the applied signatures, so later decompiles start from them.
        project.save(program)
    finally:
        project.close()


def apply_signature(program, function, signatures: dict[str, str]) -> None:
    from ghidra.app.cmd.function import ApplyFunctionSignatureCmd, FunctionRenameOption
    from ghidra.app.util.cparser.C import CParserUtils
    from ghidra.program.model.symbol import SourceType

    entry = function.getEntryPoint()
    signature = signatures.get(str(entry.subtract(program.getImageBase())))
    if signature is None or function.getSignatureSource() == SourceType.USER_DEFINED:
        return
    try:
        definition = CParserUtils.parseSignature(None, program, signature, False)
    except Exception:
        return  # a few signatures use C++ that Ghidra's C parser can't read
    if definition is not None:
        ApplyFunctionSignatureCmd(
            entry,
            definition,
            SourceType.USER_DEFINED,
            False,
            FunctionRenameOption.NO_CHANGE,
        ).applyTo(program)


if __name__ == "__main__":
    main()
