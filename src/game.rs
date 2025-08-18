use process_memory::{DataMember, Memory, ProcessHandle, TryIntoProcessHandle};
use std::ffi::OsStr;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, MODULEENTRY32, Module32First, Module32Next, TH32CS_SNAPMODULE,
};

pub fn get_base_module_address(module_name: &str, pid: u32) -> Option<usize> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid).ok()?;
        if snapshot.is_invalid() {
            return None;
        }

        let mut module_entry = MODULEENTRY32::default();
        module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        if Module32First(snapshot, &mut module_entry).as_bool() {
            loop {
                let name_bytes = module_entry.szModule;
                let first_null = name_bytes
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(name_bytes.len());
                let name = String::from_utf8_lossy(&name_bytes[..first_null]);

                if name.eq_ignore_ascii_case(module_name) {
                    return Some(module_entry.modBaseAddr as usize);
                }

                if !Module32Next(snapshot, &mut module_entry).as_bool() {
                    break;
                }
            }
        }
    }

    None
}

#[derive(Debug)]
pub struct GameProcess {
    handle: ProcessHandle,
    base_address: usize,
}

impl GameProcess {
    #[allow(dead_code)]
    pub fn read<T: Copy>(&self, offsets: &[usize]) -> Result<T, std::io::Error> {
        let mut offsets = Vec::<usize>::from(offsets);
        if !offsets.is_empty() {
            offsets[0] += self.base_address;
        }

        let member = DataMember::<T>::new_offset(self.handle, offsets);
        unsafe { member.read() }
    }

    pub fn write<T: Copy>(&self, offsets: &[usize], value: T) -> Result<(), std::io::Error> {
        let mut offsets = Vec::<usize>::from(offsets);
        if !offsets.is_empty() {
            offsets[0] += self.base_address;
        }

        let member = DataMember::<T>::new_offset(self.handle, offsets);
        member.write(&value)
    }
}

impl Default for GameProcess {
    fn default() -> Self {
        let system = sysinfo::System::new_all();
        // system.refresh_all();

        let popcapgame = system
            .processes_by_exact_name(OsStr::new("popcapgame1.exe"))
            .next()
            .expect("Popcapgame1.exe is not running");

        let handle = (popcapgame.pid().as_u32() as process_memory::Pid)
            .try_into_process_handle()
            .expect("Couldn't get handle to popcapgame1.exe");

        let module_base_address =
            get_base_module_address("popcapgame1.exe", popcapgame.pid().as_u32())
                .expect("popcapgame1.exe module not found");

        Self {
            handle: handle,
            base_address: module_base_address,
        }
    }
}
