use std::str::FromStr;

use anyhow::{Context, ensure};

/// A byte pattern in which `??` matches any byte.
///
/// Bytes can be written one per token (`83 AB 8C 00 00 00 04`) or grouped the way
/// Cheat Engine shows instructions (`83 AB 8C000000 04`), so patterns can be
/// pasted straight from a cheat's doc comment.
#[derive(Debug, Clone)]
pub struct Pattern(Vec<Option<u8>>);

impl Pattern {
    /// A pattern that matches exactly `bytes`.
    pub fn exact(bytes: &[u8]) -> Self {
        Pattern(bytes.iter().copied().map(Some).collect())
    }

    /// Offsets in `haystack` where the pattern matches.
    pub fn find_in<'a>(&'a self, haystack: &'a [u8]) -> impl Iterator<Item = usize> + 'a {
        haystack
            .windows(self.0.len())
            .enumerate()
            .filter(|(_, window)| self.matches(window))
            .map(|(offset, _)| offset)
    }

    fn matches(&self, bytes: &[u8]) -> bool {
        self.0
            .iter()
            .zip(bytes)
            .all(|(expected, actual)| expected.is_none_or(|byte| byte == *actual))
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = Vec::new();
        for token in s.split_whitespace() {
            if token.chars().all(|c| c == '?') {
                bytes.push(None);
                continue;
            }
            ensure!(
                token.len() % 2 == 0,
                "`{token}` has an odd number of hex digits"
            );
            for pair in token.as_bytes().chunks(2) {
                let pair = std::str::from_utf8(pair)?;
                let byte = u8::from_str_radix(pair, 16)
                    .with_context(|| format!("`{token}` is not a hex byte sequence"))?;
                bytes.push(Some(byte));
            }
        }
        ensure!(!bytes.is_empty(), "the pattern is empty");
        Ok(Pattern(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_grouped_bytes_and_wildcards() {
        let pattern: Pattern = "83 AB 8C000000 ?? 04".parse().unwrap();
        assert_eq!(
            pattern.0,
            [
                Some(0x83),
                Some(0xAB),
                Some(0x8C),
                Some(0),
                Some(0),
                Some(0),
                None,
                Some(0x04)
            ]
        );
    }

    #[test]
    fn finds_every_match() {
        let pattern: Pattern = "7E ??".parse().unwrap();
        let haystack = [0x7E, 0x1A, 0x90, 0x7E, 0x05];
        assert_eq!(pattern.find_in(&haystack).collect::<Vec<_>>(), [0, 3]);
    }

    #[test]
    fn rejects_bad_input() {
        assert!("".parse::<Pattern>().is_err());
        assert!("8C0".parse::<Pattern>().is_err());
        assert!("ZZ".parse::<Pattern>().is_err());
    }
}
