# [Tauri](https://github.com/tauri-apps/tauri) + [Svelte](https://github.com/sveltejs/svelte) + [Material UI](https://github.com/hperrin/svelte-material-ui) Template

This is a quick-start template for creating a Tauri app using Sveltejs and Material UI, based on the Tauri Svelte template at https://github.com/happybeing/tauri-svelte-template.

This template includes a basic command event loop between front end (Svelte) and back end (Rust).

## Prerequisites
* [Tauri](https://tauri.studio/en/docs/getting-started/intro)
* [Yarn](https://classic.yarnpkg.com/en/docs/install) or npm

## Development

Install the dependencies
```bash 
yarn
# OR
npm install
```

Start your Svelte development server:
```bash
yarn dev
# OR
npm run dev
```

In another console, start the Tauri development environment:
```bash
yarn tauri dev
# OR
npm run tauri dev
```