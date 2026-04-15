<div align="center">
  <img src="assets/images/aranara.png" width="128"/>
  
  # aranara
  ### Display Genshin Impact chat emojis & artifacts in your terminal.
</div>

## Features:
- Display character or NPC chat emojis in your terminal, as well as artifacts.
- Has all artifact images & chat emojis *(Set #1 - Set #48, Genshin Impact × OnePlus Keqing Set)* up until game version **Luna V** *(25 Feb 2026)*.
- The images are embedded into the binary, so nothing else has to be downloaded. Works fully offline. *(which is why the program is 120mb+)*

## 🚧 TO-DO:
- Add the remaining sets of emojis:
  - Kiehl's *(Genshin Impact × Kiehl's (2023))* 
  - ~~Fortuitous Encounter the Coral Sea *(Genshin Impact × Xiaohongshu (2023))*~~ *(might not be added since the only available images are 120x120)*
  - Migration to the Coral Sea *(Genshin Impact × Xiaohongshu (2024))*
- Add weapon images ***(?)**

## Requirements:
- A supported terminal.
    - [kitty](https://github.com/kovidgoyal/kitty) / [ghostty](https://github.com/ghostty-org/ghostty) *(Linux / macOS)*

## Installation
### Build from Source
##### Dependencies
- git
- cargo


```sh
git clone https://github.com//memoriilane/aranara
cd aranara
cargo build
cargo install --path .
```

## Usage

`aranara NAME [SUB_NAME] [type]`
- Leave `SUB_NAME` empty or pass `-r` / `--randomize` to randomize it.
- Default `type` is character.
- When providing a name with a space in it, it should be wrapped in quotes. (ex: `For Me?` -> `"For Me?"`)
- Provided names do not have to be case sensitive (ex: `"Wanderers Troupe"` -> `wandererstroupe`).
- Running the command with no args will display a random image of Yoimiya. *(best nara :])*
- Provided names / subnames *can* be sanitized or converted to lowercase, but the program already sanitizes input. (cases like `"hu tao"` <-> `hutao` or `"For Me?"` <-> `forme`)

Examples:
    
- Random Yoimiya image: `aranara` 
- Skirk performing: `aranara skirk performing` 
- Azhdaha perplexed: `aranara azhdaha perplexed -n` 
- Random artifact from the **Viridescent Venerer** set: `aranara viridescentvenerer -a` / `aranara viridescentvenerer -a -r`
- Specific artifact type from the **Finale of the Deep Galleries** set: `aranara finaleofthedeepgalleries plume -a` 

Flags:
    
- `--npc` / `(-n)` 
- `--artifact` / `(-a)` 
- `--randomize` / `(-r) `
- `--width` / `(-w)`

Please see the following for:
- [All emojis *("Paimon's Paintings")*](https://genshin-impact.fandom.com/wiki/Paimon%27s_Paintings)
- [Emoji names](https://genshin-impact.fandom.com/wiki/Chat/Gallery)
- [Character names](https://genshin-impact.fandom.com/wiki/Character#Playable_Characters)
- [Artifact set names](https://genshin-impact.fandom.com/wiki/Artifact#Artifact_Sets)

## Why?
Some time ago, while clearing out my old bookmarks, I saw that there was a subreddit named [r/unixporn](http://reddit.com/r/unixporn) *(I know the name looks bad, it's just a subreddit where people share their setups/rices (typically on unix systems, hence the name.))* I hadn't visited the subreddit in a while *(since probably 2022)*, and so I decided to check it out before deleting the bookmark. As soon as the page loaded, I saw a pretty nice setup with a Kyurem in the terminal *(or was it a Salamence? I don't remember, the entire setup looked nord-themed & blue)*. I happen to be a fan of Pokémon *(or so I call myself while knowing full well I only like Pokémon Brick Bronze & the 3DS games)*, so I wanted to also be able to do this on my computer. After a few minutes of searching, and I find this tool called [pokeget-plus](https://github.com/Criomby/pokeget-plus). I found pokeget-plus to be super cool, and I wanted to see how it worked under the hood. While doing so, I thought to myself **"I could probably do something like this for another video game franchise I like."** And so, I decided to write this. This is my first project in any language other than Lua, so anyone with even the slightest Rust experience can probably tell the code isn't as good as it could be and has several flaws. Even while reading pokeget-plus as a guide, I've had to spend a few hours trying to understand how everything in Rust works. Still, this was fun to make *(especially after the headache of learning ownership & borrowing)* and was a pretty good way to learn how to use Rust.

## ⚠️ Known Issues
- On the **kitty** terminal, the program will sometimes output `Gi=31;OK^[\^[[?62;52;c` above the image. This is an issue with viuer.
- Windows is **NOT** supported. Maybe it will be in the future, but Windows is a terrible operating system and is an absolute displeasure to use.

## Credits
- [pokeget-plus](https://github.com/Criomby/pokeget-plus) was the main inspiration behind this.
- Artifact images were sourced from someone who shall remain nameless. 
- Chat emojis were sourced from the [Genshin Impact Wiki](https://genshin-impact.fandom.com/wiki/Genshin_Impact_Wiki). 
- All images used are property of [miHoYo / HoYoverse / COGNOSPHERE PTE. LTD.](https://en.wikipedia.org/wiki/MiHoYo). 
