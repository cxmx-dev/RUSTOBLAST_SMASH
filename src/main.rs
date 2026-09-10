use macroquad::prelude::*;

mod assets;
mod constants;
mod input;
mod scenes;
mod serde_helper;
mod spawn;
mod state_features;

use constants::*;
use state_features::windowing::WindowManager;

fn window_conf() -> Conf {
    let windowed = std::env::var("RUSTOBLAST_WINDOWED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: !windowed,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut window_manager = WindowManager::new();
    show_mouse(false);

    // Auto-correct working directory if running from src
    if !std::path::Path::new("Audio").exists() && std::path::Path::new("../Audio").exists() {
        if let Err(e) = std::env::set_current_dir("..") {
            println!("Failed to change directory to project root: {}", e);
        } else {
            println!("Automatically switched working directory to project root.");
        }
    }

    let shoot_laser_sound = macroquad::audio::load_sound("Audio/player-laser-bullets.wav").await;
    if shoot_laser_sound.is_err() {
        println!("Failed to load Audio/player-laser-bullets.wav");
    }
    let shoot_laser_sound = shoot_laser_sound.ok();

    let shoot_sound = macroquad::audio::load_sound("Audio/player_bullets-laser_chirp.wav").await;
    if shoot_sound.is_err() {
        println!("Failed to load Audio/player_bullets-laser_chirp.wav");
    }
    let shoot_sound = shoot_sound.ok();

    let ufo_sound = macroquad::audio::load_sound("Audio/ufo.wav").await;
    if ufo_sound.is_err() {
        println!("Failed to load Audio/ufo.wav");
    }
    let ufo_sound = ufo_sound.ok();

    let guided_missile_sound = macroquad::audio::load_sound("Audio/guided_missile.wav").await;
    if guided_missile_sound.is_err() {
        println!("Failed to load Audio/guided_missile.wav");
    }
    let guided_missile_sound = guided_missile_sound.ok();

    let guided_missile_ground_sound = macroquad::audio::load_sound("Audio/deep-pulse.wav").await;
    if guided_missile_ground_sound.is_err() {
        println!("Failed to load Audio/deep-pulse.wav");
    }
    let guided_missile_ground_sound = guided_missile_ground_sound.ok();

    let big_spinner_sound = macroquad::audio::load_sound("Audio/big_ass_bomb.wav").await;
    if big_spinner_sound.is_err() {
        println!("Failed to load Audio/big_ass_bomb.wav");
    }
    let big_spinner_sound = big_spinner_sound.ok();

    let small_spinner_sound = macroquad::audio::load_sound("Audio/small_ass_bomb.wav").await;
    if small_spinner_sound.is_err() {
        println!("Failed to load Audio/small_ass_bomb.wav");
    }
    let small_spinner_sound = small_spinner_sound.ok();

    let multiplier_sound = macroquad::audio::load_sound("Audio/multiplier.wav").await;
    if multiplier_sound.is_err() {
        println!("Failed to load Audio/multiplier.wav");
    }
    let multiplier_sound = multiplier_sound.ok();

    let rock_explode_sound = macroquad::audio::load_sound("Audio/ass_pop.wav").await;
    if rock_explode_sound.is_err() {
        println!("Failed to load Audio/ass_pop.wav");
    }
    let rock_explode_sound = rock_explode_sound.ok();

    let spinner_explode_sound = macroquad::audio::load_sound("Audio/explode.wav").await;
    if spinner_explode_sound.is_err() {
        println!("Failed to load Audio/explode.wav");
    }
    let spinner_explode_sound = spinner_explode_sound.ok();

    let big_spinner_explode_sound = macroquad::audio::load_sound("Audio/big_explode.wav").await;
    if big_spinner_explode_sound.is_err() {
        println!("Failed to load Audio/big_explode.wav");
    }
    let big_spinner_explode_sound = big_spinner_explode_sound.ok();

    let player_die_sound = macroquad::audio::load_sound("Audio/player_die.wav").await;
    if player_die_sound.is_err() {
        println!("Failed to load Audio/player_die.wav");
    }
    let player_die_sound = player_die_sound.ok();

    let game_over_sound_1 = macroquad::audio::load_sound("Audio/1_Game-Over.wav").await;
    if game_over_sound_1.is_err() {
        println!("Failed to load Audio/1_Game-Over.wav");
    }
    let game_over_sound_1 = game_over_sound_1.ok();

    let game_over_sound_2 = macroquad::audio::load_sound("Audio/2_Game-Over.wav").await;
    if game_over_sound_2.is_err() {
        println!("Failed to load Audio/2_Game-Over.wav");
    }
    let game_over_sound_2 = game_over_sound_2.ok();

    let game_over_sound_3 = macroquad::audio::load_sound("Audio/3_Game-Over.wav").await;
    if game_over_sound_3.is_err() {
        println!("Failed to load Audio/3_Game-Over.wav");
    }
    let game_over_sound_3 = game_over_sound_3.ok();

    // Main Menu Speech
    let main_menu_speech = macroquad::audio::load_sound("Audio/Main-Menu-speech.wav").await;
    if main_menu_speech.is_err() {
        println!("Failed to load Audio/Main-Menu-speech.wav");
    }
    let main_menu_speech = main_menu_speech.ok();

    let mut startup_timer = 0.0f32;
    let mut startup_speech_played = false;

    let mut sfx_volume = 0.5f32; // Default 50%
    let mut music_volume = 1.0f32; // Default 100%
    let mut master_volume = 1.0f32; // Default 100%

    // Load Music
    // Load Music
    let menu_music = macroquad::audio::load_sound("Music/Main_Menu-4571205M45H-hit.wav")
        .await
        .ok();

    let track1 = macroquad::audio::load_sound("Music/1-4571205M45H-hit.wav")
        .await
        .ok();
    let track2 = macroquad::audio::load_sound("Music/2-4571205M45H-hit.wav")
        .await
        .ok();
    let track3 = macroquad::audio::load_sound("Music/3-4571205M45H-hit.wav")
        .await
        .ok();

    let mut game_playlist = Vec::new();
    if let Some(t) = track1 {
        game_playlist.push(t);
    }
    if let Some(t) = track2 {
        game_playlist.push(t);
    }
    if let Some(t) = track3 {
        game_playlist.push(t);
    }

    let mut current_track_index = 0;
    let mut last_prev_press_time = -10.0f64; // For double-tap detection

    // Play Menu Music Loop
    if let Some(music) = &menu_music {
        let params = macroquad::audio::PlaySoundParams {
            looped: true,
            volume: music_volume * master_volume,
        };
        macroquad::audio::play_sound(music, params);
    }

    enum AppState {
        MainMenu(scenes::menu::MainMenuState),
        Game(scenes::game::GameState),
        GameOver(scenes::game_over::GameOverState),
        Pause(scenes::pause::PauseState),
        Options(scenes::options::OptionsState),
    }

    // Start at Main Menu
    let mut app_state = AppState::MainMenu(scenes::menu::MainMenuState::new());
    enum AfterOptions {
        MainMenu,
        Pause,
        Game,
    }
    let mut after_options = AfterOptions::MainMenu;

    loop {
        // Global Systems
        window_manager.update();

        // State Machine Switch
        let mut next_state: Option<AppState> = None;

        match &mut app_state {
            AppState::MainMenu(menu_state) => {
                // Update Startup Speech Timer
                if !startup_speech_played {
                    startup_timer += get_frame_time();
                    if startup_timer >= 3.0 {
                        if let Some(speech) = &main_menu_speech {
                            let params = macroquad::audio::PlaySoundParams {
                                looped: false,
                                volume: sfx_volume * master_volume,
                            };
                            macroquad::audio::play_sound(speech, params);
                        }
                        startup_speech_played = true;
                    }
                }

                let action = menu_state.update();
                menu_state.draw();

                match action {
                    scenes::menu::MainMenuAction::NewGame => {
                        // Play speech again on start
                        if let Some(speech) = &main_menu_speech {
                            let params = macroquad::audio::PlaySoundParams {
                                looped: false,
                                volume: sfx_volume * master_volume,
                            };
                            macroquad::audio::play_sound(speech, params);
                        }

                        // Stop Menu Music, Start Game Music
                        if let Some(music) = &menu_music {
                            macroquad::audio::stop_sound(music);
                        }

                        // Start first track of playlist
                        if !game_playlist.is_empty() {
                            current_track_index = 0;
                            let params = macroquad::audio::PlaySoundParams {
                                looped: true, // We loop individual tracks, user manually skips? Or we loop list?
                                // User said: "loop the playlist back to track 1... after the 3rd track plays"
                                // Since we can't easily detect end, we loop the *current* track until user skips?
                                // User said: "ensure... loop forever... on end".
                                // Actually, for the playlist, the user might expect auto-advance.
                                // But without duration, auto-advance is hard.
                                // I'll set looped: true so at least silence doesn't happen.
                                volume: music_volume * master_volume,
                            };
                            macroquad::audio::play_sound(
                                &game_playlist[current_track_index],
                                params,
                            );
                        }

                        next_state = Some(AppState::Game(scenes::game::GameState::new(
                            shoot_sound.clone(),
                            shoot_laser_sound.clone(),
                            ufo_sound.clone(),
                            guided_missile_sound.clone(),
                            guided_missile_ground_sound.clone(),
                            big_spinner_sound.clone(),
                            small_spinner_sound.clone(),
                            multiplier_sound.clone(),
                            rock_explode_sound.clone(),
                            spinner_explode_sound.clone(),
                            big_spinner_explode_sound.clone(),
                            player_die_sound.clone(),
                            master_volume,
                            sfx_volume,
                            music_volume,
                        )));
                    }
                    scenes::menu::MainMenuAction::Options => {
                        after_options = AfterOptions::MainMenu;
                        next_state = Some(AppState::Options(scenes::options::OptionsState::new(
                            master_volume,
                            sfx_volume,
                            music_volume,
                            None, // No game state from main menu
                        )));
                    }
                    scenes::menu::MainMenuAction::LoadGame => {
                        if let Some(loaded_state) = scenes::game::GameState::load(
                            shoot_sound.clone(),
                            shoot_laser_sound.clone(),
                            ufo_sound.clone(),
                            guided_missile_sound.clone(),
                            guided_missile_ground_sound.clone(),
                            big_spinner_sound.clone(),
                            small_spinner_sound.clone(),
                            multiplier_sound.clone(),
                            rock_explode_sound.clone(),
                            spinner_explode_sound.clone(),
                            big_spinner_explode_sound.clone(),
                            player_die_sound.clone(),
                            master_volume,
                            sfx_volume,
                            music_volume,
                        ) {
                            next_state = Some(AppState::Game(loaded_state));
                        }
                    }
                    scenes::menu::MainMenuAction::Quit => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            AppState::Game(game_state) => {
                let action = game_state.update();
                game_state.draw();

                let mut play_new_track = false;

                // Playlist Controls
                if is_key_pressed(KeyCode::RightBracket) {
                    // Next Track
                    if !game_playlist.is_empty() {
                        macroquad::audio::stop_sound(&game_playlist[current_track_index]);
                        current_track_index = (current_track_index + 1) % game_playlist.len();
                        play_new_track = true;
                    }
                }

                if is_key_pressed(KeyCode::LeftBracket) {
                    // Prev / Restart
                    if !game_playlist.is_empty() {
                        let now = get_time();
                        if now - last_prev_press_time < 1.0 {
                            // Double press: Go Previous
                            macroquad::audio::stop_sound(&game_playlist[current_track_index]);
                            if current_track_index == 0 {
                                current_track_index = game_playlist.len() - 1;
                            } else {
                                current_track_index -= 1;
                            }
                        } else {
                            // Single press (so far): Restart current
                            macroquad::audio::stop_sound(&game_playlist[current_track_index]);
                            // Index stays same
                        }
                        last_prev_press_time = now;
                        play_new_track = true;
                    }
                }

                if play_new_track && !game_playlist.is_empty() {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: true,
                        volume: music_volume * master_volume,
                    };
                    macroquad::audio::play_sound(&game_playlist[current_track_index], params);
                }

                match action {
                    scenes::game::GameAction::Pause => {
                        next_state = Some(AppState::Pause(scenes::pause::PauseState::new(
                            game_state.clone(),
                        )));
                    }
                    scenes::game::GameAction::Options => {
                        after_options = AfterOptions::Game;
                        next_state = Some(AppState::Options(scenes::options::OptionsState::new(
                            master_volume,
                            sfx_volume,
                            music_volume,
                            Some(game_state.clone()),
                        )));
                    }
                    scenes::game::GameAction::GameOver => {
                        // Stop all gameplay sounds
                        game_state.stop_all_sounds();

                        // Stop Game Music
                        if !game_playlist.is_empty() {
                            macroquad::audio::stop_sound(&game_playlist[current_track_index]);
                        }

                        let chosen_sound = match rand::gen_range(0, 3) {
                            0 => game_over_sound_1.clone(),
                            1 => game_over_sound_2.clone(),
                            _ => game_over_sound_3.clone(),
                        };

                        next_state =
                            Some(AppState::GameOver(scenes::game_over::GameOverState::new(
                                game_state.score,
                                chosen_sound,
                                sfx_volume,
                                music_volume,
                            )));
                    }
                    _ => {}
                }
            }
            AppState::Pause(pause_state) => {
                let action = pause_state.update();
                pause_state.draw();

                match action {
                    scenes::pause::PauseAction::Resume => {
                        // Restore from Memory
                        if let Some(state) = pause_state.game_state.take() {
                            next_state = Some(AppState::Game(state));
                        } else {
                            // Fallback (shouldn't happen if initialized correctly)
                            if let Some(loaded_state) = scenes::game::GameState::load(
                                shoot_sound.clone(),
                                shoot_laser_sound.clone(),
                                ufo_sound.clone(),
                                guided_missile_sound.clone(),
                                guided_missile_ground_sound.clone(),
                                big_spinner_sound.clone(),
                                small_spinner_sound.clone(),
                                multiplier_sound.clone(),
                                rock_explode_sound.clone(),
                                spinner_explode_sound.clone(),
                                big_spinner_explode_sound.clone(),
                                player_die_sound.clone(),
                                master_volume,
                                sfx_volume,
                                music_volume,
                            ) {
                                next_state = Some(AppState::Game(loaded_state));
                            } else {
                                next_state = Some(AppState::Game(scenes::game::GameState::new(
                                    shoot_sound.clone(),
                                    shoot_laser_sound.clone(),
                                    ufo_sound.clone(),
                                    guided_missile_sound.clone(),
                                    guided_missile_ground_sound.clone(),
                                    big_spinner_sound.clone(),
                                    small_spinner_sound.clone(),
                                    multiplier_sound.clone(),
                                    rock_explode_sound.clone(),
                                    spinner_explode_sound.clone(),
                                    big_spinner_explode_sound.clone(),
                                    player_die_sound.clone(),
                                    master_volume,
                                    sfx_volume,
                                    music_volume,
                                )));
                            }
                        }
                    }
                    scenes::pause::PauseAction::Save => {
                        if let Some(state) = &mut pause_state.game_state {
                            state.save();
                        }
                    }
                    scenes::pause::PauseAction::Load => {
                        if let Some(loaded_state) = scenes::game::GameState::load(
                            shoot_sound.clone(),
                            shoot_laser_sound.clone(),
                            ufo_sound.clone(),
                            guided_missile_sound.clone(),
                            guided_missile_ground_sound.clone(),
                            big_spinner_sound.clone(),
                            small_spinner_sound.clone(),
                            multiplier_sound.clone(),
                            rock_explode_sound.clone(),
                            spinner_explode_sound.clone(),
                            big_spinner_explode_sound.clone(),
                            player_die_sound.clone(),
                            master_volume,
                            sfx_volume,
                            music_volume,
                        ) {
                            next_state = Some(AppState::Game(loaded_state));
                        }
                    }
                    scenes::pause::PauseAction::Options => {
                        after_options = AfterOptions::Pause;

                        // TAKE game_state from pause_state
                        let state = pause_state.game_state.take();

                        next_state = Some(AppState::Options(scenes::options::OptionsState::new(
                            master_volume,
                            sfx_volume,
                            music_volume,
                            state, // Pass it to Options
                        )));
                    }
                    scenes::pause::PauseAction::MainMenu => {
                        // Stop Game Music, Start Menu Music
                        if !game_playlist.is_empty() {
                            macroquad::audio::stop_sound(&game_playlist[current_track_index]);
                        }
                        if let Some(music) = &menu_music {
                            let params = macroquad::audio::PlaySoundParams {
                                looped: true,
                                volume: music_volume * master_volume,
                            };
                            macroquad::audio::play_sound(music, params);
                        }
                        next_state = Some(AppState::MainMenu(scenes::menu::MainMenuState::new()));
                    }
                    scenes::pause::PauseAction::Quit => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            AppState::GameOver(game_over_state) => {
                let action = game_over_state.update();
                game_over_state.draw();

                match action {
                    scenes::game_over::GameOverAction::Replay => {
                        // Stop Game Over sound
                        if let Some(sound) = &game_over_state.game_over_sound {
                            macroquad::audio::stop_sound(sound);
                        }

                        // Restart Playlist
                        if !game_playlist.is_empty() {
                            current_track_index = 0;
                            let params = macroquad::audio::PlaySoundParams {
                                looped: true,
                                volume: music_volume,
                            };
                            macroquad::audio::play_sound(
                                &game_playlist[current_track_index],
                                params,
                            );
                        }

                        next_state = Some(AppState::Game(scenes::game::GameState::new(
                            shoot_sound.clone(),
                            shoot_laser_sound.clone(),
                            ufo_sound.clone(),
                            guided_missile_sound.clone(),
                            guided_missile_ground_sound.clone(),
                            big_spinner_sound.clone(),
                            small_spinner_sound.clone(),
                            multiplier_sound.clone(),
                            rock_explode_sound.clone(),
                            spinner_explode_sound.clone(),
                            big_spinner_explode_sound.clone(),
                            player_die_sound.clone(),
                            master_volume,
                            sfx_volume,
                            music_volume,
                        )));
                    }
                    scenes::game_over::GameOverAction::MainMenu => {
                        if let Some(sound) = &game_over_state.game_over_sound {
                            macroquad::audio::stop_sound(sound);
                        }
                        // Play Menu Music
                        if let Some(music) = &menu_music {
                            let params = macroquad::audio::PlaySoundParams {
                                looped: true,
                                volume: music_volume * master_volume,
                            };
                            macroquad::audio::play_sound(music, params);
                        }
                        next_state = Some(AppState::MainMenu(scenes::menu::MainMenuState::new()));
                    }
                    scenes::game_over::GameOverAction::Quit => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            AppState::Options(options_state) => {
                let action = options_state.update();
                options_state.draw();

                if let scenes::options::OptionsAction::Back(
                    new_master,
                    new_sfx,
                    new_music,
                    preserved_state,
                ) = action
                {
                    // Update global settings
                    master_volume = new_master;
                    sfx_volume = new_sfx;
                    music_volume = new_music;

                    match after_options {
                        AfterOptions::Pause | AfterOptions::Game => {
                            if !game_playlist.is_empty() {
                                macroquad::audio::stop_sound(&game_playlist[current_track_index]);
                                let params = macroquad::audio::PlaySoundParams {
                                    looped: true,
                                    volume: music_volume * master_volume,
                                };
                                macroquad::audio::play_sound(
                                    &game_playlist[current_track_index],
                                    params,
                                );
                            }
                        }
                        AfterOptions::MainMenu => {
                            if let Some(music) = &menu_music {
                                macroquad::audio::stop_sound(music);
                                let params = macroquad::audio::PlaySoundParams {
                                    looped: true,
                                    volume: music_volume,
                                };
                                macroquad::audio::play_sound(music, params);
                            }
                        }
                    }

                    if matches!(after_options, AfterOptions::Pause | AfterOptions::Game) {
                        let mut state = if let Some(saved) = preserved_state {
                            saved // We got our state back! perfect.
                        } else {
                            // Fallback
                            if let Some(loaded) = scenes::game::GameState::load(
                                shoot_sound.clone(),
                                shoot_laser_sound.clone(),
                                ufo_sound.clone(),
                                guided_missile_sound.clone(),
                                guided_missile_ground_sound.clone(),
                                big_spinner_sound.clone(),
                                small_spinner_sound.clone(),
                                multiplier_sound.clone(),
                                rock_explode_sound.clone(),
                                spinner_explode_sound.clone(),
                                big_spinner_explode_sound.clone(),
                                player_die_sound.clone(),
                                master_volume,
                                sfx_volume,
                                music_volume,
                            ) {
                                loaded
                            } else {
                                scenes::game::GameState::new(
                                    shoot_sound.clone(),
                                    shoot_laser_sound.clone(),
                                    ufo_sound.clone(),
                                    guided_missile_sound.clone(),
                                    guided_missile_ground_sound.clone(),
                                    big_spinner_sound.clone(),
                                    small_spinner_sound.clone(),
                                    multiplier_sound.clone(),
                                    rock_explode_sound.clone(),
                                    spinner_explode_sound.clone(),
                                    big_spinner_explode_sound.clone(),
                                    player_die_sound.clone(),
                                    master_volume,
                                    sfx_volume,
                                    music_volume,
                                )
                            }
                        };

                        // Update volume on existing/new state
                        state.master_volume = master_volume;
                        state.sfx_volume = sfx_volume;
                        state.music_volume = music_volume;
                        state.shoot_sound = shoot_sound.clone(); // re-inject sound handle
                        state.ufo_sound = ufo_sound.clone();
                        state.guided_missile_sound = guided_missile_sound.clone();
                        state.guided_missile_ground_sound = guided_missile_ground_sound.clone();
                        state.big_spinner_sound = big_spinner_sound.clone();
                        state.small_spinner_sound = small_spinner_sound.clone();
                        state.multiplier_sound = multiplier_sound.clone();
                        state.rock_explode_sound = rock_explode_sound.clone();
                        state.spinner_explode_sound = spinner_explode_sound.clone();
                        state.big_spinner_explode_sound = big_spinner_explode_sound.clone();
                        state.player_die_sound = player_die_sound.clone();

                        next_state = if matches!(after_options, AfterOptions::Game) {
                            Some(AppState::Game(state))
                        } else {
                            Some(AppState::Pause(scenes::pause::PauseState::new(state)))
                        };
                    } else {
                        next_state = Some(AppState::MainMenu(scenes::menu::MainMenuState::new()));
                    }
                }
            }
        }

        if let Some(new_state) = next_state {
            app_state = new_state;
        }

        // Draw Custom Cursor (Only if not in Game state)
        let show_cursor = match app_state {
            AppState::Game(_) => false,
            _ => true,
        };

        if show_cursor {
            let (mx, my) = mouse_position();
            let size = CURSOR_SIZE;

            // Draw a simple arrow cursor
            draw_triangle(
                vec2(mx, my),
                vec2(mx, my + size),
                vec2(mx + size * 0.7, my + size * 0.7),
                PALETTE_CURSOR_ORANGE,
            );
        }

        next_frame().await
    }
}
