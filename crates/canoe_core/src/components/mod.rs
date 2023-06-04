mod container_component;
mod text_component;

use crate::render_tree::RenderTree;
use crate::{RenderFn, Renderable, UiDependencies};
use bevy::prelude::Component;
pub use container_component::{container, ContainerProps, ContainerState};
pub use text_component::{text, TextProps, TextState};

pub type RenderableBox = Box<dyn Renderable>;

#[derive(Component)]
pub struct UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static + Default,
{
    pub props: P,
    pub state: S,
    pub render_fn: RenderFn<P, S>,
    pub children: Vec<RenderableBox>,
    pub component_name: String,
}

impl<P, S> Renderable for UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static + Default,
{
    fn render_tree(&self) -> RenderTree {
        let renderables: Box<dyn Renderable> =
            (self.render_fn)(&self.props, &self.state, &self.children);
        renderables.render_tree()
    }
}

impl<'a, P, S> UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static + Default,
{
}
