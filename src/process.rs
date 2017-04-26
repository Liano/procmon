extern crate winapi;
extern crate kernel32;

use std::ffi::OsString;
use std::io::{Error};
use std::mem;
use std::ptr;
use std::os::windows::raw::HANDLE;
use std::os::windows::ffi::OsStringExt;
use std::string::String;
use std::vec::{Vec};

use self::winapi::shlobj::INVALID_HANDLE_VALUE;
use self::winapi::winerror::ERROR_NO_MORE_FILES;
use self::winapi::tlhelp32::{PROCESSENTRY32W
                             , MODULEENTRY32W
                             , MAX_MODULE_NAME32};
use self::winapi::minwindef::{DWORD
                              , FALSE
                              , MAX_PATH};
use self::winapi::tlhelp32::{TH32CS_SNAPPROCESS
                             , TH32CS_SNAPMODULE32
                             , TH32CS_SNAPMODULE};

const INITIAL_PROCESS_VECTOR_SIZE: u32 = 20;

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


  /// Get a list of currently running process in the system as `Vec`
  /// # Example
  /// ```
  /// use process::Process;
  ///
  /// let processes = Process::get_processes();
  /// for process in processes {
  ///   println!("{}: {}", process.get_process_id(), process.get_process_name().unwrap();
  /// }
  /// ```
  pub fn get_processes() -> Result<Vec<Process>, String> {
    let mut vec: Vec<Process> = Vec::new();
    let (handle, first_process) = try!(Process::get_first());
    vec.push(first_process);
    println!("got first");

    loop {
      let next_process = Process::get_next(handle);
      match next_process {
        Ok(opt_process) => {
          if let Some(process) = opt_process {
            vec.push(process);
          } else { break; }
        },
        Err(error) => println!("{:?}", error),
      }
    }

//    while let Ok(next_process) = Process::get_next(handle) {
//      vec.push(next_process);
//      println!("got next");
//    }
    println!("process retrieved {}", vec.len());
    //close the handle at OS side
    Process::close_handle(handle);
    return Ok(vec);
  }
  fn get_next(snapshot_handle: HANDLE) -> Result<Option<Process>, String> {
    let mut win_process_entry = Process::create_new_process_entry();
    {
      let mut_win_process_entry = &mut win_process_entry;
      let return_state = unsafe { kernel32::Process32NextW(snapshot_handle, mut_win_process_entry) };
      if return_state == FALSE {
        let last_error = Error::last_os_error();
        if let Some(raw_error) = last_error.raw_os_error() {
          if raw_error == ERROR_NO_MORE_FILES as i32 {
            println!("No more processes");
            return Ok(None);
          }
        }
        return Err(format!("Could not retrieve next process entry: {:?}"
                           , Error::last_os_error()));
      }
    }
    println!("process entry retrieved");

    let module = try!(Process::get_main_module(&win_process_entry.th32ProcessID));
    println!("process module retrieved");

    Ok(Some(Process::new(win_process_entry, module)))
  }
  fn get_first() -> Result<(HANDLE, Process), String> {
    let snapshot_handle = unsafe { kernel32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot_handle == INVALID_HANDLE_VALUE {
      return Err(format!("Could not create snapshot: {:?}"
                         , Error::last_os_error()));
    }
    let mut win_process_entry = Process::create_new_process_entry();
    {
      let a = &mut win_process_entry;
      let return_state = unsafe { kernel32::Process32FirstW(snapshot_handle, a) };
      if return_state == FALSE {
        return Err(format!("Could not retrieve 1st process entry: {:?}"
                           , Error::last_os_error()));
      }
    }
    println!("process entry retrieved");
    let process_id = win_process_entry.th32ProcessID;
    let module = try!(Process::get_main_module(&process_id));
    println!("process module retrieved");
    //    let module = match Process::get_main_module(process_id) {
    //      Ok(mdl) => mdl,
    //      Err(msg) => return Err(msg),
    //    };

    Ok((snapshot_handle, Process::new(win_process_entry, module)))
  }
  fn get_main_module(process_id: &u32) -> Result<MODULEENTRY32W, String> {
    let snapshot_handle = unsafe {
      kernel32
      ::CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, *process_id)
    };
    if snapshot_handle == INVALID_HANDLE_VALUE {
      return Err(format!("Could not create snapshot: {:?}"
                         , Error::last_os_error()));
    }

    let mut module = Process::create_new_module();
    {
      let mdl = &mut module;
      let return_state = unsafe {
        kernel32::Module32FirstW(snapshot_handle, mdl)
      };

      if return_state == FALSE {
        return Err(format!("Could not get main module: {:?}"
                           , Error::last_os_error()));
      }
    }

    //close the snapshot handle
    Process::close_handle(snapshot_handle);
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
    PROCESSENTRY32W {
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
    unsafe { kernel32::CloseHandle(handle) };
  }

  pub fn get_process_id(&self) -> u32 {
    self.win_process_entry.th32ProcessID
  }
  pub fn get_thread_count(&self) -> u32 {
    self.win_process_entry.cntThreads
  }
  pub fn get_parent_process_id(&self) -> u32 {
    self.win_process_entry.th32ParentProcessID
  }
  pub fn get_base_priority(&self) -> i32 {
    self.win_process_entry.pcPriClassBase
  }
  pub fn get_name(&self) -> Result<String, String> {
    let name = self.win_process_entry.szExeFile;
    get_string_from_wide(&name)
    //    let exe_name : OsString = OsString::from_wide(name.iter()
    //        .position(|c| *c == 0)
    //        .map(|i| &name[..i])
    //        .unwrap_or(&name));
    //
    //    exe_name.into_string()
    //      .or(Err("Could not retrieve process name".to_string()))
  }
  pub fn get_location(&self) -> Result<String, String> {
    get_string_from_wide(&self.main_module.szExePath)
  }
}

fn get_string_from_wide(wide_array: &[u16]) -> Result<String, String> {
  let trimmed_wide = wide_array.iter()
    .position(|char| *char == 0)
    .map(|i| &wide_array[..i])
    .unwrap_or(wide_array);
  let os_str = OsString::from_wide(trimmed_wide);

  os_str.into_string()
    .or(Err("Could not convert `OsString` to `String`".to_string()))
}