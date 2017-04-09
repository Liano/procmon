use std:: { thread, process };

mod process_entry;
mod process_iterator;

fn main() {
  let child = thread::spawn(|| {
    let mut counter = 0;
    loop {
      thread::sleep_ms(294);
      println!("ligour_sei {}", "wayyeh");
      counter += 1;
      if counter == 10 {
        process::exit(0);
      }
    }
  });

  if child.join().is_err() {
    println!("ligour occurred");
  }
}
