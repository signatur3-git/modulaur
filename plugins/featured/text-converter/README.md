# Text Converter Plugin

**Version:** 1.0.0  
**Type:** Frontend-only  
**Author:** Modulaur Team

---

## 🎯 Overview

A versatile text conversion utility plugin that provides encoding, decoding, case conversion, format transformation, hashing, and JSON formatting capabilities.

## ✨ Features

### 🔐 Encoding & Decoding
- **Base64** - Encode/decode to Base64
- **URL** - Encode/decode URLs
- **HTML** - Escape/unescape HTML entities

### 🔤 Case Conversion
- **UPPERCASE** - Convert to all caps
- **lowercase** - Convert to all lowercase
- **Title Case** - Capitalize Each Word
- **Capitalize First** - Only first letter uppercase

### 📝 Format Conversion
- **snake_case** - Underscore separated
- **kebab-case** - Hyphen separated
- **camelCase** - JavaScript style
- **PascalCase** - Class name style
- **CONSTANT_CASE** - Uppercase with underscores

### #️⃣ Hashing
- **SHA-1** - Generate SHA-1 hash
- **SHA-256** - Generate SHA-256 hash

### {} JSON Formatting
- **Pretty Print** - Format with indentation
- **Minify** - Remove whitespace
- **Escape/Unescape** - Handle quotes

---

## 🛠️ Building

### Prerequisites
- Node.js and npm installed
- Run from the plugin directory

### Build Steps

```bash
# Windows
build.bat

# Manual build
cd frontend
npm install
npm run build
```

### Output
- `frontend/dist/text-converter.umd.js` - Plugin bundle
- `frontend/dist/text-converter.css` - Plugin styles

---

## 📦 Installation

The build script automatically copies the plugin to:
```
src-tauri/plugins/text-converter/
```

The plugin will be discovered on next app restart.

---

## 🎮 Usage

### In Dashboard
1. Open Modulaur
2. Navigate to a dashboard
3. Click "Edit" → "Add Panel"
4. Select "🔄 Text Converter"
5. Start converting text!

### Interface
1. **Input** - Enter text to convert
2. **Tabs** - Choose conversion category
3. **Actions** - Click conversion button
4. **Output** - See result (click Copy to clipboard)

---

## 🏗️ Architecture

### Type: Frontend-Only Plugin
- No backend/WASM component needed
- Pure JavaScript implementation
- Runs entirely in browser
- No data storage required

### Technologies
- **Vue 3** - Component framework
- **Web Crypto API** - Hashing (SHA-1, SHA-256)
- **Native JS** - Base64, URL encoding
- **UMD Bundle** - Standalone module format

---

## 🎨 Styling

The plugin uses a clean, professional design:
- Responsive layout
- Tab-based navigation
- Grid layout for action buttons
- Gradient button styling (purple theme)
- Clear visual feedback
- Error message display

---

## 🔧 Development

### File Structure
```
text-converter/
├── manifest.json           # Plugin metadata
├── build.bat              # Build script
├── README.md              # This file
└── frontend/
    ├── package.json       # Dependencies
    ├── vite.config.js     # Build config
    ├── index.js           # Plugin entry
    ├── TextConverterPanel.vue  # Main component
    └── dist/              # Build output
        ├── text-converter.umd.js
        └── text-converter.css
```

### Extending

To add new conversions:

1. Add tab (if new category):
```javascript
{ id: 'newcat', label: 'New Category', icon: '🆕' }
```

2. Add UI in template:
```vue
<div v-if="activeTab === 'newcat'" class="action-group">
  <button @click="convert('new-action')">New Action</button>
</div>
```

3. Add conversion logic:
```javascript
case 'new-action':
  outputText.value = transformText(inputText.value)
  break
```

---

## 💡 Use Cases

- **Developers** - Encode/decode API payloads
- **DevOps** - Convert config formats
- **Testers** - Generate test data
- **General** - Format text for various needs

---

## 🐛 Known Limitations

- **MD5** - Not available via Web Crypto API (uses SHA-256 instead)
- **Large Text** - Browser memory limits apply
- **Binary Data** - Base64 limited to text input

---

## 📝 License

Part of Modulaur project.

---

## 🤝 Contributing

This plugin demonstrates:
- Frontend-only plugin structure
- UMD bundle creation
- Vue 3 Composition API
- Web APIs (Crypto, Clipboard)
- Professional UI design

Use as template for other utility plugins!

