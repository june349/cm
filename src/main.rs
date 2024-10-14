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
    /// Convert a game sensitivity to cm/360°
    From {
        /// The game you're converting from
        game: String,
        /// Your sensitivity
        sensitivity: f64,
        /// Your DPI
        dpi: u32,
    },

    /// Convert cm/360° to a game sensitivity
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
    todo!();
}
