use proc_mem::{Module, ProcMemError, Process};
use windows::Win32::{
    Foundation::{HWND, RECT},
    UI::WindowsAndMessaging,
};

use crate::models::MemoryParseable;

#[derive(Debug)]
pub struct Popcapgame {
    proc: Process,
    base_module: Module,
}

#[allow(dead_code)]
impl Popcapgame {
    pub fn get_rect_size(&self) -> RECT {
        let mut rect = RECT::default();

        unsafe {
            let _ = WindowsAndMessaging::GetWindowRect(
                main_window_by_pid(self.proc.process_id).unwrap(),
                &mut rect,
            );
        }

        rect
    }

    // Parseable
    pub fn read_parseable<T: MemoryParseable>(&self, offsets: &[usize]) -> Result<T, ProcMemError> {
        let buf = self.read_bytes(offsets, T::size_of())?.unwrap();
        Ok(T::from_bytes(buf))
    }

    pub fn read_parseable_with_base_addr<T: MemoryParseable>(
        &self,
        offsets: &[usize],
    ) -> Result<T, ProcMemError> {
        let mut out = offsets.to_vec();
        out.insert(0, self.base_module.base_address());
        self.read_parseable(&out)
    }

    pub fn read_parseable_at<T: MemoryParseable>(&self, addr: usize) -> Result<T, ProcMemError> {
        let buf = self.read_bytes_at(addr, T::size_of()).unwrap();
        Ok(T::from_bytes(buf))
    }

    // More flexible
    pub fn read<T: Default>(&self, offsets: &[usize]) -> Result<T, ProcMemError> {
        self.proc.read_mem_chain::<T>(offsets.to_vec())
    }

    pub fn read_with_base_addr<T: Default>(&self, offsets: &[usize]) -> Result<T, ProcMemError> {
        let mut out = offsets.to_vec();
        out.insert(0, self.base_module.base_address());
        self.read(&out)
    }

    pub fn read_at<T: Default>(&self, addr: usize) -> Result<T, ProcMemError> {
        self.proc.read_mem::<T>(addr)
    }

    // Raw bytes
    pub fn read_bytes(
        &self,
        offsets: &[usize],
        count: usize,
    ) -> Result<Option<Vec<u8>>, ProcMemError> {
        let addr: usize = self.proc.read_ptr_chain(offsets.to_vec())?;
        let mut buf = vec![0; count];
        if self.proc.read_bytes(addr, buf.as_mut_ptr(), count) {
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }

    pub fn read_bytes_with_base_addr(
        &self,
        offsets: &[usize],
        count: usize,
    ) -> Result<Option<Vec<u8>>, ProcMemError> {
        let mut out = offsets.to_vec();
        out.insert(0, self.base_module.base_address());
        self.read_bytes(&out, count)
    }

    pub fn read_bytes_at(&self, addr: usize, count: usize) -> Option<Vec<u8>> {
        let mut buf = vec![0; count];
        if self.proc.read_bytes(addr, buf.as_mut_ptr(), count) {
            Some(buf)
        } else {
            None
        }
    }

    pub fn write_at<T: Default>(&self, addr: usize, value: T) -> bool {
        self.proc.write_mem::<T>(addr, value)
    }

    pub fn write<T: Default>(&self, offsets: &[usize], value: T) -> Result<(), ProcMemError> {
        let mut offsets = offsets.to_vec();
        offsets.insert(0, self.base_module.base_address());

        self.proc
            .write_mem::<T>(self.proc.read_ptr_chain(offsets)?, value);

        Ok(())
    }

    pub fn init() -> Result<Self, ProcMemError> {
        let proc = Process::with_name("popcapgame1.exe")?;
        let base_module = proc.module("popcapgame1.exe")?;
        Ok(Self { proc, base_module })
    }
}

impl Default for Popcapgame {
    fn default() -> Self {
        Popcapgame::init().unwrap()
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
