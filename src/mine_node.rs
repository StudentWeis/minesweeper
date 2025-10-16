use bevy::prelude::*;

#[derive(Component)]
pub struct MineNode {
    pub state: MineState,
    pub is_bomb: bool,
    pub position: (i32, i32),
    pub neighbor_bombs: u8,
}

impl MineNode {
    pub fn get_color(&self) -> Color {
        match self.state {
            MineState::Normal => Color::WHITE,
            MineState::Bombed => Color::srgb(1.0, 0.0, 0.0),
            MineState::Flaged => Color::srgb(0.0, 0.0, 1.0),
            MineState::Revealed => Color::NONE,
        }
    }
}

impl Default for MineNode {
    fn default() -> Self {
        Self {
            state: MineState::Normal,
            is_bomb: false,
            position: (0, 0),
            neighbor_bombs: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MineState {
    Normal,
    Bombed,
    Flaged,
    Revealed,
}
