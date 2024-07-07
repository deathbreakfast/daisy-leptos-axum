<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Daisy Componet Library

This Leptos component library and design patterns use [Leptos Axum Daisy Template]().

## Testing The Components

A server is set up to host and view the components locally.

```bash
cargo leptos watch
```
## Compiling The Site For Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing The Components

```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.
