import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import https from 'https';
import { execSync } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');
const srcTauriDir = path.join(rootDir, 'src-tauri');
const binariesDir = path.join(srcTauriDir, 'binaries');
const vlcDir = path.join(srcTauriDir, 'vlc');

// Configuration
const FFMPEG_VERSION = '6.1';
const VLC_VERSION = '3.0.20';
const USER_AGENT = 'CineVault-Installer/1.0';

// Platform detection
const platform = process.platform; // 'win32', 'linux', 'darwin'
const arch = process.arch; // 'x64', 'arm64'

console.log(`Setting up dependencies for ${platform}-${arch}...`);

// Ensure directories exist
if (!fs.existsSync(binariesDir)) fs.mkdirSync(binariesDir, { recursive: true });
if (!fs.existsSync(vlcDir)) fs.mkdirSync(vlcDir, { recursive: true });

// --- Helpers ---

function getTargetTriple() {
    if (platform === 'win32') return 'x86_64-pc-windows-msvc';
    if (platform === 'darwin') return arch === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
    if (platform === 'linux') return 'x86_64-unknown-linux-gnu';
    return '';
}

async function downloadFile(url, destPath) {
    console.log(`Downloading ${url}...`);
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(destPath);
        const request = https.get(url, { headers: { 'User-Agent': USER_AGENT } }, response => {
            if (response.statusCode === 302 || response.statusCode === 301) {
                downloadFile(response.headers.location, destPath).then(resolve).catch(reject);
                return;
            }
            if (response.statusCode !== 200) {
                reject(new Error(`Failed to download: ${response.statusCode}`));
                return;
            }
            response.pipe(file);
            file.on('finish', () => {
                file.close();
                resolve();
            });
        });
        request.on('error', err => {
            fs.unlink(destPath, () => {});
            reject(err);
        });
    });
}

function extractZip(zipPath, destDir) {
    console.log(`Extracting ${zipPath}...`);
    try {
        if (platform === 'win32') {
            // Use PowerShell to extract
            execSync(`powershell -command "Expand-Archive -Force '${zipPath}' '${destDir}'"`);
        } else {
            // Use unzip
            execSync(`unzip -o "${zipPath}" -d "${destDir}"`);
        }
    } catch (e) {
        console.error("Extraction failed (zip):", e.message);
        throw e;
    }
}

function extractTar(tarPath, destDir) {
    console.log(`Extracting ${tarPath}...`);
    try {
        // tar -xf file -C dest
        if (!fs.existsSync(destDir)) fs.mkdirSync(destDir, { recursive: true });
        execSync(`tar -xf "${tarPath}" -C "${destDir}"`);
    } catch (e) {
        console.error("Extraction failed (tar):", e.message);
        throw e;
    }
}

// --- Tasks ---

async function setupFFmpeg() {
    const target = getTargetTriple();
    const exeExt = platform === 'win32' ? '.exe' : '';
    const ffmpegBinaryName = `ffmpeg-${target}${exeExt}`;
    const ffprobeBinaryName = `ffprobe-${target}${exeExt}`;

    const ffmpegDest = path.join(binariesDir, ffmpegBinaryName);
    const ffprobeDest = path.join(binariesDir, ffprobeBinaryName);

    if (fs.existsSync(ffmpegDest) && fs.existsSync(ffprobeDest)) {
        console.log('FFmpeg binaries already exist.');
        return;
    }

    // Using mwader/static-ffmpeg or similar is popular, or ffbinaries.
    // For reliability, we use github releases from `eugeneware/ffmpeg-static` or `BtbN`.
    // BtbN is good for Windows/Linux.
    // Mac: evermeet.cx or OSXExperts?

    // Let's use `ffbinaries` approach via specific URLs if possible, or a consistent GitHub repo.
    // `ffmpeg-static` uses:
    // Win: https://github.com/eugeneware/ffmpeg-static/releases/download/b6.0/ffmpeg-win32-x64.gz (Just gz? Need to check).

    // Simplification: Use direct links to a known working release.
    let url = '';
    let archiveName = '';
    let isZip = false;

    // Source: BtbN/FFmpeg-Builds (GPL)
    // https://github.com/BtbN/FFmpeg-Builds/releases
    // Win: ffmpeg-master-latest-win64-gpl.zip
    // Linux: ffmpeg-master-latest-linux64-gpl.tar.xz
    // Mac: https://evermeet.cx/ffmpeg/ffmpeg-6.1.1.zip

    // Using a specific commit/version is safer than 'latest'.

    if (platform === 'win32') {
        url = 'https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip';
        archiveName = 'ffmpeg.zip';
        isZip = true;
    } else if (platform === 'linux') {
        url = 'https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz';
        archiveName = 'ffmpeg.tar.xz';
    } else if (platform === 'darwin') {
        url = 'https://evermeet.cx/ffmpeg/ffmpeg-6.1.1.zip'; // ffmpeg only
        archiveName = 'ffmpeg.zip';
        isZip = true;
        // evermeet only provides ffmpeg. ffprobe is separate.
    }

    const tempDir = path.join(srcTauriDir, 'temp_ffmpeg');
    if (!fs.existsSync(tempDir)) fs.mkdirSync(tempDir);
    const archivePath = path.join(tempDir, archiveName);

    await downloadFile(url, archivePath);

    if (isZip) {
        extractZip(archivePath, tempDir);
    } else {
        extractTar(archivePath, tempDir);
    }

    // Move binaries
    // BtbN zip structure: ffmpeg-master-latest-win64-gpl/bin/ffmpeg.exe
    // Linux tar structure: ffmpeg-master-latest-linux64-gpl/bin/ffmpeg

    // Find the binary in tempDir (recursive search)
    function findFile(dir, name) {
        const files = fs.readdirSync(dir);
        for (const file of files) {
            const fullPath = path.join(dir, file);
            if (fs.statSync(fullPath).isDirectory()) {
                const found = findFile(fullPath, name);
                if (found) return found;
            } else if (file === name) {
                return fullPath;
            }
        }
        return null;
    }

    const ffmpegSrc = findFile(tempDir, `ffmpeg${exeExt}`);
    if (ffmpegSrc) {
        fs.copyFileSync(ffmpegSrc, ffmpegDest);
        console.log(`Copied ffmpeg to ${ffmpegDest}`);
    } else {
        throw new Error("Could not find ffmpeg binary in extracted archive");
    }

    // For Mac (evermeet), we need to download ffprobe separately
    if (platform === 'darwin') {
        const ffprobeUrl = 'https://evermeet.cx/ffmpeg/ffprobe-6.1.1.zip';
        const ffprobeArchive = path.join(tempDir, 'ffprobe.zip');
        await downloadFile(ffprobeUrl, ffprobeArchive);
        extractZip(ffprobeArchive, tempDir);
        const ffprobeSrc = findFile(tempDir, 'ffprobe');
        if (ffprobeSrc) fs.copyFileSync(ffprobeSrc, ffprobeDest);
    } else {
         const ffprobeSrc = findFile(tempDir, `ffprobe${exeExt}`);
         if (ffprobeSrc) {
            fs.copyFileSync(ffprobeSrc, ffprobeDest);
         }
    }

    // Cleanup
    // fs.rmSync(tempDir, { recursive: true, force: true });
    console.log("FFmpeg setup complete.");
}

async function setupVLC() {
    if (platform === 'linux') {
        console.log('Skipping VLC bundle for Linux (using system dependency).');
        return;
    }

    // Check if already exists
    if (platform === 'win32' && fs.existsSync(path.join(vlcDir, 'libvlc.dll'))) {
         console.log('VLC already set up.');
         return;
    }

    const tempDir = path.join(srcTauriDir, 'temp_vlc');
    if (!fs.existsSync(tempDir)) fs.mkdirSync(tempDir);

    if (platform === 'win32') {
        const url = `https://get.videolan.org/vlc/${VLC_VERSION}/win64/vlc-${VLC_VERSION}-win64.zip`;
        const archivePath = path.join(tempDir, 'vlc.zip');
        await downloadFile(url, archivePath);
        extractZip(archivePath, tempDir);

        // Move files
        // Structure: vlc-3.0.20/libvlc.dll
        const vlcRoot = path.join(tempDir, `vlc-${VLC_VERSION}`);

        const filesToCopy = ['libvlc.dll', 'libvlccore.dll', 'plugins'];
        for (const item of filesToCopy) {
            const src = path.join(vlcRoot, item);
            const dest = path.join(vlcDir, item);
            if (fs.existsSync(src)) {
                if (fs.statSync(src).isDirectory()) {
                    // Recursive copy (simple version)
                    // Node 16.7+ has fs.cpSync
                    fs.cpSync(src, dest, { recursive: true });
                } else {
                    fs.copyFileSync(src, dest);
                }
            }
        }
    } else if (platform === 'darwin') {
        // MacOS
        // Assuming we are building ON Mac.
        try {
             const url = `https://get.videolan.org/vlc/${VLC_VERSION}/macosx/vlc-${VLC_VERSION}-intel64.dmg`;
             const archivePath = path.join(tempDir, 'vlc.dmg');
             await downloadFile(url, archivePath);

             // Mount DMG
             console.log("Mounting DMG...");
             execSync(`hdiutil attach "${archivePath}" -mountpoint "${tempDir}/mount"`);

             // Copy LibVLC
             // VLC.app/Contents/MacOS/lib/libvlc.dylib
             // Plugins: VLC.app/Contents/MacOS/plugins
             const mountPoint = path.join(tempDir, 'mount');
             const vlcApp = path.join(mountPoint, 'VLC.app');
             const libDir = path.join(vlcApp, 'Contents/MacOS/lib');
             const pluginsDir = path.join(vlcApp, 'Contents/MacOS/plugins');

             if (fs.existsSync(libDir)) {
                  fs.cpSync(libDir, path.join(vlcDir, 'lib'), { recursive: true });
             }
             if (fs.existsSync(pluginsDir)) {
                  fs.cpSync(pluginsDir, path.join(vlcDir, 'plugins'), { recursive: true });
             }

             // Unmount
             execSync(`hdiutil detach "${mountPoint}"`);
        } catch (e) {
            console.warn("MacOS VLC setup failed (might not be on Mac or missing tools). Skipping.", e.message);
        }
    }

    // Cleanup
    // fs.rmSync(tempDir, { recursive: true, force: true });
    console.log("VLC setup complete.");
}

// --- Main ---

(async () => {
    try {
        await setupFFmpeg();
        await setupVLC();
        console.log('\nDependency setup complete.');
    } catch (e) {
        console.error('Setup failed:', e);
        process.exit(1);
    }
})();
