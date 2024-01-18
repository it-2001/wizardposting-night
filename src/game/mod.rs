//! Module containg all the nice stuff for the game

pub struct Player {
    pub name: String,
    pub stats: Stats,
    pub inventory: Inventory,
}

pub trait Unit {}

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
    Armor(armor::Armor),
    Consumable(consumable::Consumable),
    Building(building::Building),
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
        //abilities: todo!("add abilities")
    }

    impl Weapon {
        pub fn new(id: usize) -> Self {
            Self { id }
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
        Spellbook,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct WeaponStats {
        pub damage: u32,
        pub speed: u32,
        pub range: u32,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WeaponDescription {
        name: &'static str,
        description: &'static str,
        level: u32,
        kind: Kind,
        stats: WeaponStats,
        abilities: AbilitySlots,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum AbilitySlots {
        None,
        One {
            main: AbilityKinds,
        },
        Two {
            main: AbilityKinds,
            secondary: AbilityKinds,
        },
    }
    pub type AbilityKinds = [AbilityKindWrapper; 3];

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum AbilityKind {
        Target,
        Unit,
        Area,
        This,
        Projectile,
        Blast,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum AbilityKindWrapper {
        Permit(AbilityKind),
        Deny(AbilityKind),
        Any,
        None,
    }

    use AbilityKind::*;
    use AbilityKindWrapper::*;
    pub const WEAPONS: [WeaponDescription; 7] = [
        WeaponDescription {
            name: "Domperidon's Gutripper",
            description: r#"{qu name="Amare"|You brought a knife to a gunfight?}
            {qu name="Domperidon"|Mayhaps, mayhaps not. Depends on how fast you can run.}"#,
            level: 1,
            kind: Kind::Dagger,
            stats: WeaponStats {
                damage: 10,
                speed: 20,
                range: 1,
            },
            abilities: AbilitySlots::None,
        },
        WeaponDescription {
            name: "Anakinra's Morningstar",
            description: r#"Once Amare's favorite weapon, now a relic of the past."#,
            level: 1,
            kind: Kind::Sword,
            stats: WeaponStats {
                damage: 20,
                speed: 10,
                range: 1,
            },
            abilities: AbilitySlots::One {
                main: [Permit(Target), None, None],
            },
        },
        WeaponDescription {
            name: "Quacksmack's Quizzical Quacker",
            description: r#"Forged by the eccentric inventor Quacksmack, this rubbery mallet produces a comical 'quack' sound upon impact. Enemies may laugh, but they won't be laughing for long."#,
            level: 3,
            kind: Kind::Hammer,
            stats: WeaponStats {
                damage: 110,
                speed: 5,
                range: 3,
            },
            abilities: AbilitySlots::Two {
                main: [Permit(Target), Permit(Area), None],
                secondary: [Permit(This), None, None],
            },
        },
        WeaponDescription {
            name: "Novice's Grimoire",
            description: r#"A fundamental spellbook tailored for novices, offering basic spells suitable for those just starting their magical journey."#,
            level: 1,
            kind: Kind::Spellbook,
            stats: WeaponStats {
                damage: 20,
                speed: 5,
                range: 10,
            },
            abilities: AbilitySlots::One {
                main: [Any, None, None],
            },
        },
        WeaponDescription {
            name: "Apprentice's Codex",
            description: r#""An instructional spellbook designed for apprentices, presenting a selection of spells that progressively challenge and expand their magical capabilities."#,
            level: 2,
            kind: Kind::Spellbook,
            stats: WeaponStats {
                damage: 60,
                speed: 8,
                range: 13,
            },
            abilities: AbilitySlots::Two {
                main: [Any, None, None],
                secondary: [Any, None, None],
            },
        },
        WeaponDescription {
            name: "Adept's Tome",
            description: r#"A spellbook for adept spellcasters, containing more advanced and nuanced spells suitable for those refining their magical skills."#,
            level: 3,
            kind: Kind::Spellbook,
            stats: WeaponStats {
                damage: 105,
                speed: 10,
                range: 15,
            },
            abilities: AbilitySlots::Two {
                main: [Any, None, None],
                secondary: [Any, None, None],
            },
        },
        WeaponDescription {
            name: "Master's Grimoire",
            description: r#"The Master's Grimoire, a fabled tome rumored to embody the pinnacle of magical knowledge."#,
            level: 4,
            kind: Kind::Spellbook,
            stats: WeaponStats {
                damage: 200,
                speed: 17,
                range: 30,
            },
            abilities: AbilitySlots::Two {
                main: [Any, None, None],
                secondary: [Any, None, None],
            },
        },
    ];
}

mod armor {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Stats {
        defense: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Armor {
        id: usize,
    }

    impl Armor {
        pub fn new(id: usize) -> Self {
            Self { id }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Kind {
        Head,
        Chest,
        Legs,
        Feet,
        Accessory,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArmorDescription {
        name: &'static str,
        description: &'static str,
        level: u32,
        kind: Kind,
        stats: Stats,
    }
}

mod consumable {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Stats {
        health: u32,
        mana: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Consumable {
        id: usize,
        quantity: u8,
    }

    impl Consumable {
        pub fn new(id: usize) -> Self {
            Self { id, quantity: 1 }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Kind {
        Food,
        Potion,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ConsumableDescription {
        name: &'static str,
        description: &'static str,
        level: u32,
        kind: Kind,
        stats: Stats,
        max_quantity: u8,
    }
}

mod building {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Stats {
        health: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Building {
        id: usize,
        quantity: u8,
    }

    impl Building {
        pub fn new(id: usize) -> Self {
            Self { id, quantity: 1 }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Kind {}

    #[derive(Debug, Clone, PartialEq)]
    pub struct BuildingDescription {
        name: &'static str,
        description: &'static str,
        level: u32,
        kind: Kind,
        stats: Stats,
        max_quantity: u8,
    }
}
