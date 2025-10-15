use bevy::prelude::*;
use bevy::ui::{BackgroundColor, Interaction, Node};

mod mine_node;

use mine_node::{MineNode, MineState};
use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, calculate_neighbor_bombs).chain())
        .add_systems(Update, (update_node_appearance, click_system, auto_reveal))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let bomb_distribution = generate_bool_array_with_ratio(256, 60);
    for x in -8..8 {
        for y in -8..8 {
            commands.spawn((
                Button,
                Node {
                    width: Val::Px(25.0),
                    height: Val::Px(25.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px((x as f32 * 30.0) + 400.0),
                    top: Val::Px((y as f32 * 30.0) + 300.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                MineNode {
                    state: MineState::Normal,
                    is_bomb: bomb_distribution[((x + 8) * 16 + (y + 8)) as usize],
                    position: (x, y),
                    neighbor_bombs: 0,
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

/// Generate a boolean array of given length with a specified number of true values.
fn generate_bool_array_with_ratio(len: usize, number: usize) -> Vec<bool> {
    let mut array = vec![true; number];
    array.extend(vec![false; len - number]);
    array.shuffle(&mut rng());
    array
}

fn calculate_neighbor_bombs(mut query: Query<&mut MineNode>) {
    use std::collections::HashSet;

    // Collect bomb positions
    let mut bomb_positions = HashSet::new();
    for mine_node in query.iter() {
        if mine_node.is_bomb {
            bomb_positions.insert(mine_node.position);
        }
    }

    // Calculate neighbor bombs for each node
    for mut mine_node in query.iter_mut() {
        if !mine_node.is_bomb {
            let mut count = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = mine_node.position.0 + dx;
                    let ny = mine_node.position.1 + dy;
                    if bomb_positions.contains(&(nx, ny)) {
                        count += 1;
                    }
                }
            }
            mine_node.neighbor_bombs = count;
        }
    }
}

fn update_node_appearance(
    mut query: Query<(&Interaction, &MineNode, &mut BackgroundColor, &mut Text), With<Button>>,
) {
    for (interaction, mine_node, mut background_color, mut text) in &mut query {
        background_color.0 = if matches!(mine_node.state, MineState::Normal) {
            match *interaction {
                Interaction::Hovered => Color::srgb(0.5, 0.5, 0.0),
                Interaction::None => Color::WHITE,
                _ => Color::WHITE,
            }
        } else {
            mine_node.get_color()
        };

        // Set text based on state
        text.0 = match mine_node.state {
            MineState::Revealed => {
                if mine_node.neighbor_bombs > 0 {
                    mine_node.neighbor_bombs.to_string()
                } else {
                    String::new()
                }
            }
            MineState::Flaged => "F".to_string(),
            MineState::Bombed => "B".to_string(),
            _ => String::new(),
        };
    }
}

/// Handle click events to change the state of MineNode.
fn click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut query: Query<(&Node, &mut MineNode), With<Button>>,
) {
    if mouse_button_input.any_just_pressed([MouseButton::Left, MouseButton::Right])
        && let Some(cursor_position) = windows.single().unwrap().cursor_position()
    {
        for (node, mut mine_node) in &mut query {
            if is_cursor_over_node(cursor_position, node) {
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    if mine_node.is_bomb {
                        mine_node.state = MineState::Bombed;
                    } else {
                        mine_node.state = MineState::Revealed;
                    }
                } else if mouse_button_input.just_pressed(MouseButton::Right) {
                    mine_node.state = match mine_node.state {
                        MineState::Flaged => MineState::Normal,
                        _ => MineState::Flaged,
                    };
                }
                break;
            }
        }
    }
}

/// Check if the cursor is over the given UI node.
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

fn auto_reveal(mut query: Query<&mut MineNode>) {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();

    // Collect all positions that are Revealed and have 0 neighbor bombs
    for mine_node in query.iter() {
        if mine_node.state == MineState::Revealed
            && mine_node.neighbor_bombs == 0
            && !mine_node.is_bomb
        {
            queue.push_back(mine_node.position);
        }
    }

    // Process the queue to reveal neighbors
    while let Some(pos) = queue.pop_front() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;
                if (-8..8).contains(&nx) && (-8..8).contains(&ny) {
                    // Find and reveal the neighbor if it's Normal
                    for mut mine_node in query.iter_mut() {
                        if mine_node.position == (nx, ny) && mine_node.state == MineState::Normal {
                            mine_node.state = MineState::Revealed;
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
