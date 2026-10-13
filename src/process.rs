//! Opening another process and reading and writing its memory, through Windows' API.

use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};

use thiserror::Error;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CREATE_TOOLHELP_SNAPSHOT_FLAGS, CreateToolhelp32Snapshot, MODULEENTRY32W, Module32FirstW,
    Module32NextW, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ,
    PROCESS_VM_WRITE,
};

/// Opening the game failed.
#[derive(Debug, Error)]
pub enum AttachError {
    #[error("{name} isn't running")]
    NotRunning { name: &'static str },

    #[error("{name} hasn't loaded {module} yet")]
    ModuleNotLoaded {
        name: &'static str,
        module: &'static str,
    },

    #[error("Windows couldn't {action}: {source}")]
    Windows {
        action: &'static str,
        source: windows::core::Error,
    },
}

/// A running process, opened to read and write its memory.
#[derive(Debug)]
pub(crate) struct Process {
    name: &'static str,
    id: u32,
    handle: OwnedHandle,
}

impl Process {
    /// Opens the running process whose executable is named `name`, like `Replanted.exe`.
    pub fn open(name: &'static str) -> Result<Self, AttachError> {
        let snapshot = snapshot(TH32CS_SNAPPROCESS, 0, "list the running processes")?;
        let entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let id = snapshot_entries(&snapshot, entry, Process32FirstW, Process32NextW)
            .find(|process| from_wide(&process.szExeFile).eq_ignore_ascii_case(name))
            .ok_or(AttachError::NotRunning { name })?
            .th32ProcessID;

        // Writing to the game's code needs `PROCESS_VM_OPERATION` too, because
        // `WriteProcessMemory` makes the code's pages writable while it writes.
        let access = PROCESS_VM_READ
            | PROCESS_VM_WRITE
            | PROCESS_VM_OPERATION
            | PROCESS_QUERY_LIMITED_INFORMATION;
        // SAFETY: `OpenProcess` returns a new handle, which nothing else owns.
        let handle = unsafe { OpenProcess(access, false, id) }
            .map(|handle| unsafe { OwnedHandle::from_raw_handle(handle.0) })
            .map_err(|source| AttachError::Windows {
                action: "open the game's process",
                source,
            })?;
        Ok(Self { name, id, handle })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Where the module named `module`, like `GameAssembly.dll`, is loaded in the process.
    pub fn module_base(&self, module: &'static str) -> Result<usize, AttachError> {
        let snapshot = snapshot(
            TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32,
            self.id,
            "list the game's modules",
        )?;
        let entry = MODULEENTRY32W {
            dwSize: size_of::<MODULEENTRY32W>() as u32,
            ..Default::default()
        };
        snapshot_entries(&snapshot, entry, Module32FirstW, Module32NextW)
            .find(|loaded| from_wide(&loaded.szModule).eq_ignore_ascii_case(module))
            .map(|loaded| loaded.modBaseAddr as usize)
            .ok_or(AttachError::ModuleNotLoaded {
                name: self.name,
                module,
            })
    }

    /// Fills `buffer` with the bytes at `address`.
    pub fn read(&self, address: usize, buffer: &mut [u8]) -> windows::core::Result<()> {
        // SAFETY: Windows writes at most `buffer.len()` bytes into `buffer`.
        unsafe {
            ReadProcessMemory(
                self.raw_handle(),
                address as *const _,
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                None,
            )
        }
    }

    /// Writes `bytes` at `address`.
    pub fn write(&self, address: usize, bytes: &[u8]) -> windows::core::Result<()> {
        // SAFETY: Windows only reads `bytes.len()` bytes from `bytes`.
        unsafe {
            WriteProcessMemory(
                self.raw_handle(),
                address as *const _,
                bytes.as_ptr().cast(),
                bytes.len(),
                None,
            )
        }
    }

    fn raw_handle(&self) -> HANDLE {
        HANDLE(self.handle.as_raw_handle())
    }
}

/// A Toolhelp snapshot of the running processes, or of one process's modules.
fn snapshot(
    flags: CREATE_TOOLHELP_SNAPSHOT_FLAGS,
    process_id: u32,
    action: &'static str,
) -> Result<OwnedHandle, AttachError> {
    // SAFETY: `CreateToolhelp32Snapshot` returns a new handle, which nothing else owns.
    unsafe { CreateToolhelp32Snapshot(flags, process_id) }
        .map(|handle| unsafe { OwnedHandle::from_raw_handle(handle.0) })
        .map_err(|source| AttachError::Windows { action, source })
}

/// What a Toolhelp snapshot has, one entry at a time. Windows hands out entries with a pair
/// of functions, like `Process32FirstW` and `Process32NextW`, which fill in `entry`.
fn snapshot_entries<E: Copy>(
    snapshot: &OwnedHandle,
    mut entry: E,
    first: unsafe fn(HANDLE, *mut E) -> windows::core::Result<()>,
    next: unsafe fn(HANDLE, *mut E) -> windows::core::Result<()>,
) -> impl Iterator<Item = E> {
    let handle = HANDLE(snapshot.as_raw_handle());
    let mut step = first;
    std::iter::from_fn(move || {
        // SAFETY: `entry` is the kind of entry `step` fills in, with its size set.
        let found = unsafe { step(handle, &mut entry) }.is_ok();
        step = next;
        found.then_some(entry)
    })
}

/// A NUL-terminated UTF-16 string from a fixed-size array, like a process's name.
fn from_wide(chars: &[u16]) -> String {
    let len = chars
        .iter()
        .position(|&char| char == 0)
        .unwrap_or(chars.len());
    String::from_utf16_lossy(&chars[..len])
}
