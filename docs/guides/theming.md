# Theme Customization Guide

**Last Updated:** 2026-01-04

This guide covers how to customize the appearance of Modulaur.

## Built-in Themes

Modulaur includes two built-in themes:

### Light Theme
- Clean, professional appearance
- High contrast for readability
- Suitable for bright environments

### Dark Theme
- Reduces eye strain in low-light conditions
- Modern appearance
- Battery-friendly on OLED screens

## Switching Themes

### Using the Theme Toggle

Click the 🌓 icon in the top-right corner of the application to instantly switch between light and dark themes.

### Default Theme Setting

1. Open **Settings** (click your avatar/username)
2. Navigate to **Appearance**
3. Select your preferred default theme
4. Changes apply immediately

## Theme Architecture

Modulaur uses CSS custom properties (variables) for theming, making it easy to create consistent, customizable interfaces.

### CSS Variable System

All colors and spacing are defined as CSS variables in `src/src/style.css`.

## Available CSS Variables

### Colors

#### Background Colors
```css
--bg-primary      /* Main background */
--bg-secondary    /* Cards, panels */
--bg-tertiary     /* Nested elements */
--bg-hover        /* Hover states */
--bg-active       /* Active/selected states */
```

#### Text Colors
```css
--text-primary    /* Primary text */
--text-secondary  /* Secondary/muted text */
--text-muted      /* Disabled/placeholder text */
```

#### UI Colors
```css
--border-color    /* Borders, dividers */
--color-primary   /* Accent color (buttons, links) */
--color-primary-dark  /* Darker accent (hover states) */
--color-success   /* Success states */
--color-warning   /* Warning states */
--color-error     /* Error states */
```

#### Panel-Specific
```css
--panel-bg        /* Panel backgrounds */
--panel-header-bg /* Panel headers */
--panel-border    /* Panel borders */
```

### Using Variables in Your Plugins

When creating plugins, use these variables for consistent theming:

```css
.my-component {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  padding: 16px;
}

.my-button {
  background: var(--color-primary);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.my-button:hover {
  background: var(--color-primary-dark);
}
```

## Creating Custom Themes

### Method 1: Browser Extensions

Use browser extensions like **Stylus** or **User CSS** to override CSS variables:

```css
/* Custom Purple Theme */
:root {
  --color-primary: #7c3aed !important;
  --color-primary-dark: #6d28d9 !important;
  --bg-primary: #f5f3ff !important;
  --bg-secondary: #ede9fe !important;
}
```

### Method 2: Developer Console

For temporary testing, use the browser console:

```javascript
document.documentElement.style.setProperty('--color-primary', '#7c3aed')
document.documentElement.style.setProperty('--bg-primary', '#f5f3ff')
```

### Method 3: Fork and Modify (Advanced)

For permanent custom themes:

1. Fork the Modulaur repository
2. Edit `src/src/style.css`
3. Add your theme definitions
4. Build and use your custom version

## Theme Examples

### Ocean Blue Theme

```css
:root {
  --color-primary: #0284c7;
  --color-primary-dark: #0369a1;
  --bg-primary: #f0f9ff;
  --bg-secondary: #e0f2fe;
  --bg-tertiary: #bae6fd;
  --text-primary: #0c4a6e;
  --text-secondary: #075985;
}
```

### Forest Green Theme

```css
:root {
  --color-primary: #16a34a;
  --color-primary-dark: #15803d;
  --bg-primary: #f0fdf4;
  --bg-secondary: #dcfce7;
  --bg-tertiary: #bbf7d0;
  --text-primary: #14532d;
  --text-secondary: #166534;
}
```

### Sunset Orange Theme

```css
:root {
  --color-primary: #ea580c;
  --color-primary-dark: #c2410c;
  --bg-primary: #fff7ed;
  --bg-secondary: #ffedd5;
  --bg-tertiary: #fed7aa;
  --text-primary: #7c2d12;
  --text-secondary: #9a3412;
}
```

## Plugin Styling Best Practices

### 1. Always Use CSS Variables

```css
/* ✅ Good - adapts to theme */
.my-element {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

/* ❌ Bad - hardcoded colors */
.my-element {
  background: #ffffff;
  color: #000000;
}
```

### 2. Scope Your Styles

```vue
<style scoped>
/* Styles only apply to this component */
.my-element {
  /* ... */
}
</style>
```

### 3. Provide Fallbacks

```css
.my-element {
  background: var(--bg-secondary, #f5f5f5);
  color: var(--text-primary, #000000);
}
```

### 4. Test Both Themes

Always test your plugin in both light and dark themes to ensure good contrast and readability.

## Accessibility Considerations

### Contrast Ratios

Ensure sufficient contrast between text and backgrounds:
- Normal text: minimum 4.5:1 ratio
- Large text (18pt+): minimum 3:1 ratio

### Testing Contrast

Use browser DevTools or online tools:
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- Chrome DevTools → Elements → Styles → Color Picker

### Color Blindness

Don't rely solely on color to convey information:
- Use icons in addition to colors
- Include text labels
- Use patterns or shapes

## Advanced Customization

### Dynamic Theme Switching

If you're building a plugin that needs to react to theme changes:

```typescript
import { ref, onMounted } from 'vue'

const isDark = ref(false)

onMounted(() => {
  // Check initial theme
  isDark.value = document.documentElement.classList.contains('dark')
  
  // Watch for theme changes
  const observer = new MutationObserver(() => {
    isDark.value = document.documentElement.classList.contains('dark')
  })
  
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['class']
  })
})
```

### Per-Component Theme Overrides

```vue
<template>
  <div class="custom-themed-component" :class="{ dark: isDarkMode }">
    <!-- content -->
  </div>
</template>

<style scoped>
.custom-themed-component {
  --local-accent: #007bff;
}

.custom-themed-component.dark {
  --local-accent: #4dabf7;
}

.element {
  color: var(--local-accent);
}
</style>
```

## Troubleshooting

### Styles Not Applying

1. Check CSS variable names (case-sensitive)
2. Ensure CSS is properly scoped
3. Check browser DevTools for override conflicts
4. Clear cache and reload

### Theme Not Persisting

1. Check browser localStorage (theme setting stored there)
2. Verify Settings page saves correctly
3. Try clearing app data and setting again

### Colors Look Different in Plugin

1. Verify you're using CSS variables, not hardcoded colors
2. Check that your CSS is scoped properly
3. Ensure no conflicting styles from parent components

## Future Enhancements

Planned theming features:
- **Custom theme creator UI** - Visual theme editor
- **Theme marketplace** - Share and download community themes
- **Per-dashboard themes** - Different themes for different workspaces
- **Scheduled themes** - Auto-switch based on time of day

## Resources

- [CSS Custom Properties (MDN)](https://developer.mozilla.org/en-US/docs/Web/CSS/--*)
- [Color Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Material Design Color System](https://material.io/design/color)

## Getting Help

If you have questions about theming:
- Check [GitHub Issues](https://github.com/yourusername/modulaur/issues) for theme-related discussions
- Share your custom themes with the community
- Report theme bugs or inconsistencies

---

Happy theming! 🎨

