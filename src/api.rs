use chrono::NaiveDateTime;
use leptos::{console_log, Scope, Serializable};
use serde::{Deserialize, Serialize};

// pub fn igdb_api(path: &str) -> String {
//     format!("https://api.igdb.com/v4/{path}")
// }

pub fn igdb_api_proxy(path: &str) -> String {
    format!("https://igdb-api.nunogois.com/{path}")
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let json = gloo_net::http::Request::get(&igdb_api_proxy(path))
        .header(
            "Authorization",
            format!("Bearer {}", dotenv!("TOKEN")).as_str(),
        )
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    T::from_json(&json).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(_cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let client = reqwest::Client::new();

    let json = client
        .post(path)
        .body(
            "fields name, first_release_date, platforms.abbreviation, cover.url, total_rating;
          where rating > 69 &
          aggregated_rating_count > 0 &
          total_rating_count > 1 &
          version_parent = null;
          sort first_release_date desc;",
        )
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::from_json(&json).map_err(|e| log::error!("{e}")).ok()
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Platform {
    name: String,
    abbreviation: String,
    logo: String,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct SimilarGame {
    id: usize,
    name: String,
    image: String,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Game {
    pub id: usize,
    pub name: String,
    pub image: String,
    pub first_release_date: String,
    pub platforms: Vec<Platform>,
    pub total_rating: u32,
    pub url: String,
    pub summary: String,
    pub genres: String,
    pub themes: String,
    pub game_modes: String,
    pub involved_companies: String,
    pub screenshots: Vec<String>,
    pub similar_games: Vec<SimilarGame>,
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawGame {
            id: usize,
            name: String,
            cover: Option<Cover>,
            first_release_date: Option<usize>,
            platforms: Option<Vec<RawPlatform>>,
            total_rating: Option<f32>,
            url: Option<String>,
            summary: Option<String>,
            genres: Option<Vec<Genre>>,
            themes: Option<Vec<Theme>>,
            game_modes: Option<Vec<GameMode>>,
            involved_companies: Option<Vec<InvolvedCompany>>,
            screenshots: Option<Vec<Screenshot>>,
            similar_games: Option<Vec<RawSimilarGame>>,
        }

        #[derive(Deserialize)]
        struct Cover {
            url: String,
        }

        #[derive(Deserialize)]
        struct RawPlatform {
            abbreviation: Option<String>,
            name: Option<String>,
            platform_logo: Option<Logo>,
        }

        #[derive(Deserialize)]
        struct Logo {
            url: String,
        }

        #[derive(Deserialize)]
        struct Genre {
            name: String,
        }

        #[derive(Deserialize)]
        struct Theme {
            name: String,
        }

        #[derive(Deserialize)]
        struct GameMode {
            name: String,
        }

        #[derive(Deserialize)]
        struct InvolvedCompany {
            company: Company,
        }

        #[derive(Deserialize)]
        struct Company {
            name: String,
        }

        #[derive(Deserialize)]
        struct Screenshot {
            url: String,
        }

        #[derive(Deserialize)]
        struct RawSimilarGame {
            id: usize,
            name: String,
            cover: Option<Cover>,
        }

        let game = match RawGame::deserialize(deserializer) {
            Ok(game) => game,
            Err(e) => {
                console_log(&format!("deserialize error: {:?}", e));
                return Err(e);
            }
        };

        Ok(Game {
            id: game.id,
            name: game.name,
            image: match game.cover {
                Some(cover) => format!("https:{}", cover.url).replace("t_thumb", "t_cover_big_2x"),
                None => "https://images.igdb.com/igdb/image/upload/t_cover_big_2x/nocover.png"
                    .to_string(),
            },
            first_release_date: match game.first_release_date {
                Some(date) => format!(
                    "{:?}",
                    NaiveDateTime::from_timestamp_opt(date as i64, 0).unwrap()
                )
                .split("T")
                .collect::<Vec<&str>>()[0]
                    .to_string(),
                None => "".to_string(),
            },
            platforms: match game.platforms {
                Some(platforms) => platforms
                    .into_iter()
                    .map(|p| Platform {
                        name: p.name.unwrap_or("".to_string()),
                        abbreviation: p.abbreviation.unwrap_or("".to_string()),
                        logo: match p.platform_logo {
                            Some(logo) => format!("https:{}", logo.url),
                            None => "".to_string(),
                        },
                    })
                    .collect(),
                None => vec![],
            },
            total_rating: match game.total_rating {
                Some(rating) => rating.round() as u32,
                None => 0,
            },
            url: game.url.unwrap_or("".to_string()),
            summary: game.summary.unwrap_or("".to_string()),
            genres: match game.genres {
                Some(genres) => genres
                    .into_iter()
                    .map(|g| g.name)
                    .collect::<Vec<String>>()
                    .join(", "),
                None => "".to_string(),
            },
            themes: match game.themes {
                Some(themes) => themes
                    .into_iter()
                    .map(|t| t.name)
                    .collect::<Vec<String>>()
                    .join(", "),
                None => "".to_string(),
            },
            game_modes: match game.game_modes {
                Some(game_modes) => game_modes
                    .into_iter()
                    .map(|gm| gm.name)
                    .collect::<Vec<String>>()
                    .join(", "),
                None => "".to_string(),
            },
            involved_companies: match game.involved_companies {
                Some(involved_companies) => involved_companies
                    .into_iter()
                    .map(|ic| ic.company.name)
                    .collect::<Vec<String>>()
                    .join(", "),
                None => "".to_string(),
            },
            screenshots: match game.screenshots {
                Some(screenshots) => screenshots
                    .into_iter()
                    .map(|s| format!("https:{}", s.url).replace("t_thumb", "t_cover_big_2x"))
                    .collect(),
                None => vec![],
            },
            similar_games: match game.similar_games {
                Some(similar_games) => similar_games
                    .into_iter()
                    .map(|sg| SimilarGame {
                        id: sg.id,
                        name: sg.name,
                        image: match sg.cover {
                            Some(cover) => {
                                format!("https:{}", cover.url).replace("t_thumb", "t_cover_big_2x")
                            }
                            None => "".to_string(),
                        },
                    })
                    .collect(),
                None => vec![],
            },
        })
    }
}
