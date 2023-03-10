use cfg_if::cfg_if;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod api;
mod routes;
use routes::game::*;
use routes::index::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos_igdb.css"/>
        <Title text="leptos-igdb"/>
        <Meta name="description" content="Leptos implementation of a IGDB demo."/>
        <Router>
            <main class="w-full h-full flex justify-center bg-neutral-900 text-gray-100 overflow-auto">
                <Routes>
                    <Route path="" view=|cx| view! { cx,  <Index/> }/>
                    <Route path="game/:id" view=|cx| view! { cx,  <Game/> }/>
                </Routes>
            </main>
        </Router>
    }
}

cfg_if! {
  if #[cfg(feature = "hydrate")] {
    use wasm_bindgen::prelude::wasm_bindgen;

      #[wasm_bindgen]
      pub fn hydrate() {
        use leptos::*;

        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
      }
  }
  // else if #[cfg(feature = "csr")] {
  //   use wasm_bindgen::prelude::wasm_bindgen;

  //   #[wasm_bindgen(start)]
  //   pub fn main() {
  //       use leptos::*;

  //       _ = console_log::init_with_level(log::Level::Debug);
  //       console_error_panic_hook::set_once();

  //       mount_to_body(|cx| {
  //           view! { cx, <App /> }
  //       });
  //   }
  // }
}
