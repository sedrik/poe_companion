#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::Command;
use std::str::MaybeOwned;
use std::default::Default;

#[deriving(Show)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
    Unknown,
} impl Default for Rarity {
    fn default() -> Rarity { Rarity::Unknown }
}

#[deriving(Show, Default)]
pub struct Item {
    rarity : Rarity,
    name : String,
    itype : String,
    hands : String, //Need better name, what can I do with this field?
    physdmg : String, // Need max, min, avg TODO support elemental
    crit_chance : String, // Translate to float
    speed : String, //attack per second, convert to float
    req_level : String, // Convert to int
    req_str : String, // Convert to int
    req_dex : String, // Convert to int
    req_int : String, // convert to int
    sockets : String, // represent, sockets, links, colours
    ilvl : String, // Convert to int
    affixes : Vec<String>, // represent differnt affixes in a good way?
} impl Item {
    pub fn new(input: String) -> Item {
        let mut item: Item = Default::default();
        let mut lines = input.split('\n');

        // Rarity
        // Ex: Rarity: Rare
        item.rarity = match lines.next(){
            Some("Rarity: Normal") => Rarity::Normal,
            Some("Rarity: Magic") => Rarity::Magic,
            Some("Rarity: Rare") => Rarity::Rare,
            Some("Rarity: Unique") => Rarity::Unique,
            other => panic!("unexpected rarity: {}", other),
        };

        // name
        // Ex: Dragon Rend
        item.name = lines.next().unwrap().to_string();
        // type
        // Ex: Labrys
        item.itype = lines.next().unwrap().to_string();

        // separator
        // --------
        lines.next();

        // hands
        // Ex: Two Handed Axe
        item.hands = lines.next().unwrap().to_string();

        // Physical Damage: 95-158 (augmented)
        item.physdmg = lines.next().unwrap().to_string();

        // Critical Strike Chance: 5.00%
        item.crit_chance = lines.next().unwrap().to_string();

        // Attacks per Second: 1.24 (augmented)
        item.speed = lines.next().unwrap().to_string();

        // separator
        // --------
        lines.next();

        // Requirements:
        lines.next();
        // Level: 49
        item.req_level = lines.next().unwrap().to_string();
        // Str: 122
        item.req_str = lines.next().unwrap().to_string();
        // Dex: 53
        item.req_dex = lines.next().unwrap().to_string();
        //req_int : String; // convert to int

        // separator
        // --------
        lines.next();

        // Sockets: B 
        item.sockets = lines.next().unwrap().to_string();

        // separator
        // --------
        lines.next();

        // Itemlevel: 68
        item.ilvl = lines.next().unwrap().to_string();

        // separator
        // --------
        lines.next();

        // Affixes
        // 34% increased Physical Damage
        // 8% increased Attack Speed
        // +9 Life gained on Kill
        // +174 to Accuracy Rating
        loop {
            match lines.next() {
                Some("") => continue, //New lines in the end and what not.
                Some(affix) => item.affixes.push(affix.to_string()),
                None => break,
            }
        }

        return item
    }
}

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
