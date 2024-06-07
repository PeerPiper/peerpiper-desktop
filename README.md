# PeerPiper Desktop

Your data node, complete with private personal AI Large Language Model.

True Native DWeb:

- [x] Connect to your own node from home, web, or mobile (libp2p + WebRTC)
- [x] Store your data to your own device (Tauri)
- [x] Remotely run Large Language Models (LLMs) privately on your own device

## Tech Stack

The power of a native app for desktop, built with:

- [x] Tauri
- [x] Svelte-Kit and Vite
- [x] TailwindCSS

## Stretch Goals

- [ ] Save your Web3 data to your devices or the network
- [ ] [Plugins](https://component-model.bytecodealliance.org/) of your choice

## Build Targets

- [x] Linux
- [ ] MacOS
- [x] Windows
- [x] 🌐 Android (Via web browser to your node runnign at home)
- [x] 🌐 iOS (Via web browser to your node running at home)

## Building

To build yourself, ensure you install ollama executable first:

```bash
just install_ollama
```

### Dist folder

If you get a `proc_macro` error, you may need to create a [`dist` folder](./dist) in the root of the project.

## Development

This command will run Svelte first which will start the vite dev server. Then it will compile the rust code, and start the Tauri dev server:

```bash
npm run tauri dev
# or using just.systems:
just tauri
```

## Release Process

- Update the version in the `tauri.conf.json` file. 
- Run the following command to merge into the release branch, create tag and push to the remote repository:

```bash
just release
```
