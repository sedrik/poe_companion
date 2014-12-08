extern crate regex;

use std::default::Default;
use std::iter::AdditiveIterator;
use std::str::CharSplits;

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
//    Lightning,
//    Ice,
//    Fire,
    Elemental, //TODO type of damage is unknown until parsing of affixes
//    Chaos,
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
    itype : String, // TODO rename to basetype
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
    implicit : String,
    affixes : Vec<String>, // represent differnt affixes in a good way? TODO
} impl Item {
    pub fn new(input: String) -> Item {
        let mut item: Item = Default::default();
        let mut lines = input.split('\n');

        // Header info, contains rarity, name and base type
        Item::parse_rarity(&mut lines, &mut item);
        Item::parse_name(&mut lines, &mut item);
        Item::parse_itype(&mut lines, &mut item);
        Item::parse_separator(&mut lines, &mut item);

        // Weapon info, item type?, attack damage, attack speed, crit chance
        Item::parse_weapon_info(&mut lines, &mut item);

        // Requirements info, needed stats and level
        Item::parse_requrements_info(&mut lines, &mut item);

        // Sockets info
        Item::parse_sockets(&mut lines, &mut item);
        Item::parse_separator(&mut lines, &mut item);

        // Item level info
        Item::parse_item_level(&mut lines, &mut item);
        Item::parse_separator(&mut lines, &mut item);

        //Implicit info TODO better way of detecting if implicit should be
        //                   included or not
        if item.hands == "Dagger".to_string() {
            Item::parse_implicit(&mut lines, &mut item);
        }

        // Affixes info
        Item::parse_affixes(&mut lines, &mut item);

        return item
    }

    fn parse_name(lines : &mut CharSplits<char>, item: &mut Item) {
        item.name = lines.next().unwrap().to_string()
    }

    fn parse_itype(lines : &mut CharSplits<char>, item: &mut Item) {
        item.itype = lines.next().unwrap().to_string()
    }

    fn parse_sockets(lines : &mut CharSplits<char>, item: &mut Item) {
        item.sockets = lines.next().unwrap().to_string()
    }

    fn parse_hands(lines : &mut CharSplits<char>, item: &mut Item) {
        item.hands = lines.next().unwrap().to_string()
    }

    fn parse_separator(lines : &mut CharSplits<char>, _item: &mut Item) {
        lines.next();
    }

    fn parse_rarity(lines: &mut CharSplits<char>, item: &mut Item) {
        item.rarity = match lines.next(){
            Some("Rarity: Normal") => Rarity::Normal,
            Some("Rarity: Magic") => Rarity::Magic,
            Some("Rarity: Rare") => Rarity::Rare,
            Some("Rarity: Unique") => Rarity::Unique,
            Some(other) => panic!("unexpected rarity: {}", other),
            None => panic!("Unable to parse rarity"),
        }
    }

    fn parse_weapon_info(lines: &mut CharSplits<char>, item: &mut Item) {
        // Ex: Two Handed Axe
        Item::parse_hands(lines, item);

        // Rest of attack info
        loop{
            match lines.next() {
                Some("--------") => break,
                Some(atk_info) => {
                    // Physical Damage: 95-158 (augmented)
                    let dmg_txt = r"([:alpha:]+) Damage: ([0-9]+)-([0-9]+)*";
                    let dmg_re = regex::Regex::new(dmg_txt).unwrap();
                    for cap in dmg_re.captures_iter(atk_info) {
                        let dmg = Dmg {
                            dmgtype : {
                                match cap.at(1) {
                                    "Physical" => DamageType::Physical,
                                    "Elemental" => DamageType::Elemental,
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
                    let crit_txt = r"Critical Strike Chance: ([0-9]+\.[0-9]+)%";
                    let crit_re = regex::Regex::new(crit_txt).unwrap();
                    for cap in crit_re.captures_iter(atk_info) {
                        item.crit_chance = from_str(cap.at(1)).unwrap();
                    }

                    // Attacks per Second: 1.24 (augmented)
                    let speed_txt = r"Attacks per Second: ([0-9]+\.[0-9]+)*";
                    let speed_re = regex::Regex::new(speed_txt).unwrap();
                    for cap in speed_re.captures_iter(atk_info) {
                        item.speed = from_str(cap.at(1)).unwrap();
                    }
                },
                None => panic!("Unexpected input during atk_info"),
            }
        }
    }

    fn parse_requrements_info(lines: &mut CharSplits<char>, item: &mut Item) {
        loop{
            match lines.next() {
                Some("--------") => break,
                Some("Requirements:") => continue,
                Some(requirement) => {
                    // Level: 49
                    let level_txt = r"Level: ([0-9]+)";
                    let level_re = regex::Regex::new(level_txt).unwrap();
                    for cap in level_re.captures_iter(requirement) {
                        item.req_level = from_str(cap.at(1)).unwrap();
                    }

                    // Str: 122
                    let req_str_txt = r"Str: ([0-9]+)";
                    let req_str_re = regex::Regex::new(req_str_txt).unwrap();
                    for cap in req_str_re.captures_iter(requirement) {
                        item.req_str = from_str(cap.at(1)).unwrap();
                    }

                    // Dex: 53
                    let req_dex_txt = r"Dex: ([0-9]+)";
                    let req_dex_re = regex::Regex::new(req_dex_txt).unwrap();
                    for cap in req_dex_re.captures_iter(requirement) {
                        item.req_dex = from_str(cap.at(1)).unwrap();
                    }
                    // Int
                    let req_int_txt = r"Int: ([0-9]+)";
                    let req_int_re = regex::Regex::new(req_int_txt).unwrap();
                    for cap in req_int_re.captures_iter(requirement) {
                        item.req_int = from_str(cap.at(1)).unwrap();
                    }
                }
                None => panic!("unexpected end of input \
                                when expecting implicit"),
            }
        }
    }

    fn parse_item_level(lines: &mut CharSplits<char>, item: &mut Item) {
        // Itemlevel: 68
        let ilvl_txt = r"Itemlevel: ([0-9]+)";
        let ilvl_re = regex::Regex::new(ilvl_txt).unwrap();
        for cap in ilvl_re.captures_iter(lines.next().unwrap()) {
            item.ilvl = from_str(cap.at(1)).unwrap();
        }
    }

    fn parse_implicit(lines: &mut CharSplits<char>, item: &mut Item) {
        loop{
            match lines.next() {
                Some("--------") => break,
                Some(implicit) => item.implicit = implicit.to_string(),
                None => panic!("unexpected end of input \
                               when expecting implicit"),
            }
        }
    }

    fn parse_affixes(lines: &mut CharSplits<char>, item: &mut Item) {
        loop {
            match lines.next() {
                Some("") => continue, //New lines in the end and what not.
                Some(affix) => item.affixes.push(affix.to_string()),
                None => break,
            }
        }
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
                         .find(|dmg| dmg.dmgtype == DamageType::Elemental)
                         //.find(|dmg| dmg.dmgtype == DamageType::Lightning ||
                         //            dmg.dmgtype == DamageType::Fire ||
                         //            dmg.dmgtype == DamageType::Ice)
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
                            implicit: "".to_string(),
                            affixes: vec!("34% increased Physical Damage".to_string(),
                                          "8% increased Attack Speed".to_string(),
                                          "+9 Life gained on Kill".to_string(),
                                          "+174 to Accuracy Rating".to_string())};
        assert_eq!(expected, item);
        assert!(item.dps() - 156.86 < 0.001);
        assert!(item.pdps() - 156.86 < 0.001);
        assert_eq!(item.edps(), 0.0);
    }

    #[test]
    fn dagger() {
        let dagger = "Rarity: Rare\n\
                      Phoenix Gutter\n\
                      Slaughter Knife\n\
                      --------\n\
                      Dagger\n\
                      Physical Damage: 9-78\n\
                      Elemental Damage: 1-10 (augmented)\n\
                      Critical Strike Chance: 6.80%\n\
                      Attacks per Second: 1.40\n\
                      --------\n\
                      Requirements:\n\
                      Level: 58\n\
                      Dex: 81\n\
                      Int: 117\n\
                      --------\n\
                      Sockets: B-B B\n\
                      --------\n\
                      Itemlevel: 60\n\
                      --------\n\
                      40% increased Global Critical Strike Chance\n\
                      --------\n\
                      57% increased Spell Damage\n\
                      +31 to Dexterity\n\
                      Adds 1-10 Lightning Damage\n\
                      13% increased Critical Strike Chance for Spells";
        let item = super::Item::new(dagger.to_string());
        let expected = Item{rarity: Rarity::Rare,
                            name: "Phoenix Gutter".to_string(),
                            itype: "Slaughter Knife".to_string(),
                            hands: "Dagger".to_string(),
                            damage: vec!(
                                Dmg {
                                    dmgtype : DamageType::Physical,
                                    min : 9,
                                    max : 78,
                                },
                                Dmg {
                                    dmgtype : DamageType::Elemental,
                                    min : 1,
                                    max : 10,
                                }),
                            crit_chance: 6.80,
                            speed: 1.40,
                            req_level: 58,
                            req_str: 0,
                            req_dex: 81,
                            req_int: 117,
                            sockets: "Sockets: B-B B".to_string(),
                            ilvl: 60,
                            implicit: "40% increased Global Critical Strike \
                                           Chance".to_string(),
                            affixes:
                                vec!("57% increased Spell Damage".to_string(),
                                     "+31 to Dexterity".to_string(),
                                     "Adds 1-10 Lightning Damage".to_string(),
                                     "13% increased Critical Strike Chance for Spells"
                                        .to_string())};
        assert_eq!(expected, item);
        assert!(item.dps() - 68.6 < 0.001);
        assert!(item.pdps() - 60.9 < 0.001);
        assert!(item.edps() - 7.7 < 0.001);
    }
}
