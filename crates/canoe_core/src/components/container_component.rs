use crate::components::RenderableBox;
use crate::render_tree::RenderTree;
use crate::Renderable;

pub struct ContainerProps {}
#[derive(Default)]
pub struct ContainerState {}

pub fn container<'a>(
    props: &ContainerProps,
    state: &ContainerState,
    children: &'a Vec<RenderableBox>,
) -> Box<dyn Renderable + 'a> {
    Box::new(children)
}

impl Renderable for &Vec<RenderableBox> {
    fn render_tree(&self) -> RenderTree {
        let render_tree_vec = self.iter().map(|rb| rb.render_tree()).collect();
        RenderTree::Container(render_tree_vec)
    }
}
