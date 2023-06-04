use crate::{RenderableBox, UiPrimitive::Container};

pub struct ContainerProps {}
#[derive(Default)]
pub struct ContainerState {}

pub fn container(
    props: &ContainerProps,
    state: &ContainerState,
    children: &Vec<RenderableBox>,
) -> RenderableBox {
    Box::new(Container(children.to_vec(), Default::default()))
}
