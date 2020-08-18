use bevy::prelude::*;

#[derive(Default)]
struct CursorMovedEventReader {
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

pub struct MousePos {
    pub pos: Vec2,
}

fn mouse_position_system(
    mut state: ResMut<CursorMovedEventReader>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut mouse_pos: ResMut<MousePos>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        mouse_pos.pos = event.position;
    }
}

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(MousePos { pos: Vec2::zero() })
            .init_resource::<CursorMovedEventReader>()
            .add_system(mouse_position_system.system());
    }
}
