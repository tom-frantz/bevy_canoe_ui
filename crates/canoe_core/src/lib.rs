use crate::ui_primitive::UiPrimitive;
use bevy::prelude::*;
use std::any::{Any, TypeId};

pub mod components;
pub mod prelude;
pub(crate) mod ui_primitive;

pub type RenderFn<P, S> = Box<dyn Fn(&P, &S, &Vec<RenderableBox>) -> RenderableBox>;
pub type RenderableBox = Box<dyn Renderable>;

pub trait Renderable {
    fn to_ui_primitives(&self) -> UiPrimitive;
}

#[derive(Component)]
pub struct UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static + Default,
{
    pub props: Box<P>,
    pub state: Box<S>,
    pub render_fn: RenderFn<P, S>,
    pub children: Vec<RenderableBox>,
    pub component_name: String,
}

impl<P, S> Renderable for UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static + Default,
{
    fn to_ui_primitives(&self) -> UiPrimitive {
        // Get the 'guts' of a component
        let primitives = (*self.render_fn)(&self.props, &self.state, &self.children);
        // Then recursively render down into it.
        primitives.to_ui_primitives()
    }
}

impl<P, S> UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static + Default,
{
    fn render(&self) {
        let primitive_tree = self.to_ui_primitives();
    }
}

pub struct CanoePlugin;
impl Plugin for CanoePlugin {
    fn build(&self, app: &mut App) {}
}
