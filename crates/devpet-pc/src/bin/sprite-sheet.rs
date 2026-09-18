use devpet_pc::sprites::{
    Sprite32, BODY, BYTE_CONCEPTS, CORE, INK, SPRITE_HEIGHT, SPRITE_WIDTH, TRANSPARENT,
};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const COLUMNS: usize = 5;
const PIXEL_SCALE: usize = 6;
const CELL_WIDTH: usize = 240;
const CELL_HEIGHT: usize = 250;
const HEADER_HEIGHT: usize = 54;

fn palette_color(index: u8) -> &'static str {
    match index {
        INK => "#19281e",
        BODY => "#96aa6b",
        CORE => "#527834",
        _ => "none",
    }
}

fn write_sprite(svg: &mut String, sprite: &Sprite32, origin_x: usize, origin_y: usize) {
    for y in 0..SPRITE_HEIGHT {
        let mut x = 0;
        while x < SPRITE_WIDTH {
            let color = sprite.pixel(x, y);
            if color == TRANSPARENT {
                x += 1;
                continue;
            }

            let start = x;
            while x < SPRITE_WIDTH && sprite.pixel(x, y) == color {
                x += 1;
            }
            let run_width = x - start;
            writeln!(
                svg,
                r##"    <rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"##,
                origin_x + start * PIXEL_SCALE,
                origin_y + y * PIXEL_SCALE,
                run_width * PIXEL_SCALE,
                PIXEL_SCALE,
                palette_color(color),
            )
            .expect("writing to a String cannot fail");
        }
    }
}

fn render_sheet() -> String {
    let rows = BYTE_CONCEPTS.len().div_ceil(COLUMNS);
    let width = COLUMNS * CELL_WIDTH;
    let height = HEADER_HEIGHT + rows * CELL_HEIGHT;
    let sprite_size = SPRITE_WIDTH * PIXEL_SCALE;
    let mut svg = String::new();

    writeln!(
        svg,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" shape-rendering="crispEdges">"##
    )
    .expect("writing to a String cannot fail");
    writeln!(
        svg,
        r##"  <rect width="100%" height="100%" fill="#f4f3e9"/>"##
    )
    .expect("writing to a String cannot fail");
    writeln!(
        svg,
        r##"  <text x="24" y="34" fill="#19281e" font-family="ui-monospace, monospace" font-size="22" font-weight="700">BYTE // 32 x 32 CONCEPT SPRITES</text>"##
    )
    .expect("writing to a String cannot fail");

    for (index, sprite) in BYTE_CONCEPTS.iter().enumerate() {
        let column = index % COLUMNS;
        let row = index / COLUMNS;
        let cell_x = column * CELL_WIDTH;
        let cell_y = HEADER_HEIGHT + row * CELL_HEIGHT;
        let sprite_x = cell_x + (CELL_WIDTH - sprite_size) / 2;
        let sprite_y = cell_y + 14;

        writeln!(svg, r##"  <g id="{}" class="byte-sprite">"##, sprite.id)
            .expect("writing to a String cannot fail");
        writeln!(
            svg,
            r##"    <rect x="{}" y="{}" width="{}" height="{}" rx="8" fill="#d4deaa" stroke="#94a66f" stroke-width="2"/>"##,
            cell_x + 8,
            cell_y + 4,
            CELL_WIDTH - 16,
            CELL_HEIGHT - 12,
        )
        .expect("writing to a String cannot fail");
        write_sprite(&mut svg, sprite, sprite_x, sprite_y);
        writeln!(
            svg,
            r##"    <text class="sprite-label" x="{}" y="{}" text-anchor="middle" fill="#19281e" font-family="ui-monospace, monospace" font-size="15" font-weight="700">{}</text>"##,
            cell_x + CELL_WIDTH / 2,
            sprite_y + sprite_size + 23,
            sprite.name,
        )
        .expect("writing to a String cannot fail");
        writeln!(
            svg,
            r##"    <text x="{}" y="{}" text-anchor="middle" fill="#52634a" font-family="ui-monospace, monospace" font-size="11">{}</text>"##,
            cell_x + CELL_WIDTH / 2,
            sprite_y + sprite_size + 41,
            sprite.id,
        )
        .expect("writing to a String cannot fail");
        writeln!(svg, "  </g>").expect("writing to a String cannot fail");
    }

    writeln!(svg, "</svg>").expect("writing to a String cannot fail");
    svg
}

fn write_sheet(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, render_sheet())
}

fn main() -> io::Result<()> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/byte-sprite-contact-sheet.svg"));
    write_sheet(&output)?;
    println!("wrote {}", output.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheet_contains_one_group_and_label_per_sprite() {
        let sheet = render_sheet();
        assert_eq!(sheet.matches("class=\"byte-sprite\"").count(), 10);
        assert_eq!(sheet.matches("class=\"sprite-label\"").count(), 10);
        for sprite in BYTE_CONCEPTS {
            assert!(sheet.contains(sprite.id));
            assert!(sheet.contains(sprite.name));
        }
    }
}
