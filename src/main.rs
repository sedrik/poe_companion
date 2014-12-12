extern crate poe_companion;

#[cfg(not(test))]
use std::io::Command;
#[cfg(not(test))]
use std::borrow::Cow;
#[cfg(not(test))]
use poe_companion::item::Item;
#[cfg(not(test))]
use std::io::timer;
use std::time::duration::Duration;

#[cfg(not(test))]
fn main() {
    let interval = Duration::seconds(4);

    loop{
        // Read clipboard
        let mut process = match Command::new("python").arg("poe-clipboard.py").spawn() {
            Ok(p) => p,
            Err(e) => panic!("Could not run python poe-clipboard.py: {}", e),
        };

        let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
        let poe_item = String::from_utf8_lossy(output.as_slice());

        let item_cstr = match poe_item {
            Cow::Owned(string) => string,
            Cow::Borrowed(slice) => slice.to_string(),
        };

        let item = Item::new(item_cstr);

        println!("{}", item);

        timer::sleep(interval);
    };
}
