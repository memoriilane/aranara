<div align="center">
  <img src="assets/images/aranara.png" width="128"/>
  
  # aranara
  ### Display Genshin Impact chat emojis & artifacts in your terminal.
</div>


## Features:
- Display character or NPC chat emojis in your terminal, as well as artifacts.
- Has all artifact images & chat emojis up until game version **Luna V** *(25 Feb 2026)*.
- The images are embedded into the binary, so nothing else has to be downloaded. Works fully offline. *(which is why the program is 120mb+)*

## Requirements:
- Your terminal must support either the kitty or iTerm2 graphics protocol.
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

Please see the following for emoji / character / artifact names:
- [Emoji names](https://genshin-impact.fandom.com/wiki/Chat#Emojis)
- [Character names](https://genshin-impact.fandom.com/wiki/Character#Playable_Characters)
- [Artifact names](https://genshin-impact.fandom.com/wiki/Artifact#Artifact_Sets)

## Why?
Some time ago, while clearing out my old bookmarks, I saw that there was a subreddit named [r/unixporn](http://reddit.com/r/unixporn) *(I know the name looks bad, it's just a subreddit where people share their setups/rices (typically on unix systems, hence the name.))* I hadn't visited the subreddit in a while *(since probably 2022)*, and so I decided to check it out before deleting the bookmark. As soon as the page loaded, I saw a pretty nice setup with a Kyurem in the terminal *(or was it a Salamence? I don't remember, the entire setup looked nord-themed & blue)*. I happen to be a fan of Pokémon *(or so I call myself while knowing full well I only like Pokémon Brick Bronze & the 3DS games)*, so I wanted to also be able to do this on my computer. After a few minutes of searching, and I find this tool called [pokeget-plus](https://github.com/Criomby/pokeget-plus). I found pokeget-plus to be super cool, and I wanted to see how it worked under the hood. While doing so, I thought to myself **"I could probably do something like this for another video game franchise I like."** And so, I decided to write this. This is my first project in any language other than Lua, so anyone with even the slightest Rust experience can probably tell the code isn't as good as it could be and has several flaws. Even while reading pokeget-plus as a guide, I've had to spend a few hours trying to understand how everything in Rust works. Still, this was fun to make *(especially after the headache of learning ownership & borrowing)* and was a pretty good way to learn how to use Rust.

## ⚠️ Known Issues
- On the **kitty** terminal, the program will sometimes output `Gi=31;OK^[\^[[?62;52;c` above the image. This is an issue with viuer.
- On the Windows Terminal/Powershell, the program will tell you: `Your terminal's image protocol is not supported.`. This is intentional, as the Windows Terminal does not truly support images. *(version 1.22 added support for Sixel, but it's terrible and has background artifacts)* Consider using [WezTerm](https://github.com/wezterm/wezterm) on Windows.

## Credits
- [pokeget-plus](https://github.com/Criomby/pokeget-plus) was the main inspiration behind this.
- Artifact images were sourced from someone who shall remain nameless. 
Chat emojis were sourced from the [Genshin Impact Wiki](https://genshin-impact.fandom.com/wiki/Genshin_Impact_Wiki). 
- All images used are property of [miHoYo / HoYoverse / COGNOSPHERE PTE. LTD.](https://en.wikipedia.org/wiki/MiHoYo). 