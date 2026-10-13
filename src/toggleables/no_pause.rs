use super::{Patch, PatchCheat};
use crate::{Dll, Location};

/// The game keeps running when its window loses focus, for example when you click this
/// cheat's window.
///
/// The Unity engine stops the whole game unless its `runInBackground` setting is on, and the
/// game itself opens the pause menu and pauses time.
pub const NO_PAUSE: PatchCheat = PatchCheat {
    name: "No Pause",
    patches: &[
        // Turn on Unity's `Application.runInBackground`, which this game leaves off. When the
        // window loses focus, the engine's window procedure (`PlayerMainWndProc`, the call at
        // `UnityPlayer.dll+7C7741`) calls `SetPlayerPause` unless
        // `GetPlayerShouldRunInBackground` (`UnityPlayer.dll+5775F0`) says to keep running,
        // which mostly returns this setting. Other engine code reads the setting directly too.
        //
        // The setting is a `bool` at `+0x24F` in the engine's player settings object, where
        // its serializer stores the field named "runInBackground". The engine keeps the
        // object's address in a private global at `UnityPlayer.dll+1F15830`, which the PDB
        // doesn't name.
        Patch {
            location: Location::ThroughPointer {
                dll: Dll::UnityPlayer,
                pointer: 0x1F15830,
                offset: 0x24F,
            },
            original: &[0x00], // false
            patched: &[0x01],  // true
        },
        // `Source.TreeStateActivities.TransitionWhenFocusLostActivity$$OnPlatformFocusChanged`
        // opens the pause menu during gameplay when the game loses focus. After
        // `test dil,dil` checks `hasFocus`, always return early as if the game had focus.
        Patch {
            location: Dll::GameAssembly.at(0x7A1DFF),
            original: &[0x75, 0x7D], // jne +0x7D (to the return)
            patched: &[0xEB, 0x7D],  // jmp +0x7D
        },
        // `Source.TreeStateActivities.PauseOnFocusChangeActivity$$OnPlatformFocusChanged`
        // calls `TimeUtil$$PauseTime` or `TimeUtil$$UnpauseTime`, depending on
        // `cmp [rbx+0x68],dil` (its `m_invertPause` against `hasFocus`). Always unpause, so
        // that focusing the game also unpauses it if it was paused before turning this on.
        Patch {
            location: Dll::GameAssembly.at(0x7971C3),
            original: &[0x74, 0x3C], // je +0x3C (to PauseTime)
            patched: &[0x90; 2],     // nop
        },
    ],
};
