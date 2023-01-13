use yew::prelude::*;

#[function_component(Counter)]
fn counter() -> HtmlResult {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    Ok(html! {
        <div>
            <button {onclick} class={classes!("inline-block","px-6","py-2.5","bg-blue-600","text-white","font-medium","text-xs","leading-tight","uppercase","rounded","shadow-md","hover:bg-blue-700","hover:shadow-lg","focus:bg-blue-700","focus:shadow-lg","focus:outline-none","focus:ring-0","active:bg-blue-800","active:shadow-lg","transition","duration-150","ease-in-out")}>{ "Counter: +1" }</button>
            <p>{ *counter }</p>
        </div>
    })
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1 class={classes!("text-3xl", "font-bold")}>
                {"Hello world!"}
            </h1>
            <Counter />
        </div>
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
