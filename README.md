# RUSTOBLAST SMASH

Original vector-art arcade shooter in Rust with Macroquad, inspired by the Astrosmash / Astroblast loop.

Windows / desktop local tool — not a browser / GitHub Pages demo. Soundtrack under `Music/` and SFX under `Audio/` ship with the repo.

## Run

From the project root:

```
cargo run
```

Optional playtest env vars:

- `RUSTOBLAST_WINDOWED=1` â€” start windowed instead of fullscreen
- `RUSTOBLAST_START_SCORE=5000` â€” start a new game at a given score band

Asset editor (separate crate):

```
cargo run --manifest-path "Macroquad-Serde Asset Engine/Cargo.toml"
```

## Controls

| Action | Key / Control |
| :--- | :--- |
| Move left / right | `A` `D` or arrows |
| Fire | `Space` or left click |
| Speed boost | Right click |
| Hyperspace | `Shift` or `Q` |
| Shield | `E` |
| Pause | `P` |
| Options | `ESC` (in game or via pause / menu `[O]`) |
| Toggle fullscreen | `Alt + Enter` |
| Menu | Mouse or arrows, Enter to confirm |

Pause also has Save / Load / Options / Main Menu. Options from pause returns to pause; options from play returns to play; options from the main menu returns to the menu.

## About

Enemies, ships, and terrain are JSON shape designs from `Macroquad-Serde Asset Engine/Assets/`. The game loads those files from the project root (or one folder up). Save files go in `SAVED GAMES - HIGHSCORES/`. Score never drops below 0. UFOs unlock at 5,000 points. Rocks, spinners, bombs, and poly land on the playfield floor (`just_landed`); guided missiles stay on that same line without a land-kill.

## Version History

2026-08-12
- README: describe as original work inspired by that loop (not an official remake).
- Landing uses `on_ground` / `just_landed` on the shared playfield floor (same Y as the player).
- Single enemy update per frame; relative asset paths only.
- UFO bomb cooldown (1.25s); score clamp at 0; P = pause, ESC = options.
- Spawn weights retuned (rock-led mix, UFOs as a late special).
- Windowed / start-score playtest env vars.


