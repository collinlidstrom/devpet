mod storage;

use devpet_core::{Action, Mood, PetState, Project, Species};
use devpet_pc::navigation::{self, Target};
use devpet_pc::sprites::{
    Sprite32, BODY as BODY_INDEX, BOT_BYTE, BYTE_CONCEPTS, CORE as CORE_INDEX, GHOST_BYTE,
    INK as INK_INDEX, ORB_BYTE, SPRITE_HEIGHT, SPRITE_WIDTH, TRANSPARENT,
};
use macroquad::prelude::*;

const W: f32 = 160.;
const H: f32 = 144.;
const BG: Color = Color::new(0.82, 0.87, 0.68, 1.);
const INK: Color = Color::new(0.10, 0.16, 0.12, 1.);
const BODY_TONE: Color = Color::new(0.59, 0.67, 0.42, 1.);
const CORE_TONE: Color = Color::new(0.32, 0.47, 0.20, 1.);
const CORE_BRIGHT: Color = Color::new(0.47, 0.61, 0.25, 1.);
const CORE_DIM: Color = Color::new(0.48, 0.52, 0.38, 1.);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Care,
    Code,
    Play,
    BugSquash,
    Profile,
    Evolution,
    SpriteGallery,
}

fn conf() -> Conf {
    Conf {
        window_title: "DevPet".into(),
        window_width: 640,
        window_height: 576,
        high_dpi: true,
        ..Default::default()
    }
}

fn txt(text: &str, x: f32, y: f32, size: u16) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: size,
            color: INK,
            ..Default::default()
        },
    );
}

fn wrapped_txt(text: &str, x: f32, y: f32, max_chars: usize, size: u16) {
    let mut line = String::new();
    let mut line_y = y;
    for word in text.split_whitespace() {
        let separator = usize::from(!line.is_empty());
        if line.len() + separator + word.len() > max_chars {
            txt(&line, x, line_y, size);
            line.clear();
            line_y += f32::from(size) + 2.;
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        txt(&line, x, line_y, size);
    }
}

fn bar(x: f32, y: f32, value: u8) {
    draw_rectangle_lines(x, y, 42., 5., 1., INK);
    draw_rectangle(x + 1., y + 1., 40. * f32::from(value) / 100., 3., INK);
}

fn choice(label: &str, y: f32, selected: bool) {
    if selected {
        draw_rectangle(10., y - 8., 140., 11., INK);
        draw_text_ex(
            label,
            14.,
            y,
            TextParams {
                font_size: 7,
                color: BG,
                ..Default::default()
            },
        );
    } else {
        txt(label, 14., y, 7);
    }
}

fn core_color(mood: Mood, time: f32) -> Color {
    match mood {
        Mood::Happy if (time * 4.) as i32 % 2 == 0 => CORE_BRIGHT,
        Mood::Coding if (time * 6.) as i32 % 2 == 0 => BODY_TONE,
        Mood::Sleeping => CORE_DIM,
        Mood::Sick if (time * 7.) as i32 % 3 == 0 => INK,
        _ => CORE_TONE,
    }
}

fn draw_sprite(sprite: &Sprite32, x: f32, y: f32, pixel_scale: f32, mood: Mood, time: f32) {
    for pixel_y in 0..SPRITE_HEIGHT {
        for pixel_x in 0..SPRITE_WIDTH {
            let color = match sprite.pixel(pixel_x, pixel_y) {
                TRANSPARENT => continue,
                INK_INDEX => INK,
                BODY_INDEX => BODY_TONE,
                CORE_INDEX => core_color(mood, time),
                _ => continue,
            };
            draw_rectangle(
                x + pixel_x as f32 * pixel_scale,
                y + pixel_y as f32 * pixel_scale,
                pixel_scale,
                pixel_scale,
                color,
            );
        }
    }
}

fn draw_egg(x: f32, y: f32, time: f32) {
    draw_ellipse(x + 16., y + 17., 11., 14., 0., INK);
    draw_ellipse(x + 16., y + 17., 8., 11., 0., BG);
    let crack = if (time * 3.) as i32 % 2 == 0 { 0. } else { 1. };
    draw_line(x + 11., y + 16., x + 15., y + 19. + crack, 1., INK);
    draw_line(x + 15., y + 19. + crack, x + 20., y + 15., 1., INK);
}

fn creature(x: f32, y: f32, pet: &PetState, time: f32) {
    if pet.species == Species::Egg {
        draw_egg(x, y, time);
        return;
    }

    let bob = if pet.mood != Mood::Sleeping && (time * 2.) as i32 % 2 != 0 {
        1.
    } else {
        0.
    };
    let sprite = match pet.species {
        Species::Bot => &BOT_BYTE,
        Species::Ghost => &GHOST_BYTE,
        _ => &ORB_BYTE,
    };
    draw_sprite(sprite, x, y + bob, 1., pet.mood, time);

    if pet.species == Species::Beast {
        draw_triangle(
            vec2(x + 5., y + 9. + bob),
            vec2(x + 10., y + 1. + bob),
            vec2(x + 14., y + 9. + bob),
            INK,
        );
        draw_triangle(
            vec2(x + 20., y + 9. + bob),
            vec2(x + 25., y + 1. + bob),
            vec2(x + 29., y + 9. + bob),
            INK,
        );
        draw_line(x + 5., y + 22. + bob, x + 1., y + 26. + bob, 2., INK);
        draw_line(x + 28., y + 22. + bob, x + 32., y + 26. + bob, 2., INK);
    }
}

fn nav(selection: &mut usize, count: usize) {
    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        *selection = (*selection + 1) % count;
    }
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        *selection = (*selection + count - 1) % count;
    }
}

fn menu_targets(screen: Screen) -> Vec<Target> {
    match screen {
        Screen::Home => (0..4).map(navigation::home).collect(),
        Screen::Care => (0..4)
            .map(|i| navigation::row(99. + i as f32 * 10.))
            .collect(),
        Screen::Code => (0..4)
            .map(|i| navigation::row(35. + i as f32 * 18.))
            .collect(),
        Screen::Profile => (0..3)
            .map(|i| navigation::row(112. + i as f32 * 11.))
            .collect(),
        Screen::Play => vec![navigation::row(110.)],
        _ => Vec::new(),
    }
}

#[macroquad::main(conf)]
async fn main() {
    // Preview mode never reads or writes the player's save.
    let preview_dir = std::env::var_os("DEVPET_UI_PREVIEW_DIR").map(std::path::PathBuf::from);
    let mut preview_frame = 0_usize;
    let mut pet = if let Some(dir) = &preview_dir {
        std::fs::create_dir_all(dir).expect("create preview directory");
        PetState {
            species: Species::Byte,
            age_minutes: 30,
            ..PetState::default()
        }
    } else {
        storage::load()
    };
    let mut screen = Screen::Home;
    let mut selection = 0_usize;
    let mut sprite_index = 0_usize;
    let mut last_tick = get_time();
    let mut game_start = 0.;
    let mut hits = 0_u8;
    let mut bug_x = 30.;
    let mut bug_y = 50.;
    let mut last_save = get_time();

    loop {
        if preview_dir.is_none() {
            if get_time() - last_tick >= 1. {
                pet.advance_minutes(1);
                last_tick = get_time();
            }
            if get_time() - last_save >= 15. {
                let _ = storage::save(&pet);
                last_save = get_time();
            }

            let pointer =
                navigation::logical_pointer(screen_width(), screen_height(), mouse_position());
            let clicked = is_mouse_button_pressed(MouseButton::Left);
            let targets = menu_targets(screen);
            let hovered =
                pointer.and_then(|p| targets.iter().position(|target| target.contains(p)));
            let mut confirm = is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Z)
                || is_key_pressed(KeyCode::Space);
            // Hover is visual only: a stationary pointer must not steal keyboard focus.
            if clicked {
                if let Some(index) = hovered {
                    selection = index;
                    confirm = true;
                }
            }
            if screen == Screen::Home {
                for (index, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4]
                    .iter()
                    .enumerate()
                {
                    if is_key_pressed(*key) {
                        selection = index;
                        confirm = true;
                    }
                }
            }
            let back = is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::X)
                || (clicked
                    && screen != Screen::Home
                    && screen != Screen::BugSquash
                    && pointer.is_some_and(|p| navigation::BACK.contains(p)));

            if back {
                match screen {
                    Screen::Home => {}
                    Screen::Evolution | Screen::SpriteGallery => {
                        screen = Screen::Profile;
                        selection = 0;
                    }
                    _ => {
                        screen = Screen::Home;
                        selection = 0;
                    }
                }
            }

            if !back {
                match screen {
                    Screen::Home => {
                        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                            selection = (selection + 1) % 4;
                        }
                        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                            selection = (selection + 3) % 4;
                        }
                        if confirm {
                            screen = match selection {
                                0 => Screen::Code,
                                1 => Screen::Care,
                                2 => Screen::Play,
                                _ => Screen::Profile,
                            };
                            selection = 0;
                        }
                    }
                    Screen::Care => {
                        nav(&mut selection, 4);
                        if confirm {
                            pet.apply(match selection {
                                0 => Action::Feed,
                                1 => Action::Coffee,
                                2 => Action::Rest,
                                _ => Action::Medicine,
                            });
                        }
                    }
                    Screen::Code => {
                        nav(&mut selection, 4);
                        if confirm {
                            pet.run_project(match selection {
                                0 => Project::FixBug,
                                1 => Project::BuildFeature,
                                2 => Project::Refactor,
                                _ => Project::ShipRelease,
                            });
                        }
                    }
                    Screen::Profile => {
                        nav(&mut selection, 3);
                        if confirm {
                            screen = match selection {
                                0 => Screen::Evolution,
                                1 => Screen::SpriteGallery,
                                _ => Screen::Home,
                            };
                            selection = 0;
                        }
                    }
                    Screen::Evolution => {
                        if confirm {
                            screen = Screen::Profile;
                            selection = 0;
                        }
                    }
                    Screen::SpriteGallery => {
                        if is_key_pressed(KeyCode::Right)
                            || is_key_pressed(KeyCode::D)
                            || (clicked && pointer.is_some_and(|p| navigation::NEXT.contains(p)))
                        {
                            sprite_index = (sprite_index + 1) % BYTE_CONCEPTS.len();
                        }
                        if is_key_pressed(KeyCode::Left)
                            || is_key_pressed(KeyCode::A)
                            || (clicked
                                && pointer.is_some_and(|p| navigation::PREVIOUS.contains(p)))
                        {
                            sprite_index =
                                (sprite_index + BYTE_CONCEPTS.len() - 1) % BYTE_CONCEPTS.len();
                        }
                        if confirm {
                            screen = Screen::Profile;
                            selection = 0;
                        }
                    }
                    Screen::Play => {
                        if confirm {
                            screen = Screen::BugSquash;
                            game_start = get_time();
                            hits = 0;
                            bug_x = 30.;
                            bug_y = 50.;
                        }
                    }
                    Screen::BugSquash => {
                        let elapsed = get_time() - game_start;
                        if elapsed >= 20. {
                            pet.bug_squash_reward(hits);
                            screen = Screen::Home;
                            selection = 0;
                        } else {
                            if clicked
                                && pointer.is_some_and(|(x, y)| {
                                    (x - bug_x).abs() < 8. && (y - bug_y).abs() < 8.
                                })
                            {
                                hits = hits.saturating_add(1);
                                bug_x = 15. + (f32::from(hits) * 37.) % 130.;
                                bug_y = 30. + (f32::from(hits) * 23.) % 70.;
                            }
                        }
                    }
                }
            }
        }
        // Two frames per case allow font/texture initialization to settle.
        let preview_case = preview_frame / 2;
        if preview_dir.is_some() {
            screen = match preview_case {
                0 => Screen::Home,
                1 => Screen::Care,
                2 => Screen::Code,
                3 => Screen::Play,
                4 => Screen::BugSquash,
                5 => Screen::Profile,
                6 => Screen::Evolution,
                _ => Screen::SpriteGallery,
            };
            sprite_index = preview_case.saturating_sub(7);
        }

        clear_background(BLACK);
        let scale = (screen_width() / W)
            .min(screen_height() / H)
            .floor()
            .max(1.);
        let offset_x = (screen_width() - W * scale) / 2.;
        let offset_y = (screen_height() - H * scale) / 2.;
        set_camera(&Camera2D {
            zoom: vec2(2. / W, -2. / H),
            target: vec2(W / 2., H / 2.),
            viewport: Some((
                offset_x as i32,
                offset_y as i32,
                (W * scale) as i32,
                (H * scale) as i32,
            )),
            ..Default::default()
        });
        draw_rectangle(0., 0., W, H, BG);

        let animation_time = if preview_dir.is_some() {
            0.0
        } else {
            get_time() as f32
        };
        match screen {
            Screen::Home => {
                txt("DEVPET", 5., 10., 7);
                txt(&format!("LV {}", pet.level), 135., 10., 7);
                creature(64., 25., &pet, animation_time);
                txt(
                    match pet.mood {
                        Mood::Coding => "Byte shipped code.",
                        Mood::Happy => "Byte is happy!",
                        Mood::Sleeping => "Byte is resting.",
                        Mood::Sick => "Byte needs care.",
                        _ => "Byte is vibing.",
                    },
                    42.,
                    68.,
                    7,
                );
                txt("HP", 5., 80., 6);
                bar(17., 77., pet.health);
                txt("EN", 65., 80., 6);
                bar(77., 77., pet.energy);
                draw_line(3., 90., 157., 90., 1., INK);
                for (index, action) in ["CODE", "CARE", "PLAY", "MENU"].iter().enumerate() {
                    let x = 5. + index as f32 * 39.;
                    if index == selection {
                        draw_rectangle(x - 2., 96., 36., 13., INK);
                        draw_text_ex(
                            action,
                            x,
                            105.,
                            TextParams {
                                font_size: 6,
                                color: BG,
                                ..Default::default()
                            },
                        );
                    } else {
                        txt(action, x, 105., 6);
                    }
                }
                txt(
                    &format!(
                        "XP {}  PROJECTS {}  BUGS {}",
                        pet.xp, pet.projects, pet.bugs_fixed
                    ),
                    5.,
                    123.,
                    6,
                );
                txt("CLICK / 1-4 / ARROWS + ENTER", 5., 137., 6);
            }
            Screen::Care => {
                txt("< CARE", 5., 10., 7);
                creature(64., 14., &pet, animation_time);
                txt(&format!("HEALTH {:3}", pet.health), 5., 57., 6);
                bar(55., 53., pet.health);
                txt(&format!("ENERGY {:3}", pet.energy), 5., 66., 6);
                bar(55., 62., pet.energy);
                txt(&format!("HAPPY  {:3}", pet.happiness), 5., 75., 6);
                bar(55., 71., pet.happiness);
                txt(&format!("FOCUS  {:3}", pet.focus), 5., 84., 6);
                bar(55., 80., pet.focus);
                for (index, action) in [
                    "FOOD +FULLNESS",
                    "COFFEE +FOCUS",
                    "SLEEP +ENERGY",
                    "MEDICINE +HEALTH",
                ]
                .iter()
                .enumerate()
                {
                    choice(action, 99. + index as f32 * 10., index == selection);
                }
            }
            Screen::Code => {
                txt("< CODE / CHOOSE PROJECT", 5., 10., 7);
                let labels = [
                    "FIX A BUG      +15 XP",
                    "BUILD FEATURE  +30 XP",
                    "REFACTOR       +20 XP",
                    "SHIP RELEASE   +50 XP",
                ];
                for (index, label) in labels.iter().enumerate() {
                    choice(label, 35. + index as f32 * 18., index == selection);
                }
                txt(
                    &format!("ENERGY {}  FOCUS {}", pet.energy, pet.focus),
                    10.,
                    116.,
                    7,
                );
                txt("UP/DOWN  ENTER   X=BACK", 10., 135., 6);
            }
            Screen::Profile => {
                txt("< BYTE PROFILE", 5., 10., 7);
                creature(64., 12., &pet, animation_time);
                txt(&format!("FORM {:?}", pet.species), 8., 54., 7);
                txt(&format!("PERSONALITY {:?}", pet.personality()), 8., 65., 7);
                txt(
                    &format!("AGE {}m  LV {}", pet.age_minutes, pet.level),
                    8.,
                    76.,
                    7,
                );
                txt(
                    &format!("XP {}  PROJECTS {}", pet.xp, pet.projects),
                    8.,
                    87.,
                    7,
                );
                txt(
                    &format!("BUGS {}  CARE MISS {}", pet.bugs_fixed, pet.care_mistakes),
                    8.,
                    98.,
                    7,
                );
                choice("EVOLUTION", 112., selection == 0);
                choice("BYTE CONCEPTS", 123., selection == 1);
                choice("BACK", 134., selection == 2);
            }
            Screen::Evolution => {
                txt("< EVOLUTION", 5., 10., 7);
                creature(64., 18., &pet, animation_time);
                if pet.species == Species::Egg {
                    txt("A SIGNAL IS FORMING...", 27., 72., 7);
                    txt("BYTE HATCHES AT AGE 3m", 22., 87., 7);
                } else if pet.is_evolved() {
                    txt(&format!("BYTE EVOLVED: {:?}", pet.species), 25., 72., 8);
                    txt(
                        match pet.species {
                            Species::Bot => "Built through focused creation.",
                            Species::Beast => "Raised through play and care.",
                            Species::Ghost => "Adapted through adversity.",
                            _ => "",
                        },
                        15.,
                        90.,
                        6,
                    );
                } else {
                    txt("HOW YOU RAISE BYTE", 28., 69., 7);
                    txt("DETERMINES ITS FUTURE.", 22., 81., 7);
                    txt(" ?       ?       ? ", 30., 103., 10);
                    txt("EVOLUTION AT AGE 240m", 20., 121., 7);
                }
                txt("ENTER/X = BACK", 42., 137., 6);
            }
            Screen::SpriteGallery => {
                txt("<", 14., 60., 12);
                txt(">", 139., 60., 12);
                let sprite = BYTE_CONCEPTS[sprite_index];
                txt("< BYTE CONCEPTS", 5., 10., 7);
                txt(
                    &format!("{}/{}", sprite_index + 1, BYTE_CONCEPTS.len()),
                    137.,
                    10.,
                    7,
                );
                draw_sprite(sprite, 48., 17., 2., Mood::Idle, animation_time);
                txt(sprite.name, 40., 92., 8);
                wrapped_txt(sprite.description, 10., 106., 47, 6);
                txt("< > BROWSE   ENTER/X BACK", 18., 137., 6);
            }
            Screen::Play => {
                txt("< PLAY", 5., 10., 7);
                txt("BUG SQUASH", 45., 45., 10);
                txt("20 SECOND MOUSE GAME", 30., 63., 7);
                txt("CLICK BUGS BEFORE THEY MOVE!", 17., 80., 6);
                choice("START", 110., true);
                txt("X = BACK", 55., 135., 6);
            }
            Screen::BugSquash => {
                let seconds_left = if preview_dir.is_some() {
                    20.
                } else {
                    (20. - (get_time() - game_start)).max(0.)
                };
                txt(
                    &format!("BUG SQUASH   {:02}s", seconds_left.ceil() as i32),
                    5.,
                    10.,
                    7,
                );
                txt(&format!("HITS {hits}"), 120., 10., 7);
                draw_circle(bug_x, bug_y, 6., INK);
                draw_line(bug_x - 8., bug_y - 6., bug_x + 8., bug_y + 6., 1., INK);
                draw_line(bug_x + 8., bug_y - 6., bug_x - 8., bug_y + 6., 1., INK);
                txt("CLICK THE BUG!", 48., 132., 7);
            }
        }

        // Resolve against the new screen after a transition, not stale targets.
        if let Some(point) = if preview_dir.is_some() {
            None
        } else {
            navigation::logical_pointer(screen_width(), screen_height(), mouse_position())
        } {
            let mut visible_targets = menu_targets(screen);
            if screen != Screen::Home && screen != Screen::BugSquash {
                visible_targets.push(navigation::BACK);
            }
            if screen == Screen::SpriteGallery {
                visible_targets.extend([navigation::PREVIOUS, navigation::NEXT]);
            }
            for target in visible_targets {
                if target.contains(point) {
                    draw_rectangle_lines(target.x, target.y, target.w, target.h, 1., CORE_BRIGHT);
                }
            }
        }
        set_default_camera();
        if let Some(dir) = &preview_dir {
            if preview_frame % 2 == 1 {
                let name = match preview_case {
                    0 => "home".to_string(),
                    1 => "care".to_string(),
                    2 => "code".to_string(),
                    3 => "play".to_string(),
                    4 => "bug-squash".to_string(),
                    5 => "profile".to_string(),
                    6 => "evolution".to_string(),
                    _ => format!("sprite-{:02}", sprite_index + 1),
                };
                let path = dir.join(format!("{preview_case:02}-{name}.png"));
                get_screen_data().export_png(path.to_str().expect("UTF-8 preview path"));
                println!("Captured {}", path.display());
                if preview_case == 6 + BYTE_CONCEPTS.len() {
                    break;
                }
            }
            preview_frame += 1;
        } else if is_key_pressed(KeyCode::Q) {
            let _ = storage::save(&pet);
            break;
        }
        next_frame().await;
    }
}
