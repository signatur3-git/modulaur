# Contributing to Modulaur

Thank you for your interest in contributing! This guide will help you get started.

## Getting Started

1. **Read the documentation**
   - [Architecture Overview](./docs/architecture/overview.md)
   - [Getting Started Guide](./docs/guides/getting-started.md)
   - [Current Status](./docs/STATUS.md)

2. **Set up your development environment**
   - Follow the [Getting Started Guide](./docs/guides/getting-started.md)
   - Make sure you can build and run the app

3. **Find something to work on**
   - Browse [GitHub Issues](https://github.com/yourusername/modulaur/issues) for tasks
   - Look for issues labeled `good first issue` or `help wanted`
   - Comment on an issue to claim it
   - Open a new issue if you find a bug or have a feature request

## Project Planning

**For Contributors:**
- All features and bugs are tracked via **GitHub Issues**
- Public roadmap and prioritization happens through **GitHub Projects** (coming soon)
- Propose new features by opening an issue with the `enhancement` label

**Feature Proposals:**
When proposing a new feature, include:
- Clear description of the feature
- Use cases and benefits
- Potential implementation approach (optional)
- Screenshots or mockups (if UI-related)


## Development Workflow

### Making Changes

1. **Create a feature branch**
   ```powershell
   git checkout -b feature/my-feature
   ```

2. **Make your changes**
   - Follow the code style (see below)
   - Write clear commit messages
   - Test your changes in both Dev and Prod stages

3. **Format and validate your code**
   ```powershell
   # Format all code (Frontend + Rust)
   npm run format
   
   # Run all checks (matches CI validation)
   npm run check:all
   
   # Or use the combined pre-commit script
   npm run precommit
   ```

4. **Test thoroughly**
   - Test in both light and dark themes
   - Check that existing features still work
   - Build all plugins if you modified plugin system
   ```powershell
   npm run test
   npm run test:rust
   ```

5. **Validate before committing**
   ```powershell
   # This runs the same checks as GitHub Actions
   npm run validate
   ```

6. **Commit your changes**
   ```powershell
   git add .
   git commit -m "feat: Add awesome new feature"
   ```

### Commit Message Format

We use conventional commits:

```
<type>: <description>

[optional body]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat: Add keyboard shortcuts for navigation
fix: Resolve settings load undefined error
docs: Update plugin development guide
refactor: Extract shared panel utilities
```

## Code Style

We use automated tools to maintain consistent code style:

### Formatting Tools

- **Prettier** - JavaScript/TypeScript/Vue formatting
- **rustfmt** - Rust code formatting
- **ESLint** - JavaScript/TypeScript linting
- **Clippy** - Rust linting

**Auto-format all code:**
```powershell
npm run format
```

**Check formatting:**
```powershell
npm run format:check
```

### TypeScript/Vue

- Use **TypeScript** with proper types (avoid `any`)
- Use **Composition API** in Vue components
- Use **Pinia stores** for state management
- Name components with **PascalCase**
- Use **camelCase** for functions and variables
- **Prettier will handle formatting** - just run `npm run format`

**Example:**
```typescript
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export const useMyStore = defineStore('myStore', () => {
  const count = ref(0);
  const doubleCount = computed(() => count.value * 2);
  
  function increment() {
    count.value++;
  }
  
  return { count, doubleCount, increment };
});
```

### Rust

- Follow **standard Rust conventions**
- Use `rustfmt` for formatting (included in `npm run format`)
- Use `clippy` for linting
- Use descriptive error messages
- Use `Result` types for fallible operations
- Document public APIs with doc comments

**Example:**
```rust
/// Creates a new page in the database
/// 
/// # Arguments
/// * `db` - Database connection
/// * `page` - Page data to create
/// 
/// # Returns
/// The created page with assigned ID
#[tauri::command]
pub async fn create_page(
    db: State<'_, DatabaseState>,
    page: PageCreate,
) -> Result<Page, String> {
    db.create_page(page)
        .await
        .map_err(|e| format!("Failed to create page: {}", e))
}
```

### File Organization

**Frontend:**
```
src/src/
├── components/       # Reusable components
│   ├── common/      # Shared across app
│   └── specific/    # Feature-specific
├── views/           # Page components
├── stores/          # Pinia stores
├── router/          # Route definitions
├── composables/     # Composition functions
└── utils/           # Utility functions
```

**Backend:**
```
src-tauri/src/
├── main.rs          # Entry point
├── db.rs            # Database layer
├── plugin_system.rs # Plugin management
├── commands.rs      # Tauri commands
└── [feature].rs     # Feature modules
```

## Plugin Development

See [Plugin Development Guide](./docs/PLUGIN-DEVELOPMENT.md) for details.

**Checklist for new plugins:**
- [ ] Follow template structure
- [ ] Complete manifest.json
- [ ] Add README.md
- [ ] Add build script
- [ ] Test in both stages
- [ ] Document usage

## Testing

### Manual Testing

Before submitting changes:

1. **Test in both stages**
   - Switch between Dev and Prod
   - Verify data separation works

2. **Test both themes**
   - Toggle light/dark mode
   - Check for contrast issues

3. **Test all affected features**
   - Navigation
   - Data persistence
   - Plugin loading

### Automated Testing

*(Coming soon - we need to add test infrastructure)*

## Documentation

### When to Update Docs

Update documentation when you:
- Add a new feature
- Change an API
- Fix a significant bug
- Refactor architecture

### Documentation Style

- Use **clear, concise language**
- Include **code examples**
- Add **screenshots** for UI features
- Use **markdown formatting** properly

## Pull Request Process

1. **Ensure your branch is up to date**
   ```powershell
   git checkout main
   git pull
   git checkout feature/my-feature
   git rebase main
   ```

2. **Create a clear PR description**
   - Describe what you changed
   - Explain why you changed it
   - List any breaking changes
   - Reference related issues

3. **Respond to feedback**
   - Address review comments
   - Update code as needed
   - Ask questions if unclear

## Questions?

- Review [documentation](./docs)
- Look at [STATUS.md](./docs/STATUS.md) for known issues

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- Git commit history
- Release notes
- (Future) Contributors list

Thank you for making Modulaur better! 🎉

