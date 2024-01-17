//! Module containg all the nice stuff for the game

pub struct Player {
    pub name: String,
    pub stats: Stats,
    pub inventory: Inventory,
}

pub trait Unit {
    
}


/// A struct containing all the stats of a unit
/// 
/// Each unit should have two of these, one for the base stats and one for the
/// current stats. The base stats are used to calculate the current stats.
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    health: u32,
    mana: u32,
    speed: u32,
    strength: u32,
}


/// inventory of a unit
/// 
/// Even enemies will have an inventory which they can use for fighting
#[derive(Debug)]
pub struct Inventory {
    items: Items,
    hand: Option<Item>,
    off_hand: Option<Item>,
    head: Option<Item>,
    chest: Option<Item>,
    legs: Option<Item>,
    feet: Option<Item>,
    accessory: Option<Item>,
}
pub type Items = [[Option<Item>; 7]; 4];


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Weapon(weapon::Weapon),
    Armor(Armor),
    Consumable(Consumable),
    Building(Building),
}

mod weapon {
    const MAX_CHARGE: u32 = 100;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Stats {
        damage: u32,
        speed: u32,
        range: u32,
        charge: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Weapon {
        id: usize,
        stats: Stats,
    }

    impl Weapon {
        pub fn new(id: usize, damage: u32, speed: u32, range: u32, cooldown: u32) -> Self {
            Self {
                id,
                stats: Stats {
                    damage,
                    speed,
                    range,
                    charge: 0,
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Kind {
        Sword,
        Hammer,
        Bow,
        Crossbow,
        Staff,
        Wand,
        Dagger,
        Spear,
        Shield,
        Spellbook,
    }

    pub struct WeaponDescription {
        name: &'static str,
        description: &'static str,
        level: u32,
        kind: Kind,
        stats: Stats,
    }

    pub const WEAPONS: [WeaponDescription; 7] = [
        WeaponDescription {
            name: "Domperidon's Gutripper",
            description: r#"{qu name="Amare"}You brought a knife to a gunfight?{/qu}
            {qu name="Domperidon"}Mayhaps, mayhaps not. Depends on how fast you can run.{/qu}"#,
            level: 1,
            kind: Kind::Dagger,
            stats: Stats {
                damage: 10,
                speed: 20,
                range: 1,
                charge: 0,
            },
        },
        WeaponDescription {
            name: "Anakinra's Morningstar",
            description: r#"Once Amare's favorite weapon, now a relic of the past."#,
            level: 1,
            kind: Kind::Sword,
            stats: Stats {
                damage: 20,
                speed: 10,
                range: 1,
                charge: 0,
            },
        },
        WeaponDescription {
            name: "Quacksmack's Quizzical Quacker",
            description: r#"Forged by the eccentric inventor Quacksmack, this rubbery mallet produces a comical 'quack' sound upon impact. Enemies may laugh, but they won't be laughing for long."#,
            level: 3,
            kind: Kind::Hammer,
            stats: Stats {
                damage: 110,
                speed: 5,
                range: 3,
                charge: 0,
            },
        },
        WeaponDescription {
            name: "Novice's Grimoire",
            description: r#"A fundamental spellbook tailored for novices, offering basic spells suitable for those just starting their magical journey."#,
            level: 1,
            kind: Kind::Spellbook,
            stats: Stats {
                damage: 20,
                speed: 5,
                range: 10,
                charge: 0,
            },
        },
        WeaponDescription {
            name: "Apprentice's Codex",
            description: r#""An instructional spellbook designed for apprentices, presenting a selection of spells that progressively challenge and expand their magical capabilities."#,
            level: 2,
            kind: Kind::Spellbook,
            stats: Stats {
                damage: 60,
                speed: 8,
                range: 13,
                charge: 0,
            },
        },
        WeaponDescription {
            name: "Adept's Tome",
            description: r#"A spellbook for adept spellcasters, containing more advanced and nuanced spells suitable for those refining their magical skills."#,
            level: 3,
            kind: Kind::Spellbook,
            stats: Stats {
                damage: 105,
                speed: 10,
                range: 15,
                charge: 0,
            },
        },
        WeaponDescription {
            name: "Master's Grimoire",
            description: r#"The Master's Grimoire, a fabled tome rumored to embody the pinnacle of magical knowledge."#,
            level: 4,
            kind: Kind::Spellbook,
            stats: Stats {
                damage: 200,
                speed: 17,
                range: 30,
                charge: 0,
            },
        },

    ];
}