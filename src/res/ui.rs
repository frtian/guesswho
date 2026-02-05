use bevy::prelude::*;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use crate::res::common::Colors;

/// Spawn a button with customizable size, color, and text
pub fn spawn_button(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    text: &str,
    width: f32,
    height: f32,
    bg_color: Color,
) {
    parent
        .spawn((
            Button,
            Node {
                display: Display::Flex,
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..Default::default()
            },
            BackgroundColor(Colors::DARK_GRAY),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text(text.to_string()),
                TextFont::default(),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });
}
