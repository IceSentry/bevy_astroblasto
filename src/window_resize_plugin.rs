use bevy::{prelude::*, window::WindowResized};

#[derive(Default)]
struct State {
    window_resized_event_reader: EventReader<WindowResized>,
}

fn update_window_size_system(
    mut state: ResMut<State>,
    window_resized_events: Res<Events<WindowResized>>,
    mut window_desc: ResMut<WindowDescriptor>,
) {
    for event in state
        .window_resized_event_reader
        .iter(&window_resized_events)
    {
        window_desc.width = event.width as u32;
        window_desc.height = event.height as u32;
    }
}

pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_system(update_window_size_system.system());
    }
}
