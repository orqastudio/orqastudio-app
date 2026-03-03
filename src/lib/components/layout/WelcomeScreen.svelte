<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { Button } from "$lib/components/ui/button";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import anvilMark from "$lib/assets/anvil-mark.svg";
	import { projectStore } from "$lib/stores/project.svelte";

	let opening = $state(false);

	async function handleOpenProject() {
		opening = true;
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Open Project Folder",
			});
			if (selected && typeof selected === "string") {
				await projectStore.openProject(selected);
			}
		} finally {
			opening = false;
		}
	}
</script>

<div class="flex h-full flex-col items-center justify-center gap-6">
	<img src={anvilMark} alt="Forge" class="h-16 w-16 opacity-50" />
	<div class="text-center">
		<h2 class="text-xl font-semibold">Welcome to Forge</h2>
		<p class="mt-2 text-sm text-muted-foreground">Open a project to get started</p>
	</div>
	{#if opening}
		<LoadingSpinner />
	{:else}
		<Button variant="outline" onclick={handleOpenProject}>
			<FolderOpenIcon class="mr-2 h-4 w-4" />
			Open Project
		</Button>
	{/if}
	{#if projectStore.error}
		<p class="text-sm text-destructive">{projectStore.error}</p>
	{/if}
</div>
