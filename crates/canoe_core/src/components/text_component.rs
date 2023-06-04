use crate::components::RenderableBox;
use crate::render_tree::RenderTree;
use crate::Renderable;

pub struct TextProps {
    pub text: String,
}
#[derive(Default)]
pub struct TextState {}

pub fn text<'a>(
    props: &TextProps,
    _state: &TextState,
    _children: &'a Vec<RenderableBox>,
) -> Box<dyn Renderable + 'a> {
    Box::new(RenderTree::Text(props.text.to_string()))
}
