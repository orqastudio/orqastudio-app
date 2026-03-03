<script lang="ts">
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { projectStore } from "$lib/stores/project.svelte";

	const statusColor = $derived.by(() => {
		switch (settingsStore.sidecarStatus.state) {
			case "connected":
				return "bg-green-500";
			case "starting":
				return "bg-yellow-500";
			case "error":
				return "bg-red-500";
			case "stopped":
			case "not_started":
			default:
				return "bg-muted-foreground";
		}
	});

	const projectName = $derived(projectStore.activeProject?.name ?? "No project");
</script>

<div
	class="flex h-6 items-center border-t border-border bg-muted/30 px-4 text-xs text-muted-foreground"
>
	<!-- Left: Sidecar connection status -->
	<div class="flex items-center gap-1.5">
		<span class="inline-block h-2 w-2 rounded-full {statusColor}"></span>
		<span>{settingsStore.sidecarStateLabel}</span>
	</div>

	<!-- Center: Active model -->
	<div class="flex flex-1 items-center justify-center">
		<span>Model: {settingsStore.modelDisplayName}</span>
	</div>

	<!-- Right: Active project name -->
	<div class="flex items-center">
		<span>{projectName}</span>
	</div>
</div>
