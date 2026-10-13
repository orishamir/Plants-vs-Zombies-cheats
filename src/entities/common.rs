/// A length of time, counted in the game's updates.
///
/// The game updates the board 120 times per second of game time
/// (`Reloaded.Constants.Gameplay.TIME_FOR_UPDATE` is 1/120), and its timers count these
/// updates. Game time can run slower than real time: the game has a time scale
/// (`TimeUtil.SetFlowingTimeScale`), and it ran at about half speed with its window in the
/// background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ticks(pub i32);

impl Ticks {
    /// How many updates there are in a second of game time.
    pub const PER_SECOND: i32 = 120;

    pub fn from_secs_f32(secs: f32) -> Self {
        Self((secs * Self::PER_SECOND as f32).round() as i32)
    }

    pub fn as_secs_f32(self) -> f32 {
        self.0 as f32 / Self::PER_SECOND as f32
    }
}

/// A square of the lawn. Rows count from the top and columns from the left, both from 0. The
/// lawn's columns go up to 8, but some things are in column 9, just past its right edge, like
/// two of Portal Combat's portals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub row: i32,
    pub column: i32,
}

/// Where something is on the board, in the game's coordinates rather than the window's pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// How much damage something can still take, out of how much it started with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[cfg(test)]
mod tests {
    use super::Ticks;

    #[test]
    fn ticks_convert_to_and_from_seconds() {
        assert_eq!(Ticks::from_secs_f32(2.5), Ticks(300));
        assert_eq!(Ticks(60).as_secs_f32(), 0.5);
    }
}
