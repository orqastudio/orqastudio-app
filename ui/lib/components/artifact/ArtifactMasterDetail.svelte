<script lang="ts">
	import ArtifactNav from "$lib/components/navigation/ArtifactNav.svelte";
	import ArtifactViewer from "./ArtifactViewer.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";

	let { activity }: { activity: ActivityView } = $props();

	/**
	 * Static fallback README paths per category — used when the navTree hasn't loaded yet
	 * or when the category doesn't have a navType entry.
	 */
	const FALLBACK_README_PATHS: Partial<Record<ActivityView, string>> = {
		milestones: ".orqa/milestones/README.md",
		epics: ".orqa/epics/README.md",
		tasks: ".orqa/tasks/README.md",
		ideas: ".orqa/ideas/README.md",
		decisions: ".orqa/decisions/README.md",
		lessons: ".orqa/lessons/README.md",
		agents: ".orqa/agents/README.md",
		rules: ".claude/rules/README.md",
		skills: ".claude/skills/README.md",
		hooks: ".claude/hooks/README.md",
		docs: "docs/README.md",
		research: ".orqa/research/README.md",
		plans: ".orqa/plans/README.md",
	};

	/**
	 * Derive the README path for the current activity.
	 * Prefer the navTree's type path to construct the README path,
	 * falling back to the static map.
	 */
	const readmePath = $derived(() => {
		const navType = navigationStore.getNavType(activity);
		if (navType) {
			// The navType path is the folder path — append README.md
			return `${navType.path}/README.md`;
		}
		return FALLBACK_README_PATHS[activity] ?? null;
	});

	const hasSelection = $derived(navigationStore.selectedArtifactPath !== null);

	/** When the activity changes and nothing is selected, auto-load the category README. */
	$effect(() => {
		const act = activity;
		if (navigationStore.selectedArtifactPath !== null) return;
		const rp = readmePath();
		if (rp) {
			navigationStore.openArtifact(rp, []);
		}
	});
</script>

<div class="flex h-full">
	<!-- File Browser (240px) -->
	<div class="w-60 shrink-0 overflow-hidden border-r border-border">
		<ArtifactNav category={activity} />
	</div>

	<!-- Viewer -->
	<div class="min-w-0 flex-1 overflow-hidden">
		{#if hasSelection}
			<ArtifactViewer />
		{:else}
			<div class="flex h-full items-center justify-center text-sm text-muted-foreground">
				Select an item to view it
			</div>
		{/if}
	</div>
</div>
