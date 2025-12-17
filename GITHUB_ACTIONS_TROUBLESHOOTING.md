# GitHub Actions - How to Read Build Logs

## 🎯 Quick Guide: Finding Build Status

### Step 1: Go to Actions Tab
```
https://github.com/YOUR_USERNAME/YOUR_REPO/actions
```

### Step 2: Identify Workflow Status

You'll see one of these:

#### ✅ Success (Green Checkmark)
```
✅ Build CineVault
   Completed in 15m 23s
```
**Action**: Download artifacts and test!

#### ❌ Failed (Red X)
```
❌ Build CineVault
   Failed after 5m 12s
```
**Action**: Click to see which step failed

#### 🟡 In Progress (Yellow Dot)
```
🟡 Build CineVault
   Running for 8m 45s
```
**Action**: Wait for completion (first build: 10-15 min)

---

## 📋 Understanding Log Output

### Normal Environment Setup (NOT ERRORS)
These lines are **normal** and appear in every build:

```bash
# Environment Variables (Configuration)
CARGO_TERM_COLOR: always
CARGO_HOME: C:\Users\runneradmin\.cargo
CARGO_INCREMENTAL: 0
NODE_VERSION: 20

# Shell Information (Normal)
shell: C:\Program Files\PowerShell\7\pwsh.EXE -command ". '{0}'"

# Setup Steps (Should all have ✅)
✅ Checkout repository
✅ Setup Node.js
✅ Install Rust stable
✅ Rust cache
```

### Actual Build Progress (What to Watch)
```bash
# Frontend Build
✅ Install frontend dependencies
   npm ci
   added 245 packages in 12s

✅ Build frontend
   npm run build
   ✓ built in 4.34s

# Backend Build (Takes longest)
⏳ Build Tauri app
   Compiling proc-macro2 v1.0.103
   Compiling serde v1.0.228
   Compiling tokio v1.48.0
   ... (400+ crates)
   Compiling tauri v1.8.3
   Finished release [optimized] target(s) in 8m 45s

✅ Upload build artifacts
   Uploaded artifact (125 MB)
```

### Error Messages (What Actual Errors Look Like)
```bash
❌ Build frontend
   Error: TypeScript compilation failed
   src/App.tsx(10,5): error TS2322: Type 'string' is not assignable to type 'number'

❌ Build Tauri app
   Error: could not compile `cinevault`
   error[E0425]: cannot find value `variable_name` in this scope

❌ Upload artifacts
   Error: ENOENT: no such file or directory
```

---

## 🔍 How to Find Actual Errors

### Method 1: Scroll to the Bottom
The most recent output is at the bottom. Scroll down to see:
- ✅ All steps completed successfully
- ❌ Failed at step X with error message

### Method 2: Look for Red Text
- Red text = errors
- Green text = success
- White/gray text = normal output

### Method 3: Check Step Status
Click on each step to expand:
```
✅ Checkout repository (2s)           <- Click to expand
✅ Setup Node.js (8s)                 <- Click to expand
✅ Install dependencies (45s)         <- Click to expand
❌ Build frontend (12s)               <- ERROR HERE! Click to see details
⊘ Build Tauri app (skipped)          <- Skipped because previous failed
```

---

## 📊 Current Workflow Jobs

Your workflow has these jobs:

### 1. **build** (3 parallel jobs)
- `build (ubuntu-22.04)` - Linux build
- `build (windows-latest)` - Windows build
- `build (macos-latest)` - macOS build

Each takes 10-15 minutes on first run.

### 2. **web-demo**
- Deploys frontend preview to GitHub Pages
- Takes 2-3 minutes

### 3. **test**
- Runs TypeScript and Rust tests
- Takes 3-5 minutes

### 4. **test-database**
- Tests database migrations
- Takes 2-3 minutes

---

## ⏱️ Expected Timeline

### First Build (Cold Cache)
```
0-2 min:   Checkout, setup Node/Rust
2-3 min:   Install npm dependencies
3-4 min:   Build frontend
4-15 min:  Compile Rust (400+ crates) ⏳ LONGEST STEP
15-16 min: Create installers
16-17 min: Upload artifacts
```

### Subsequent Builds (Warm Cache)
```
0-2 min:   Checkout, setup (cached)
2-3 min:   Install dependencies (cached)
3-4 min:   Build frontend
4-6 min:   Compile Rust (cached, only changed files)
6-7 min:   Create installers
7-8 min:   Upload artifacts
```

---

## 🎯 What to Do Based on Status

### ✅ If All Green (Success)
1. Click on the completed workflow run
2. Scroll to "Artifacts" section at bottom
3. Download artifacts:
   - `cinevault-windows-latest` (Windows .msi)
   - `cinevault-macos-latest` (macOS .dmg)
   - `cinevault-ubuntu-22.04` (Linux AppImage)
4. Install and test!

### ❌ If Red X (Failed)
1. Click on the failed job (red X)
2. Find the failed step (red X)
3. Click to expand the step
4. Copy the **red error text**
5. Share it with me for a fix

### 🟡 If Yellow Dot (Running)
1. Wait patiently (10-15 minutes for first build)
2. Watch the progress in real-time
3. When you see "Compiling tauri" you're almost done!

---

## 🐛 Common Issues & Solutions

### Issue 1: "npm ERR! code ELIFECYCLE"
**Cause**: Frontend build failed (TypeScript error)
**Solution**: We already fixed all TS errors, this shouldn't happen

### Issue 2: "error: could not compile `cinevault`"
**Cause**: Rust compilation error
**Solution**: Share the specific error for a fix

### Issue 3: "Error: No artifacts found"
**Cause**: Build didn't create expected files
**Solution**: Check if previous steps succeeded

### Issue 4: "Workflow run timed out"
**Cause**: Build took longer than 6 hours (GitHub limit)
**Solution**: This shouldn't happen, typical build is 10-15 min

---

## 📱 Quick Checklist

To help me help you, tell me:

- [ ] What is the workflow status? (✅ ❌ or 🟡)
- [ ] Which job failed? (build/web-demo/test/test-database)
- [ ] Which step failed? (checkout/install/build frontend/build tauri)
- [ ] What is the error message? (copy red text)

---

## 💡 Pro Tip: Re-run Failed Workflows

If a build fails due to temporary issues:
1. Click "Re-run jobs" button (top right)
2. Select "Re-run failed jobs"
3. Wait for it to complete

---

## 🎬 Example: Successful Build Log

```bash
Run actions/checkout@v4
  Syncing repository...
  ✅ Checked out commit abc1234

Run actions/setup-node@v4
  Version: 20.11.0
  ✅ Successfully setup Node.js

Run npm ci
  added 245 packages in 12.3s
  ✅ Dependencies installed

Run npm run build
  vite v5.4.21 building for production...
  ✓ 87 modules transformed
  ✓ built in 4.34s
  ✅ Frontend built successfully

Run tauri-apps/tauri-action@v0
  Compiling cinevault v0.1.0
  Compiling 403 crates...
  Finished release [optimized] target(s) in 8m 45s
  Creating installers...
  Created: cinevault_0.1.0_x64.msi
  ✅ Tauri app built successfully

Run actions/upload-artifact@v4
  Uploaded: cinevault-windows-latest (125 MB)
  ✅ Artifacts uploaded successfully
```

This is what a **successful build** looks like!

---

## 🆘 Still Confused?

Take a screenshot of your GitHub Actions page and I'll tell you exactly what's happening!

Or simply tell me:
- "Build is still running" ⏳
- "Build succeeded" ✅  
- "Build failed at [step name]" ❌

Then I can help appropriately!
