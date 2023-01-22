use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct GridProps {
    pub children: Children,
}

#[function_component]
pub fn Grid(props: &GridProps) -> Html {
    html! {
        <div class="grid">
            {for props.children.iter()}
        </div>
    }
}
