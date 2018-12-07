pub fn damage_dealt(damage: u64, defense: u64) -> u64 {
    let dealt_damage = 1. - 0.048 * defense as f64 / (1. + 0.048 * defense as f64);
    1
}