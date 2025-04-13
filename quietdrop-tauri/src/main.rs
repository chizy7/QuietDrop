use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="padding: 20px;">
            <h1>{"QuietDrop"}</h1>
            <p>{"End-to-End Encrypted Messaging"}</p>
            <p>{"Coming soon..."}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}