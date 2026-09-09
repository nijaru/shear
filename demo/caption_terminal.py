# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow>=11"]
# ///
"""Add caption bands to real VHS footage; never replace terminal content."""

import argparse
import json
import subprocess
from itertools import pairwise
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("video", type=Path)
    parser.add_argument("captions", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument(
        "--font", default="/System/Library/Fonts/Supplemental/Arial.ttf"
    )
    args = parser.parse_args()
    metadata = json.loads(
        subprocess.check_output(
            [
                "ffprobe",
                "-v",
                "error",
                "-show_entries",
                "format=duration:stream=width,height",
                "-of",
                "json",
                str(args.video),
            ],
            text=True,
        )
    )
    assert metadata["streams"][0]["width"] == 1920
    assert metadata["streams"][0]["height"] == 900
    duration = float(metadata["format"]["duration"])
    captions = json.loads(args.captions.read_text())
    assert captions[0]["start"] == 0
    assert all(a["start"] < b["start"] for a, b in pairwise(captions))
    assert captions[-1]["start"] < duration
    assets = args.output.parent / "caption-bands"
    assets.mkdir(parents=True, exist_ok=True)
    fonts = [ImageFont.truetype(args.font, size) for size in (24, 40, 26)]
    command = ["ffmpeg", "-y", "-i", str(args.video)]
    filters = ["[0:v]pad=1920:1080:0:0:color=0x14151d[base]"]
    previous = "base"
    for i, caption in enumerate(captions):
        image = Image.new("RGB", (1920, 180), "#14151d")
        draw = ImageDraw.Draw(image)
        draw.line((30, 0, 1890, 0), fill="#64748b", width=2)
        for text, y, font, color in zip(
            (caption["label"], caption["text"], caption["detail"]),
            (18, 55, 113),
            fonts,
            ("#a5b4fc", "#ffffff", "#d1d5db"),
            strict=True,
        ):
            assert draw.textlength(text, font=font) <= 1860, text
            draw.text((30, y), text, font=font, fill=color)
        path = assets / f"{i}.png"
        image.save(path)
        command.extend(["-i", str(path)])
        end = captions[i + 1]["start"] if i + 1 < len(captions) else duration
        output = f"caption{i}"
        filters.append(
            f"[{previous}][{i + 1}:v]overlay=0:900:"
            f"enable='gte(t,{caption['start']})*lt(t,{end})'[{output}]"
        )
        previous = output
    temporary = args.output.with_name(args.output.stem + ".pending.mp4")
    command.extend(
        [
            "-filter_complex",
            ";".join(filters),
            "-map",
            f"[{previous}]",
            "-an",
            "-c:v",
            "libx264",
            "-crf",
            "18",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
            str(temporary),
        ]
    )
    subprocess.run(command, check=True)
    temporary.replace(args.output)


if __name__ == "__main__":
    main()
