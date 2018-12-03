fn main() {
    let mut character = Character::new(String::from("Pel Nervil"), 100, 50, 10, 0);
    let mut enemy = Character::new(String::from("Pouya"), 100, 50, 20, 5);

    character.print();
    enemy.print();

    character.attack(&mut enemy);
    enemy.attack(&mut character);

    character.print();
    enemy.print();
}

fn print_seperator() {
    let seperator = "================================";
    
    println!("{seperator}", seperator = seperator);
}

struct Character {
    name: String,
    health_limit: u64,
    health: u64,
    energy: u64,
    damage: u64,
    defense: u64,
}

impl Character {
    fn new(name: String, health: u64, energy: u64, damage: u64, defense: u64) -> Character {
        Character {
            name,
            health_limit: health,
            health,
            energy,
            damage,
            defense,
        }
    }

    fn print(&self) {
        print_seperator();
        println!("Name\t : {character_name}", character_name = self.name);
        println!("Health\t : {character_health}/{character_health_limit}", character_health = self.health, character_health_limit=self.health_limit);
        println!("Energy\t : {character_energy}", character_energy = self.energy);
        println!("Damage\t : {character_damage}", character_damage = self.damage);
        println!("Defense\t : {character_defense}", character_defense = self.defense);
        print_seperator();
    }

    fn attack(&self, character: &mut Character) {
        character.attacked(self.damage);
    }

    fn attacked(&mut self, damage: u64) {
        self.health -= damage - self.defense;
    }

}
