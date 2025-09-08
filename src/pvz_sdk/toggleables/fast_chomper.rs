use std::time::Duration;

use num_enum::IntoPrimitive;

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
pub struct FastChomperCheat {}

#[derive(Debug, IntoPrimitive)]
#[repr(u8)]
#[expect(dead_code)]
enum ChomperState {
    WaitingForPrey = 1, // Pretty much "idle"
    Targeting = 10,
    KilledZombie = 11,   // Not so sure about this
    Digesting = 13,      // The one that takes most time
    FinishedEating = 14, // I'm not sure the difference between this and WaitingForPrey.
}

impl Toggleable for FastChomperCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Hungry Chompers"
    }

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
        process.write::<[u8; _]>(
            &COOLDOWN_INSTRUCTION_OFFSET,
            [
                0x83, 0x67, 0x54, 0x00, // and dword ptr [edi + 54], 0x0
            ],
        )?;
        std::thread::sleep(Duration::from_millis(500));
        process.write::<[u8; _]>(
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
        process.write::<[u8; _]>(
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
        process.write::<[u8; _]>(
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

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSET1, true)?, 7)
            .unwrap();

        Ok(current_instructions[0..7]
            == [
                0xC7,
                0x47,
                0x3C,
                ChomperState::FinishedEating.into(),
                0x00,
                0x00,
                0x00,
            ])
    }
}
