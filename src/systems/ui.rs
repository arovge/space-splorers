use crate::components::{
    CoordinatesText, Health, HealthText, LaserCooldown, LaserCooldownText, Player,
};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                update_coordinates_text,
                update_health_text,
                update_cooldown_text,
            ),
        );
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
        TextBundle::from_section("0, 0", text_style.clone())
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(45.),
                right: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Right),
        CoordinatesText,
    ));

    commands.spawn((
        TextBundle::from_section("0/0", text_style.clone())
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                right: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Right),
        HealthText,
    ));

    commands.spawn((
        TextBundle::from_section("Ready", text_style)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(75.),
                right: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Right),
        LaserCooldownText,
    ));
}

fn update_coordinates_text(
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
    ship_query: Query<&Transform, With<Player>>,
) {
    if let Ok(ship) = ship_query.get_single() {
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("{0:.2}, {1:.2}", ship.translation.x, ship.translation.y);
    }
}

fn update_health_text(
    mut text_query: Query<&mut Text, With<HealthText>>,
    ship_query: Query<&Health, With<Player>>,
) {
    if let Ok(ship_health) = ship_query.get_single() {
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("{}/100", ship_health.0);
    }
}

fn update_cooldown_text(
    mut text_query: Query<&mut Text, With<LaserCooldownText>>,
    ship_query: Query<Entity, With<Player>>,
    cooldowns: Query<&LaserCooldown, With<Player>>,
) {
    let mut text = text_query.single_mut();
    if let Ok(ship) = ship_query.get_single() {
        let ship_cooldown = cooldowns.get_component::<LaserCooldown>(ship);

        text.sections[0].value = match ship_cooldown {
            Ok(cooldown) => format!("{:.2}%", cooldown.0.percent() * 100.),
            _ => String::from("Ready"),
        };
    }
}
