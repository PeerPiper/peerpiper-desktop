# PeerPiper Native

True Native Web3:

- [x] Connect to your own node from home, web, or mobile (libp2peasy + WebRTC)
- [x] Store your data to your own device (Tauri)

The power of a native app for desktop and mobile, built with:

- [x] Tauri
- [x] Svelte-Kit and Vite
- [x] Tailwind

## Stretch Goals

- [ ] Save your Web3 data to your devices or the network
- [ ] Remote Procedure Call (RPC) to your devices for Identity calls (read: Wallet/Keys)
- [ ] Plugins of your choice

## Development

The command `npm run dev:tauri` will run Svelte first which will start the vite dev server. Then it will compile the rust code, and start the Tauri dev server:

```bash
npm run dev:tauri
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
