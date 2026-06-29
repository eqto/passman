#!/usr/bin/env python3
from PIL import Image, ImageDraw, ImageFont
import os

sizes = [32, 128]
icon_dir = os.path.dirname(os.path.abspath(__file__))

for size in sizes:
    img = Image.new("RGBA", (size, size), (59, 130, 246, 255))  # blue-500
    draw = ImageDraw.Draw(img)
    try:
        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", int(size * 0.5))
    except:
        font = ImageFont.load_default()
    text = "P"
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    x = (size - text_width) // 2 - bbox[0]
    y = (size - text_height) // 2 - bbox[1]
    draw.text((x, y), text, fill=(255, 255, 255, 255), font=font)

    if size == 128:
        img.save(os.path.join(icon_dir, f"{size}x{size}.png"))
        img2 = img.resize((256, 256), Image.LANCZOS)
        img2.save(os.path.join(icon_dir, f"{size}x{size}@2x.png"))
    else:
        img.save(os.path.join(icon_dir, f"{size}x{size}.png"))

# Create a simple 512x512 for icns/ico
img512 = Image.new("RGBA", (512, 512), (59, 130, 246, 255))
draw = ImageDraw.Draw(img512)
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 256)
except:
    font = ImageFont.load_default()
bbox = draw.textbbox((0, 0), "P", font=font)
text_width = bbox[2] - bbox[0]
text_height = bbox[3] - bbox[1]
x = (512 - text_width) // 2 - bbox[0]
y = (512 - text_height) // 2 - bbox[1]
draw.text((x, y), "P", fill=(255, 255, 255, 255), font=font)
img512.save(os.path.join(icon_dir, "icon.png"))

print("Icons generated")
