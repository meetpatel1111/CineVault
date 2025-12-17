# GitHub Actions Setup for CineVault

## 🚀 Automated Workflows

I've created two GitHub Actions workflows for building and deploying CineVault.

---

## Workflow 1: Build and Release
**File**: `.github/workflows/build-and-release.yml`

### What It Does:
1. **Multi-Platform Builds**: Builds for Windows, macOS, and Linux
2. **Automated Testing**: Runs Rust and TypeScript tests
3. **Artifact Upload**: Packages executables for download
4. **Draft Releases**: Creates draft releases with installers

### Triggers:
- Push to `main` or `develop` branch
- Pull requests to `main`
- Manual trigger via GitHub UI

### Jobs:
- **build**: Compiles app for all platforms
- **web-demo**: Deploys frontend preview
- **test**: Runs frontend and backend tests
- **test-database**: Tests database migrations

### Outputs:
- Windows: `.msi` and `.exe` installers
- macOS: `.dmg` and `.app.tar.gz`
- Linux: `.AppImage` and `.deb` packages

---

## Workflow 2: Deploy Demo
**File**: `.github/workflows/deploy-demo.yml`

### What It Does:
1. Builds frontend for web preview
2. Deploys to GitHub Pages
3. Creates info page explaining it's a desktop app

### Triggers:
- Push to `main` branch
- Manual trigger via GitHub UI

### Output URL:
```
https://YOUR_USERNAME.github.io/YOUR_REPO/
```

---

## 📋 Setup Instructions

### 1. Enable GitHub Pages
```
Repository Settings → Pages → Source: GitHub Actions
```

### 2. Configure Repository Settings
```bash
# Repository Settings → Actions → General
✅ Allow all actions and reusable workflows
✅ Read and write permissions for GITHUB_TOKEN
```

### 3. Update URLs in deploy-demo.yml
Replace placeholders:
- `YOUR_USERNAME` → Your GitHub username
- `YOUR_REPO` → Your repository name

### 4. Commit and Push Workflows
```bash
git add .github/
git commit -m "Add GitHub Actions workflows"
git push origin main
```

---

## 🎯 Accessing Builds

### After Push to Main:

1. **View Workflow Runs**:
   ```
   https://github.com/YOUR_USERNAME/YOUR_REPO/actions
   ```

2. **Download Artifacts**:
   - Go to completed workflow run
   - Scroll to "Artifacts" section
   - Download for your platform:
     - `cinevault-windows-latest`
     - `cinevault-macos-latest`
     - `cinevault-ubuntu-22.04`

3. **Frontend Preview**:
   ```
   https://YOUR_USERNAME.github.io/YOUR_REPO/
   ```
   (Shows UI preview + download instructions)

---

## 🌐 Accessing via Public URL

### Important Note:
**CineVault is a desktop application**, not a web app. You cannot run it directly in a browser because it requires:
- File system access
- Native media codecs
- SQLite database
- Tauri runtime

### What You CAN Access:

#### 1. Frontend UI Preview (GitHub Pages)
```
https://YOUR_USERNAME.github.io/YOUR_REPO/
```
**Shows:**
- Visual preview of the UI
- Mock data display
- Design system demo
- Download instructions

**Limitations:**
- ❌ Cannot scan files (no file system access)
- ❌ Cannot play media (no Tauri backend)
- ❌ No database functionality
- ✅ Can see UI components and design

#### 2. Download Installers (GitHub Releases)
```
https://github.com/YOUR_USERNAME/YOUR_REPO/releases
```
**Provides:**
- Windows installer (.msi)
- macOS disk image (.dmg)
- Linux packages (.AppImage, .deb)

#### 3. View Build Logs
```
https://github.com/YOUR_USERNAME/YOUR_REPO/actions
```
**Shows:**
- Build status
- Test results
- Compilation logs
- Artifact downloads

---

## 🔧 Alternative: Web-Accessible Demo

If you want a **fully functional web demo**, you'd need to:

### Option A: Convert to Web App (Not Recommended)
- Replace Tauri with Electron or pure web
- Use browser APIs for file access (limited)
- Lose native performance and features
- Requires significant refactoring

### Option B: Cloud-Based Demo Instance
Deploy a demo server with:
1. **Backend**: Host Rust backend on cloud VM
2. **Frontend**: Deploy React to Vercel/Netlify
3. **Storage**: Cloud storage for demo media
4. **Streaming**: Stream media through web API

**Setup:**
```bash
# Example with Docker
docker build -t cinevault-demo .
docker run -p 8080:8080 cinevault-demo
```

But this defeats CineVault's **offline-first, privacy-focused** purpose!

---

## 📦 Recommended Approach: GitHub Releases

### Best Way to Share CineVault:

1. **Push to GitHub**:
   ```bash
   git push origin main
   ```

2. **Workflow Builds Automatically**:
   - Windows .msi installer
   - macOS .dmg installer  
   - Linux AppImage

3. **Create Release**:
   ```bash
   # Workflow creates draft release
   # Edit and publish in GitHub UI
   ```

4. **Share Release URL**:
   ```
   https://github.com/YOUR_USERNAME/YOUR_REPO/releases/latest
   ```

5. **Users Download and Install**:
   - Windows: Run .msi installer
   - macOS: Open .dmg and drag to Applications
   - Linux: Make AppImage executable and run

---

## 🎬 Demo Video Alternative

Since CineVault can't run in a browser, consider:

### Create a Demo Video:
```bash
# Record a demo video showing:
1. Installation process
2. Scanning media folder
3. Playing a video
4. Resume functionality
5. UI navigation

# Upload to:
- YouTube
- GitHub README (embed)
- Loom or similar
```

### Or: Interactive Screenshots
```bash
# Take screenshots and create interactive demo
- Use tools like: ScreenToGif, Peek
- Upload to GitHub wiki
- Link from README
```

---

## 🔐 Security Notes

### GitHub Secrets Required:
- `GITHUB_TOKEN` - Automatically provided by GitHub

### Optional Secrets (for signing):
- `APPLE_CERTIFICATE` - For macOS code signing
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_ID` - For notarization
- `WINDOWS_CERTIFICATE` - For Windows code signing

---

## 📊 Workflow Status Badge

Add to your README.md:
```markdown
![Build Status](https://github.com/YOUR_USERNAME/YOUR_REPO/actions/workflows/build-and-release.yml/badge.svg)
```

---

## 🎯 Summary

### What GitHub Actions Provides:
✅ **Automated builds** for 3 platforms  
✅ **Test execution** on every commit  
✅ **Artifact downloads** from Actions tab  
✅ **Frontend preview** on GitHub Pages  
✅ **Draft releases** with installers  

### What You CANNOT Get:
❌ **Direct browser access** to full app  
❌ **Public URL** with full functionality  
❌ **Web-based media playback** (without major refactoring)  

### Best Solution:
**Share GitHub Release URL** → Users download and install → Full functionality!

---

## 💡 Quick Test Workflow

```bash
# 1. Push to GitHub
git add .github/
git commit -m "Add CI/CD workflows"
git push origin main

# 2. Watch workflow run
# Go to: https://github.com/YOUR_USERNAME/YOUR_REPO/actions

# 3. Download artifacts when complete
# Click on workflow run → Artifacts section

# 4. Install on your machine
# Run the downloaded installer

# 5. Share release URL with testers
# https://github.com/YOUR_USERNAME/YOUR_REPO/releases
```

---

**This is the standard approach for desktop apps like VS Code, Discord, Slack, etc.**
