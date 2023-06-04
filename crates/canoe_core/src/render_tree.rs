use crate::Renderable;
use bevy::prelude::{Val::*, *};

#[derive(Debug, Clone, Resource)]
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

impl RenderTree {
    pub fn spawn(&self, cmds: &mut ChildBuilder, asset_server: &AssetServer) {
        match self {
            RenderTree::Container(v) => {
                cmds.spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Px(100.)),
                        border: UiRect::all(Px(20.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                    ..default()
                })
                .with_children(|cb| {
                    v.iter().for_each(|rt| rt.spawn(cb, asset_server));
                });
            }
            RenderTree::Text(t) => {
                cmds.spawn(TextBundle::from_section(
                    t.to_string(),
                    TextStyle {
                        font: asset_server.load("roboto.ttf"),
                        ..default()
                    },
                ));
            }
        }
    }
}
