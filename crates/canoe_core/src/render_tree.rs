use crate::Renderable;

#[derive(Debug, Clone)]
pub enum RenderTree {
    Container(Vec<RenderTree>),
    Text(String),
}

impl Renderable for RenderTree {
    fn render_tree(&self) -> RenderTree {
        match self {
            RenderTree::Container(c) => RenderTree::Container(c.clone()),
            RenderTree::Text(t) => RenderTree::Text(t.into()),
        }
    }
}
