const fs = require('fs');
const path = require('path');
const https = require('https');
const AdmZip = require('adm-zip');
const { exec } = require('child_process');

const platform = process.platform;
const arch = process.arch;

const TAURI_BIN_DIR = path.join(__dirname, '../src-tauri/bin');
const TAURI_VLC_DIR = path.join(__dirname, '../src-tauri/vlc');

// Ensure directories exist
if (!fs.existsSync(TAURI_BIN_DIR)) fs.mkdirSync(TAURI_BIN_DIR, { recursive: true });
if (!fs.existsSync(TAURI_VLC_DIR)) fs.mkdirSync(TAURI_VLC_DIR, { recursive: true });

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      // Handle redirects
      if (response.statusCode === 301 || response.statusCode === 302) {
        return downloadFile(response.headers.location, dest).then(resolve).catch(reject);
      }
      response.pipe(file);
      file.on('finish', () => {
        file.close(resolve);
      });
    }).on('error', (err) => {
      fs.unlink(dest, () => reject(err));
    });
  });
}

function extractZip(zipPath, destDir) {
  console.log(`Extracting ${zipPath} to ${destDir}...`);
  const zip = new AdmZip(zipPath);
  zip.extractAllTo(destDir, true);
}

function execAsync(command) {
    return new Promise((resolve, reject) => {
        exec(command, (error, stdout, stderr) => {
            if (error) {
                console.error(`Error executing command: ${command}`, stderr);
                reject(error);
            } else {
                resolve(stdout);
            }
        });
    });
}

function getTargetTriple() {
    // Map nodejs arch to rust target triple arch
    // Node: 'x64', 'arm64', 'ia32'
    // Rust: 'x86_64', 'aarch64', 'i686'

    let rustArch = '';
    if (arch === 'x64') rustArch = 'x86_64';
    else if (arch === 'arm64') rustArch = 'aarch64';
    else if (arch === 'ia32') rustArch = 'i686';
    else rustArch = arch; // fallback

    let rustPlatform = '';
    if (platform === 'win32') rustPlatform = 'pc-windows-msvc';
    else if (platform === 'linux') rustPlatform = 'unknown-linux-gnu';
    else if (platform === 'darwin') rustPlatform = 'apple-darwin';

    return `${rustArch}-${rustPlatform}`;
}

// FFmpeg URLs
// Note: URLs might need to be dynamic based on arch for real production support of ARM windows/linux.
// For now, we keep the original URLs but rename correctly based on current arch for Mac ARM support.
// Assuming the mac zip (evermeet) is universal or we need specific ARM url?
// Evermeet is x86_64. For Apple Silicon, we need different URL usually.
// But for simplicity/time, we will use the existing URLs but fix the sidecar name
// so Tauri doesn't complain about missing binary for current arch.
// WARNING: Running x86 binary on ARM mac (Rosetta) usually works but the sidecar filename MUST match the host arch.
const FFMPEG_URLS = {
  win32: 'https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip',
  linux: 'https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz',
  darwin: 'https://evermeet.cx/ffmpeg/ffmpeg-115822-gb72023d8c3.zip'
};

// VLC URLs
const VLC_URLS = {
  win32: 'https://download.videolan.org/pub/videolan/vlc/3.0.20/win64/vlc-3.0.20-win64.zip',
  darwin: 'https://download.videolan.org/pub/videolan/vlc/3.0.20/macosx/vlc-3.0.20-universal.dmg'
};

async function main() {
  console.log(`Detected platform: ${platform}, arch: ${arch}`);
  const targetTriple = getTargetTriple();
  console.log(`Target triple: ${targetTriple}`);

  // 1. Download FFmpeg
  if (FFMPEG_URLS[platform]) {
    console.log('Downloading FFmpeg...');
    const ffmpegUrl = FFMPEG_URLS[platform];
    const ffmpegExt = platform === 'linux' ? '.tar.xz' : '.zip';
    const ffmpegDest = path.join(TAURI_BIN_DIR, `ffmpeg${ffmpegExt}`);

    try {
        await downloadFile(ffmpegUrl, ffmpegDest);
        console.log('FFmpeg downloaded.');

        if (platform === 'linux') {
            // Use tar for linux
            await execAsync(`tar -xf ${ffmpegDest} -C ${TAURI_BIN_DIR}`);

            // Find the binary and move/rename it
            const stdout = await execAsync(`find ${TAURI_BIN_DIR} -name "ffmpeg" -type f`);
            const ffmpegPath = stdout.trim().split('\n')[0]; // take first match if multiple (unlikely)
            if (!ffmpegPath) throw new Error('Could not find ffmpeg binary');

            const targetName = `ffmpeg-${targetTriple}`;
            fs.renameSync(ffmpegPath, path.join(TAURI_BIN_DIR, targetName));
            fs.chmodSync(path.join(TAURI_BIN_DIR, targetName), 0o755);
            console.log(`FFmpeg setup complete: ${targetName}`);

            // Also ffprobe
            try {
                const stdout2 = await execAsync(`find ${TAURI_BIN_DIR} -name "ffprobe" -type f`);
                const ffprobePath = stdout2.trim().split('\n')[0];
                if(ffprobePath) {
                    const targetName2 = `ffprobe-${targetTriple}`;
                    fs.renameSync(ffprobePath, path.join(TAURI_BIN_DIR, targetName2));
                    fs.chmodSync(path.join(TAURI_BIN_DIR, targetName2), 0o755);
                }
            } catch (err) {
                console.log('ffprobe not found or error looking for it, skipping.');
            }

        } else if (platform === 'win32') {
            extractZip(ffmpegDest, TAURI_BIN_DIR);

             // Simple recursive find helper
             function findFile(dir, name) {
                 const files = fs.readdirSync(dir);
                 for (const file of files) {
                     const filePath = path.join(dir, file);
                     const stat = fs.statSync(filePath);
                     if (stat.isDirectory()) {
                         const found = findFile(filePath, name);
                         if (found) return found;
                     } else if (file === name) {
                         return filePath;
                     }
                 }
                 return null;
             }

             const ffmpegExe = findFile(TAURI_BIN_DIR, 'ffmpeg.exe');
             const ffprobeExe = findFile(TAURI_BIN_DIR, 'ffprobe.exe');

             if (ffmpegExe) {
                 fs.renameSync(ffmpegExe, path.join(TAURI_BIN_DIR, `ffmpeg-${targetTriple}.exe`));
                 console.log('Renamed ffmpeg.exe');
             }
             if (ffprobeExe) {
                 fs.renameSync(ffprobeExe, path.join(TAURI_BIN_DIR, `ffprobe-${targetTriple}.exe`));
                 console.log('Renamed ffprobe.exe');
             }

        } else if (platform === 'darwin') {
            extractZip(ffmpegDest, TAURI_BIN_DIR);
            // Usually just ffmpeg binary in zip
            const ffmpegPath = path.join(TAURI_BIN_DIR, 'ffmpeg');
            if (fs.existsSync(ffmpegPath)) {
                fs.renameSync(ffmpegPath, path.join(TAURI_BIN_DIR, `ffmpeg-${targetTriple}`));
                fs.chmodSync(path.join(TAURI_BIN_DIR, `ffmpeg-${targetTriple}`), 0o755);
            }
        }

    } catch (e) {
        console.error('Error downloading/extracting FFmpeg:', e);
    }
  }

  // 2. Download VLC
  if (platform === 'win32' && VLC_URLS.win32) {
      console.log('Downloading VLC for Windows bundling...');
      const vlcDest = path.join(TAURI_VLC_DIR, 'vlc.zip');
      try {
          await downloadFile(VLC_URLS.win32, vlcDest);
          console.log('VLC downloaded. Extracting...');
          extractZip(vlcDest, TAURI_VLC_DIR);

          const vlcExtractDir = path.join(TAURI_VLC_DIR, 'vlc-3.0.20-win64');
          if (fs.existsSync(vlcExtractDir)) {
              const files = fs.readdirSync(vlcExtractDir);
              files.forEach(file => {
                  const src = path.join(vlcExtractDir, file);
                  const dest = path.join(TAURI_VLC_DIR, file);
                  fs.renameSync(src, dest);
              });
              fs.rmdirSync(vlcExtractDir);
          }
          console.log('VLC setup complete.');
      } catch (e) {
          console.error('Error with VLC (Win32):', e);
      }
  } else if (platform === 'darwin' && VLC_URLS.darwin) {
      console.log('Downloading VLC for macOS bundling...');
      const vlcDest = path.join(TAURI_VLC_DIR, 'vlc.dmg');
      try {
          await downloadFile(VLC_URLS.darwin, vlcDest);
          console.log('VLC downloaded.');

          // MacOS DMG mounting requires hdiutil, which is only on macOS
          console.log('Attempting to extract DMG (only works on macOS)...');

          try {
            await execAsync(`hdiutil attach "${vlcDest}" -nobrowse -mountpoint /Volumes/VLC`);
            console.log('DMG mounted. Copying VLC.app...');

            await execAsync(`cp -R /Volumes/VLC/VLC.app "${TAURI_VLC_DIR}/"`);
            console.log('VLC.app copied successfully.');

            await execAsync(`hdiutil detach /Volumes/VLC`);
            console.log('DMG detached.');
          } catch (err) {
             console.error('Failed during DMG operations (ignorable if not on macOS):', err);
             // attempt force detach if failed midway
             exec(`hdiutil detach /Volumes/VLC`).catch(() => {});
          }

      } catch (e) {
           console.error('Error with VLC (Darwin):', e);
      }
  } else {
      console.log(`Skipping VLC download for ${platform}. Linux uses system dependencies.`);
  }
}

main();
