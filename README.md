# Tauri + Svelte + Material App Template

This is a quick-start template for creating a Tauri app using Sveltejs and Material UI, based on
the Tauri Svelte template at https://github.com/happybeing/tauri-svelte-template.

This template includes a basic command event loop between front end (Svelte) and back end (Rust).

## Pre-requisites
Go to https://tauri.studio and set up Tauri for your operating system. Also
install `yarn` if you want to follow the instructions here verbatim.

## Development

Install the dependencies and start your Svelte development server:
```bash
yarn && yarn dev
```

In another console, start the Tauri development environment:
```bash
yarn tauri dev
```