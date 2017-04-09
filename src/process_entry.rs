extern crate winapi;

use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::mem;

use self::winapi::tlhelp32::PROCESSENTRY32W;
use self::winapi::minwindef::{DWORD, MAX_PATH};

pub struct ProcessEntry {
  process : PROCESSENTRY32W
}

impl ProcessEntry {
  fn get_pid(&self) -> u32 {
    self.process.th32ProcessID
  }
  fn get_full_path(&self) -> PathBuf {
    let process_path = self.process.szExeFile;
    PathBuf::from(OsString::from_wide(process_path.iter()
      .position(|c| *c == 0)
      .map(|i| &process_path[..i])
      .unwrap_or(&process_path)))
  }
}
impl Default for ProcessEntry {
  fn default() -> ProcessEntry {
    ProcessEntry {
        process : PROCESSENTRY32W {
        dwSize: mem::size_of::<PROCESSENTRY32W>() as DWORD,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; MAX_PATH],
      }
    }
  }
}