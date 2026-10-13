use proc_mem::{Module, ProcMemError, Process};
use thiserror::Error;

use crate::Dll;
use crate::il2cpp::ClassCache;
use windows::Win32::{
    Foundation::{HWND, RECT},
    UI::WindowsAndMessaging,
};

/// Reading or writing the game's memory failed, for example because the game closed.
#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("couldn't read {len} bytes of the game's memory at {address:#X}")]
    Read { address: usize, len: usize },

    #[error("couldn't write {len} bytes of the game's memory at {address:#X}")]
    Write { address: usize, len: usize },
}

/// A number the game stores in its memory as little-endian bytes, like an `i32` or a pointer.
pub trait Primitive: Copy {
    /// Decodes it from exactly `size_of::<Self>()` bytes.
    ///
    /// # Panics
    ///
    /// If `bytes` is a different length.
    fn from_le_slice(bytes: &[u8]) -> Self;

    fn to_le_vec(self) -> Vec<u8>;

    /// Decodes it from the bytes at `offset` in `bytes`, like a field from a copy of a struct.
    ///
    /// # Panics
    ///
    /// If `bytes` ends before the number does.
    fn from_le_slice_at(bytes: &[u8], offset: usize) -> Self {
        Self::from_le_slice(&bytes[offset..][..size_of::<Self>()])
    }
}

macro_rules! impl_primitive {
    ($($type:ty),*) => {$(
        impl Primitive for $type {
            fn from_le_slice(bytes: &[u8]) -> Self {
                let bytes = bytes.try_into().unwrap_or_else(|_| {
                    panic!("{} takes {} bytes, not {}", stringify!($type), size_of::<$type>(), bytes.len())
                });
                <$type>::from_le_bytes(bytes)
            }

            fn to_le_vec(self) -> Vec<u8> {
                <$type>::to_le_bytes(self).to_vec()
            }
        }
    )*};
}

impl_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, f32, f64);

/// The running game, and what's been learned about its classes so far.
#[derive(Debug)]
pub struct Popcapgame {
    proc: Process,
    game_assembly: Module,
    unity_player: Module,
    classes: ClassCache,
}

impl Popcapgame {
    pub fn init() -> Result<Self, ProcMemError> {
        let proc = Process::with_name("Replanted.exe")?;
        let game_assembly = proc.module(Dll::GameAssembly.file_name())?;
        let unity_player = proc.module(Dll::UnityPlayer.file_name())?;
        Ok(Self {
            proc,
            game_assembly,
            unity_player,
            classes: ClassCache::default(),
        })
    }

    pub fn get_rect_size(&self) -> Option<RECT> {
        let mut rect = RECT::default();

        unsafe {
            let _ = WindowsAndMessaging::GetWindowRect(
                main_window_by_pid(self.proc.process_id)?,
                &mut rect,
            );
        }

        Some(rect)
    }

    /// The address `dll` is loaded at in the game.
    pub fn dll_base(&self, dll: Dll) -> usize {
        match dll {
            Dll::GameAssembly => self.game_assembly.base_address(),
            Dll::UnityPlayer => self.unity_player.base_address(),
        }
    }

    /// Reads `len` bytes at `address`.
    pub fn read_memory(&self, address: usize, len: usize) -> Result<Vec<u8>, MemoryError> {
        let mut buf = vec![0; len];
        self.proc
            .read_bytes(address, buf.as_mut_ptr(), len)
            .then_some(buf)
            .ok_or(MemoryError::Read { address, len })
    }

    /// Reads the number at `address`.
    pub fn read<T: Primitive>(&self, address: usize) -> Result<T, MemoryError> {
        Ok(T::from_le_slice(
            &self.read_memory(address, size_of::<T>())?,
        ))
    }

    /// Reads the pointer stored at `address`.
    pub fn read_pointer(&self, address: usize) -> Result<usize, MemoryError> {
        self.read(address)
    }

    /// Writes `bytes` at `address`.
    pub fn write_memory(&self, address: usize, bytes: &[u8]) -> Result<(), MemoryError> {
        // `proc_mem` asks for a mutable pointer, although it only reads from it.
        let mut buf = bytes.to_vec();
        self.proc
            .write_bytes(address, buf.as_mut_ptr(), buf.len())
            .then_some(())
            .ok_or(MemoryError::Write {
                address,
                len: bytes.len(),
            })
    }

    /// What's been read about the game's classes, so that each is only read once.
    pub(crate) fn classes(&self) -> &ClassCache {
        &self.classes
    }
}

/// Heuristic: visible, not owned, not toolwindow, not DWM-cloaked, has title.
/// Completely vibe coded
pub fn main_window_by_pid(pid: u32) -> Option<HWND> {
    use std::mem::size_of;
    use windows::Win32::Foundation::{HWND, LPARAM};
    use windows::Win32::Graphics::Dwm::{DWMWA_CLOAKED, DwmGetWindowAttribute};
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GW_OWNER, GWL_EXSTYLE, GetWindow, GetWindowLongPtrW, GetWindowTextLengthW,
        GetWindowThreadProcessId, IsWindowVisible, WINDOW_EX_STYLE, WS_EX_TOOLWINDOW,
    };
    use windows::core::BOOL;

    struct Data {
        pid: u32,
        found: Option<HWND>,
    }

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = unsafe { &mut *(lparam.0 as *mut Data) };

        // PID match
        let mut wpid = 0u32;
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut wpid)) };
        if wpid != data.pid {
            return true.into();
        }

        // Visible
        if !unsafe { IsWindowVisible(hwnd).as_bool() } {
            return true.into();
        }

        // Not owned (owner == NULL)
        let owner = unsafe { GetWindow(hwnd, GW_OWNER).unwrap_or_default() }; // HWND(null_mut()) on none/error
        if !owner.0.is_null() {
            return true.into();
        }

        // Not a tool window
        let ex = WINDOW_EX_STYLE(unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32);
        if ex.contains(WS_EX_TOOLWINDOW) {
            return true.into();
        }

        // Not DWM-cloaked
        let mut cloaked: u32 = 0;
        let _ = unsafe {
            DwmGetWindowAttribute(
                hwnd,
                DWMWA_CLOAKED,
                &mut cloaked as *mut _ as *mut _,
                size_of::<u32>() as u32,
            )
        };
        if cloaked != 0 {
            return true.into();
        }

        // Has any title text (heuristic)
        if unsafe { GetWindowTextLengthW(hwnd) } == 0 {
            return true.into();
        }

        data.found = Some(hwnd);
        false.into()
    }

    let mut data = Data { pid, found: None };
    unsafe {
        let _ = EnumWindows(Some(enum_proc), LPARAM(&mut data as *mut _ as isize));
    }
    data.found
}
