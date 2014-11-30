extern crate poe_companion;

#[cfg(not(test))]
use std::io::Command;
#[cfg(not(test))]
use std::str::MaybeOwned;
#[cfg(not(test))]
use poe_companion::item::Item;

#[cfg(not(test))]
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

    println!("{} (ilvl: {})  DPS: {}, pDPS: {}, eDPS: {}", item.name,
                                                           item.ilvl,
                                                           item.dps(),
                                                           item.pdps(),
                                                           item.edps());
}
