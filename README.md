# Anbardar - Cross-Platform Inventory Management

This is a cross-platform inventory management application built with Tauri and SurrealDB, designed to work on desktop platforms (Windows, macOS, Linux) and mobile platforms (Android, iOS).

## Architecture

The application follows a clean, modular architecture:

### Core Components

- **Database Layer**: Uses SurrealDB with platform-specific storage engines:
  - Desktop: RocksDB for persistent, high-performance storage
  - Mobile: In-memory storage for performance and cache management

- **Repository Pattern**: Generic repository implementation for consistent data access across all services

- **Service Modules**:
  - Warehouse: Manages warehouse data
  - Product: Handles product inventory
  - Person: Manages person/user information
  - Location: Manages location data
  - Orders: Handles order processing

### Cross-Platform Support

The application uses conditional compilation to provide the appropriate implementation based on the target platform:

```rust
#[cfg(not(any(target_os = "android", target_os = "ios")))]
// Desktop-specific implementation

#[cfg(any(target_os = "android", target_os = "ios"))]
// Mobile-specific implementation
```

## Building and Running

### Prerequisites

- Rust toolchain
- Node.js and npm
- Tauri CLI

### Development

1. Install dependencies:
   ```
   npm install
   ```

2. Run in development mode:
   ```
   npm run tauri dev
   ```

### Building for Production

```
npm run tauri build
```

### Mobile Development

Follow the Tauri mobile setup instructions at https://tauri.app/v1/guides/getting-started/prerequisites

## API Reference

The application exposes several Tauri commands for frontend interaction, organized by domain:

- **Warehouse API**:
  - `list_warehouses`: Get all warehouses
  - `create_warehouse`: Create a new warehouse

- **Product API**:
  - `get_products`: Get all products
  - `add_product`: Add a new product
  - `get_categories`: Get all product categories
  - `add_category`: Add a new product category

- **Location API**:
  - `list_locations`: Get all locations
  - `create_location`: Create a new location

- **Person API**:
  - `list_people`: Get all people
  - `create_person`: Create a new person

- **Order API**:
  - `list_orders`: Get all orders
  - `create_order`: Create a new order

# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).

---

Setup Android using: `yarn tauri android init`, before that make sure `ANDROID_HOME` ENV variable is set.
