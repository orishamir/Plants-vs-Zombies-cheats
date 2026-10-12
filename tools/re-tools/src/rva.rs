use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;

/// A relative virtual address: an offset from the start of a loaded module.
///
/// The cheats patch `GameAssembly.dll` at RVAs, and Cheat Engine shows the same
/// numbers (`GameAssembly.dll+6F10D6` is RVA `0x6F10D6`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rva(pub u32);

impl fmt::Display for Rva {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{:#X}", self.0))
    }
}

impl FromStr for Rva {
    type Err = ParseIntError;

    /// Parses hex, with or without `0x`, or Cheat Engine's `GameAssembly.dll+6F10D6`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.rsplit_once('+').map_or(s, |(_module, offset)| offset);
        let hex = hex
            .strip_prefix("0x")
            .or_else(|| hex.strip_prefix("0X"))
            .unwrap_or(hex);
        u32::from_str_radix(hex, 16).map(Rva)
    }
}

impl Add<u32> for Rva {
    type Output = Rva;

    fn add(self, offset: u32) -> Rva {
        Rva(self.0 + offset)
    }
}

impl Sub for Rva {
    type Output = u32;

    /// The distance in bytes from `other` up to `self`.
    fn sub(self, other: Rva) -> u32 {
        self.0 - other.0
    }
}
