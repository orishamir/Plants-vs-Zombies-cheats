//! Keeping copies of the game's files for every version, because Steam overwrites them when the
//! game updates. Comparing a new version with the previous one is much easier than without it.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

/// The files a snapshot keeps, relative to the game's folder: the game's code, the engine, and
/// the IL2CPP metadata that Il2CppDumper needs along with the code.
const FILES: [&str; 3] = [
    "GameAssembly.dll",
    "UnityPlayer.dll",
    "Replanted_Data/il2cpp_data/Metadata/global-metadata.dat",
];

/// Copies the game's files from `game_dir` into `<snapshots_dir>/<Steam build ID>/`, and returns
/// that folder. Files already copied for this build are left alone.
pub fn snapshot(game_dir: &Path, snapshots_dir: &Path) -> Result<PathBuf> {
    let destination = snapshots_dir.join(build_id(game_dir)?);
    std::fs::create_dir_all(&destination)
        .with_context(|| format!("creating {}", destination.display()))?;

    for file in FILES {
        let source = game_dir.join(file);
        let target = destination.join(Path::new(file).file_name().context("no file name")?);
        if !target.exists() {
            std::fs::copy(&source, &target)
                .with_context(|| format!("copying {}", source.display()))?;
        }
    }
    Ok(destination)
}

/// The Steam build ID of the game installed in `game_dir`. It changes with every update.
///
/// Steam keeps it in `steamapps/appmanifest_<app ID>.acf`, two folders up from the game, in the
/// manifest whose `installdir` is the game's folder name.
fn build_id(game_dir: &Path) -> Result<String> {
    let install_dir = game_dir
        .file_name()
        .context("the game's folder has no name")?
        .to_string_lossy();
    let steamapps = game_dir
        .parent()
        .and_then(Path::parent)
        .context("the game isn't in a Steam library")?;

    for entry in std::fs::read_dir(steamapps)? {
        let path = entry?.path();
        let is_manifest = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("appmanifest_") && name.ends_with(".acf"));
        if !is_manifest {
            continue;
        }
        let manifest = std::fs::read_to_string(&path)?;
        if manifest_value(&manifest, "installdir") == Some(&*install_dir) {
            return manifest_value(&manifest, "buildid")
                .map(str::to_owned)
                .with_context(|| format!("{} has no build ID", path.display()));
        }
    }
    bail!("Steam has no manifest for {}", game_dir.display())
}

/// The value of the first `"key"  "value"` line for `key` in a Steam manifest.
fn manifest_value<'a>(manifest: &'a str, key: &str) -> Option<&'a str> {
    manifest.lines().find_map(|line| {
        // Splitting `"key"  "value"` on quotes leaves the key and value at the odd positions.
        let mut quoted = line.split('"').skip(1).step_by(2);
        (quoted.next()? == key).then(|| quoted.next()).flatten()
    })
}

#[cfg(test)]
mod tests {
    use super::manifest_value;

    #[test]
    fn reads_manifest_values() {
        let manifest =
            "\"AppState\"\n{\n\t\"appid\"\t\t\"3654560\"\n\t\"buildid\"\t\t\"21398230\"\n}";
        assert_eq!(manifest_value(manifest, "buildid"), Some("21398230"));
        assert_eq!(manifest_value(manifest, "appid"), Some("3654560"));
        assert_eq!(manifest_value(manifest, "missing"), None);
    }
}
