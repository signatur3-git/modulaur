# Scripts Directory

This folder contains PowerShell scripts for building, testing, and maintaining the Modulaur project.

## Usage

**Don't run these scripts directly!** Use the npm scripts defined in the root `package.json` instead:

```powershell
# See all available commands
npm run

# Common commands
npm run plugins:build       # Build all plugins
npm run plugin:build <name> # Build single plugin
npm run build:release       # Full release build
npm run clean               # Clean all artifacts
```

## Directory Structure

### `plugins/`
Plugin-related build and deployment scripts.

- **`build-all.ps1`** - Builds all plugins (frontend)
- **`build-one.ps1 <name>`** - Builds a single plugin by name
- **`deploy.ps1`** - Deploys built plugins to tauri/plugins directory
- **`verify.ps1`** - Verifies plugin structure (planned)

**Examples:**
```powershell
pwsh scripts/plugins/build-all.ps1
pwsh scripts/plugins/build-one.ps1 time-tracker
pwsh scripts/plugins/deploy.ps1
```

### `utils/`
General utility scripts for building and maintenance.

- **`clean-all.ps1`** - Deep clean all build artifacts
- **`check-size.ps1`** - Check binary sizes after build

**Examples:**
```powershell
pwsh scripts/utils/clean-all.ps1
pwsh scripts/utils/clean-all.ps1 -SkipInstall
pwsh scripts/utils/check-size.ps1
```

### `dev/`
Development and testing utilities (planned).

- **`verify-setup.ps1`** - Verify development environment (planned)
- **`test-db.ps1`** - Test database functionality (planned)

## Script Features

### Path-Agnostic
All scripts support both old and new folder structures:
- `sandbox-app-gui-spa` ↔ `src`
- `standalone-tauri` ↔ `src-tauri`

Scripts will automatically detect which structure you're using.

### Error Handling
Scripts use `$ErrorActionPreference = "Stop"` and proper error checking.

### Verbose Mode
Many scripts support a `-Verbose` flag for detailed output:
```powershell
pwsh scripts/plugins/build-all.ps1 -Verbose
```

### Skip Options
Some scripts support skip flags:
```powershell
# Build plugins but don't deploy
pwsh scripts/plugins/build-all.ps1 -SkipDeploy

# Clean but don't reinstall
pwsh scripts/utils/clean-all.ps1 -SkipInstall
```

## Why PowerShell?

- **Cross-platform** - Works on Windows, macOS, Linux (with PowerShell Core)
- **Rich scripting** - Better than bash/bat for complex logic
- **Error handling** - Proper try/catch and error propagation
- **File operations** - Native support for file system operations
- **Color output** - Built-in colored console output

## Migration from Old Scripts

Old structure (deprecated):
```
build-all-plugins.bat/.ps1
deploy-plugins.bat/.ps1
clean-build.ps1
check-binary-size.bat
... 30+ more files in root
```

New structure (current):
```
scripts/
├── plugins/
│   ├── build-all.ps1
│   ├── build-one.ps1
│   └── deploy.ps1
└── utils/
    ├── clean-all.ps1
    └── check-size.ps1
```

Access everything via npm scripts in `package.json`.

## Contributing

When adding new scripts:

1. **Place in correct subfolder** - plugins/, utils/, or dev/
2. **Use PowerShell** - Not .bat files
3. **Support both paths** - Old and new folder structures
4. **Add error handling** - Use try/catch and check exit codes
5. **Add to package.json** - Create an npm script wrapper
6. **Document here** - Update this README

## See Also

- [package.json](../package.json) - npm script definitions
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Development guidelines
- [docs/guides/getting-started.md](../docs/guides/getting-started.md) - Setup guide

