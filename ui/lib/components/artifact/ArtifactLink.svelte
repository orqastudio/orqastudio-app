<script lang="ts">
	import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	let { id }: { id: string } = $props();

	function handleClick() {
		navigationStore.navigateToArtifact(id);
	}
</script>

<button
	class="inline-flex items-center gap-1 rounded border border-border bg-secondary/60 px-1.5 py-0.5 font-mono text-[11px] font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
	onclick={handleClick}
	title="Navigate to {id}"
>
	{id}
	<ExternalLinkIcon class="h-3 w-3 shrink-0 text-muted-foreground" />
</button>
