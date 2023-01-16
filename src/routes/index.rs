use leptos::*;

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos - Index!"</h1>
        <button class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg" on:click=on_click>
          {move || if count() == 0 {
              "Click me!".to_string()
          } else {
              count().to_string()
          }}
        </button>
    }
}
