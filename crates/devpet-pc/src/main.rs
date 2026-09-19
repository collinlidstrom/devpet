mod storage;

use devpet_core::{Action, Mood, PetState, Project, Species};
use devpet_pc::navigation::{self, Target};
use devpet_pc::sprites::{
    Sprite32, BODY as BODY_INDEX, BOT_BYTE, BYTE_CONCEPTS, CORE as CORE_INDEX, GHOST_BYTE,
    INK as INK_INDEX, ORB_BYTE, SPRITE_HEIGHT, SPRITE_WIDTH, TRANSPARENT,
};
use devpet_pc::theme::{self, Palette, Theme};
use macroquad::prelude::*;
use std::path::PathBuf;

const W: f32 = 160.;
const H: f32 = 144.;
const CRITICAL_STAT: u8 = 25;
const HOME_ACTIONS: [&str; 4] = ["CODE", "CARE", "PLAY", "MENU"];
const PROFILE_ACTIONS: [&str; 4] = ["EVOLUTION", "BYTE CONCEPTS", "SETTINGS", "BACK"];
const SETTINGS_CHOICES: [Theme; 2] = [Theme::PocketColor, Theme::ClassicLcd];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Care,
    Code,
    Play,
    BugSquash,
    Profile,
    Evolution,
    Settings,
    SpriteGallery,
}

#[derive(Clone)]
struct PreviewCapture {
    name: String,
    screen: Screen,
    pet: PetState,
    selection: usize,
    sprite_index: usize,
}

#[derive(Clone, Copy)]
enum StatKind {
    Health,
    Energy,
    Happiness,
    Focus,
}

#[derive(Clone, Copy)]
struct SpriteRender {
    theme: Theme,
    species: Species,
    mood: Mood,
    time: f32,
}

impl PreviewCapture {
    fn screen(name: impl Into<String>, screen: Screen, pet: PetState) -> Self {
        Self {
            name: name.into(),
            screen,
            pet,
            selection: 0,
            sprite_index: 0,
        }
    }

    fn with_selection(mut self, selection: usize) -> Self {
        self.selection = selection;
        self
    }

    fn with_sprite(mut self, sprite_index: usize) -> Self {
        self.sprite_index = sprite_index;
        self
    }
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

fn txt(text: &str, x: f32, y: f32, size: u16, color: Color) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: size,
            color,
            ..Default::default()
        },
    );
}

fn wrapped_txt(text: &str, x: f32, y: f32, max_chars: usize, size: u16, color: Color) {
    let mut line = String::new();
    let mut line_y = y;
    for word in text.split_whitespace() {
        let separator = usize::from(!line.is_empty());
        if line.len() + separator + word.len() > max_chars {
            txt(&line, x, line_y, size, color);
            line.clear();
            line_y += f32::from(size) + 2.;
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        txt(&line, x, line_y, size, color);
    }
}

fn stat_color(kind: StatKind, palette: &Palette) -> Color {
    match kind {
        StatKind::Health => palette.health,
        StatKind::Energy => palette.energy,
        StatKind::Happiness => palette.happiness,
        StatKind::Focus => palette.focus,
    }
}

fn bar(x: f32, y: f32, value: u8, kind: StatKind, palette: &Palette) {
    draw_rectangle_lines(x, y, 42., 5., 1., palette.ink);
    draw_rectangle(x + 1., y + 1., 40., 3., palette.bar_track);
    if value > 0 {
        draw_rectangle(
            x + 1.,
            y + 1.,
            40. * f32::from(value) / 100.,
            3.,
            stat_color(kind, palette),
        );
    }
    if value <= CRITICAL_STAT {
        txt("!", x + 45., y + 5., 7, palette.warning);
    }
}

fn choice(label: &str, y: f32, selected: bool, palette: &Palette) {
    if selected {
        draw_rectangle(10., y - 8., 140., 11., palette.selected_fill);
        txt(label, 14., y, 7, palette.selected_text);
    } else {
        txt(label, 14., y, 7, palette.ink);
    }
}

fn core_color(theme: Theme, mood: Mood, species: Species, time: f32) -> Color {
    let palette = theme.palette();
    let creature = theme.creature_palette(species);
    match mood {
        Mood::Happy if (time * 4.) as i32 % 2 == 0 => creature.core_bright,
        Mood::Coding if (time * 6.) as i32 % 2 == 0 => creature.body,
        Mood::Sleeping => creature.core_dim,
        Mood::Sick if (time * 7.) as i32 % 3 == 0 => palette.warning,
        _ => creature.core,
    }
}

fn draw_sprite(sprite: &Sprite32, x: f32, y: f32, pixel_scale: f32, render: SpriteRender) {
    let palette = render.theme.palette();
    let creature = render.theme.creature_palette(render.species);
    for pixel_y in 0..SPRITE_HEIGHT {
        for pixel_x in 0..SPRITE_WIDTH {
            let color = match sprite.pixel(pixel_x, pixel_y) {
                TRANSPARENT => continue,
                INK_INDEX => palette.ink,
                BODY_INDEX => creature.body,
                CORE_INDEX => core_color(render.theme, render.mood, render.species, render.time),
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

fn draw_egg(x: f32, y: f32, time: f32, palette: &Palette) {
    draw_ellipse(x + 16., y + 17., 11., 14., 0., palette.ink);
    draw_ellipse(x + 16., y + 17., 8., 11., 0., palette.egg_fill);
    let crack = if (time * 3.) as i32 % 2 == 0 { 0. } else { 1. };
    draw_line(x + 11., y + 16., x + 15., y + 19. + crack, 1., palette.ink);
    draw_line(x + 15., y + 19. + crack, x + 20., y + 15., 1., palette.ink);
}

fn creature(x: f32, y: f32, pet: &PetState, time: f32, theme: Theme) {
    let palette = theme.palette();
    if pet.species == Species::Egg {
        draw_egg(x, y, time, &palette);
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
    draw_sprite(
        sprite,
        x,
        y + bob,
        1.,
        SpriteRender {
            theme,
            species: pet.species,
            mood: pet.mood,
            time,
        },
    );

    if pet.species == Species::Beast {
        draw_triangle(
            vec2(x + 5., y + 9. + bob),
            vec2(x + 10., y + 1. + bob),
            vec2(x + 14., y + 9. + bob),
            palette.ink,
        );
        draw_triangle(
            vec2(x + 20., y + 9. + bob),
            vec2(x + 25., y + 1. + bob),
            vec2(x + 29., y + 9. + bob),
            palette.ink,
        );
        draw_line(
            x + 5.,
            y + 22. + bob,
            x + 1.,
            y + 26. + bob,
            2.,
            palette.ink,
        );
        draw_line(
            x + 28.,
            y + 22. + bob,
            x + 32.,
            y + 26. + bob,
            2.,
            palette.ink,
        );
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
        Screen::Profile => (0..4)
            .map(|i| navigation::row(101. + i as f32 * 11.))
            .collect(),
        Screen::Settings => (0..3)
            .map(|i| navigation::row(97. + i as f32 * 11.))
            .collect(),
        Screen::Play => vec![navigation::row(110.)],
        _ => Vec::new(),
    }
}

fn preview_byte() -> PetState {
    PetState {
        species: Species::Byte,
        age_minutes: 30,
        health: 82,
        energy: 74,
        happiness: 88,
        focus: 68,
        fullness: 72,
        xp: 48,
        level: 3,
        projects: 4,
        bugs_fixed: 7,
        play_sessions: 2,
        feeds: 3,
        rests: 1,
        mood: Mood::Idle,
        ..PetState::default()
    }
}

fn preview_evolved(species: Species) -> PetState {
    let mut pet = preview_byte();
    pet.species = species;
    pet.age_minutes = 360;
    pet.level = 6;
    match species {
        Species::Bot => {
            pet.projects = 18;
            pet.bugs_fixed = 21;
            pet.focus = 91;
            pet.energy = 78;
            pet.mood = Mood::Coding;
        }
        Species::Beast => {
            pet.play_sessions = 16;
            pet.feeds = 12;
            pet.happiness = 95;
            pet.energy = 83;
            pet.mood = Mood::Happy;
        }
        Species::Ghost => {
            pet.care_mistakes = 5;
            pet.health = 42;
            pet.focus = 58;
            pet.mood = Mood::Sick;
        }
        _ => {}
    }
    pet
}

fn preview_warning_pet() -> PetState {
    PetState {
        species: Species::Byte,
        age_minutes: 180,
        health: 18,
        energy: 14,
        happiness: 20,
        focus: 19,
        fullness: 16,
        xp: 72,
        level: 4,
        bugs_fixed: 9,
        projects: 6,
        care_mistakes: 2,
        mood: Mood::Sick,
        ..PetState::default()
    }
}

fn preview_captures(theme: Theme) -> Vec<PreviewCapture> {
    let byte = preview_byte();
    let mut captures = vec![
        PreviewCapture::screen("home", Screen::Home, byte.clone()),
        PreviewCapture::screen("care", Screen::Care, byte.clone()),
        PreviewCapture::screen("code", Screen::Code, byte.clone()),
        PreviewCapture::screen("play", Screen::Play, byte.clone()),
        PreviewCapture::screen("bug-squash", Screen::BugSquash, byte.clone()),
        PreviewCapture::screen("profile", Screen::Profile, byte.clone()),
        PreviewCapture::screen("evolution", Screen::Evolution, byte.clone()),
        PreviewCapture::screen("settings", Screen::Settings, byte.clone())
            .with_selection(theme.settings_index()),
    ];
    for index in 0..BYTE_CONCEPTS.len() {
        captures.push(
            PreviewCapture::screen(
                format!("sprite-{:02}", index + 1),
                Screen::SpriteGallery,
                byte.clone(),
            )
            .with_sprite(index),
        );
    }
    captures.extend([
        PreviewCapture::screen("bot", Screen::Profile, preview_evolved(Species::Bot)),
        PreviewCapture::screen("beast", Screen::Profile, preview_evolved(Species::Beast)),
        PreviewCapture::screen("ghost", Screen::Profile, preview_evolved(Species::Ghost)),
        PreviewCapture::screen("low-stat-warning", Screen::Care, preview_warning_pet()),
    ]);
    captures
}

#[macroquad::main(conf)]
async fn main() {
    let preview_dir = std::env::var_os("DEVPET_UI_PREVIEW_DIR").map(PathBuf::from);
    let preview_theme = std::env::var("DEVPET_UI_PREVIEW_THEME")
        .ok()
        .as_deref()
        .and_then(Theme::parse);
    let mut preview_frame = 0_usize;
    let mut theme = if preview_dir.is_some() {
        preview_theme.unwrap_or_default()
    } else {
        theme::load()
    };
    let preview_cases = preview_dir.as_ref().map(|dir| {
        std::fs::create_dir_all(dir).expect("create preview directory");
        preview_captures(theme)
    });
    let mut pet = if preview_cases.is_some() {
        preview_byte()
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
        let palette = theme.palette();
        if preview_cases.is_none() {
            if get_time() - last_tick >= 1. {
                pet.advance_minutes(1);
                last_tick = get_time();
            }
            if get_time() - last_save >= 15. {
                let _ = storage::save(&pet);
                let _ = theme::save(theme);
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
                    Screen::Evolution => {
                        screen = Screen::Profile;
                        selection = 0;
                    }
                    Screen::SpriteGallery => {
                        screen = Screen::Profile;
                        selection = 1;
                    }
                    Screen::Settings => {
                        screen = Screen::Profile;
                        selection = 2;
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
                            selection = (selection + 1) % HOME_ACTIONS.len();
                        }
                        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                            selection = (selection + HOME_ACTIONS.len() - 1) % HOME_ACTIONS.len();
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
                        nav(&mut selection, PROFILE_ACTIONS.len());
                        if confirm {
                            match selection {
                                0 => {
                                    screen = Screen::Evolution;
                                    selection = 0;
                                }
                                1 => {
                                    screen = Screen::SpriteGallery;
                                    selection = 0;
                                }
                                2 => {
                                    screen = Screen::Settings;
                                    selection = theme.settings_index();
                                }
                                _ => {
                                    screen = Screen::Home;
                                    selection = 0;
                                }
                            }
                        }
                    }
                    Screen::Evolution => {
                        if confirm {
                            screen = Screen::Profile;
                            selection = 0;
                        }
                    }
                    Screen::Settings => {
                        nav(&mut selection, 3);
                        if confirm {
                            if selection < SETTINGS_CHOICES.len() {
                                theme = SETTINGS_CHOICES[selection];
                                let _ = theme::save(theme);
                            } else {
                                screen = Screen::Profile;
                                selection = 2;
                            }
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
                            selection = 1;
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
                        } else if clicked
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

        let preview_case = preview_frame / 2;
        if let Some(cases) = &preview_cases {
            let capture = &cases[preview_case.min(cases.len() - 1)];
            screen = capture.screen;
            pet = capture.pet.clone();
            selection = capture.selection;
            sprite_index = capture.sprite_index;
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
        draw_rectangle(0., 0., W, H, palette.background);

        let animation_time = if preview_cases.is_some() {
            0.0
        } else {
            get_time() as f32
        };
        match screen {
            Screen::Home => {
                txt("DEVPET", 5., 10., 7, palette.ink);
                txt(&format!("LV {}", pet.level), 135., 10., 7, palette.ink);
                creature(64., 25., &pet, animation_time, theme);
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
                    palette.ink,
                );
                txt("HP", 5., 80., 6, palette.ink);
                bar(17., 77., pet.health, StatKind::Health, &palette);
                txt("EN", 65., 80., 6, palette.ink);
                bar(77., 77., pet.energy, StatKind::Energy, &palette);
                draw_line(3., 90., 157., 90., 1., palette.divider);
                for (index, action) in HOME_ACTIONS.iter().enumerate() {
                    let x = 5. + index as f32 * 39.;
                    if index == selection {
                        draw_rectangle(x - 2., 96., 36., 13., palette.selected_fill);
                        txt(action, x, 105., 6, palette.selected_text);
                    } else {
                        txt(action, x, 105., 6, palette.ink);
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
                    palette.ink,
                );
                txt("CLICK / 1-4 / ARROWS + ENTER", 5., 137., 6, palette.ink);
            }
            Screen::Care => {
                txt("< CARE", 5., 10., 7, palette.ink);
                creature(64., 14., &pet, animation_time, theme);
                txt(&format!("HEALTH {:3}", pet.health), 5., 57., 6, palette.ink);
                bar(55., 53., pet.health, StatKind::Health, &palette);
                txt(&format!("ENERGY {:3}", pet.energy), 5., 66., 6, palette.ink);
                bar(55., 62., pet.energy, StatKind::Energy, &palette);
                txt(
                    &format!("HAPPY  {:3}", pet.happiness),
                    5.,
                    75.,
                    6,
                    palette.ink,
                );
                bar(55., 71., pet.happiness, StatKind::Happiness, &palette);
                txt(&format!("FOCUS  {:3}", pet.focus), 5., 84., 6, palette.ink);
                bar(55., 80., pet.focus, StatKind::Focus, &palette);
                for (index, action) in [
                    "FOOD +FULLNESS",
                    "COFFEE +FOCUS",
                    "SLEEP +ENERGY",
                    "MEDICINE +HEALTH",
                ]
                .iter()
                .enumerate()
                {
                    choice(
                        action,
                        99. + index as f32 * 10.,
                        index == selection,
                        &palette,
                    );
                }
            }
            Screen::Code => {
                txt("< CODE / CHOOSE PROJECT", 5., 10., 7, palette.ink);
                let labels = [
                    "FIX A BUG      +15 XP",
                    "BUILD FEATURE  +30 XP",
                    "REFACTOR       +20 XP",
                    "SHIP RELEASE   +50 XP",
                ];
                for (index, label) in labels.iter().enumerate() {
                    choice(
                        label,
                        35. + index as f32 * 18.,
                        index == selection,
                        &palette,
                    );
                }
                txt(
                    &format!("ENERGY {}  FOCUS {}", pet.energy, pet.focus),
                    10.,
                    116.,
                    7,
                    palette.ink,
                );
                txt("UP/DOWN  ENTER   X=BACK", 10., 135., 6, palette.ink);
            }
            Screen::Profile => {
                txt("< BYTE PROFILE", 5., 10., 7, palette.ink);
                creature(64., 12., &pet, animation_time, theme);
                txt(&format!("FORM {:?}", pet.species), 8., 54., 7, palette.ink);
                txt(
                    &format!("PERSONALITY {:?}", pet.personality()),
                    8.,
                    65.,
                    7,
                    palette.ink,
                );
                txt(
                    &format!("AGE {}m  LV {}", pet.age_minutes, pet.level),
                    8.,
                    76.,
                    7,
                    palette.ink,
                );
                txt(
                    &format!("XP {}  PROJECTS {}", pet.xp, pet.projects),
                    8.,
                    87.,
                    7,
                    palette.ink,
                );
                txt(
                    &format!("BUGS {}  CARE MISS {}", pet.bugs_fixed, pet.care_mistakes),
                    8.,
                    98.,
                    7,
                    palette.ink,
                );
                for (index, action) in PROFILE_ACTIONS.iter().enumerate() {
                    choice(
                        action,
                        101. + index as f32 * 11.,
                        index == selection,
                        &palette,
                    );
                }
            }
            Screen::Evolution => {
                txt("< EVOLUTION", 5., 10., 7, palette.ink);
                creature(64., 18., &pet, animation_time, theme);
                if pet.species == Species::Egg {
                    txt("A SIGNAL IS FORMING...", 27., 72., 7, palette.ink);
                    txt("BYTE HATCHES AT AGE 3m", 22., 87., 7, palette.ink);
                } else if pet.is_evolved() {
                    txt(
                        &format!("BYTE EVOLVED: {:?}", pet.species),
                        25.,
                        72.,
                        8,
                        palette.ink,
                    );
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
                        palette.ink,
                    );
                } else {
                    txt("HOW YOU RAISE BYTE", 28., 69., 7, palette.ink);
                    txt("DETERMINES ITS FUTURE.", 22., 81., 7, palette.ink);
                    txt(" ?       ?       ? ", 30., 103., 10, palette.ink);
                    txt("EVOLUTION AT AGE 240m", 20., 121., 7, palette.ink);
                }
                txt("ENTER/X = BACK", 42., 137., 6, palette.ink);
            }
            Screen::Settings => {
                txt("< SETTINGS", 5., 10., 7, palette.ink);
                txt("DESKTOP THEME", 42., 32., 7, palette.ink);
                draw_rectangle(40., 38., 80., 26., palette.panel);
                draw_rectangle_lines(40., 38., 80., 26., 1., palette.ink);
                let creature_palette = theme.creature_palette(Species::Byte);
                draw_rectangle(47., 45., 12., 12., creature_palette.body);
                draw_rectangle(63., 45., 12., 12., creature_palette.core);
                draw_rectangle(79., 45., 12., 12., palette.health);
                draw_rectangle(95., 45., 12., 12., palette.energy);
                txt(theme.label(), 50., 75., 7, palette.ink);
                for (index, option) in SETTINGS_CHOICES.iter().enumerate() {
                    let label = if *option == theme {
                        format!("{}  ACTIVE", option.menu_label())
                    } else {
                        option.menu_label().to_string()
                    };
                    choice(
                        &label,
                        97. + index as f32 * 11.,
                        index == selection,
                        &palette,
                    );
                }
                choice("BACK", 119., selection == 2, &palette);
            }
            Screen::SpriteGallery => {
                txt("<", 14., 60., 12, palette.ink);
                txt(">", 139., 60., 12, palette.ink);
                let sprite = BYTE_CONCEPTS[sprite_index];
                txt("< BYTE CONCEPTS", 5., 10., 7, palette.ink);
                txt(
                    &format!("{}/{}", sprite_index + 1, BYTE_CONCEPTS.len()),
                    137.,
                    10.,
                    7,
                    palette.ink,
                );
                draw_sprite(
                    sprite,
                    48.,
                    17.,
                    2.,
                    SpriteRender {
                        theme,
                        species: Species::Byte,
                        mood: Mood::Idle,
                        time: animation_time,
                    },
                );
                txt(sprite.name, 40., 92., 8, palette.ink);
                wrapped_txt(sprite.description, 10., 106., 47, 6, palette.ink);
                txt("< > BROWSE   ENTER/X BACK", 18., 137., 6, palette.ink);
            }
            Screen::Play => {
                txt("< PLAY", 5., 10., 7, palette.ink);
                txt("BUG SQUASH", 45., 45., 10, palette.ink);
                txt("20 SECOND MOUSE GAME", 30., 63., 7, palette.ink);
                txt("CLICK BUGS BEFORE THEY MOVE!", 17., 80., 6, palette.ink);
                choice("START", 110., true, &palette);
                txt("X = BACK", 55., 135., 6, palette.ink);
            }
            Screen::BugSquash => {
                let seconds_left = if preview_cases.is_some() {
                    20.
                } else {
                    (20. - (get_time() - game_start)).max(0.)
                };
                txt(
                    &format!("BUG SQUASH   {:02}s", seconds_left.ceil() as i32),
                    5.,
                    10.,
                    7,
                    palette.ink,
                );
                txt(&format!("HITS {hits}"), 120., 10., 7, palette.ink);
                draw_circle(bug_x, bug_y, 6., palette.warning);
                draw_line(
                    bug_x - 8.,
                    bug_y - 6.,
                    bug_x + 8.,
                    bug_y + 6.,
                    1.,
                    palette.ink,
                );
                draw_line(
                    bug_x + 8.,
                    bug_y - 6.,
                    bug_x - 8.,
                    bug_y + 6.,
                    1.,
                    palette.ink,
                );
                txt("CLICK THE BUG!", 48., 132., 7, palette.ink);
            }
        }

        if let Some(point) = if preview_cases.is_some() {
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
                    draw_rectangle_lines(
                        target.x,
                        target.y,
                        target.w,
                        target.h,
                        1.,
                        palette.hover_outline,
                    );
                }
            }
        }
        set_default_camera();
        if let Some(dir) = &preview_dir {
            if preview_frame % 2 == 1 {
                let capture =
                    &preview_cases.as_ref().expect("preview cases available")[preview_case];
                let path = dir.join(format!("{preview_case:02}-{}.png", capture.name));
                get_screen_data().export_png(path.to_str().expect("UTF-8 preview path"));
                println!("Captured {}", path.display());
                if preview_case + 1
                    == preview_cases
                        .as_ref()
                        .expect("preview cases available")
                        .len()
                {
                    break;
                }
            }
            preview_frame += 1;
        } else if is_key_pressed(KeyCode::Q) {
            let _ = storage::save(&pet);
            let _ = theme::save(theme);
            break;
        }
        next_frame().await;
    }
}
