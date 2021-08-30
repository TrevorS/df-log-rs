extern crate notify;

mod event;
mod gamelog;

use gamelog::Gamelog;

fn main() {
    let filename = "./data/gamelog.txt";

    let mut gamelog = Gamelog::new(filename);
    let rx = gamelog.connect().expect("Failed to read gamelog.txt!");

    loop {
        match rx.recv() {
            Ok(event) => {
                dbg!(event);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
