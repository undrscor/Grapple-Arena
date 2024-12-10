use bevy_ecs_ldtk::LevelSelection;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_kira_audio::{Audio, AudioControl};


#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct RulesButton;

#[derive(Component)]
pub struct MenuElement; // Add this component to identify menu elements

#[derive(Component)]
pub struct RulesPopup;

#[derive(Component)]
pub struct ClosePopupButton;



#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Game,
}


//Menu
pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>,
) {

    let font = asset_server.load("FiraSans-Bold.ttf");


    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column, // Stack buttons vertically
                ..Default::default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        },
        MenuElement,
    ))
        .with_children(|parent| {
            // Start Game Button
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
                    parent.spawn((TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ), MenuElement));
                });

            // Rules Button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)), // Add some spacing
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                    ..Default::default()
                },
                RulesButton,
                MenuElement,
            ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Rules",
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ), MenuElement));
                });
        });
}
//rules segment

pub fn close_popup(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<ClosePopupButton>)>,
    popup_query: Query<Entity, With<RulesPopup>>,
) {
    for (interaction, entity) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Despawn the popup and its children
            if let Ok(popup_entity) = popup_query.get_single() {
                commands.entity(popup_entity).despawn_recursive();
            }
            // Optionally, despawn the button itself
            commands.entity(entity).despawn();
        }
    }
}

pub fn rules_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RulesButton>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Rules button pressed!");
                // Call show_rules_popup directly here
                show_rules_popup(&mut commands, &asset_server);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.7, 0.7, 0.7).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.9, 0.9, 0.9).into();
            }
        }
    }
}

pub fn show_rules_popup(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("FiraSans-Bold.ttf");

    commands.spawn((
        // Spawn a container for both the background and the popup
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(), // Dimmed background
            ..Default::default()
        },
        RulesPopup, // Attach the RulesPopup marker to the entire container
    ))
        .with_children(|parent| {
            // Popup container
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(300.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.3).into(), // Dark blue-gray
                ..Default::default()
            })
                .with_children(|popup| {
                    // Rules text
                    popup.spawn(TextBundle::from_section(
                        "Help little man escape!\n\nTraverse the arena to find 'redacted' in order to escape.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ).with_style(Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    }));

                    // Close button
                    popup.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.8, 0.0, 0.0).into(), // Bright red
                            ..Default::default()
                        },
                        ClosePopupButton,
                    ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Close",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}


// System to handle button interaction
pub fn button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StartButton>)> , audio: Res<Audio>, asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let start = asset_server.load("startEffect.ogg");

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                audio.play(start.clone());
                info!("Start button pressed!");
                next_state.set(GameState::Loading);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.7, 0.7, 0.7).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.9, 0.9, 0.9).into();
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
    crate::progression_ui::ProgressionVisible(true);

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
