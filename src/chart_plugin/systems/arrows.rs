use bevy::{prelude::*, window::PrimaryWindow};

use std::collections::HashSet;

use super::ui_helpers::{create_arrow, ArrowConnect, ArrowMeta, CreateArrow};
use crate::{AppState, MainCamera, RedrawArrow};

pub fn create_arrow_start(
    mut interaction_query: Query<
        (&Interaction, &ArrowConnect),
        (Changed<Interaction>, With<ArrowConnect>),
    >,
    mut state: ResMut<AppState>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, arrow_connect) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => match state.arrow_to_draw_start {
                Some(start_arrow) => {
                    if start_arrow.id == arrow_connect.id {
                        continue;
                    }
                    state.arrow_to_draw_start = None;
                    create_arrow.send(CreateArrow {
                        start: start_arrow,
                        end: *arrow_connect,
                        arrow_type: state.arrow_type,
                    });
                }
                None => {
                    state.arrow_to_draw_start = Some(*arrow_connect);
                }
            },
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Crosshair;
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub fn create_arrow_end(
    mut commands: Commands,
    mut events: EventReader<CreateArrow>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    arrow_markers: Query<(&ArrowConnect, &GlobalTransform), With<ArrowConnect>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    for event in events.iter() {
        let (arrow_hold_vec, arrow_move_vec): (Vec<_>, Vec<_>) = arrow_markers
            .iter()
            .filter(|(x, _)| x.id == event.end.id || x.id == event.start.id)
            .map(|(ac, gt)| Some((ac, get_pos(gt, primary_window, camera, camera_transform)?)))
            .flatten()
            .partition(|(x, _)| x.id == event.end.id);
        let arrow_pos = arrow_hold_vec
            .iter()
            .flat_map(move |x| std::iter::repeat(x).zip(arrow_move_vec.clone()))
            .map(|(arrow_hold, arrow_move)| (arrow_hold.1, arrow_move.1))
            .min_by_key(|(arrow_hold, arrow_move)| arrow_hold.distance(*arrow_move) as u32);

        if let Some((start, end)) = arrow_pos {
            create_arrow(
                &mut commands,
                start,
                end,
                ArrowMeta {
                    start: event.start,
                    end: event.end,
                    arrow_type: event.arrow_type,
                },
            );
        }
    }
}

pub fn redraw_arrows(
    mut redraw_arrow: EventReader<RedrawArrow>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut arrow_query: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut commands: Commands,
) {
    let mut despawned: HashSet<ArrowMeta> = HashSet::new();

    for event in redraw_arrow.iter() {
        for (entity, arrow) in &mut arrow_query.iter_mut() {
            if despawned.contains(arrow) {
                continue;
            }
            if arrow.start.id == event.id || arrow.end.id == event.id {
                if let Some(entity) = commands.get_entity(entity) {
                    despawned.insert(*arrow);
                    entity.despawn_recursive();
                }
            }
        }
    }

    for arrow_meta in despawned {
        create_arrow.send(CreateArrow {
            start: arrow_meta.start,
            end: arrow_meta.end,
            arrow_type: arrow_meta.arrow_type,
        });
    }
}

fn get_pos(
    global_transform: &GlobalTransform,
    primary_window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let world_position = global_transform.affine().translation;
    let point = Vec2::new(world_position.x, primary_window.height() - world_position.y);
    camera.viewport_to_world_2d(camera_transform, point)
}
