use bevy::prelude::*;
use bevy_canoe_ui::prelude::*;
use canoe_core::{text_component, Renderable, TextState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CanoePlugin)
        .add_startup_system(render_basic_ui)
        .run();
}

fn my_screen(_props: &(), _state: &()) -> Box<dyn Renderable> {
    Box::new(UiComponent {
        props: Box::new(()),
        state: Box::new(()),
        render_fn: Box::new(my_comp),
    })
}

fn my_comp(_props: &(), _state: &()) -> Box<dyn Renderable> {
    // return <TextComponent>Hello!</TextComponent>

    Box::new(UiComponent {
        props: Box::new(TextProps {
            text: String::from("Hey there gamer!"),
        }),
        state: Box::new(TextState {}),
        render_fn: Box::new(text_component),
    })
}

fn render_basic_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let my_comp = UiComponent {
        props: Box::new(()),
        state: Box::new(()),
        render_fn: Box::new(my_screen),
    };
    let x = my_comp.render();
}
