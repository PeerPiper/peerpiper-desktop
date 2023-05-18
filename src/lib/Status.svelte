<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';
	import { once } from '@tauri-apps/api/event';

	import type { UnlistenFn } from '@tauri-apps/api/event';

	// @ts-ignore
	import Chat from '@peerpiper/piperchat';

	// Status for WebRTC Server
	export let listening = false;

	let unlisten: null | UnlistenFn = null;
	let serverMultiaddr: string;

	let unlistenPromise = once('serverMultiaddr', async (evt) => {
		const { event, id, payload } = evt;
		console.log({ event });
		serverMultiaddr = payload as string;
		console.log({ serverMultiaddr });
		listening = true;
	});

	onMount(async () => {
		// invoke ready, tell Tauri we're ready to listen
		invoke('js2rs', { message: 'ready' });
		unlisten = await unlistenPromise;

		return () => {
			if (unlisten) unlisten();
		};
	});
</script>

<div
	class="absolute top-0 right-0 m-2 flex items-center rounded bg-slate-500/50 p-2 font-semibold text-slate-100 drop-shadow"
>
	{#if !listening}
		<div class="mr-1 h-3 w-3 animate-bounce rounded-full bg-amber-300" />
		<div class="flex-initial">Connecting...</div>
	{:else}
		<div class="m-1 h-2 w-2 animate-ping rounded-full bg-emerald-400" />
		<div class="flex-initial">Live</div>
	{/if}
	<div class="mx-2 flex-1">⚙️</div>
</div>

{#if serverMultiaddr}
	<Chat {serverMultiaddr} />
{/if}
