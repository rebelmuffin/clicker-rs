pub struct Upgrade {
    pub is_active: bool,
    pub cost: i64,
    pub activate_call: fn(&mut GameState) -> bool,
}

pub struct GameState {
    pub money: i64,
    pub click_value: i64,
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
        return 15.0 + self.money as f32 / 50.0;
    }
}
