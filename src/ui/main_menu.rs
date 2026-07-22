use bevy::{
    color::palettes::tailwind::{BLUE_200, GRAY_800},
    prelude::*,
};

pub fn spawn_start_menu(mut commands: Commands) {
    commands.spawn((
        Text::default(),
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
