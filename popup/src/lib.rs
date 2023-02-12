use yew::{Properties, function_component, Html, html};
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "clear" }</button>
        </div>
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
