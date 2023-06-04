use crate::{Renderable, RenderableBox};
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct CanoeStyle {}

pub enum UiPrimitive {
    Container(Vec<RenderableBox>, CanoeStyle),
    Text(String, CanoeStyle),
}

impl Renderable for UiPrimitive {
    fn to_ui_primitives(&self) -> UiPrimitive {
        match self {
            UiPrimitive::Container(c, s) => {
                println!("rendering container");
                UiPrimitive::Container(
                    c.iter()
                        .map(|i| Box::new(i.to_ui_primitives()) as RenderableBox)
                        .collect(),
                    *s,
                )
            }
            UiPrimitive::Text(t, s) => {
                println!("rendering text '{t}'");
                UiPrimitive::Text(t.clone(), *s)
            }
        }
    }
}
