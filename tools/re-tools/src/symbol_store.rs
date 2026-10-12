//! Finding and downloading a DLL's PDB, the file with its symbols (names for its functions and
//! globals). Unity publishes the engine's PDBs on a symbol server; the game's own code has none,
//! so its names come from Il2CppDumper instead.

use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use goblin::pe::debug::CodeviewPDB70DebugInfo;

const UNITY_SYMBOL_SERVER: &str = "https://symbolserver.unity3d.com";

/// Which PDB has a DLL's symbols: its file name, and the GUID and age the linker gave this
/// build. Symbol servers file PDBs under these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdbId {
    pub file_name: String,
    guid: [u8; 16],
    age: u32,
}

impl PdbId {
    pub fn from_codeview(info: &CodeviewPDB70DebugInfo) -> Option<Self> {
        let path = std::str::from_utf8(info.filename)
            .ok()?
            .trim_end_matches('\0');
        // The linker records the full path it wrote the PDB to; servers file it by name only.
        let file_name = path.rsplit(['\\', '/']).next()?.to_owned();
        Some(Self {
            file_name,
            guid: info.signature,
            age: info.age,
        })
    }

    /// The folder name symbol servers use for this build: the GUID in hex, with its first
    /// three fields read little-endian like Windows stores GUIDs, followed by the age in hex.
    fn key(&self) -> String {
        let [a0, a1, a2, a3, b0, b1, c0, c1, rest @ ..] = self.guid;
        let data1 = u32::from_le_bytes([a0, a1, a2, a3]);
        let data2 = u16::from_le_bytes([b0, b1]);
        let data3 = u16::from_le_bytes([c0, c1]);
        let rest: String = rest.iter().map(|byte| format!("{byte:02X}")).collect();
        format!("{data1:08X}{data2:04X}{data3:04X}{rest}{:X}", self.age)
    }

    /// Where this PDB goes in a symbol store: `<store>/<file name>/<key>/<file name>`, the
    /// layout that symbol servers and debuggers use.
    pub fn path_in(&self, store: &Path) -> PathBuf {
        store
            .join(&self.file_name)
            .join(self.key())
            .join(&self.file_name)
    }

    /// Downloads this PDB from Unity's symbol server into `store`, unless it's already there.
    pub fn download_from_unity(&self, store: &Path) -> Result<PathBuf> {
        let path = self.path_in(store);
        if path.exists() {
            return Ok(path);
        }
        let url = format!(
            "{UNITY_SYMBOL_SERVER}/{name}/{key}/{name}",
            name = self.file_name,
            key = self.key()
        );
        let response = ureq::get(&url)
            .call()
            .with_context(|| format!("downloading {url}"))?;

        let folder = path.parent().context("no folder")?;
        std::fs::create_dir_all(folder)?;
        // Download under another name first, so an interrupted download doesn't look finished.
        let partial = path.with_extension("partial");
        std::io::copy(
            &mut response.into_body().into_reader(),
            &mut File::create(&partial)?,
        )
        .with_context(|| format!("downloading {url}"))?;
        std::fs::rename(&partial, &path)?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::PdbId;

    #[test]
    fn formats_the_symbol_server_key() {
        let id = PdbId {
            file_name: "UnityPlayer_Win64_player_il2cpp_x64.pdb".to_owned(),
            guid: [
                0x26, 0xD0, 0xB1, 0x0D, 0x85, 0x68, 0x80, 0x41, 0xB1, 0x67, 0x81, 0xD5, 0x27, 0xBD,
                0x3A, 0x8E,
            ],
            age: 1,
        };
        assert_eq!(id.key(), "0DB1D02668854180B16781D527BD3A8E1");
    }
}
