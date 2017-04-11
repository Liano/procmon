use std::{thread};
use std::path::PathBuf;

//static PROCESS_TO_KEEP: &'static str = "C:\\Users\\nmestaoui\\Desktop\\tester\\bin\\Debug\\tester.exe";

mod process_entry;
mod process_iterator;
mod toolhelp_32_snapshot_handle;
mod process;

use process_iterator::ProcessIterator;

fn main() {
  let child = thread::spawn(|| {
    let mut counter = 0;
    loop {
      show_process_running();
      counter += 1;
      if counter == 1 {
        break;
      }
      thread::sleep_ms(10_000);
    }
  });

  if child.join().is_err() {
    println!("ligour occurred");
  }
}

fn show_process_running() {
  let res_process_iterator = ProcessIterator::new();
  if let Ok(mut process_iterator) = res_process_iterator {
    let mut process_count = 0;
    while let Some(res_process_entry) = process_iterator.next() {
      if let Ok(process) = res_process_entry {
        process_count += 1;
        let process_path : PathBuf = process.executable_name();
        if let Some(process_path_str) = process_path.to_str() {
          println!("{}: {}", process.process_id(), process_path_str);
        }
      }
    }

    println!("There are {} process(s)", process_count);
  }
}
