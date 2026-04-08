use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub games: Vec<Game>,
    pub quotes: Vec<Quote>,
    pub start_menu: StartMenu,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub description: String,
    pub contributors: Vec<String>,
    pub tech: Vec<Tech>,
    pub icon: String,
    pub demo: Option<String>,
    pub launch_url: String,
    pub launch_type: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Tech {
    pub name: String,
    pub icon: String,
}

#[derive(Deserialize, Clone)]
pub struct Quote {
    pub text: String,
}

#[derive(Deserialize, Clone)]
pub struct StartMenu {
    pub about_url: String,
    pub github_url: String,
    pub itchio_url: String,
}

pub fn load_config() -> Config {
    let raw = include_str!("../games.toml");
    toml::from_str(raw).expect("failed to parse games.toml")
}
