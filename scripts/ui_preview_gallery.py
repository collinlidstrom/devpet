"""Validate desktop captures and build a dependency-free, offline HTML gallery."""

import html
from pathlib import Path
import struct
import sys


def main():
    directory = Path(sys.argv[1])
    screens = ["home", "care", "code", "play", "bug-squash", "profile", "evolution"]
    names = screens + [f"sprite-{i:02}" for i in range(1, 11)]
    expected = [f"{i:02}-{name}.png" for i, name in enumerate(names)]
    actual = sorted(p.name for p in directory.glob("*.png"))
    if actual != expected:
        raise SystemExit(f"Incomplete or unexpected capture set: {actual}; expected {expected}")
    figures = []
    for filename, name in zip(expected, names):
        data = (directory / filename).read_bytes()
        if data[:8] != b"\x89PNG\r\n\x1a\n" or len(data) < 24:
            raise SystemExit(f"Invalid PNG: {filename}")
        if struct.unpack(">II", data[16:24]) != (640, 576):
            raise SystemExit(f"Unexpected image dimensions: {filename}")
        title = html.escape(name.replace("-", " ").title())
        figures.append(f'<figure><a href="{filename}"><img src="{filename}" alt="{title}"></a>'
                       f'<figcaption>{title}</figcaption></figure>')
    document = """<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>DevPet UI preview</title>
<style>
body{margin:0;padding:24px;background:#18251e;color:#e1e9ce;font:16px system-ui}
main{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:24px}
figure{margin:0;padding:12px;background:#26382b;border-radius:12px}
img{width:100%;height:auto;image-rendering:pixelated}figcaption{padding:12px 0 0}
</style><h1>DevPet UI preview</h1>
<p>Actual desktop renderer · Fixed sample state · 640 × 576 screenshots. Click to view full size.</p>
<main>""" + "\n".join(figures) + "</main></html>"
    (directory / "index.html").write_text(document, encoding="utf-8")
    print(f"Validated {len(expected)} captures and created {directory / 'index.html'}")


if __name__ == "__main__":
    main()
