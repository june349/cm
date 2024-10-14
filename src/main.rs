use clap::{Parser, Subcommand};

mod games;

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Convert your in-game sensitivity to cm/360°
    From {
        /// The game you're converting from
        game: String,
        /// Your sensitivity
        sensitivity: f64,
        /// Your DPI
        dpi: u32,
    },

    /// Convert your cm/360° to an in-game sensitivity
    To {
        /// The game you're converting to
        game: String,
        /// Your cm/360°
        cm: f64,
        /// Your DPI
        dpi: u32,
    },

    /// List supported games
    List {
        /// List aliases along with each game (if any)
        #[arg(short, long)]
        aliases: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::From {
            game,
            sensitivity,
            dpi,
        } => match games::get_game(&game) {
            Some(game) => {
                let cm_per_360 = 360.0 / (game.yaw * dpi as f64 * sensitivity) * 2.54;
                println!("{:.3}", cm_per_360);
            }
            None => {
                eprintln!("Error: Game '{}' not found.", game);
            }
        },

        Command::To { game, cm, dpi } => match games::get_game(&game) {
            Some(game) => {
                let sensitivity = 360.0 / (game.yaw * dpi as f64 * (cm / 2.54));
                println!("{:.3}", sensitivity);
            }
            None => {
                eprintln!("Error: Game '{}' not found.", game);
            }
        },

        Command::List { aliases } => {
            for game in games::GAMES {
                if aliases {
                    if game.aliases.is_empty() {
                        println!("{} (Aliases: none)", game.title);
                    } else {
                        let alias_list = game.aliases.join(", ");
                        println!("{} (Aliases: {})", game.title, alias_list);
                    }
                } else {
                    println!("{}", game.title);
                }
            }
        }
    }
}
