use std::string::String;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub fn get_string_from_wide(wide_array: &[u16]) -> Result<String, String> {
  let trimmed_wide = wide_array.iter()
    .position(|char| *char == 0)
    .map(|i| &wide_array[..i])
    .unwrap_or(wide_array);
  let os_str = OsString::from_wide(trimmed_wide);

  os_str.into_string()
    .or(Err("Could not convert `OsString` to `String`".to_string()))
}
