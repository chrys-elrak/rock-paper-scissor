use std::{thread, io::{self, Write}, time};
#[allow(dead_code)]
pub fn loading(loader_text: &str, duration: u16) {
    for v in loader_text.as_bytes().iter() {
        thread::sleep(time::Duration::from_millis(duration as u64));
        print!("{}", *v as char);
        io::stdout().flush().expect("Failed to flush loading text");
    }
}