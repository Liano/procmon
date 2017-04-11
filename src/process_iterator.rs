extern crate kernel32;
extern crate winapi;

use std::io::{Error, Result};

use self::winapi::shlobj::INVALID_HANDLE_VALUE;
use self::winapi::tlhelp32:: { TH32CS_SNAPPROCESS, TH32CS_SNAPMODULE };
use self::winapi::winerror::{ERROR_NO_MORE_FILES, ERROR_SUCCESS};

use process_entry::ProcessEntry;
use toolhelp_32_snapshot_handle::Toolhelp32SnapshotHandle;

#[derive(Debug)]
pub struct ProcessIterator {
  handle: Toolhelp32SnapshotHandle,
  first: Option<ProcessEntry>,
}

impl ProcessIterator {
  pub fn new() -> Result<ProcessIterator> {
    let handle = unsafe { kernel32::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS | TH32CS_SNAPMODULE, 0) };
    if handle == INVALID_HANDLE_VALUE {
      return Err(Error::last_os_error());
    }
    let mut handle = Toolhelp32SnapshotHandle::new(handle);
    let entry = handle.first()?;

    Ok(ProcessIterator {
      handle: handle,
      first: Some(entry),
    })
  }
}

impl Iterator for ProcessIterator {
  type Item = Result<ProcessEntry>;

  fn next(&mut self) -> Option<Result<ProcessEntry>> {
    match self.first.take() {
      Some(entry) => Some(Ok(entry)),
      None => {
        match self.handle.next() {
          Err(ref error) if error.raw_os_error().unwrap_or(ERROR_SUCCESS as i32) ==
            ERROR_NO_MORE_FILES as i32 => None,
          x => Some(x),
        }
      }
    }
  }
}