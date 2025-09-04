use std::time::Duration;

use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSET1: [usize; 1] = [0x678E7]; // "popcapgame1.exe" + 0x678E7
const INSTRUCTION_OFFSET2: [usize; 1] = [0x6789E]; // "popcapgame1.exe" + 0x6789E
const COOLDOWN_INSTRUCTION_OFFSET: [usize; 1] = [0x69648]; // "popcapgame1.exe" + 0x6789E

/// This cheat works like this:
/// All plants have a "state" variable at offset `Plant.OffsetPlantState`, but only some plants use it.
/// In the Chomper's case, its state can have several things as in ChomperState enum.
///
/// For this cheat, we modify two instructions:
/// First one is responsible for setting the Chomper's state to "Eating"
/// Second one is responsible for setting the Chomper's state to "Digesting"
///
/// We can get away with just modifying the second one, but for minimal cooldown between attacks we modify both.
#[derive(Default)]
pub struct FastChomperCheat {}

#[expect(dead_code)]
enum ChomperState {
    WaitingForPrey, // Pretty much "idle"
    Targeting,
    KilledZombie,   // Not so sure about this
    Digesting,      // The one that takes most time
    FinishedEating, // I'm not sure the difference between this and WaitingForPrey.
}

impl From<ChomperState> for u8 {
    fn from(val: ChomperState) -> Self {
        match val {
            ChomperState::WaitingForPrey => 1,
            ChomperState::Targeting => 10,
            ChomperState::KilledZombie => 11,
            ChomperState::Digesting => 13,
            ChomperState::FinishedEating => 14,
        }
    }
}

impl Toggleable for FastChomperCheat {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        // mov [edi + 3C], 0xE
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSET1,
            [
                0xC7,
                0x47,
                0x3C,
                ChomperState::FinishedEating.into(),
                0x00,
                0x00,
                0x00,
            ],
        )?;

        // mov [edi + 3C], 0xE
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSET2,
            [
                0xC7,
                0x47,
                0x3C,
                ChomperState::FinishedEating.into(),
                0x00,
                0x00,
                0x00,
            ],
        )?;

        // Momenterally restore all currently-digesting Choppers
        // to their normal state by setting the timer to 0.
        process.write::<[u8; 4]>(
            &COOLDOWN_INSTRUCTION_OFFSET,
            [
                0x83, 0x67, 0x54, 0x00, // and dword ptr [edi + 54], 0x0
            ],
        )?;
        std::thread::sleep(Duration::from_millis(500));
        process.write::<[u8; 4]>(
            &COOLDOWN_INSTRUCTION_OFFSET,
            [
                0x48, // dec eax
                0x89, 0x47, 0x54, // mov [edi + 54], eax
            ],
        )?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        // mov [edi + 3C], 0xD
        process.write::<[u8; 7]>(
            &INSTRUCTION_OFFSET1,
            [
                0xC7,
                0x47,
                0x3C,
                ChomperState::Digesting.into(),
                0x00,
                0x00,
                0x00,
            ],
        )?;

        // mov [edi + 3C], 0xB
        process.write::<[u8; 7]>(
            &INSTRUCTION_OFFSET2,
            [
                0xC7,
                0x47,
                0x3C,
                ChomperState::KilledZombie.into(),
                0x00,
                0x00,
                0x00,
            ],
        )?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Hungry Chompers"
    }
}
