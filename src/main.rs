mod models;
mod utils;

use models::character::Character;

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
