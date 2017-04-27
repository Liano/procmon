extern crate procmon;

use std::{thread};
use std::path::PathBuf;

//static PROCESS_TO_KEEP: &'static str = "C:\\Users\\nmestaoui\\Desktop\\tester\\bin\\Debug\\tester.exe";

use procmon::process::process:: { Process };
//mod process;


fn main() {
  let child = thread::spawn(|| {
    let mut counter = 0;
    loop {
      //show_process_running();
      show_all_processes();
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


fn show_all_processes() {
  if let  Ok(mut processes) = Process::get_processes() {
    println!("running processes {}", processes.len());
    while let Some(process) = processes.pop() {
      println!("{}: {} [{}]"
          , process.get_process_id()
          , process.get_name().unwrap()
          , process.get_location().unwrap_or("".to_string()));
    }
  }
}
