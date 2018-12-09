fn damage_multiplier(defense: &f64) -> f64 {
    1. - ((0.052 * defense) / (0.9 + 0.048 * defense))
}

pub fn damage_dealt(damage: &f64, defense: &f64) -> f64 {
    damage * damage_multiplier(defense)
}