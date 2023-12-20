use crate::components::{CoordinatesText, Ship};
use bevy::prelude::*;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_coordinates_text);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/clacon2.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_section("0, 0", text_style)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                right: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Right),
        CoordinatesText,
    ));
}

fn update_coordinates_text(
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
    ship_query: Query<&Transform, With<Ship>>,
) {
    let ship_translation = ship_query.single().translation;
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("{0:.2}, {1:.2}", ship_translation.x, ship_translation.y);
}
