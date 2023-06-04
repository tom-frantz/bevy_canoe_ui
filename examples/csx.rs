use bevy::prelude::*;
use bevy_canoe_ui::prelude::*;
use canoe_core::{components::*, Renderable, RootUiComponent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CanoePlugin)
        .add_startup_system(render_basic_ui)
        .run();
}

fn root<'children>(
    _: &(),
    _: &(),
    _: &'children Vec<RenderableBox>,
) -> Box<dyn Renderable + 'children> {
    csx! {
        <Container>
            <Text
                text={String::from("123")}
            />
            <Container>
                <Text text={String::from("456")}/>
            </Container>
        </Container>
    }
}

fn render_basic_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(RootUiComponent(Box::new(root)));
}
