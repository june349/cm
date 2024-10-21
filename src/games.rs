pub struct Game {
    pub title: &'static str,
    pub aliases: &'static [&'static str],
    pub yaw: f64,
}

pub fn get_game(query: &str) -> Option<&Game> {
    GAMES
        .iter()
        .find(|&game| game.title == query || game.aliases.iter().any(|&alias| alias == query))
}

pub const GAMES: &[Game; 5] = &[
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
    Game {
        title: "Valorant",
        aliases: &["val"],
        yaw: 0.0066,
    },
    Game {
        title: "Apex Legends",
        aliases: &["apex"],
        yaw: 0.0066,
    },
    Game {
        title: "Team Fortress 2",
        aliases: &["tf2"],
        yaw: 0.0066,
    },
];
