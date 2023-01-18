use leptos::{
    web_sys::{HtmlInputElement, KeyboardEvent},
    *,
};

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);
    let on_keypress = move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            let input = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let value = input.value();
            log!("Search for: {}", value);
        }
    };

    view! { cx,
        <div class="w-full max-w-5xl p-8 flex flex-col justify-center items-center gap-8">
          <a href="https://github.com/nunogois/leptos-igdb" target="_blank">
            <h1 class="text-4xl text-violet-400">"leptos-igdb"</h1>
          </a>
          <input type="text" class="bg-neutral-800 border border-white focus:border-violet-400 outline-none rounded-lg px-3 py-2 w-full max-w-xs text-center" placeholder="Search for a game..." on:keypress=on_keypress/>
          // <button class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg" on:click=on_click>
          //   {move || if count() == 0 {
          //       "Click me!".to_string()
          //   } else {
          //       count().to_string()
          //   }}
          // </button>
        </div>
    }
}
