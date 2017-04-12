extern crate winapi;
extern crate kernel32;

use std::io::{Error};
use std::mem;
use std::ptr;
use std::os::windows::raw::HANDLE;
use std::string::String;

use self::winapi::shlobj::INVALID_HANDLE_VALUE;
use self::winapi::tlhelp32::{PROCESSENTRY32W
                             , MODULEENTRY32W
                             , MAX_MODULE_NAME32};
use self::winapi::minwindef::{DWORD
                              , FALSE
                              , MAX_PATH};
use self::winapi::tlhelp32::{TH32CS_SNAPPROCESS
                             , TH32CS_SNAPMODULE32
                             , TH32CS_SNAPMODULE};

pub struct Process {
  win_process_entry: PROCESSENTRY32W,
  main_module: MODULEENTRY32W,
}

impl Process {
  pub fn new(processentry: PROCESSENTRY32W
             , mainmodule: MODULEENTRY32W) -> Process {
    Process {
      win_process_entry: processentry,
      main_module: mainmodule,
    }
  }

  pub fn get_processes() -> Vec<Process> {

  }
  fn get_first() -> Result<(HANDLE, Process), String> {
    let snapshot_handle = unsafe { kernel32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot_handle == INVALID_HANDLE_VALUE {
      return Err(format!("Could not create snapshot: {:?}"
                         , Error::last_os_error()));
    }
    let mut win_process_entry = Process::create_new_process_entry();
    let a = &mut win_process_entry;
    let return_state = unsafe { kernel32::Process32FirstW(snapshot_handle, a) };
    if return_state == FALSE {
      return Err(format!("Could not retrieve 1st process entry: {:?}"
                         , Error::last_os_error()));
    }
    let process_id = win_process_entry.th32ProcessID;
    let module = try!(Process::get_main_module(process_id));
//    let module = match Process::get_main_module(process_id) {
//      Ok(mdl) => mdl,
//      Err(msg) => return Err(msg),
//    };

    Ok((snapshot_handle, Process::new(win_process_entry, module)))
  }
  fn get_main_module(process_id: u32) -> Result<MODULEENTRY32W, String> {
    let snapshot_handle = unsafe {
      kernel32
      ::CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id)
    };
    if snapshot_handle == INVALID_HANDLE_VALUE {
      return Err(format!("Could not create snapshot: {:?}"
                         , Error::last_os_error()));
    }

    let mut module = Process::create_new_module();
    let mdl = &mut module;
    let return_state = unsafe {
      kernel32
      ::Module32FirstW(snapshot_handle, mdl)
    };

    if return_state == FALSE {
      return Err(format!("Could not get main module: {:?}"
                         , Error::last_os_error()));
    }

    Ok(module)
  }
  fn create_new_module() -> MODULEENTRY32W {
    MODULEENTRY32W {
      dwSize: mem::size_of::<MODULEENTRY32W>() as DWORD,
      th32ModuleID: 0,
      th32ProcessID: 0,
      GlblcntUsage: 0,
      ProccntUsage: 0,
      modBaseAddr: ptr::null_mut(),
      modBaseSize: 0,
      hModule: ptr::null_mut(),
      szModule: [0; MAX_MODULE_NAME32 + 1],
      szExePath: [0; MAX_PATH],
    }
  }
  fn create_new_process_entry() -> PROCESSENTRY32W {
    PROCESSENTRY32W{
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
  fn close_handle(handle: HANDLE) {
    unsafe { kernel32::CloseHandle(handle)};
  }
}