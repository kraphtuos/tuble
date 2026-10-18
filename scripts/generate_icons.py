"""Generate the app icons in icons/ from one SVG design.

Run with: uv run --no-project --with playwright python scripts/generate_icons.py
It renders the PNGs with an installed Chrome, so no browser download is needed.
"""

from pathlib import Path

from playwright.sync_api import sync_playwright

ICONS = Path(__file__).resolve().parent.parent / "icons"


def svg(rounded: bool) -> str:
    """A stretch of line with three stations, like the header logo.

    The artwork stays within the central 80% circle so it survives masking.
    """
    rx = 112 if rounded else 0
    stations = "".join(f'<circle cx="{x}" cy="256" r="36"/>' for x in (136, 256, 376))
    return f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <rect width="512" height="512" rx="{rx}" fill="#0019a8"/>
  <rect x="72" y="238" width="368" height="36" rx="18" fill="#ffffff"/>
  <g fill="#0019a8" stroke="#ffffff" stroke-width="22">{stations}</g>
</svg>
"""


def main() -> None:
    ICONS.mkdir(exist_ok=True)
    (ICONS / "favicon.svg").write_text(svg(rounded=True))
    outputs = [
        ("icon-192.png", 192, True),
        ("icon-512.png", 512, True),
        ("icon-maskable-512.png", 512, False),
        ("apple-touch-icon.png", 180, False),
    ]
    with sync_playwright() as p:
        browser = p.chromium.launch(channel="chrome")
        for name, size, rounded in outputs:
            page = browser.new_page(viewport={"width": size, "height": size})
            page.set_content(
                f'<style>*{{margin:0}}svg{{display:block;width:{size}px;height:{size}px}}</style>'
                + svg(rounded)
            )
            page.screenshot(path=ICONS / name, omit_background=True)
            page.close()
        browser.close()


if __name__ == "__main__":
    main()
