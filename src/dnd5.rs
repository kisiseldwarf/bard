use std::iter::Enumerate;
use rand::{prelude, random};
use crate::dnd5::MethodAligment::LAWFUL;
use crate::dnd5::MoralAlignment::GOOD;

pub const CARAC_BOUNDS: (u32, u32) = (0, 20);

#[derive(PartialEq)]
#[derive(Debug)]
pub enum MoralAlignment {
    EVIL,
    NEUTRAL,
    GOOD
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum MethodAligment {
    CHAOTIC,
    NEUTRAL,
    LAWFUL
}

impl MoralAlignment {
    fn random() -> MoralAlignment {
        return MoralAlignment::from(random::<u32>() % 2)
    }
}

impl MethodAligment {
    fn random() -> MethodAligment {
        return MethodAligment::from(random::<u32>() % 2)
    }
}

impl From<u32> for MethodAligment {
    fn from(value: u32) -> Self {
        match value {
            0 => MethodAligment::CHAOTIC,
            1 => MethodAligment::NEUTRAL,
            2 => MethodAligment::LAWFUL,
            _ => panic!("Should have a value less or equal than 2")
        }
    }
}

impl From<u32> for MoralAlignment {
    fn from(value: u32) -> Self {
        return MoralAlignment::
        match value {
            0 => MoralAlignment::NEUTRAL,
            1 => MoralAlignment::GOOD,
            2 => MoralAlignment::EVIL,
            _ => panic!("Should have a value less or equal than 2")
        }
    }
}

pub struct Npc {
    pub str: u32,
    pub wis: u32,
    pub con: u32,
    pub dex: u32,
    pub cha: u32,
    pub int: u32,
    pub race: Race,
    pub alignment: (MethodAligment, MoralAlignment)
}

fn generate_carac() -> u32 {
    random::<u32>() % CARAC_BOUNDS.1
}

impl crate::Npc for Npc {
    fn generate_npc() -> Npc {
        return Npc {
            str: generate_carac(),
            wis: generate_carac(),
            con: generate_carac(),
            dex: generate_carac(),
            cha: generate_carac(),
            int: generate_carac(),
            race: Race,
            alignment: (MethodAligment::random(), MoralAlignment::random())
        }
    }
}

enum Race {
    DWARF,
    ELF,
    HUMAN
}
