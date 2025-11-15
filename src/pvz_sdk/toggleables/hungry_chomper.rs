use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const INSTRUCTION_OFFSET1: [usize; 1] = [0x6AB757];

/// This cheat works like this:
/// All plants have a "state" variable at offset `Plant.OffsetPlantState`, but only some plants use it.
/// In the Chomper's case, its state can have several things as in ChomperState enum.
///
/// ```diff
/// - GameAssembly.dll+6AB757 - C7 83 88000000 0B000000 - mov [rbx+00000088],0000000B
/// + GameAssembly.dll+6AB757 - 90 90 90909090 90909090 - nop
/// ```
pub struct HungryCompersCheat {}

/// may be outdated!
#[derive(Debug)]
#[expect(dead_code)]
enum ChomperState {
    WaitingForPrey = 1, // Pretty much "idle"
    Attacking = 10,
    KilledZombie = 11,    // Not so sure about this
    Digesting = 13,       // The one that takes most time
    FinishedEating = 0xe, // I'm not sure the difference between this and WaitingForPrey.
}

impl Toggleable for HungryCompersCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Hungry Chompers"
    }

    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; 10]>(&INSTRUCTION_OFFSET1, [0x90; _])?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSET1,
            [0xC7, 0x83, 0x88, 0, 0, 0, 0xb, 0, 0, 0],
        )?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSET1, true)?, 4)
            .unwrap();

        Ok(current_instructions[0..4] == [0x90, 0x90, 0x90, 0x90])
    }
}
