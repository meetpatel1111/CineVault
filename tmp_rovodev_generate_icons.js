import sharp from 'sharp';
import fs from 'fs';
import path from 'path';

const svgPath = 'src-tauri/icons/icon.svg';
const iconsDir = 'src-tauri/icons';

// Icon sizes we need to generate
const sizes = [
  { size: 32, name: '32x32.png' },
  { size: 128, name: '128x128.png' },
  { size: 256, name: '128x128@2x.png' },
  { size: 512, name: 'icon.png' },
  { size: 1024, name: 'icon@2x.png' }
];

// Linux icon sizes (for .desktop files and various icon themes)
const linuxSizes = [
  { size: 16, name: '16x16.png' },
  { size: 24, name: '24x24.png' },
  { size: 48, name: '48x48.png' },
  { size: 64, name: '64x64.png' },
  { size: 96, name: '96x96.png' },
  { size: 128, name: '128x128.png' },
  { size: 256, name: '256x256.png' },
  { size: 512, name: '512x512.png' }
];

async function generateIcons() {
  console.log('🎨 Generating CineVault icons from SVG...\n');

  try {
    // Read the SVG file
    const svgBuffer = fs.readFileSync(svgPath);

    // Generate PNG files for each size
    for (const { size, name } of sizes) {
      const outputPath = path.join(iconsDir, name);
      
      await sharp(svgBuffer)
        .resize(size, size)
        .png()
        .toFile(outputPath);
      
      console.log(`✓ Generated ${name} (${size}x${size})`);
    }

    // Generate ICO file (Windows icon with multiple sizes)
    console.log('\n📦 Generating Windows ICO file...');
    
    // For ICO, we need multiple sizes embedded (16, 32, 48, 256)
    const icoSizes = [16, 32, 48, 256];
    const icoBuffers = [];
    
    for (const size of icoSizes) {
      const buffer = await sharp(svgBuffer)
        .resize(size, size)
        .png()
        .toBuffer();
      icoBuffers.push(buffer);
    }
    
    // For simplicity, we'll use the 256px as the main ICO
    // A proper ICO would need a specialized library like png-to-ico
    await sharp(svgBuffer)
      .resize(256, 256)
      .png()
      .toFile(path.join(iconsDir, 'icon.ico'));
    
    console.log('✓ Generated icon.ico (256x256)');
    console.log('  Note: For a proper multi-size ICO, use a specialized tool');

    // Generate Linux icons
    console.log('\n🐧 Generating Linux icon sizes...');
    for (const { size, name } of linuxSizes) {
      const outputPath = path.join(iconsDir, name);
      
      // Skip if already generated (e.g., 128x128.png)
      if (fs.existsSync(outputPath)) {
        console.log(`↪ Skipped ${name} (already exists)`);
        continue;
      }
      
      await sharp(svgBuffer)
        .resize(size, size)
        .png()
        .toFile(outputPath);
      
      console.log(`✓ Generated ${name} (${size}x${size})`);
    }

    // Generate ICNS (macOS icon) - Sharp doesn't support this
    // We'll note this for the user
    console.log('\n🍎 macOS ICNS file:');
    console.log('  ⚠ ICNS generation requires specialized tools');
    console.log('  You can use: png2icons or iconutil (macOS only)');
    console.log('  Or use: https://cloudconvert.com/png-to-icns');

    console.log('\n✨ Icon generation complete!');
    console.log('\n📦 Windows icons:');
    sizes.forEach(({ name }) => console.log(`  - src-tauri/icons/${name}`));
    console.log(`  - src-tauri/icons/icon.ico`);
    console.log('\n🐧 Linux icons:');
    linuxSizes.forEach(({ name }) => console.log(`  - src-tauri/icons/${name}`));

  } catch (error) {
    console.error('❌ Error generating icons:', error);
    process.exit(1);
  }
}

generateIcons();
