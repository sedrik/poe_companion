#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate poe;

use std::io::Command;
use std::str::MaybeOwned;
use poe::item::Item;

fn main() {
    // Read clipboard
    let mut process = match Command::new("python").arg("poe-clipboard.py").spawn() {
        Ok(p) => p,
        Err(e) => panic!("Could not run python poe-clipboard.py: {}", e),
    };

    let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
    let poe_item = String::from_utf8_lossy(output.as_slice());

    let item_cstr = match poe_item {
        MaybeOwned::Owned(string) => string,
        MaybeOwned::Slice(slice) => slice.to_string(),
    };

    let item = Item::new(item_cstr);

    println!("{}", item);
}
