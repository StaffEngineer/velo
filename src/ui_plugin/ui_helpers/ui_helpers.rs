use linkify::{LinkFinder, LinkKind};

use bevy::{prelude::*, text::BreakLineOn};

use crate::TextPos;
#[path = "components.rs"]
mod components;
pub use components::*;
#[path = "spawn_node.rs"]
mod spawn_node;
pub use spawn_node::*;

#[path = "spawn_modal.rs"]
mod spawn_modal;
pub use spawn_modal::*;

#[path = "add_tab.rs"]
mod add_tab;
pub use add_tab::*;

#[path = "add_list_item.rs"]
mod add_list_item;
pub use add_list_item::*;

fn get_marker_style(position: UiRect, size: f32) -> Style {
    Style {
        position_type: PositionType::Absolute,
        top: position.top,
        bottom: position.bottom,
        left: position.left,
        right: position.right,
        border: UiRect::all(Val::Px(1.)),
        width: Val::Px(size),
        height: Val::Px(size),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn add_rectangle_txt(text: String) -> TextBundle {
    let text_style = TextStyle {
        font_size: 18.0,
        color: Color::BLACK,
        ..default()
    };
    TextBundle::from_section(text, text_style).with_style(Style {
        position_type: PositionType::Relative,
        ..default()
    })
}

pub fn pos_to_style(text_pos: TextPos) -> (JustifyContent, AlignItems) {
    match text_pos {
        TextPos::TopRight => (JustifyContent::FlexEnd, AlignItems::FlexStart),
        TextPos::TopLeft => (JustifyContent::FlexStart, AlignItems::FlexStart),
        TextPos::BottomRight => (JustifyContent::FlexEnd, AlignItems::FlexEnd),
        TextPos::BottomLeft => (JustifyContent::FlexStart, AlignItems::FlexEnd),
        TextPos::Center => (JustifyContent::Center, AlignItems::Center),
    }
}

pub fn style_to_pos(style: (JustifyContent, AlignItems)) -> TextPos {
    match style {
        (JustifyContent::FlexEnd, AlignItems::FlexStart) => TextPos::TopRight,
        (JustifyContent::FlexStart, AlignItems::FlexStart) => TextPos::TopLeft,
        (JustifyContent::FlexEnd, AlignItems::FlexEnd) => TextPos::BottomRight,
        (JustifyContent::FlexStart, AlignItems::FlexEnd) => TextPos::BottomLeft,
        (JustifyContent::Center, AlignItems::Center) => TextPos::Center,
        _ => panic!("Invalid style! {:?}", style),
    }
}

fn create_rectangle_btn(
    bg_color: Color,
    image: Option<UiImage>,
    z_index: i32,
    text_pos: TextPos,
) -> ButtonBundle {
    let (justify_content, align_items) = pos_to_style(text_pos);
    let mut button = ButtonBundle {
        background_color: bg_color.into(),
        z_index: ZIndex::Local(z_index),
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            // position: UiRect {
            //     left: Val::Px(-3.),
            //     right: Val::Px(0.),
            //     top: Val::Px(-3.),
            //     bottom: Val::Px(0.),
            // },
            justify_content,
            align_items,
            // //overflow: Overflow::Hidden,
            ..default()
        },
        ..default()
    };
    if let Some(image) = image {
        button.image = image;
    }
    button
}

fn create_arrow_marker(left: f32, right: f32, top: f32, bottom: f32) -> ButtonBundle {
    ButtonBundle {
        style: get_marker_style(
            UiRect {
                left: Val::Percent(left),
                right: Val::Percent(right),
                top: Val::Percent(top),
                bottom: Val::Percent(bottom),
            },
            4.,
        ),
        ..default()
    }
}

fn create_resize_marker(left: f32, right: f32, top: f32, bottom: f32) -> ButtonBundle {
    ButtonBundle {
        style: get_marker_style(
            UiRect {
                left: Val::Percent(left),
                right: Val::Percent(right),
                top: Val::Percent(top),
                bottom: Val::Percent(bottom),
            },
            10.,
        ),
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    }
}

pub fn get_sections(text: String) -> (Vec<TextSection>, Vec<bool>) {
    let text_style = TextStyle {
        font_size: 18.0,
        color: Color::rgb(33.0 / 255.0, 33.0 / 255.0, 33.0 / 255.0),
        ..default()
    };
    let link_style = TextStyle {
        font_size: 18.0,
        color: Color::rgb(63.0 / 255.0, 81.0 / 255.0, 181.0 / 255.0),
        ..default()
    };
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let links: Vec<_> = finder.links(&text).collect();
    if links.is_empty() {
        return (
            vec![
                TextSection {
                    value: text,
                    style: text_style.clone(),
                },
                TextSection {
                    value: " ".to_string(),
                    style: text_style,
                },
            ],
            vec![false, false],
        );
    }
    let mut sections = vec![];
    let mut is_link = vec![];
    let mut idx = 0;
    for link in links {
        let start = link.start();
        let end = link.end();
        if start > idx {
            sections.push(TextSection {
                value: text[idx..start].to_string(),
                style: text_style.clone(),
            });
            is_link.push(false);
        }
        sections.push(TextSection {
            value: text[start..end].to_string(),
            style: link_style.clone(),
        });
        is_link.push(true);
        idx = end;
    }
    if idx < text.len() {
        sections.push(TextSection {
            value: text[idx..text.len()].to_string(),
            style: text_style.clone(),
        });
        is_link.push(false);
    }
    sections.push(TextSection {
        value: " ".to_string(),
        style: text_style,
    });
    is_link.push(false);
    (sections, is_link)
}

pub fn create_rectangle_txt(
    text: String,
    max_size: Option<(Val, Val)>,
    is_active: bool,
) -> TextBundle {
    let text = Text {
        sections: get_sections(text).0,
        alignment: TextAlignment::Left,
        linebreak_behavior: BreakLineOn::WordBoundary,
    };
    let mut text_bundle_style = Style {
        padding: UiRect::all(Val::Px(10.)),
        ..default()
    };
    if let Some((x, y)) = max_size {
        text_bundle_style.max_width = x;
        text_bundle_style.max_height = y;
    };
    if is_active {
        text_bundle_style.display = Display::Flex;
    } else {
        text_bundle_style.display = Display::None;
    }
    TextBundle {
        text,
        style: text_bundle_style,
        ..default()
    }
}

pub enum TooltipPosition {
    Top,
    Bottom,
}

pub fn get_tooltip(text: String, size: f32, tooltip_position: TooltipPosition) -> TextBundle {
    let text = Text {
        sections: vec![TextSection {
            value: text,
            style: TextStyle {
                font_size: size,
                color: Color::BLACK,
                ..default()
            },
        }],
        alignment: TextAlignment::Left,
        linebreak_behavior: BreakLineOn::WordBoundary,
    };
    let position = match tooltip_position {
        TooltipPosition::Bottom => UiRect {
            left: Val::Px(30.),
            right: Val::Px(0.),
            top: Val::Px(30.),
            bottom: Val::Px(0.),
        },
        TooltipPosition::Top => UiRect {
            left: Val::Px(30.),
            right: Val::Px(0.),
            top: Val::Px(-30.),
            bottom: Val::Px(0.),
        },
    };
    let text_bundle_style = Style {
        left: position.left,
        right: position.right,
        top: position.top,
        bottom: position.bottom,
        padding: UiRect::all(Val::Px(5.)),
        ..default()
    };
    TextBundle {
        z_index: ZIndex::Global(1),
        visibility: Visibility::Hidden,
        background_color: Color::WHITE.into(),
        text,
        style: text_bundle_style,
        ..default()
    }
}
