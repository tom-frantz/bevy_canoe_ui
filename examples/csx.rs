use bevy::prelude::*;
use bevy_canoe_ui::prelude::*;
use canoe_core::{components::*, Renderable, RenderableBox};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CanoePlugin)
        .add_startup_system(render_basic_ui)
        .run();
}

#[derive(Debug, Default)]
struct FancyTextProps {}
#[derive(Debug, Default)]
struct FancyTextState {}

fn bordered_page(_: &FancyTextProps, _: &FancyTextState, children: Vec<RenderableBox>) {
    csx! {
        <container>

        </container>
    }
}

#[derive(Debug, Default)]
struct MyScreenProps {}
#[derive(Debug, Default)]
struct MyScreenState {}

fn my_screen(
    _props: &MyScreenProps,
    _state: &MyScreenState,
    _children: &Vec<RenderableBox>,
) -> RenderableBox {
    println!("in my screen!");
    csx! {
        <container>
            <text
                text={String::from("123")}
            />
            <text
                text={String::from("456")}
            />
        </container>

    }

    // Box::new(UiComponent {
    //     props: Box::new(ContainerProps {}),
    //     state: Default::default(),
    //     render_fn: Box::new(container),
    //     children: <[_]>::into_vec(
    //         #[rustc_box]
    //         ::alloc::boxed::Box::new([
    //             Box::new(UiComponent {
    //                 props: Box::new(TextProps {
    //                     text: { String::from("123") },
    //                 }),
    //                 state: Default::default(),
    //                 render_fn: Box::new(text),
    //                 children: ::alloc::vec::Vec::new(),
    //                 component_name: "Text".into(),
    //             }),
    //             Box::new(UiComponent {
    //                 props: Box::new(TextProps {
    //                     text: { String::from("456") },
    //                 }),
    //                 state: Default::default(),
    //                 render_fn: Box::new(text),
    //                 children: ::alloc::vec::Vec::new(),
    //                 component_name: "Text".into(),
    //             }),
    //         ]),
    //     ),
    //     component_name: "Container".into(),
    // })
}

fn render_basic_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let abc = csx! {
        <MyScreen/>
    };

    abc.to_ui_primitives();
}
