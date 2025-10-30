# Plugin Directory Junction

This `plugins` folder is a **junction** (symlink) pointing to `../../src-tauri/plugins/`.

## Why a junction?

- Plugins are deployed to `src-tauri/plugins/` by the build process
- The Tauri backend loads plugin manifests from there
- Vite serves static files from `public/` in dev mode
- This junction ensures both Vite and Tauri see the same plugins **without copying**

## Benefits

✅ **No copy overhead in dev mode** - Changes to plugins are immediately visible
✅ **Single source of truth** - Both frontend and backend read from same location
✅ **Fast iteration** - Rebuild plugin, refresh browser, see changes

## If this folder is not a junction

Run `npm run plugins:build` which automatically creates the junction.

Or manually:

```powershell
# Remove the regular directory if it exists
Remove-Item -Recurse -Force "src/public/plugins"

# Create a junction
cmd /c "mklink /J src\public\plugins src-tauri\plugins"
```

## Do NOT commit plugin files here

Plugin builds are committed in `plugins/featured/*/frontend/dist/` and deployed to `src-tauri/plugins/` during the build process.

This directory itself should not contain any files - it's just a pointer (junction) to `src-tauri/plugins/`.

