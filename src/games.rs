pub struct Game {
    pub title: &'static str,
    pub aliases: &'static [&'static str],
    pub yaw: f64,
}

fn get_game(query: &str) -> Option<&Game> {
    GAMES
        .iter()
        .find(|&game| game.title == query || game.aliases.iter().any(|&alias| alias == query))
}

pub const GAMES: &[Game] = &[
    Game {
        title: "Counter-Strike",
        aliases: &["cs", "css", "cs2", "csgo"],
        yaw: 0.022,
    },
    Game {
        title: "Overwatch",
        aliases: &["ow", "ow2"],
        yaw: 0.0066,
    },
];