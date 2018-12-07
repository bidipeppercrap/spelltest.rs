use utils::logger::print_seperator;
use models::attack::damage_dealt;

pub struct Character {
    name: String,
    health_limit: u64,
    health: u64,
    energy_limit: u64,
    energy: u64,
    damage: u64,
    defense: u64,
}

impl Character {
    pub fn print(&self) {
        print_seperator();
        println!("Name\t : {character_name}", character_name = self.name);
        println!("Health\t : {character_health}/{character_health_limit}", character_health = self.health, character_health_limit = self.health_limit);
        println!("Energy\t : {character_energy}/{character_energy_limit}", character_energy = self.energy, character_energy_limit = self.energy_limit);
        println!("Damage\t : {character_damage}", character_damage = self.damage);
        println!("Defense\t : {character_defense}", character_defense = self.defense);
        print_seperator();
    }

    pub fn attack(&self, character: &mut Character) {
        character.attacked(self.damage);
    }

    fn attacked(&mut self, damage: u64) {
        self.health -= damage_dealt(damage, self.defense);
    }
}

impl Character {
    pub fn new(name: String, health: u64, energy: u64, damage: u64, defense: u64) -> Character {
        Character {
            name,
            health_limit: health,
            health,
            energy_limit: energy,
            energy,
            damage,
            defense,
        }
    }
}