/*

    File:        | cli.rs
    Author:      | memoriilane
    Description: | argument parsing & terminal stuff

*/
/********************************************************************************************************************************/


// # Important stuff
use clap::Parser;
use std::{
    env
};


// # Main logic
pub fn get_image_protocol() -> Option<&'static str> {
    if env::var("KITTY_WINDOW_ID").is_ok() {
        return Some("kitty");
    }

    if env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app")
        || env::var("WEZTERM_PANE").is_ok() {
        return Some("iterm2");
    }

    None
}

// sanitize strings (remove all characters that arent in the english alphabet)
// (it keeps spaces though
fn sanitize(s: &str) -> Result<String, String> {
    Ok(s.chars().filter(|c| c.is_alphabetic()).collect())
}


#[derive(Parser, Debug)]
pub struct Args {
    // Name of the Character/NPC
    #[arg(value_parser = sanitize, default_value = "yoimiya")]
    pub name: String,

    // Name of the images to display
    #[arg(value_parser = sanitize, default_value = "")]
    pub image_name: String,

    // for NPCs (i.e. Azhdaha)
    // -n / --npc
    #[arg(short, long, default_value_t = false)]
    pub npc: bool,

    // for artifacts (e.g. gladiatorsfinale, wandererstroupe, etc.)
    // -a / --artifact
    #[arg(short, long, default_value_t = false)]
    pub artifact: bool,

    // could also just leave image_name blank if you want to get a random emoji
    // -r / --randomize
    #[arg(short, long, default_value_t = false)]
    pub randomize: bool,

    // width (in pixels)
    // -w / --width
    #[arg(short, long, default_value_t = 32)]
    pub width: u32
}