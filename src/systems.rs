use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::ui::{BackgroundColor, Interaction, Node};
use bevy::window::Window;

use crate::board::{compute_neighbor_counts, generate_random_bool_array, grid_index, is_in_bounds};
use crate::cell::{Cell, CellState};
use crate::config::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let bomb_distribution = generate_random_bool_array(GRID_CELLS, BOMB_COUNT);
    let neighbor_counts = compute_neighbor_counts(&bomb_distribution);

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let idx = grid_index(x, y);
            let position = (x as i32, y as i32);
            commands.spawn((
                Button,
                Node {
                    width: Val::Px(25.0),
                    height: Val::Px(25.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(((x as f32 - GRID_WIDTH_HALF) * CELL_STRIDE) + WINDOW_HALF_WIDTH),
                    top: Val::Px(
                        ((y as f32 - GRID_HEIGHT_HALF) * CELL_STRIDE) + WINDOW_HALF_HEIGHT,
                    ),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                Cell {
                    state: CellState::Hidden,
                    is_bomb: bomb_distribution[idx],
                    position,
                    neighbor_bombs: neighbor_counts[idx],
                },
                Text::new(""),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));
        }
    }
}

pub fn update_node_appearance(
    mut query: Query<(&Interaction, &Cell, &mut BackgroundColor, &mut Text), With<Button>>,
) {
    for (interaction, mine_node, mut background_color, mut text) in &mut query {
        background_color.0 = if matches!(mine_node.state, CellState::Hidden) {
            match *interaction {
                Interaction::Hovered => Color::srgb(0.5, 0.5, 0.0),
                _ => mine_node.get_color(),
            }
        } else {
            mine_node.get_color()
        };

        text.0 = match mine_node.state {
            CellState::Revealed => {
                if mine_node.neighbor_bombs > 0 {
                    mine_node.neighbor_bombs.to_string()
                } else {
                    String::new()
                }
            }
            CellState::Flagged => "F".to_string(),
            CellState::Exploded => "B".to_string(),
            _ => String::new(),
        };
    }
}

pub fn click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut query: Query<(&Node, &mut Cell), With<Button>>,
) {
    if mouse_button_input.any_just_pressed([MouseButton::Left, MouseButton::Right])
        && let Some(cursor_position) = windows.single().unwrap().cursor_position()
    {
        for (node, mut mine_node) in &mut query {
            if is_cursor_over_node(cursor_position, node) {
                if mouse_button_input.just_pressed(MouseButton::Left)
                    && mine_node.state == CellState::Hidden
                {
                    if mine_node.is_bomb {
                        mine_node.state = CellState::Exploded;
                    } else {
                        mine_node.state = CellState::Revealed;
                    }
                } else if mouse_button_input.just_pressed(MouseButton::Right) {
                    mine_node.state = match mine_node.state {
                        CellState::Hidden => CellState::Flagged,
                        CellState::Flagged => CellState::Hidden,
                        _ => mine_node.state,
                    };
                }
                break;
            }
        }
    }
}

pub fn auto_reveal(mut query: Query<&mut Cell>) {
    let mut queue = VecDeque::new();

    for mine_node in query.iter() {
        if mine_node.state == CellState::Revealed
            && mine_node.neighbor_bombs == 0
            && !mine_node.is_bomb
        {
            queue.push_back(mine_node.position);
        }
    }

    while let Some(pos) = queue.pop_front() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;
                if is_in_bounds(nx, ny) {
                    for mut mine_node in query.iter_mut() {
                        if mine_node.position == (nx, ny) && mine_node.state == CellState::Hidden {
                            mine_node.state = CellState::Revealed;
                            if mine_node.neighbor_bombs == 0 {
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_cursor_over_node(cursor_position: Vec2, node: &Node) -> bool {
    let left = if let Val::Px(v) = node.left { v } else { 0.0 };
    let top = if let Val::Px(v) = node.top { v } else { 0.0 };
    let width = if let Val::Px(v) = node.width { v } else { 0.0 };
    let height = if let Val::Px(v) = node.height { v } else { 0.0 };

    cursor_position.x >= left
        && cursor_position.x <= left + width
        && cursor_position.y >= top
        && cursor_position.y <= top + height
}
