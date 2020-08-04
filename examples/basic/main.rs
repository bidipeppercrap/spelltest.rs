use spelltest::*;

fn main() {
    // --- Create a player character ---
    // Character::new(name, health, energy, damage, defense)
    // Set mutable variable because we expect a change to health and energy
    let mut player = Character::new("Pel Nervil", 100, 20, 5, 0);

    // --- Create a dummy enemy ---
    let mut dummy_enemy = Character::new("Player killer", 200, 50, 10, 5);

    // Let's print their attributes
    player.print();
    dummy_enemy.print();

    // Let's do a turn-based combat
    // First, the player attack the enemy
    player.attack(&mut dummy_enemy); // Borrow and use as mutable to apply a change to enemy health

    // Let's print enemy attributes again, we expect the enemy health is reduced by the attack from the player
    dummy_enemy.print();

    // It's the enemy turn to attack
    dummy_enemy.attack(&mut player); // Like before, borrow and use as mutable

    // Expecting reduced player health
    player.print();
}