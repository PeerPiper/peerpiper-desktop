<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';

	let value = 'Hello Rust';
	let values: { timestamp: number; message: string }[] = [];
	let inputs: { timestamp: number; message: string }[] = [];

	async function handleSubmit(e: SubmitEvent) {}

	let unlistenPromise = listen('serverMultiaddr', (event) => {
		// event.event is the event name (useful if you want to use a single callback fn for multiple event types)
		// event.payload is the payload object
		console.log('js: serverMultiaddr: ' + event);
		let input = event.payload as string;
		inputs = [...inputs, { timestamp: Date.now(), message: input }];
	});

	function sendOutput() {
		console.log('js: js2rs: ' + value);
		values = [...values, { timestamp: Date.now(), message: value }];
		invoke('js2rs', { message: value });
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<input
		id="greet-input"
		placeholder="Enter a greeting..."
		bind:value={name}
		class="m-2 rounded border p-2"
	/>
	<input type="submit" value="Greet" class="rounded bg-green-500 p-2 text-white shadow" />
</form>

<div style="display: grid; grid-template-columns: auto auto;">
	<div style="grid-column: span 2; grid-row: 1;">
		<label for="input" style="display: block;">Message</label>
		<input id="input" bind:value />
		<br />
		<button on:click={sendOutput} class="m-2 rounded bg-red-300 p-2 text-red-800"
			>Send to Rust 🦀</button
		>
	</div>
	<!-- <div style="grid-column: 1; grid-row: 2;">
		<h3>js2rs events</h3>
		<ol>
			{#each values as val}
				<li>
					{val.message}
				</li>
			{/each}
		</ol>
	</div> -->
	{#await unlistenPromise then unlisten}
		<div style="grid-column: 2; grid-row: 2;">
			<h3>from_tauri events</h3>
			<ol>
				{#each inputs as input}
					<li>
						{input.message}
					</li>
				{/each}
			</ol>
		</div>
	{/await}
</div>

<style lang="postcss">
	li {
		@apply break-all;
	}
</style>
