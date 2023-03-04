use yew::prelude::*;
mod parse;
use web_sys::Node;
use yew::virtual_dom::VNode;

#[function_component(App)]
fn app() -> Html {
    let html = parse::parse();
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html);

    let node = Node::from(div);
    let vnode = VNode::VRef(node);

    html! {
        <div class="markdown-body">
            {vnode}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
