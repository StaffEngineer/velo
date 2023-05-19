use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use crate::{
    ui_plugin::ui_helpers::{
        get_tooltip, EditableText, GenericButton, SearchButton, SearchText, Tooltip,
        TooltipPosition,
    },
    utils::ReflectableUuid,
};

pub fn add_search_box(commands: &mut Commands) -> Entity {
    let id = ReflectableUuid::generate();
    let root = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(80.),
                height: Val::Percent(8.),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    top: Val::Px(0.),
                    bottom: Val::Px(10.),
                },
                ..default()
            },
            ..default()
        },))
        .id();
    let search_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::WHITE.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::GRAY.with_a(0.5)),
            GenericButton,
            SearchButton { id },
        ))
        .id();
    let tooltip = commands
        .spawn((
            get_tooltip(
                "Filter documents by text in nodes".to_string(),
                14.,
                TooltipPosition::Top,
            ),
            Tooltip,
        ))
        .id();
    let search_label = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font_size: 14.,
                                color: Color::BLACK,
                                ..default()
                            },
                        },
                        TextSection {
                            value: " ".to_string(),
                            style: TextStyle {
                                font_size: 14.,
                                color: Color::BLACK,
                                ..default()
                            },
                        },
                    ],
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            SearchText { id },
            EditableText { id },
        ))
        .id();
    commands.entity(search_button).add_child(tooltip);
    commands.entity(search_button).add_child(search_label);
    commands.entity(root).add_child(search_button);
    root
}
