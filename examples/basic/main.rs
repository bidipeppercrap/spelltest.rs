extern crate spelltest;

use spelltest::models::character::Character;

fn main() {
    let mut player = Character::new(String::from("Pel Nervil"), 100, 20, 10, 0);
    let mut creep = Character::new(String::from("Balack"), 300, 200, 25, 10);

    player.print();
    creep.print();

    player.attack(&mut creep);
    creep.attack(&mut player);

    player.print();
    creep.print();
}