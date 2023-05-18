<script>
	import Status from '$lib/Status.svelte';
	import Greet from '$lib/Greet.svelte';
</script>

<div class="inline text-5xl ">
	<span class="font-extrabold text-green-700 drop-shadow-[0_10px_10px_rgba(255,255,255,0.25)]">
		PeerPiper
	</span>
	<span
		class="bg-gradient-to-b from-amber-400 to-red-600 bg-clip-text font-extrabold text-transparent
		drop-shadow-[0px_0px_4px_rgba(255,255,255,0.75)]"
	>
		is Hot!
	</span>
	<span class="font-light drop-shadow-[0px_0px_8px_rgba(255,255,255,0.75)]">🔥🌶️</span>
</div>
<Greet />

<div class="z-10">
	<Status />
</div>
