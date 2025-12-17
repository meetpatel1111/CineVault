# CineVault Icon Generation

## What Was Created

I've created a beautiful SVG icon for CineVault featuring:
- **Film vault theme**: Combines a vault door with a film reel
- **Color scheme**: Dark blue/grey vault with red/orange film reel accents
- **Film strip accent**: Top decoration showing the media focus
- **Professional look**: Gradient effects and proper shadows

**Location**: `src-tauri/icons/icon.svg`

## How to Generate All Icon Formats

You have **two options** for generating icons:

### Option 1: Node.js Script (Recommended)

Open a terminal in the project root and run:

```bash
npm run generate-icons
```

### Option 2: Python Script

If you prefer Python or have issues with Node:

```bash
# Install dependencies first
pip install cairosvg pillow

# Run the script
python tmp_rovodev_generate_icons.py
```

This will automatically create:
- `32x32.png` - Small icon
- `128x128.png` - Medium icon
- `128x128@2x.png` - Retina medium (256x256)
- `icon.png` - Large icon (512x512)
- `icon@2x.png` - Retina large (1024x1024)
- `icon.ico` - Windows icon (256x256)

### Step 2: Generate macOS ICNS (Optional)

For the macOS `.icns` file, you have a few options:

#### Option A: Use Online Converter (Easiest)
1. Go to https://cloudconvert.com/png-to-icns
2. Upload `src-tauri/icons/icon.png`
3. Convert and download as `icon.icns`
4. Place in `src-tauri/icons/`

#### Option B: Use Command Line (macOS only)
```bash
# Create iconset directory
mkdir icon.iconset

# Copy and resize (requires sips on macOS)
sips -z 16 16 src-tauri/icons/icon.png --out icon.iconset/icon_16x16.png
sips -z 32 32 src-tauri/icons/icon.png --out icon.iconset/icon_16x16@2x.png
# ... (repeat for all sizes)

# Convert to ICNS
iconutil -c icns icon.iconset -o src-tauri/icons/icon.icns
```

#### Option C: Use png2icons (Cross-platform)
```bash
npm install -g png2icons
png2icons src-tauri/icons/icon.png src-tauri/icons/icon.icns
```

## Update Tauri Configuration

The `tauri.conf.json` has already been updated to reference these icons:

```json
"icon": [
  "icons/32x32.png",
  "icons/icon.ico"
]
```

For production builds, Tauri will automatically look for:
- `icon.icns` (macOS)
- `icon.ico` (Windows)
- `*.png` files (Linux)

## View the Icon

To preview the SVG icon:
1. Open `src-tauri/icons/icon.svg` in any web browser
2. Or use VS Code's SVG preview extension

## Customization

To modify the icon design, edit `src-tauri/icons/icon.svg` and re-run the generation script.

Key design elements you can customize:
- Colors (search for color codes in the SVG)
- Vault ring thickness
- Film reel hole positions
- Gradient effects

## Troubleshooting

**Error: Cannot find module 'sharp'**
- Run: `npm install` to install dependencies

**Icons look blurry**
- The SVG is 512x512, which should scale well
- For better quality at specific sizes, edit the SVG before generating

**ICO file doesn't work on Windows**
- The generated ICO is a single-size PNG converted
- For multi-resolution ICO, use specialized tools like `png-to-ico`
