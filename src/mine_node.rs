use bevy::prelude::*;

#[derive(Component)]
pub struct MineNode {
    pub state: MineState,
}

#[derive(Clone, Copy)]
pub enum MineState {
    Normal,
    LeftClicked,
    RightClicked,
}

impl MineNode {
    pub fn get_color(&self) -> Color {
        match self.state {
            MineState::Normal => Color::WHITE,
            MineState::LeftClicked => Color::srgb(1.0, 0.0, 0.0),
            MineState::RightClicked => Color::srgb(0.0, 0.0, 1.0),
        }
    }
}

impl Default for MineNode {
    fn default() -> Self {
        Self {
            state: MineState::Normal,
        }
    }
}
