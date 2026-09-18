//! Original 32 x 32 Byte concept sprites.
//!
//! Sprites use a four-entry indexed palette so the same source data can be
//! rendered by the PC host, exported for art review, or translated to a future
//! hardware sprite ROM.

pub const SPRITE_WIDTH: usize = 32;
pub const SPRITE_HEIGHT: usize = 32;
pub const SPRITE_PIXELS: usize = SPRITE_WIDTH * SPRITE_HEIGHT;

pub const TRANSPARENT: u8 = 0;
pub const INK: u8 = 1;
pub const BODY: u8 = 2;
pub const CORE: u8 = 3;
pub const PALETTE_SIZE: u8 = 4;

#[derive(Debug)]
pub struct Sprite32 {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub pixels: [u8; SPRITE_PIXELS],
}

impl Sprite32 {
    #[must_use]
    pub const fn pixel(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * SPRITE_WIDTH + x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpriteBounds {
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

#[must_use]
pub fn occupied_bounds(sprite: &Sprite32) -> Option<SpriteBounds> {
    let mut min_x = SPRITE_WIDTH;
    let mut min_y = SPRITE_HEIGHT;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..SPRITE_HEIGHT {
        for x in 0..SPRITE_WIDTH {
            if sprite.pixel(x, y) == TRANSPARENT {
                continue;
            }
            found = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    found.then_some(SpriteBounds {
        min_x,
        min_y,
        max_x,
        max_y,
    })
}

struct Canvas {
    pixels: [u8; SPRITE_PIXELS],
}

impl Canvas {
    const fn new() -> Self {
        Self {
            pixels: [TRANSPARENT; SPRITE_PIXELS],
        }
    }

    const fn put(&mut self, x: i32, y: i32, color: u8) {
        if x >= 0 && x < SPRITE_WIDTH as i32 && y >= 0 && y < SPRITE_HEIGHT as i32 {
            self.pixels[y as usize * SPRITE_WIDTH + x as usize] = color;
        }
    }

    const fn rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u8) {
        let mut py = y;
        while py < y + height {
            let mut px = x;
            while px < x + width {
                self.put(px, py, color);
                px += 1;
            }
            py += 1;
        }
    }

    const fn hline(&mut self, x: i32, y: i32, width: i32, color: u8) {
        self.rect(x, y, width, 1, color);
    }

    const fn vline(&mut self, x: i32, y: i32, height: i32, color: u8) {
        self.rect(x, y, 1, height, color);
    }

    const fn ellipse(&mut self, cx: i32, cy: i32, rx: i32, ry: i32, color: u8) {
        let rx_squared = rx * rx;
        let ry_squared = ry * ry;
        let threshold = rx_squared * ry_squared;
        let mut y = cy - ry;
        while y <= cy + ry {
            let mut x = cx - rx;
            while x <= cx + rx {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx * ry_squared + dy * dy * rx_squared <= threshold {
                    self.put(x, y, color);
                }
                x += 1;
            }
            y += 1;
        }
    }

    const fn diamond(&mut self, cx: i32, cy: i32, radius: i32, color: u8) {
        let mut y = cy - radius;
        while y <= cy + radius {
            let dy = if y < cy { cy - y } else { y - cy };
            let half_width = radius - dy;
            self.hline(cx - half_width, y, half_width * 2 + 1, color);
            y += 1;
        }
    }

    const fn finish(self) -> [u8; SPRITE_PIXELS] {
        self.pixels
    }
}

const fn smiling_face(canvas: &mut Canvas, eye_y: i32, mouth_y: i32) {
    canvas.rect(10, eye_y, 3, 3, INK);
    canvas.rect(20, eye_y, 3, 3, INK);
    canvas.put(13, mouth_y, INK);
    canvas.put(14, mouth_y + 1, INK);
    canvas.hline(15, mouth_y + 2, 4, INK);
    canvas.put(19, mouth_y + 1, INK);
    canvas.put(20, mouth_y, INK);
}

const fn little_feet(canvas: &mut Canvas, y: i32) {
    canvas.rect(8, y, 6, 3, INK);
    canvas.rect(19, y, 6, 3, INK);
    canvas.hline(9, y + 3, 4, INK);
    canvas.hline(20, y + 3, 4, INK);
}

const fn orb_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.ellipse(16, 14, 13, 12, INK);
    canvas.ellipse(16, 14, 11, 10, BODY);
    smiling_face(&mut canvas, 9, 15);
    canvas.diamond(16, 22, 2, CORE);
    little_feet(&mut canvas, 25);
    canvas.finish()
}

const fn pixel_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.rect(5, 4, 22, 22, INK);
    canvas.rect(7, 6, 18, 18, BODY);
    canvas.rect(5, 4, 2, 2, TRANSPARENT);
    canvas.rect(25, 4, 2, 2, TRANSPARENT);
    canvas.rect(5, 24, 2, 2, TRANSPARENT);
    canvas.rect(25, 24, 2, 2, TRANSPARENT);
    canvas.rect(9, 10, 4, 3, INK);
    canvas.rect(19, 10, 4, 3, INK);
    canvas.hline(12, 17, 8, INK);
    canvas.rect(15, 21, 2, 2, CORE);
    little_feet(&mut canvas, 26);
    canvas.finish()
}

const fn terminal_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.rect(4, 5, 24, 21, INK);
    canvas.rect(6, 7, 20, 17, BODY);
    canvas.put(4, 5, TRANSPARENT);
    canvas.put(27, 5, TRANSPARENT);
    canvas.put(4, 25, TRANSPARENT);
    canvas.put(27, 25, TRANSPARENT);

    canvas.put(9, 10, INK);
    canvas.put(10, 11, INK);
    canvas.put(11, 12, INK);
    canvas.put(10, 13, INK);
    canvas.put(9, 14, INK);
    canvas.hline(19, 14, 5, INK);
    canvas.hline(11, 19, 10, INK);
    canvas.rect(22, 18, 2, 2, CORE);
    little_feet(&mut canvas, 26);
    canvas.finish()
}

const fn bit_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.ellipse(16, 12, 11, 10, INK);
    canvas.ellipse(16, 12, 9, 8, BODY);
    canvas.rect(10, 9, 3, 3, INK);
    canvas.rect(20, 9, 3, 3, INK);
    canvas.put(15, 16, INK);
    canvas.put(16, 17, INK);
    canvas.put(17, 16, INK);
    canvas.rect(11, 20, 10, 7, INK);
    canvas.rect(13, 21, 6, 5, BODY);
    canvas.rect(15, 22, 2, 2, CORE);
    canvas.rect(10, 27, 4, 3, INK);
    canvas.rect(18, 27, 4, 3, INK);
    canvas.finish()
}

const fn crt_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.rect(3, 4, 26, 21, INK);
    canvas.rect(5, 6, 22, 17, BODY);
    canvas.rect(7, 8, 18, 12, TRANSPARENT);
    canvas.rect(8, 9, 16, 10, BODY);
    canvas.rect(10, 11, 3, 3, INK);
    canvas.rect(19, 11, 3, 3, INK);
    canvas.hline(13, 16, 6, INK);
    canvas.rect(24, 21, 2, 2, CORE);
    canvas.rect(14, 25, 4, 3, INK);
    canvas.rect(9, 28, 14, 3, INK);
    canvas.finish()
}

const fn bug_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.put(9, 2, INK);
    canvas.put(10, 3, INK);
    canvas.put(11, 4, INK);
    canvas.put(22, 2, INK);
    canvas.put(21, 3, INK);
    canvas.put(20, 4, INK);
    canvas.vline(11, 4, 4, INK);
    canvas.vline(20, 4, 4, INK);
    canvas.ellipse(16, 16, 13, 10, INK);
    canvas.ellipse(16, 16, 11, 8, BODY);
    canvas.vline(16, 8, 17, INK);
    canvas.rect(10, 11, 3, 3, INK);
    canvas.rect(20, 11, 3, 3, INK);
    canvas.put(13, 18, INK);
    canvas.hline(14, 19, 5, INK);
    canvas.put(19, 18, INK);
    canvas.rect(6, 15, 6, 2, CORE);
    canvas.rect(21, 15, 5, 2, CORE);
    canvas.hline(1, 13, 4, INK);
    canvas.hline(27, 13, 4, INK);
    canvas.hline(1, 20, 5, INK);
    canvas.hline(26, 20, 5, INK);
    canvas.finish()
}

const fn slime_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.ellipse(16, 16, 14, 11, INK);
    canvas.ellipse(16, 16, 12, 9, BODY);
    canvas.rect(3, 16, 27, 9, INK);
    canvas.rect(5, 16, 23, 7, BODY);
    canvas.rect(10, 13, 3, 3, INK);
    canvas.rect(20, 13, 3, 3, INK);
    canvas.put(13, 19, INK);
    canvas.hline(14, 20, 5, INK);
    canvas.put(19, 19, INK);
    canvas.diamond(16, 24, 2, CORE);
    canvas.rect(4, 23, 5, 4, INK);
    canvas.rect(14, 23, 5, 7, INK);
    canvas.rect(24, 23, 5, 5, INK);
    canvas.hline(6, 27, 2, INK);
    canvas.hline(15, 30, 3, INK);
    canvas.hline(25, 28, 3, INK);
    canvas.finish()
}

const fn bot_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.vline(16, 1, 4, INK);
    canvas.rect(14, 0, 5, 3, INK);
    canvas.rect(5, 5, 22, 16, INK);
    canvas.rect(7, 7, 18, 12, BODY);
    canvas.rect(2, 10, 3, 7, INK);
    canvas.rect(27, 10, 3, 7, INK);
    canvas.rect(9, 10, 4, 4, INK);
    canvas.rect(19, 10, 4, 4, INK);
    canvas.hline(12, 17, 3, INK);
    canvas.hline(17, 17, 3, INK);
    canvas.rect(9, 21, 14, 7, INK);
    canvas.rect(11, 22, 10, 5, BODY);
    canvas.diamond(16, 24, 2, CORE);
    canvas.rect(7, 27, 7, 4, INK);
    canvas.rect(19, 27, 7, 4, INK);
    canvas.finish()
}

const fn ghost_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.ellipse(16, 12, 12, 10, INK);
    canvas.ellipse(16, 12, 10, 8, BODY);
    canvas.rect(4, 12, 25, 16, INK);
    canvas.rect(6, 12, 21, 11, BODY);
    canvas.rect(10, 10, 3, 4, INK);
    canvas.rect(20, 10, 3, 4, INK);
    canvas.put(15, 17, INK);
    canvas.put(16, 18, INK);
    canvas.put(17, 17, INK);
    canvas.diamond(16, 21, 2, CORE);
    canvas.ellipse(9, 28, 4, 4, TRANSPARENT);
    canvas.ellipse(16, 28, 4, 4, TRANSPARENT);
    canvas.ellipse(23, 28, 4, 4, TRANSPARENT);
    canvas.finish()
}

const fn core_byte_pixels() -> [u8; SPRITE_PIXELS] {
    let mut canvas = Canvas::new();
    canvas.ellipse(16, 12, 11, 10, INK);
    canvas.ellipse(16, 12, 9, 8, BODY);
    canvas.rect(10, 9, 3, 3, INK);
    canvas.rect(20, 9, 3, 3, INK);
    canvas.put(14, 15, INK);
    canvas.hline(15, 16, 3, INK);
    canvas.put(18, 15, INK);
    canvas.rect(8, 20, 16, 7, INK);
    canvas.rect(10, 20, 12, 6, BODY);
    canvas.diamond(16, 22, 5, INK);
    canvas.diamond(16, 22, 3, CORE);
    canvas.put(16, 22, INK);
    canvas.rect(7, 26, 7, 4, INK);
    canvas.rect(19, 26, 7, 4, INK);
    canvas.finish()
}

pub static ORB_BYTE: Sprite32 = Sprite32 {
    id: "orb-byte",
    name: "01 ORB BYTE",
    description: "Round mascot silhouette with expressive eyes and a state Core.",
    pixels: orb_byte_pixels(),
};

pub static PIXEL_BYTE: Sprite32 = Sprite32 {
    id: "pixel-byte",
    name: "02 PIXEL BYTE",
    description: "A living square pixel with crisp digital corners.",
    pixels: pixel_byte_pixels(),
};

pub static TERMINAL_BYTE: Sprite32 = Sprite32 {
    id: "terminal-byte",
    name: "03 TERMINAL BYTE",
    description: "A prompt-eyed Byte whose face reads like a terminal.",
    pixels: terminal_byte_pixels(),
};

pub static BIT_BYTE: Sprite32 = Sprite32 {
    id: "bit-byte",
    name: "04 BIT BYTE",
    description: "A tiny Tamagotchi-like body with an oversized head.",
    pixels: bit_byte_pixels(),
};

pub static CRT_BYTE: Sprite32 = Sprite32 {
    id: "crt-byte",
    name: "05 CRT BYTE",
    description: "An old monitor brought to life as a compact creature.",
    pixels: crt_byte_pixels(),
};

pub static BUG_BYTE: Sprite32 = Sprite32 {
    id: "bug-byte",
    name: "06 BUG BYTE",
    description: "A friendly software bug with antennae and split wings.",
    pixels: bug_byte_pixels(),
};

pub static SLIME_BYTE: Sprite32 = Sprite32 {
    id: "slime-byte",
    name: "07 SLIME BYTE",
    description: "A deformable digital blob built for expressive animation.",
    pixels: slime_byte_pixels(),
};

pub static BOT_BYTE: Sprite32 = Sprite32 {
    id: "bot-byte",
    name: "08 BOT BYTE",
    description: "An antenna-topped AI robot with a chest Core.",
    pixels: bot_byte_pixels(),
};

pub static GHOST_BYTE: Sprite32 = Sprite32 {
    id: "ghost-byte",
    name: "09 GHOST BYTE",
    description: "A floating digital entity with a shifting pixel fringe.",
    pixels: ghost_byte_pixels(),
};

pub static CORE_BYTE: Sprite32 = Sprite32 {
    id: "core-byte",
    name: "10 CORE BYTE",
    description: "A compact guardian organized around a large luminous Core.",
    pixels: core_byte_pixels(),
};

pub static BYTE_CONCEPTS: [&Sprite32; 10] = [
    &ORB_BYTE,
    &PIXEL_BYTE,
    &TERMINAL_BYTE,
    &BIT_BYTE,
    &CRT_BYTE,
    &BUG_BYTE,
    &SLIME_BYTE,
    &BOT_BYTE,
    &GHOST_BYTE,
    &CORE_BYTE,
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_contains_the_ten_byte_concepts() {
        assert_eq!(BYTE_CONCEPTS.len(), 10);
    }

    #[test]
    fn identifiers_and_names_are_unique() {
        let ids: HashSet<_> = BYTE_CONCEPTS.iter().map(|sprite| sprite.id).collect();
        let names: HashSet<_> = BYTE_CONCEPTS.iter().map(|sprite| sprite.name).collect();
        assert_eq!(ids.len(), BYTE_CONCEPTS.len());
        assert_eq!(names.len(), BYTE_CONCEPTS.len());
    }

    #[test]
    fn every_pixel_uses_the_indexed_palette() {
        for sprite in BYTE_CONCEPTS {
            assert!(
                sprite.pixels.iter().all(|pixel| *pixel < PALETTE_SIZE),
                "{} contains an invalid palette index",
                sprite.id
            );
        }
    }

    #[test]
    fn every_sprite_has_art_and_clear_padding() {
        for sprite in BYTE_CONCEPTS {
            let bounds = occupied_bounds(sprite).expect("sprite must contain visible pixels");
            assert!(bounds.min_x > 0, "{} touches the left edge", sprite.id);
            assert!(bounds.min_y > 0, "{} touches the top edge", sprite.id);
            assert!(
                bounds.max_x < SPRITE_WIDTH - 1,
                "{} touches the right edge",
                sprite.id
            );
            assert!(
                bounds.max_y < SPRITE_HEIGHT - 1,
                "{} touches the bottom edge",
                sprite.id
            );
        }
    }

    #[test]
    fn every_concept_has_a_distinct_bitmap() {
        for (index, sprite) in BYTE_CONCEPTS.iter().enumerate() {
            for other in BYTE_CONCEPTS.iter().skip(index + 1) {
                assert_ne!(
                    sprite.pixels, other.pixels,
                    "{} duplicates {}",
                    sprite.id, other.id
                );
            }
        }
    }

    #[test]
    fn canonical_orb_byte_has_a_core() {
        assert!(ORB_BYTE.pixels.contains(&CORE));
    }
}
