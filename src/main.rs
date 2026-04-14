/*
    File:        | main.rs
    Author:      | memoriilane
    Description: | A tool to display Genshin Impact's chat emojis in your terminal.

    To-Do:
        * Maybe add weapon icons/images?
*/

/********************************************************************************************************************************/

// # Important stuff
mod assets;
mod cli;
use clap::Parser;
use cli::Args;

// # Main logic
fn main() {
    let image_protocol = cli::get_image_protocol();

    // sorry windows terminal users (use WezTerm)
    if image_protocol == None {
        println!(
            "{}Your terminal's image protocol is not supported.{}",
            "\x1b[31m", "\x1b[0m"
        );
        std::process::exit(1);
    }

    // ok now time to actually do stuff
    let args = Args::parse();

    // if no image name is provided
    if args.name.is_empty() {
        println!(
            "{}Please specify the name of the artifact set / character / NPC.{}",
            "\x1b[31m", "\x1b[0m"
        );
        std::process::exit(1);
    }

    // handle args (artifact / NPC flag) (defaults to character)
    let target_folder: &str = if args.artifact {
        "artifacts"
    } else if args.npc {
        "npcs"
    } else {
        "characters"
    };

    let randomize: bool = if args.randomize {
        true
    } else if args.image_name.is_empty() {
        true
    } else {
        false
    };

    // locate the image and actually load it here
    if let Some(bytes) = assets::find_image(&target_folder, &args.name, &args.image_name, randomize)
    {
        let img = image::load_from_memory(&bytes).expect("Failed to decode image");
        let config = viuer::Config {
            use_iterm: image_protocol == Some("iterm2"),
            use_kitty: image_protocol == Some("kitty"),
            absolute_offset: false,
            width: Some(args.width),
            restore_cursor: false,
            ..Default::default()
        };

        viuer::print(&img, &config).expect("Failed to print image");
    } else {
        println!(
            "{}Emoji {} not found.{}",
            "\x1b[31m", &args.image_name, "\x1b[0m"
        );
    }
}
