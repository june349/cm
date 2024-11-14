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

pub const GAMES: &[Game; 11] = &[
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
        yaw:  0.06996,
    },
    Game {
        title: "Apex Legends",
        aliases: &["apex"],
        yaw: 0.022,
    },
    Game {
        title: "Team Fortress 2",
        aliases: &["tf2"],
        yaw: 0.022,
    },
    Game {
        title: "Rainbow Six: Siege",
        aliases: &["r6", "r6s", "siege"],
        yaw: 0.005729577951308232,
    },
    Game {
        title: "Fortnite",
        aliases: &["fn"],
        yaw: 0.005555,
    },
    Game {
        title: "Destiny 2",
        aliases: &["destiny", "d2"],
        yaw: 0.0066,
    },
    Game {
        title: "Rust",
        aliases: &["rs"],
        yaw: 0.1125,
    },
    Game {
        title: "THE FINALS",
        aliases: &["finals"],
        yaw: 0.00101545,
    },
    Game {
        title: "CALL OF DUTY",
        aliases: &["cod"],
        yaw: 0.0066,
    },
];
