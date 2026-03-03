<script lang="ts">
	import { navigationStore } from "$lib/stores/navigation.svelte";
</script>

<div class="w-[200px] overflow-y-auto border-r border-border bg-muted/10 p-2">
	<div class="text-xs font-semibold uppercase text-muted-foreground">
		{navigationStore.activeActivity}
	</div>
	<div class="mt-2 text-sm text-muted-foreground">
		Navigation content for {navigationStore.activeActivity} view
	</div>
</div>
