extern crate regex;

use std::default::Default;
use std::iter::AdditiveIterator;

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

#[deriving(Show, PartialEq)]
enum DamageType {
    Physical,
    Lightning,
    Ice,
    Fire,
    Chaos
} impl Default for DamageType {
    fn default() -> DamageType { DamageType::Physical }
}

#[deriving(Show, PartialEq)]
struct Dmg {
    dmgtype : DamageType,
    min : int,
    max : int,
} impl Default for Dmg {
    fn default() -> Dmg { Dmg{dmgtype : DamageType::Physical,
                          min : 0,
                          max : 0} }
}

#[deriving(Show, Default, PartialEq)]
pub struct Item {
    rarity : Rarity,
    pub name : String,
    itype : String,
    hands : String, //Need better name, what can I do with this field?
    damage : Vec<Dmg>, // TODO Should probably be a Vec with all the damage types
    crit_chance : f64,
    speed : f64,
    req_level : int,
    req_str : int,
    req_dex : int,
    req_int : int,
    sockets : String, // represent, sockets, links, colours TODO
    pub ilvl : int,
    affixes : Vec<String>, // represent differnt affixes in a good way? TODO
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
        let dmg_txt = r"([:alpha:]+) Damage: ([0-9]+)-([0-9]+) \(augmented\)";
        let dmg_re = regex::Regex::new(dmg_txt).unwrap();
        for cap in dmg_re.captures_iter(lines.next().unwrap()) {
            let dmg = Dmg {
                dmgtype : {
                    match cap.at(1) {
                        "Physical" => DamageType::Physical,
                        "" => panic!("No dmgtype found"),
                        _ => panic!("Dmg regex did not match"),
                    }
                },
                min : from_str(cap.at(2)).unwrap(),
                max : from_str(cap.at(3)).unwrap(),
            };
            item.damage.push(dmg);
        }

        // Critical Strike Chance: 5.00%
        let crit_txt = r"Critical Strike Chance: ([0-9]+\.[0-9]+?)%";
        let crit_re = regex::Regex::new(crit_txt).unwrap();
        for cap in crit_re.captures_iter(lines.next().unwrap()) {
            item.crit_chance = from_str(cap.at(1)).unwrap();
        }

        // Attacks per Second: 1.24 (augmented)
        let speed_txt = r"Attacks per Second: ([0-9]+\.[0-9]+?) \(augmented\)";
        let speed_re = regex::Regex::new(speed_txt).unwrap();
        for cap in speed_re.captures_iter(lines.next().unwrap()) {
            item.speed = from_str(cap.at(1)).unwrap();
        }

        // separator
        // --------
        lines.next();

        // Requirements:
        lines.next();
        // Level: 49
        let level_txt = r"Level: ([0-9]+)";
        let level_re = regex::Regex::new(level_txt).unwrap();
        for cap in level_re.captures_iter(lines.next().unwrap()) {
            item.req_level = from_str(cap.at(1)).unwrap();
        }

        // Str: 122
        let req_str_txt = r"Str: ([0-9]+)";
        let req_str_re = regex::Regex::new(req_str_txt).unwrap();
        for cap in req_str_re.captures_iter(lines.next().unwrap()) {
            item.req_str = from_str(cap.at(1)).unwrap();
        }

        // Dex: 53
        let req_dex_txt = r"Dex: ([0-9]+)";
        let req_dex_re = regex::Regex::new(req_dex_txt).unwrap();
        for cap in req_dex_re.captures_iter(lines.next().unwrap()) {
            item.req_dex = from_str(cap.at(1)).unwrap();
        }
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
        let ilvl_txt = r"Itemlevel: ([0-9]+)";
        let ilvl_re = regex::Regex::new(ilvl_txt).unwrap();
        for cap in ilvl_re.captures_iter(lines.next().unwrap()) {
            item.ilvl = from_str(cap.at(1)).unwrap();
        }

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

    pub fn dps(&self) -> f64 {
        self.damage.iter()
                   .map(|dmg| Item::dps_calc(dmg.min, dmg.max, self.speed))
                   .sum()
    }

    pub fn pdps(&self) -> f64 {
        self.damage.iter()
                   .find(|dmg| dmg.dmgtype == DamageType::Physical)
                   .map(|dmg| Item::dps_calc(dmg.min, dmg.max, self.speed))
                   .unwrap()
    }

    pub fn edps(&self) -> f64 {
        match self.damage.iter()
                         .find(|dmg| dmg.dmgtype == DamageType::Lightning ||
                                     dmg.dmgtype == DamageType::Fire ||
                                     dmg.dmgtype == DamageType::Ice)
                         .map(|dmg| Item::dps_calc(dmg.min, dmg.max,
                                                   self.speed)) {
                Some(dps) => dps,
                None => 0.0
            }

    }

    fn dps_calc(min : int, max : int, speed : f64) -> f64 {
        (min + max) as f64 / 2.0 * speed
    }
}

#[cfg(test)]
mod test{
    use super::Item;
    use super::Rarity;
    use super::DamageType;
    use super::Dmg;

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
        let item = super::Item::new(axe.to_string());
        let expected = Item{rarity: Rarity::Rare,
                            name: "Dragon Rend".to_string(),
                            itype: "Labrys".to_string(),
                            hands: "Two Handed Axe".to_string(),
                            damage: vec!(
                                Dmg {
                                    dmgtype : DamageType::Physical,
                                    min : 95,
                                    max : 158,
                                }),
                            crit_chance: 5.00,
                            speed: 1.24,
                            req_level: 49,
                            req_str: 122,
                            req_dex: 53,
                            req_int: 0,
                            sockets: "Sockets: B".to_string(),
                            ilvl: 68,
                            affixes: vec!("34% increased Physical Damage".to_string(),
                                          "8% increased Attack Speed".to_string(),
                                          "+9 Life gained on Kill".to_string(),
                                          "+174 to Accuracy Rating".to_string())};
        assert_eq!(expected, item);
        assert!(item.dps() - 156.86 < 0.001);
        assert!(item.pdps() - 156.86 < 0.001);
        assert_eq!(item.edps(), 0.0);
    }
}
