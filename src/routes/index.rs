use leptos::*;

use crate::api;

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    let (search, set_search) = create_signal(cx, "".to_string());
    let games = create_resource(
        cx,
        move || search(),
        move |search| async move {
            let path = format!("games?search={}", search);
            api::fetch_api::<Vec<api::Game>>(cx, &path).await
        },
    );

    view! { cx,
        <div class="w-full max-w-5xl flex flex-col items-center lg:justify-center">
          <div class="w-full flex flex-col justify-center items-center p-4 lg:p-8 gap-4 lg:gap-8 sticky z-10 top-0 bg-neutral-900">
            <a href="https://github.com/nunogois/leptos-igdb" target="_blank">
              <h1 class="text-2xl lg:text-4xl text-violet-400">"leptos-igdb"</h1>
            </a>
            <input type="text" class="bg-neutral-800 transition border border-white focus:border-violet-400 outline-none rounded-lg px-3 py-2 w-full max-w-xs text-center" placeholder="Search for a game..."
              prop:value={move || search()}
              on:input=move |e| {
                let val = event_target_value(&e);
                set_search(val);
              }
            />
            {move || {
                if search().is_empty() {
                    return view! { cx, <h2 class="text-lg lg:text-xl">"Popular Games"</h2> };
                }
                match games.read() {
                    Some(Some(games)) => {
                        if games.len() > 0 {
                            view! { cx, <h2 class="text-lg lg:text-xl">{format!("Results for: {}.", search())}</h2> }
                        } else {
                            view! { cx, <h2 class="text-lg lg:text-xl">{format!("No results found for {}.", search())}</h2> }
                        }
                    }
                    _ => {
                        view! { cx, <h2 class="text-lg lg:text-xl">"Something went wrong."</h2> }
                    }
                }
            }}
          </div>
          <div class="min-h-[528px] w-full max-w-md lg:max-w-none">
          {move || match games.read() {
              None => None,
              Some(None) => Some(view! { cx,  <p>"Error loading games."</p> }.into_any()),
              Some(Some(games)) => {
                  Some(view! { cx,
                    <div class="grid grid-flow-row lg:grid-flow-col grid-rows-2 gap-4 w-full p-4">
                      <For
                          each=move || games.clone()
                          key=|game| game.id
                          view=move |game: api::Game| {
                              view! { cx,
                                <GameItem game />
                              }
                          }
                      />
                      </div>
                  }.into_any())
              }
          }}
          </div>
        </div>
    }
}

#[component]
pub fn GameItem(cx: Scope, game: api::Game) -> impl IntoView {
    let name = game.name.clone();

    view! { cx,
        <a href={format!("/game/{}", game.id)}>
          <div class="bg-neutral-800 rounded-lg flex justify-start w-full h-[120px] lg:w-[180px] lg:h-[240px] lg:relative lg:rounded-none">
            <img src=game.image class="w-[90px] rounded-l-lg h-full lg:w-full lg:rounded-l-none" />
            <div class="hidden w-full p-2 bg-black opacity-90 absolute bottom-0 lg:block">
              {name}
            </div>
            <div class="lg:hidden p-4 w-full flex items-center justify-between">
              <div class="flex flex-col justify-between h-full">
                  <span>{game.name}</span>
                  <span class="text-xs">{game.first_release_date}</span>
              </div>
              <div>
                {move || {
                    let color = if game.total_rating > 90 {
                        "text-green-400"
                    } else if game.total_rating > 85 {
                        "text-green-500"
                    } else if game.total_rating > 80 {
                        "text-green-600"
                    } else if game.total_rating > 75 {
                        "text-yellow-400"
                    } else if game.total_rating > 70 {
                        "text-yellow-500"
                    } else if game.total_rating > 65 {
                        "text-yellow-600"
                    } else {
                        "text-red-400"
                    };
                    view! { cx, <span class=format!("text-lg {}", color)>{game.total_rating}</span> }
                }}
              </div>
            </div>
          </div>
        </a>
    }
}
