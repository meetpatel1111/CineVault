import png2icons from 'png2icons';
import fs from 'fs';
import path from 'path';

const iconPath = 'src-tauri/icons/icon.png';
const outputPath = 'src-tauri/icons/icon.icns';

async function generateICNS() {
  console.log('🍎 Generating macOS ICNS file...\n');

  try {
    // Read the PNG file
    const input = fs.readFileSync(iconPath);
    
    console.log('📖 Read source PNG:', iconPath);
    console.log('📏 Size:', (input.length / 1024).toFixed(2), 'KB');
    
    // Generate ICNS with high quality (BILINEAR interpolation)
    console.log('\n⚙️  Converting to ICNS format...');
    const output = png2icons.createICNS(input, png2icons.BILINEAR, 0);
    
    // Write the ICNS file
    fs.writeFileSync(outputPath, output);
    
    console.log('✅ Generated:', outputPath);
    console.log('📏 Size:', (output.length / 1024).toFixed(2), 'KB');
    
    console.log('\n✨ ICNS generation complete!');
    console.log('\n📋 The ICNS file contains multiple icon sizes:');
    console.log('   - 16x16, 32x32, 64x64, 128x128, 256x256, 512x512, 1024x1024');
    console.log('   - Including @2x retina versions');
    console.log('\n🎯 Ready for macOS builds!');
    
  } catch (error) {
    console.error('❌ Error generating ICNS:', error.message);
    console.error('\nTroubleshooting:');
    console.error('  1. Make sure icon.png exists in src-tauri/icons/');
    console.error('  2. Ensure the PNG is a valid image file');
    console.error('  3. Try using a 1024x1024 PNG for best results');
    process.exit(1);
  }
}

generateICNS();
