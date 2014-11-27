extern crate regex;

use std::default::Default;

#[deriving(Show, PartialEq)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
    Unknown,
} impl Default for Rarity {
    fn default() -> Rarity { Rarity::Unknown }
}

#[deriving(Show, Default, PartialEq)]
pub struct Item {
    rarity : Rarity,
    name : String,
    itype : String,
    hands : String, //Need better name, what can I do with this field?
    physdmg : String, // Need max, min, avg TODO support elemental
    crit_chance : String, // Translate to float
    speed : f64, //attack per second, convert to float
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
        let speed_txt = r"Attacks per Second: ([0-9]+\.[0-9]+?) \(augmented\)";
        let speed_re = regex::Regex::new(speed_txt).unwrap();
        for cap in speed_re.captures_iter(lines.next().unwrap()) {
            println!("speed matched: {}", cap.at(1));
            item.speed = from_str(cap.at(1)).unwrap();
        }

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

#[cfg(test)]
mod test{
    use super::Item;
    use super::Rarity;

    #[test]
    fn axe() {
        let axe = "Rarity: Rare\n\
                   Dragon Rend\n\
                   Labrys\n\
                   --------\n\
                   Two Handed Axe\n\
                   Physical Damage: 95-158 (augmented)\n\
                   Critical Strike Chance: 5.00%\n\
                   Attacks per Second: 1.24 (augmented)\n\
                   --------\n\
                   Requirements:\n\
                   Level: 49\n\
                   Str: 122\n\
                   Dex: 53\n\
                   --------\n\
                   Sockets: B\n\
                   --------\n\
                   Itemlevel: 68\n\
                   --------\n\
                   34% increased Physical Damage\n\
                   8% increased Attack Speed\n\
                   +9 Life gained on Kill\n\
                   +174 to Accuracy Rating";
        let result = super::Item::new(axe.to_string());
        let expected = Item{rarity: Rarity::Rare,
                            name: "Dragon Rend".to_string(),
                            itype: "Labrys".to_string(),
                            hands: "Two Handed Axe".to_string(),
                            physdmg: "Physical Damage: 95-158 (augmented)".to_string(),
                            crit_chance: "Critical Strike Chance: 5.00%".to_string(),
                            speed: 1.24,
                            req_level: "Level: 49".to_string(),
                            req_str: "Str: 122".to_string(),
                            req_dex: "Dex: 53".to_string(),
                            req_int: "".to_string(),
                            sockets: "Sockets: B".to_string(),
                            ilvl: "Itemlevel: 68".to_string(),
                            affixes: vec!("34% increased Physical Damage".to_string(),
                            "8% increased Attack Speed".to_string(),
                            "+9 Life gained on Kill".to_string(),
                            "+174 to Accuracy Rating".to_string())};
        assert_eq!(expected, result);
    }
}
