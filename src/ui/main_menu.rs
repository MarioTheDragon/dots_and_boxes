use bevy::{
    color::palettes::tailwind::{BLUE_200, GRAY_800},
    prelude::*,
};

use crate::{
    arena::spawn_arena, current_player::spawn_current_player,
    score::spawn_score,
};

/// This struct must only be used once: for the root of the main menu.
#[derive(Component)]
pub struct MainMenuRoot;

pub fn spawn_start_menu(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        MainMenuRoot,
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(50.0),
            height: Val::Percent(50.0),
            left: Val::Percent(25.0),
            top: Val::Percent(25.0),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![title(), start_button()],
    ));
}

fn title() -> (
    Text,
    TextLayout,
    bevy::ecs::spawn::SpawnRelatedBundle<
        ChildOf,
        Spawn<(TextSpan, TextColor, TextFont)>,
    >,
) {
    (
        Text::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        children![(
            TextSpan::new("Dots and Boxes"),
            TextColor(BLUE_200.into()),
            TextFont {
                font_size: 50.0,
                ..default()
            },
        )],
    )
}

fn start_button() -> (
    Button,
    Node,
    BackgroundColor,
    bevy::ecs::spawn::SpawnRelatedBundle<ChildOf, Spawn<(Text, TextColor)>>,
) {
    (
        Button,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(80.0),
            height: Val::Percent(20.0),
            ..default()
        },
        BackgroundColor(GRAY_800.into()),
        children![(Text::new("Start Game"), TextColor(BLUE_200.into()),)],
    )
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    main_menu_query: Single<Entity, With<MainMenuRoot>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                if mouse.just_released(MouseButton::Left) {
                    commands.entity(main_menu_query.entity()).despawn();

                    spawn_score(&mut commands);
                    spawn_current_player(&mut commands);
                    spawn_arena(&mut commands, &mut meshes, &mut materials);
                } else {
                    *color = HOVERED_BUTTON.into();
                }
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
