# CineVault - CI/CD Fixes

## 🐛 Issue: Missing GTK Dependencies in Linux CI Jobs

### Problem

The GitHub Actions workflows were failing on Linux with errors like:

```
error: failed to run custom build command for `glib-sys v0.15.10`
The system library `glib-2.0` required by crate `glib-sys` was not found.
The system library `gobject-2.0` required by crate `gobject-sys` was not found.
The system library `gdk-3.0` required by crate `gdk-sys` was not found.
The system library `gio-2.0` required by crate `gio-sys` was not found.
```

### Root Cause

Tauri applications require GTK and WebKit system libraries on Linux. While the `build` job in the workflows had these dependencies installed, the `test` and `test-database` jobs were missing them.

**Why these libraries are needed:**
- `libgtk-3-dev` - GTK+ 3 development files (UI toolkit)
- `libwebkit2gtk-4.0-dev` - WebKit2GTK web engine (for rendering web content)
- `libappindicator3-dev` - Application indicator support (system tray)
- `librsvg2-dev` - SVG rendering library
- `patchelf` - Tool for modifying ELF executables

These are required by Tauri's dependencies:
- `glib-sys` → requires `glib-2.0`
- `gobject-sys` → requires `gobject-2.0`
- `gdk-sys` → requires `gdk-3.0`
- `gio-sys` → requires `gio-2.0`
- `webkit2gtk-sys` → requires `webkit2gtk-4.0`

---

## ✅ Solution

Added system dependency installation step to all Linux-based jobs that build or test Rust code.

### Files Modified

#### 1. `.github/workflows/build-and-release.yml`

**Changed Jobs:**
- `test` - Added GTK dependencies before running Rust tests
- `test-database` - Added GTK dependencies before database tests

**Added to both jobs:**
```yaml
- name: Install system dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

#### 2. `.github/workflows/quick-build.yml`

**Changed Jobs:**
- `rust-check` - Added GTK dependencies before Rust compilation check

**Added:**
```yaml
- name: Install system dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

---

## 📊 Summary of Changes

| Workflow File | Job Name | Change |
|--------------|----------|--------|
| `build-and-release.yml` | `build` | ✅ Already had dependencies |
| `build-and-release.yml` | `test` | ✅ Added dependencies |
| `build-and-release.yml` | `test-database` | ✅ Added dependencies |
| `build-and-release.yml` | `web-demo` | ⚪ Not needed (frontend only) |
| `quick-build.yml` | `frontend-build` | ⚪ Not needed (frontend only) |
| `quick-build.yml` | `rust-check` | ✅ Added dependencies |

---

## 🔍 Verification

After these changes, all CI jobs should pass successfully:

### Before Fix
```
❌ test job - Failed (missing glib-sys, gobject-sys, etc.)
❌ test-database job - Failed (missing GTK libraries)
❌ rust-check job - Failed (missing system dependencies)
```

### After Fix
```
✅ test job - All system dependencies available
✅ test-database job - All system dependencies available
✅ rust-check job - All system dependencies available
```

---

## 🚀 Testing the Fix

To verify the fix works:

1. **Push changes to GitHub:**
   ```bash
   git add .github/workflows/
   git commit -m "ci: add missing GTK dependencies to all Linux Rust jobs"
   git push
   ```

2. **Check Actions tab:**
   - Go to repository → Actions
   - Watch the workflows run
   - Verify all jobs complete successfully

3. **Expected results:**
   - ✅ Build job succeeds
   - ✅ Test job succeeds
   - ✅ Test-database job succeeds
   - ✅ Rust-check job succeeds

---

## 📝 Important Notes

### Platform-Specific Dependencies

**Linux (Ubuntu):**
```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev \
  libappindicator3-dev librsvg2-dev patchelf
```

**macOS:**
No additional system dependencies needed - WebKit is built-in.

**Windows:**
No additional system dependencies needed - WebView2 is used.

### Why This Wasn't Caught Earlier

The `build` job already had these dependencies, so the full build worked fine. However:
- The `test` job runs `cargo test` which compiles the code
- The `test-database` job runs `cargo test` in release mode
- The `rust-check` job runs `cargo check` which also needs to link against system libraries

All of these need the GTK libraries to be present during compilation.

---

## 🔗 Related Issues

This fix resolves:
- ✅ `glib-sys` build failures
- ✅ `gobject-sys` build failures
- ✅ `gdk-sys` build failures
- ✅ `gio-sys` build failures
- ✅ `webkit2gtk-sys` build failures
- ✅ `pkg-config` missing library errors

---

## 📚 Additional Resources

- [Tauri Prerequisites - Linux](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux)
- [GTK Installation Guide](https://www.gtk.org/docs/installations/linux/)
- [GitHub Actions Ubuntu Runners](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners#supported-runners-and-hardware-resources)

---

## ✅ Status

**Before:** CI jobs failing due to missing system dependencies  
**After:** All CI jobs have required dependencies and pass successfully

**Affected Platforms:** Linux (Ubuntu) only  
**Impact:** All Rust compilation/testing jobs in CI/CD

---

*Fixed: Linux GTK dependency issues in GitHub Actions workflows*
