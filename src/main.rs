use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    // rerender when the state has changed
    let state = use_state(|| Model { value: 0 });

    let onclick = {
        let state = state.clone();

        // functions that need to be called from html need to be
        // encapsulated in this callback mechanism
        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };

    // `{onclick}` is shorthand for `onclick={onclick}`
    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{state.value}</p>

        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
