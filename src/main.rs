use bevy::prelude::*;
use bevy::ui::{BackgroundColor, Interaction, Node};

mod mine_node;

use crate::mine_node::{MineNode, MineState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (change_color_on_hover, left_click_system, right_click_system),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

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
                MineNode::default(),
            ));
        }
    }
}

fn change_color_on_hover(
    mut query: Query<(&Interaction, &MineNode, &mut BackgroundColor), With<Button>>,
) {
    for (interaction, mine_node, mut background_color) in &mut query {
        if matches!(mine_node.state, MineState::Normal) {
            match *interaction {
                Interaction::Hovered => {
                    background_color.0 = Color::srgb(0.5, 0.5, 0.0);
                }
                Interaction::None => {
                    background_color.0 = Color::WHITE;
                }
                _ => {}
            }
        } else {
            background_color.0 = mine_node.get_color();
        }
    }
}

fn left_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut query: Query<(&Node, &mut MineNode), With<Button>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        && let Some(cursor_position) = windows.single().unwrap().cursor_position()
    {
        for (node, mut mine_node) in &mut query {
            let left = if let Val::Px(v) = node.left { v } else { 0.0 };
            let top = if let Val::Px(v) = node.top { v } else { 0.0 };
            let width = if let Val::Px(v) = node.width { v } else { 0.0 };
            let height = if let Val::Px(v) = node.height { v } else { 0.0 };

            if cursor_position.x >= left
                && cursor_position.x <= left + width
                && cursor_position.y >= top
                && cursor_position.y <= top + height
            {
                mine_node.state = match mine_node.state {
                    MineState::LeftClicked => MineState::Normal,
                    _ => MineState::LeftClicked,
                };
                break;
            }
        }
    }
}

fn right_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut query: Query<(&Node, &mut MineNode), With<Button>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right)
        && let Some(cursor_position) = windows.single().unwrap().cursor_position()
    {
        for (node, mut mine_node) in &mut query {
            let left = if let Val::Px(v) = node.left { v } else { 0.0 };
            let top = if let Val::Px(v) = node.top { v } else { 0.0 };
            let width = if let Val::Px(v) = node.width { v } else { 0.0 };
            let height = if let Val::Px(v) = node.height { v } else { 0.0 };

            if cursor_position.x >= left
                && cursor_position.x <= left + width
                && cursor_position.y >= top
                && cursor_position.y <= top + height
            {
                mine_node.state = match mine_node.state {
                    MineState::RightClicked => MineState::Normal,
                    _ => MineState::RightClicked,
                };
                break;
            }
        }
    }
}
