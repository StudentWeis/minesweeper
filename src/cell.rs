use bevy::prelude::*;

#[derive(Component)]
pub struct Cell {
    pub state: CellState,
    pub is_bomb: bool,
    pub position: (i32, i32),
    pub neighbor_bombs: u8,
}

impl Cell {
    pub fn get_color(&self) -> Color {
        match self.state {
            CellState::Hidden => Color::WHITE,
            CellState::Exploded => Color::srgb(1.0, 0.0, 0.0),
            CellState::Flagged => Color::srgb(0.0, 0.0, 1.0),
            CellState::Revealed => Color::NONE,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::Hidden,
            is_bomb: false,
            position: (0, 0),
            neighbor_bombs: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Hidden,
    Exploded,
    Flagged,
    Revealed,
}
