use bevy::prelude::*;
use crate::game_menu::GameState;
use crate::player::Player;

#[derive(Component)]
struct ProgressionText;

#[derive(Component)]
pub(crate) struct ProgressionVisible(pub bool);

pub fn toggle_progression_ui(
    game_state: Res<State<GameState>>,
    mut query: Query<(&mut Style, &mut ProgressionVisible)>,
) {
    let is_game = *game_state == GameState::Loading;

    for (mut style, mut visible) in query.iter_mut() {
        visible.0 = is_game;
        style.display = if is_game {
            Display::Flex
        } else {
            Display::None
        };
        // info!(
        //     "Updated UI visibility: is_game = {}, display = {:?}, visible = {}",
        //     is_game, style.display, visible.0
        // );
    }
}

pub fn setup_progression_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");

    commands.spawn((
        TextBundle::from_section(
            "Progression: 0",
            TextStyle {
                font,
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                display: Display::None, // Initially hidden
                ..Default::default()
            }),
        ProgressionText,
        ProgressionVisible(false),
    ));
}

pub fn update_progression_ui(
    player_query: Query<&Player, Changed<Player>>,
    mut query: Query<&mut Text, With<ProgressionText>>,
) {
    if let Ok(player) = player_query.get_single() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Progression: {}", player.progression);
        }
    }
}

pub struct ProgressionUiPlugin;

impl Plugin for ProgressionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_progression_ui)
            .add_systems(Update, toggle_progression_ui)
            .add_systems(Update, update_progression_ui);
    }
}
