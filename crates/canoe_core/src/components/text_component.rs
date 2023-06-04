use crate::{ui_primitive::UiPrimitive::Text, Renderable, RenderableBox};

pub struct TextProps {
    pub text: String,
}
#[derive(Default)]
pub struct TextState {}
pub fn text(
    props: &TextProps,
    state: &TextState,
    _children: &Vec<RenderableBox>,
) -> Box<dyn Renderable> {
    Box::new(Text(props.text.to_string(), Default::default()))
}
