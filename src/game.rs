use process_memory::{DataMember, Memory, ProcessHandle, TryIntoProcessHandle};
use std::ffi::OsStr;
use sysinfo::Pid;
use thiserror::Error;
use windows::Win32::{
    Foundation::{HWND, RECT},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, MODULEENTRY32, Module32First, Module32Next, TH32CS_SNAPMODULE,
    },
    UI::WindowsAndMessaging,
};

pub fn get_base_module_address(module_name: &str, pid: u32) -> Option<usize> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid).ok()?;
        if snapshot.is_invalid() {
            return None;
        }

        let mut module_entry = MODULEENTRY32 {
            dwSize: size_of::<MODULEENTRY32>() as u32,
            ..Default::default()
        };
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
    pid: Pid,
    base_address: usize,
}

#[allow(dead_code)]
impl GameProcess {
    pub fn get_rect_size(&self) -> RECT {
        let mut rect = RECT::default();
        unsafe {
            let _ = WindowsAndMessaging::GetWindowRect(
                main_window_by_pid(self.pid.as_u32()).expect("asd"),
                &mut rect,
            );
        }
        rect
    }

    pub fn read<T: Copy>(&self, offsets: &[usize]) -> Result<T, std::io::Error> {
        let offsets = Vec::<usize>::from(offsets);

        let member = DataMember::<T>::new_offset(self.handle, offsets);
        unsafe { member.read() }
    }

    pub fn read_with_base_addr<T: Copy>(&self, offsets: &[usize]) -> Result<T, std::io::Error> {
        let mut out = offsets.to_vec();
        if let Some(first) = out.first_mut() {
            *first += self.base_address
        }

        self.read(&out)
    }

    /// Read memory at `addr`
    pub fn read_at<T: Copy>(&self, addr: usize) -> Result<T, std::io::Error> {
        self.read(&[addr])
    }

    pub fn write<T: Copy>(&self, offsets: &[usize], value: T) -> Result<(), std::io::Error> {
        let mut offsets = Vec::<usize>::from(offsets);
        if !offsets.is_empty() {
            offsets[0] += self.base_address;
        }

        let member = DataMember::<T>::new_offset(self.handle, offsets);
        member.write(&value)
    }

    pub fn init() -> Result<Self, GameProcessInitError> {
        let system = sysinfo::System::new_all();
        // system.refresh_all();

        let popcapgame = system
            .processes_by_exact_name(OsStr::new("popcapgame1.exe"))
            .next()
            .ok_or(GameProcessInitError::GameNotRunning)?;

        let handle =
            (popcapgame.pid().as_u32() as process_memory::Pid).try_into_process_handle()?;

        let module_base_address =
            get_base_module_address("popcapgame1.exe", popcapgame.pid().as_u32())
                .ok_or(GameProcessInitError::BaseModuleNotFound)?;

        Ok(Self {
            handle,
            pid: popcapgame.pid(),
            base_address: module_base_address,
        })
    }
}

impl Default for GameProcess {
    fn default() -> Self {
        GameProcess::init().unwrap()
    }
}

#[derive(Error, Debug)]
pub enum GameProcessInitError {
    #[error("popcapgame1.exe is not running")]
    GameNotRunning,
    #[error("Couldn't get handle to popcapgame1.exe")]
    CoudlntGetProcessHandle(#[from] std::io::Error),
    #[error("popcapgame1.exe module not found")]
    BaseModuleNotFound,
}

/// Heuristic: visible, not owned, not toolwindow, not DWM-cloaked, has title.
/// Completely vibe coded
fn main_window_by_pid(pid: u32) -> Option<HWND> {
    use std::mem::size_of;
    use windows::Win32::Foundation::{BOOL, LPARAM};
    use windows::Win32::Graphics::Dwm::{DWMWA_CLOAKED, DwmGetWindowAttribute};
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GW_OWNER, GWL_EXSTYLE, GetWindow, GetWindowLongPtrW, GetWindowTextLengthW,
        GetWindowThreadProcessId, IsWindowVisible, WS_EX_TOOLWINDOW,
    };
    #[derive(Debug)]
    struct Data {
        pid: u32,
        found: Option<HWND>,
    }

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = unsafe { &mut *(lparam.0 as *mut Data) };
        let mut wpid = 0u32;
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut wpid)) };
        if wpid != data.pid {
            return true.into();
        }
        if !unsafe { IsWindowVisible(hwnd).as_bool() }
            || unsafe { GetWindow(hwnd, GW_OWNER).0 } != 0
        {
            return true.into();
        }
        let ex = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32;
        if (ex & WS_EX_TOOLWINDOW.0) != 0 {
            return true.into();
        }
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
        if unsafe { GetWindowTextLengthW(hwnd) } == 0 {
            return true.into();
        }

        data.found = Some(hwnd);
        false.into()
    }

    let mut data = Data { pid, found: None };
    unsafe {
        EnumWindows(Some(enum_proc), LPARAM(&mut data as *mut _ as isize));
    }
    data.found
}
