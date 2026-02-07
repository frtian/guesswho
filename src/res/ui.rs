use crate::res::common::*;
use bevy::core_pipeline::core_2d::graph::input;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::state::commands;
use bevy::{input_focus::InputFocus, prelude::*};

// ==============================================================
// = UI EVENTS, COMPONENTS
// ==============================================================


#[derive(EntityEvent)]
pub struct ButtonClicked {
    pub entity: Entity,
}

#[derive(Component)]
pub struct TitleScreenRoot;

// ==============================================================
// = UI SYSTEMS
// ==============================================================

pub fn button_behavior_system(
    mut commands: Commands,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut Button,
            &Children,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut bg_color, mut button, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *bg_color = Colors::PRESSED.into();
                button.set_changed();
                commands.trigger(ButtonClicked { entity });
            }
            Interaction::Hovered => {
                *bg_color = Colors::LIGHT_GRAY.into();
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *bg_color = Colors::DARK_GRAY.into();
            }
        }
    }
}


pub fn spawn_button<'builder>(
    parent: &'builder mut RelatedSpawnerCommands<'_, ChildOf>,
    text: &str,
    width: f32,
    height: f32,
    bg_color: Color,
) -> EntityCommands<'builder>
{
    let mut cmd = parent.spawn((
            Button,
            Node {
                display: Display::Flex,
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..Default::default()
            },
            BackgroundColor(bg_color),
        ));

        cmd.with_children(|btn| {
            btn.spawn((
                Text(text.to_string()),
                TextFont::default(),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });

        cmd
}

// ==============================================================
// = UI SCREENS
// ==============================================================

pub fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hero_img: Handle<Image> = asset_server.load("textures/ui/hero.png");
    // Criar uma tela de título com um botão de host e join
    commands
        .spawn((
            TitleScreenRoot,
            Node {
                display: Display::Grid,
                grid_template_rows: vec![GridTrack::percent(70.0), GridTrack::percent(30.0)],
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(60.0),
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Imagem Hero
            parent.spawn((
                ImageNode {
                    image: hero_img.clone().into(),
                    image_mode: NodeImageMode::Auto,
                    ..Default::default()
                },
                Node {
                    width: Val::Px(600.0),
                    height: Val::Px(600.0),
                    margin: UiRect {
                        bottom: Val::Px(120.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
            // Botão Host
            parent
                .spawn((Node {
                    display: Display::Grid,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::auto()],
                    row_gap: Val::Px(20.0),
                    ..Default::default()
                },))
                .with_children(|grid| {
                    // Botão Host
                    spawn_button(grid, "Host Game", 250.0, 70.0, Colors::DARK_GRAY).observe(
                        |trigger: On<ButtonClicked>, mut next_state: ResMut<NextState<GameState>>| {
                            println!("Host Game button clicked!");
                            next_state.set(GameState::Lobby);
                        },
                    );

                    // Botão Join
                    // spawn_button(grid, "Join Game", 250.0, 70.0, Colors::DARK_GRAY).observe(
                    //     |trigger: On<ButtonClicked>| {
                    //         println!("Join Game button clicked!");
                    //     },
                    // );
                });
        });
}

pub fn despawn_title_screen(mut commands: Commands, query: Query<Entity, With<TitleScreenRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_lobby_for_host(mut commands: Commands, mut game_seed: ResMut<GameSeed>) {
    commands.spawn((
        Node {
            display: Display::Grid,
            grid_template_rows: vec![GridTrack::auto(), GridTrack::auto()],
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    )).with_children(|parent| {
        parent.spawn((
            Text(format!("Seu código é: {}", game_seed.0)),
            TextFont::default(),
            TextLayout::new_with_justify(Justify::Center),
        ));
        spawn_button(parent, "Gerar minha carta", 250.0, 70.0, Colors::DARK_GRAY).observe(
             |trigger: On<ButtonClicked>| {
                println!("precisa gerar uma carta...")
            }
        );

    });
}