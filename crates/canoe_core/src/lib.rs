use crate::render_tree::RenderTree;
use bevy::prelude::*;
use components::RenderableBox;
use std::any::{Any, TypeId};

pub mod components;
pub mod prelude;
pub(crate) mod render_tree;

pub type RenderFn<P, S> = Box<dyn Sync + Send + Fn(&P, &S, &Vec<RenderableBox>) -> RenderableBox>;

pub struct UiDependencies {}

pub trait Renderable {
    fn render_tree(&self) -> RenderTree;
}

#[derive(Component)]
pub struct RootUiComponent(pub RenderFn<(), ()>);

fn register_root(q_root: Query<&RootUiComponent, Added<RootUiComponent>>) {
    let root = q_root.single(); // fn(_, _, _) -> UiComponent(..., canoe::text_fn)
    let empty_vec = vec![];
    let root_renderable: RenderableBox = (root.0)(&(), &(), &empty_vec); // UiComponent(..., canoe::text_fn)

    let render_tree = root_renderable.render_tree();
    println!("{render_tree:#?}");
    println!("Found root once.")
}

pub struct CanoePlugin;
impl Plugin for CanoePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(register_root.in_base_set(StartupSet::PostStartup));
    }
}
