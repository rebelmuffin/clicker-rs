pub struct Upgrade {
    pub is_active: bool,
    pub cost: i128,
    pub activate_call: fn(&mut GameState) -> bool,
}

pub struct GameState {
    pub money: i128,
    pub click_value: i128,
    pub upgrades: Vec<Upgrade>,
}

impl GameState {
    pub fn buy_upgrade(&mut self, upgrade_index: usize) {
        let upgrade = &self.upgrades[upgrade_index];
        if upgrade.is_active == false && self.can_afford(upgrade_index) {
            let result = (upgrade.activate_call)(self);
            if result {
                self.money -= self.upgrades[upgrade_index].cost;
                self.upgrades[upgrade_index].is_active = true;
            }
        }
    }

    pub fn can_afford(&self, upgrade_index: usize) -> bool {
        let upgrade = &self.upgrades[upgrade_index];
        return self.money >= upgrade.cost;
    }

    pub fn calculate_circle_radius(&self) -> f32 {
        const MIN_CIRCLE_RADIUS: f32 = 15.0;
        return f32::max(
            50.0 * f32::log(self.money as f32 / 150.0, 10.0),
            MIN_CIRCLE_RADIUS,
        );
    }
}
