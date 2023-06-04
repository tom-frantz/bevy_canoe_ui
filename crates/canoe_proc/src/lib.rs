use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn_rsx::{parse, Node, NodeAttribute, NodeBlock, NodeElement, NodeFragment, NodeText};

#[proc_macro]
pub fn csx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let source = input.to_string();
    let res = parse(input);

    let mut result = TokenStream::new();

    let nodes = res.unwrap();

    let tokens = match nodes.len() {
        len if len == 1 => {
            let node = nodes.first().unwrap();
            Some(handle_node(node))
        }
        len if len > 1 => {
            todo!("impl a container first")
        }
        _ => None,
    };

    if let Some(res) = tokens {
        result.append_all(res)
    }

    proc_macro::TokenStream::from(result)
}

fn handle_node(n: &Node) -> proc_macro2::TokenStream {
    let tokens = match n {
        Node::Element(e) => Some(handle_element(e)),
        Node::Text(t) => Some(handle_text(t)),
        Node::Fragment(f) => Some(handle_fragment(f)),
        Node::Block(b) => Some(handle_block(b)),
        // Don't care about below
        Node::Attribute(_) => None,
        Node::Comment(_) => None,
        Node::Doctype(_) => None,
    };

    tokens.unwrap_or(TokenStream::new())
}

fn handle_element(node: &NodeElement) -> proc_macro2::TokenStream {
    let name = &node.name.to_string();
    let pascal = name.to_case(Case::Pascal);
    let snake = name.to_case(Case::Snake);

    let render_fn_ident = format_ident!("{snake}");
    let props_expr = handle_props(&node.attributes, format_ident!("{pascal}Props"));
    let children_expr = handle_children(&node.children);

    quote! {
        Box::new(UiComponent {
            props: #props_expr,
            state: Default::default(),
            render_fn: Box::new(#render_fn_ident),
            children: #children_expr,
            component_name: #pascal.into()
        })
    }
}

fn handle_props(attrs: &[Node], props_ident: proc_macro2::Ident) -> proc_macro2::TokenStream {
    let props: Vec<TokenStream> = {
        attrs
            .iter()
            .filter_map(|node| match node {
                Node::Attribute(a) => {
                    let ident = &a.key;
                    let val = a.value.as_ref().unwrap().as_ref();
                    Some(quote! {
                        #ident: #val,
                    })
                }
                _ => None,
            })
            .collect()
    };

    quote! {
        #props_ident {
            #(#props)*
        }
    }
}

fn handle_children(children: &[Node]) -> proc_macro2::TokenStream {
    let children: Vec<TokenStream> = children.iter().map(handle_node).collect();

    quote! {
        vec![#(#children),*]
    }
}

fn handle_text(t: &NodeText) -> proc_macro2::TokenStream {
    todo!("Implement handle_text")
}

fn handle_fragment(f: &NodeFragment) -> proc_macro2::TokenStream {
    todo!("Implement handle_fragment")
}

fn handle_block(b: &NodeBlock) -> proc_macro2::TokenStream {
    todo!("Implement handle_block")
}
