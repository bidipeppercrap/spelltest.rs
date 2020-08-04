use crate::utils::logger::print_separator;
use super::attack::damage_dealt;

pub struct Character {
    name: String,
    health_limit: f64,
    health: f64,
    energy_limit: f64,
    energy: f64,
    damage: f64,
    defense: f64,
}

impl Character {
    pub fn print(&self) {
        print_separator();
        println!("Name\t : {character_name}", character_name = self.name);
        println!("Health\t : {character_health}/{character_health_limit}", character_health = self.health, character_health_limit = self.health_limit);
        println!("Energy\t : {character_energy}/{character_energy_limit}", character_energy = self.energy, character_energy_limit = self.energy_limit);
        println!("Damage\t : {character_damage}", character_damage = self.damage);
        println!("Defense\t : {character_defense}", character_defense = self.defense);
        print_separator();
    }

    pub fn attack(&self, character: &mut Character) {
        character.attacked(&self.damage);
    }

    fn attacked(&mut self, damage: &f64) {
        self.health -= damage_dealt(damage, &self.defense);
    }
}

impl Character {
    pub fn new<S: Into<String>>(name: S, health: u64, energy: u64, damage: u64, defense: u64) -> Character {
        Character {
            name: name.into(),
            health_limit: health as f64,
            health: health as f64,
            energy_limit: energy as f64,
            energy: energy as f64,
            damage: damage as f64,
            defense: defense as f64,
        }
    }
}