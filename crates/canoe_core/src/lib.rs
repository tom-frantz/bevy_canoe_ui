use bevy::prelude::*;

pub mod prelude;

pub trait Renderable {
    fn render(&self) -> UiNode;
}

#[derive(Component)]
pub struct UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    pub props: Box<P>,
    pub state: Box<S>,
    pub render_fn: Box<dyn Fn(&P, &S) -> Box<dyn Renderable>>,
}
impl<P, S> Renderable for UiComponent<P, S>
where
    P: Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    fn render(&self) -> UiNode {
        println!("Rendering UiComponent!!!");
        (self.render_fn)(&self.props, &self.state).render()
    }
}

#[derive(Clone)]
pub enum UiNode {
    Text(String),
    // Container(Vec<RenderComponent>),
}

impl Renderable for UiNode {
    fn render(&self) -> UiNode {
        println!("Rendering UiNode!!!");

        self.clone()
    }
}

pub struct TextProps {
    pub text: String,
}
pub struct TextState {}
pub fn text_component(_props: &TextProps, _state: &TextState) -> Box<dyn Renderable> {
    Box::new(UiNode::Text(String::from("123")))
}

pub struct CanoePlugin;
impl Plugin for CanoePlugin {
    fn build(&self, app: &mut App) {}
}
