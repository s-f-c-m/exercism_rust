// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
use std::cmp::Ordering;

#[allow(unused)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match &self.health.cmp(&0) {
            Ordering::Equal => match &self.level.cmp(&10) {
                Ordering::Less => Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                }),
                _ => Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                }),
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match &self.mana {
            Some(m) => match &m.cmp(&mana_cost) {
                Ordering::Less => 0,
                _ => {
                    self.mana = Some(m - mana_cost);
                    return 2 * mana_cost;
                }
            },
            None => {
                match &self.health.cmp(&mana_cost) {
                    Ordering::Less => {
                        self.health = 0;
                    }
                    _ => {
                        self.health = self.health - mana_cost;
                    }
                }
                0
            }
        }
    }
}
