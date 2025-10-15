use bevy::prelude::*;
use bevy::ui::{BackgroundColor, FocusPolicy, Interaction, Node};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, change_color_on_click)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for x in -5..5 {
        for y in -5..5 {
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
                FocusPolicy::default(),
            ));
        }
    }
}

fn change_color_on_click(mut query: Query<(&Interaction, &mut BackgroundColor), With<Button>>) {
    for (interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(1.0, 0.0, 0.0);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(1.0, 1.0, 0.0);
            }
            Interaction::None => {
                background_color.0 = Color::WHITE;
            }
        }
    }
}
