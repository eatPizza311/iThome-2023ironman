use leptos::{html::Input, *};

#[component]
pub fn MessageInputField(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();

    view! {
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t bg-white border-gray-300">
            <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("")
            }>
                <input type="text" class="w-2/3 p-4 border rounded-full input-field border-gray-300 text-black" placeholder="Say something..." node_ref=input_ref/>
                <input type="submit" class="h-full p-4 rounded-full cursor-pointer bg-blue-500 text-white" />
            </form>
        </div>
    }
}
