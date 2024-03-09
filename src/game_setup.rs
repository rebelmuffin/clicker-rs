use crate::game_types::*;
use crate::upgrades::*;

pub fn create_default_game_state() -> GameState {
    return GameState {
        money: 1000,
        click_value: 1,
        upgrades: vec![
            Upgrade {
                is_active: false,
                cost: 25,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 25,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 25,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 25,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 25,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 150,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 400,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 1000,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 2000,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 5000,
                activate_call: double_click_value,
            },
            Upgrade {
                is_active: false,
                cost: 10000,
                activate_call: double_click_value,
            },
        ],
    };
}
