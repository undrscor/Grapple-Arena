use bevy_ecs_ldtk::LevelSelection;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct MenuElement; // Add this component to identify menu elements

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Game,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf"); // Ensure the correct font path

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        },
        MenuElement,
    ))
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                    ..Default::default()
                },
                StartButton,
                MenuElement,
            ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Start Game",
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ),
                        MenuElement,
                    ));
                });
        });
}


// System to handle button interaction
pub fn button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Start button pressed!");
                next_state.set(GameState::Loading);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.8, 0.8, 0.8).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.9, 0.9, 0.9).into();
            }
        }
    }
}


// Add a new system to handle the loading transition
pub fn handle_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Simulate resource loading or other setup logic
    info!("Loading resources...");

    commands.insert_resource(LevelSelection::index(0));
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDTK-test.ldtk"),
        ..Default::default()
    });

    next_state.set(GameState::Game);
}


// Updated cleanup system to use MenuElement component
pub fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuElement>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Menu cleaned up");
}

pub fn update_game() {
    // Your game logic here
    info!("Game running...");
}
