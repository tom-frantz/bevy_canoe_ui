use crate::components::RenderableBox;
use crate::render_tree::RenderTree;
use crate::Renderable;

pub struct TextProps {
    pub text: String,
}
#[derive(Default)]
pub struct TextState {}

pub fn text(
    props: &TextProps,
    _state: &TextState,
    _children: &Vec<RenderableBox>,
) -> Box<dyn Renderable> {
    Box::new(RenderTree::Text(props.text.to_string()))
}
