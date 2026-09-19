use devpet_core::Species;
use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

const SETTINGS_SCHEMA: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    #[default]
    PocketColor,
    ClassicLcd,
}

impl Theme {
    pub const ALL: [Self; 2] = [Self::PocketColor, Self::ClassicLcd];

    #[must_use]
    pub fn slug(self) -> &'static str {
        match self {
            Self::PocketColor => "pocket-color",
            Self::ClassicLcd => "classic-lcd",
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::PocketColor => "Pocket Color",
            Self::ClassicLcd => "Classic LCD",
        }
    }

    #[must_use]
    pub fn menu_label(self) -> &'static str {
        match self {
            Self::PocketColor => "POCKET COLOR",
            Self::ClassicLcd => "CLASSIC LCD",
        }
    }

    #[must_use]
    pub fn settings_index(self) -> usize {
        match self {
            Self::PocketColor => 0,
            Self::ClassicLcd => 1,
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "pocket-color" | "pocket_color" | "pocketcolor" => Some(Self::PocketColor),
            "classic-lcd" | "classic_lcd" | "classiclcd" => Some(Self::ClassicLcd),
            _ => None,
        }
    }

    #[must_use]
    pub fn palette(self) -> Palette {
        match self {
            Self::PocketColor => Palette {
                background: hex(0xF4EBD8),
                ink: hex(0x243047),
                divider: hex(0x172338),
                panel: hex(0xEEE1C7),
                selected_fill: hex(0x172338),
                selected_text: hex(0xF4EBD8),
                hover_outline: hex(0x39BFA5),
                bar_track: hex(0xD9CCB4),
                health: hex(0x75B875),
                energy: hex(0x6FAFE0),
                happiness: hex(0xF2BC57),
                focus: hex(0x39BFA5),
                warning: hex(0xE97968),
                egg_fill: hex(0xF4EBD8),
            },
            Self::ClassicLcd => Palette {
                background: hex(0xD1DDAE),
                ink: hex(0x19281E),
                divider: hex(0x19281E),
                panel: hex(0xC4D39A),
                selected_fill: hex(0x19281E),
                selected_text: hex(0xD1DDAE),
                hover_outline: hex(0x789C40),
                bar_track: hex(0xB1C07F),
                health: hex(0x789C40),
                energy: hex(0x527834),
                happiness: hex(0x96AA6B),
                focus: hex(0x6B8650),
                warning: hex(0x415934),
                egg_fill: hex(0xDCE7B6),
            },
        }
    }

    #[must_use]
    pub fn creature_palette(self, species: Species) -> CreaturePalette {
        match (self, species) {
            (Self::PocketColor, Species::Bot) => CreaturePalette {
                body: hex(0x6B88A9),
                core: hex(0xF2BC57),
                core_bright: hex(0xFFD37A),
                core_dim: hex(0xC99744),
            },
            (Self::PocketColor, Species::Beast) => CreaturePalette {
                body: hex(0xC97A5A),
                core: hex(0xF4EBD8),
                core_bright: hex(0xFFF7E7),
                core_dim: hex(0xD5C5A8),
            },
            (Self::PocketColor, Species::Ghost) => CreaturePalette {
                body: hex(0xB996E6),
                core: hex(0x84E3F2),
                core_bright: hex(0xB7F4FF),
                core_dim: hex(0x7BAAC7),
            },
            (Self::PocketColor, _) => CreaturePalette {
                body: hex(0x39BFA5),
                core: hex(0xF2BC57),
                core_bright: hex(0xFFD37A),
                core_dim: hex(0xD49B42),
            },
            (Self::ClassicLcd, Species::Bot) => CreaturePalette {
                body: hex(0x7B8D5C),
                core: hex(0x4F6C33),
                core_bright: hex(0x8CAE5F),
                core_dim: hex(0x65754B),
            },
            (Self::ClassicLcd, Species::Beast) => CreaturePalette {
                body: hex(0x97A86C),
                core: hex(0x647E40),
                core_bright: hex(0xABC07C),
                core_dim: hex(0x74835B),
            },
            (Self::ClassicLcd, Species::Ghost) => CreaturePalette {
                body: hex(0xA3B28A),
                core: hex(0x6C8366),
                core_bright: hex(0xB7C7A0),
                core_dim: hex(0x7A876E),
            },
            (Self::ClassicLcd, _) => CreaturePalette {
                body: hex(0x96AA6B),
                core: hex(0x527834),
                core_bright: hex(0x799C40),
                core_dim: hex(0x7A8460),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub background: Color,
    pub ink: Color,
    pub divider: Color,
    pub panel: Color,
    pub selected_fill: Color,
    pub selected_text: Color,
    pub hover_outline: Color,
    pub bar_track: Color,
    pub health: Color,
    pub energy: Color,
    pub happiness: Color,
    pub focus: Color,
    pub warning: Color,
    pub egg_fill: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct CreaturePalette {
    pub body: Color,
    pub core: Color,
    pub core_bright: Color,
    pub core_dim: Color,
}

#[derive(Debug, Serialize, Deserialize)]
struct SettingsFile {
    schema_version: u16,
    theme: Theme,
}

fn hex(value: u32) -> Color {
    Color::new(
        ((value >> 16) & 0xff) as f32 / 255.,
        ((value >> 8) & 0xff) as f32 / 255.,
        (value & 0xff) as f32 / 255.,
        1.,
    )
}

fn path() -> PathBuf {
    PathBuf::from("devpet-settings.json")
}

#[must_use]
pub fn load() -> Theme {
    let Ok(raw) = fs::read_to_string(path()) else {
        return Theme::default();
    };
    let Ok(settings) = serde_json::from_str::<SettingsFile>(&raw) else {
        return Theme::default();
    };
    if settings.schema_version != SETTINGS_SCHEMA {
        return Theme::default();
    }
    settings.theme
}

pub fn save(theme: Theme) -> io::Result<()> {
    let data = serde_json::to_string_pretty(&SettingsFile {
        schema_version: SETTINGS_SCHEMA,
        theme,
    })
    .map_err(io::Error::other)?;
    let tmp = path().with_extension("json.tmp");
    fs::write(&tmp, data)?;
    fs::rename(tmp, path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_slugs_round_trip() {
        for theme in Theme::ALL {
            assert_eq!(Theme::parse(theme.slug()), Some(theme));
        }
    }
}
