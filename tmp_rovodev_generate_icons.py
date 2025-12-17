#!/usr/bin/env python3
"""
CineVault Icon Generator
Converts the SVG icon to all required formats using cairosvg and Pillow
"""

try:
    import cairosvg
    from PIL import Image
    import io
    import os
except ImportError:
    print("❌ Required packages not found.")
    print("\nPlease install:")
    print("  pip install cairosvg pillow")
    print("\nOr use the Node.js version:")
    print("  npm run generate-icons")
    exit(1)

SVG_PATH = "src-tauri/icons/icon.svg"
ICONS_DIR = "src-tauri/icons"

# Icon sizes to generate
SIZES = [
    (32, "32x32.png"),
    (128, "128x128.png"),
    (256, "128x128@2x.png"),
    (512, "icon.png"),
    (1024, "icon@2x.png"),
]

def generate_png(svg_path, output_path, size):
    """Generate a PNG from SVG at specified size"""
    png_data = cairosvg.svg2png(
        url=svg_path,
        output_width=size,
        output_height=size
    )
    with open(output_path, 'wb') as f:
        f.write(png_data)

def generate_ico(svg_path, output_path):
    """Generate a Windows ICO file with multiple sizes"""
    # ICO files typically contain 16, 32, 48, and 256 pixel versions
    ico_sizes = [16, 32, 48, 256]
    images = []
    
    for size in ico_sizes:
        png_data = cairosvg.svg2png(
            url=svg_path,
            output_width=size,
            output_height=size
        )
        img = Image.open(io.BytesIO(png_data))
        images.append(img)
    
    # Save as ICO with all sizes
    images[0].save(
        output_path,
        format='ICO',
        sizes=[(img.width, img.height) for img in images],
        append_images=images[1:]
    )

def main():
    print("🎨 Generating CineVault icons from SVG...\n")
    
    # Check if SVG exists
    if not os.path.exists(SVG_PATH):
        print(f"❌ SVG file not found: {SVG_PATH}")
        return
    
    # Generate PNG files
    for size, filename in SIZES:
        output_path = os.path.join(ICONS_DIR, filename)
        generate_png(SVG_PATH, output_path, size)
        print(f"✓ Generated {filename} ({size}x{size})")
    
    # Generate ICO file
    print("\n📦 Generating Windows ICO file...")
    ico_path = os.path.join(ICONS_DIR, "icon.ico")
    generate_ico(SVG_PATH, ico_path)
    print("✓ Generated icon.ico (multi-size)")
    
    # ICNS note
    print("\n🍎 macOS ICNS file:")
    print("  Use online converter or png2icons tool")
    print("  See ICON_GENERATION.md for instructions")
    
    print("\n✨ Icon generation complete!")
    print(f"\nGenerated files in {ICONS_DIR}:")
    for _, filename in SIZES:
        print(f"  - {filename}")
    print("  - icon.ico")

if __name__ == "__main__":
    main()
