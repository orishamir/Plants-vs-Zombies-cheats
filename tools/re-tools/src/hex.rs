use std::fmt;

/// Formats bytes the way Cheat Engine and the cheats' doc comments do: `83 AB 8C 00`.
///
/// The output can be pasted back into `re-tools find` as a pattern.
pub struct HexBytes<'a>(pub &'a [u8]);

impl fmt::Display for HexBytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self
            .0
            .iter()
            .map(|byte| format!("{byte:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        f.pad(&text)
    }
}
