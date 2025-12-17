# CineVault Icons

## 🎨 Icon Design

The CineVault icon features:
- **Film vault theme**: A combination of a secure vault door with a film reel
- **Color palette**: 
  - Dark blue/grey vault background (#1a1a2e, #16213e)
  - Red/crimson film reel (#e94560, #d63447)
  - Gold/orange accents (#f39c12, #e67e22)
- **Film strip decoration**: At the top to emphasize media management

## 📁 Icon Files

**Source File:**
- `icon.svg` - Vector source (512x512), editable

**Generated Files:**
- `32x32.png` - Small icon
- `128x128.png` - Medium icon  
- `128x128@2x.png` - Retina medium (256x256)
- `icon.png` - Large icon (512x512)
- `icon@2x.png` - Retina large (1024x1024)
- `icon.ico` - Windows icon (multi-size)
- `icon.icns` - macOS icon (optional, for production)

## 🔨 Generating Icons

From the project root directory:

```bash
# Node.js method (recommended)
npm run generate-icons

# OR Python method
python tmp_rovodev_generate_icons.py
```

See `ICON_GENERATION.md` in the project root for detailed instructions.

## ✏️ Customizing the Icon

To modify the design:
1. Edit `icon.svg` in any vector editor or text editor
2. Re-run the generation script
3. Test the new icons in your build

## 🔗 Platform Requirements

- **Windows**: `icon.ico` (included)
- **macOS**: `icon.icns` (use online converter or png2icons)
- **Linux**: PNG files (included)
