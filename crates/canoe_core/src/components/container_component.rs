use crate::components::RenderableBox;
use crate::render_tree::RenderTree;
use crate::Renderable;

pub struct ContainerProps {}
#[derive(Default)]
pub struct ContainerState {}

pub fn container(
    props: &ContainerProps,
    state: &ContainerState,
    children: &Vec<RenderableBox>,
) -> RenderableBox {
    let managed: Vec<&RenderableBox> = children.iter().map(|r| r).collect();
    Box::new(managed)
}

impl Renderable for Vec<&RenderableBox> {
    fn render_tree(&self) -> RenderTree {
        let render_tree_vec = self.iter().map(|rb| rb.render_tree()).collect();
        RenderTree::Container(render_tree_vec)
    }
}
