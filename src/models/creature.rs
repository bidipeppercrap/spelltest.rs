use crate::utils::logger::print_separator;
use super::attack::damage_dealt;

pub struct Creature {
    name: String,
    health_limit: f64,
    health: f64,
    energy_limit: f64,
    energy: f64,
    damage: f64,
    defense: f64,
}

impl Creature {
    pub fn print(&self) {
        print_separator();
        println!("Name\t : {creature_name}", creature_name = self.name);
        println!("Health\t : {creature_health}/{creature_health_limit}", creature_health = self.health, creature_health_limit = self.health_limit);
        println!("Energy\t : {creature_energy}/{creature_energy_limit}", creature_energy = self.energy, creature_energy_limit = self.energy_limit);
        println!("Damage\t : {creature_damage}", creature_damage = self.damage);
        println!("Defense\t : {creature_defense}", creature_defense = self.defense);
        print_separator();
    }

    pub fn attack(&self, creature: &mut Creature) {
        creature.attacked(self.damage);
    }

    fn attacked(&mut self, damage: f64) {
        self.health -= damage_dealt(damage, self.defense);
    }
}

impl Creature {
    pub fn new<S: Into<String>>(name: S, health: u64, energy: u64, damage: u64, defense: u64) -> Creature {
        Creature {
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