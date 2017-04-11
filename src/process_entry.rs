extern crate winapi;

use std::ffi::OsString;
use std::fmt;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;

use self::winapi::minwindef::{DWORD, MAX_PATH};
pub use self::winapi::tlhelp32::{PROCESSENTRY32W, MODULEENTRY32W, MAX_MODULE_NAME32};

pub struct ProcessEntry(PROCESSENTRY32W, MODULEENTRY32W);

impl ProcessEntry {
  pub fn raw_proc(&mut self) -> &mut PROCESSENTRY32W {
    &mut self.0
  }
  pub fn raw_module(&mut self) -> &mut MODULEENTRY32W {
    &mut self.1
  }

  pub fn process_id(&self) -> u32 {
    self.0.th32ProcessID
  }

  pub fn threads(&self) -> u32 {
    self.0.cntThreads
  }

  pub fn parent_process_id(&self) -> u32 {
    self.0.th32ParentProcessID
  }

  pub fn base_priority(&self) -> i32 {
    self.0.pcPriClassBase
  }

  pub fn executable_name(&self) -> PathBuf {
    let name = self.0.szExeFile;
    PathBuf::from(OsString::from_wide(name.iter()
      .position(|c| *c == 0)
      .map(|i| &name[..i])
      .unwrap_or(&name)))
  }
}

impl Default for ProcessEntry {
  fn default() -> ProcessEntry {
    //println!("init proc entry");
    ProcessEntry(PROCESSENTRY32W {
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
    }, MODULEENTRY32W {
      dwSize: mem::size_of::<MODULEENTRY32W>() as DWORD,
      th32ModuleID : 0,
      th32ProcessID : 0,
      GlblcntUsage : 0,
      ProccntUsage : 0,
      modBaseAddr : ptr::null_mut(),
      modBaseSize : 0,
      hModule : ptr::null_mut(),
      szModule : [0; MAX_MODULE_NAME32 + 1],
      szExePath : [0; MAX_PATH],
    })
  }
}

impl fmt::Debug for ProcessEntry {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.debug_struct("ProcessEntry")
      .field("process_id", &self.process_id())
      .field("threads", &self.threads())
      .field("parent_process_id", &self.parent_process_id())
      .field("base_priority", &self.base_priority())
      .field("executable_name", &self.executable_name())
      .finish()
  }
}