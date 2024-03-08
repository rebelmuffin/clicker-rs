use crate::game_types::*;

pub fn double_click_value(game: &mut GameState) -> bool {
    game.click_value *= 2;
    return true;
}
