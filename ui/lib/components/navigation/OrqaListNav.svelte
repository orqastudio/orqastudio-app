<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import TargetIcon from "@lucide/svelte/icons/target";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import CheckSquareIcon from "@lucide/svelte/icons/check-square";
	import LightbulbIcon from "@lucide/svelte/icons/lightbulb";
	import ScrollTextIcon from "@lucide/svelte/icons/scroll-text";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import type { ArtifactSummary } from "$lib/types";
	import type { Component } from "svelte";

	let { category }: { category: ActivityView } = $props();

	interface CategoryConfig {
		icon: Component;
		label: string;
		emptyTitle: string;
		emptyDescription: string;
		getItems: () => ArtifactSummary[];
		isLoading: () => boolean;
	}

	const categoryConfig: Record<string, CategoryConfig> = {
		milestones: {
			icon: TargetIcon,
			label: "Milestones",
			emptyTitle: "No milestones yet",
			emptyDescription: "Milestones define strategic goals and gate questions for the project.",
			getItems: () => artifactStore.milestones,
			isLoading: () => artifactStore.milestonesLoading,
		},
		epics: {
			icon: LayersIcon,
			label: "Epics",
			emptyTitle: "No epics yet",
			emptyDescription: "Epics are trackable work units that group related tasks together.",
			getItems: () => artifactStore.epics,
			isLoading: () => artifactStore.epicsLoading,
		},
		tasks: {
			icon: CheckSquareIcon,
			label: "Tasks",
			emptyTitle: "No tasks yet",
			emptyDescription: "Tasks are scoped work items within an epic.",
			getItems: () => artifactStore.tasks,
			isLoading: () => artifactStore.tasksLoading,
		},
		ideas: {
			icon: LightbulbIcon,
			label: "Ideas",
			emptyTitle: "No ideas captured yet",
			emptyDescription: "Ideas are candidate features that need research and validation before promotion.",
			getItems: () => artifactStore.ideas,
			isLoading: () => artifactStore.ideasLoading,
		},
		decisions: {
			icon: ScrollTextIcon,
			label: "Decisions",
			emptyTitle: "No decisions recorded yet",
			emptyDescription: "Architecture decisions capture why key choices were made.",
			getItems: () => artifactStore.decisions,
			isLoading: () => artifactStore.decisionsLoading,
		},
		lessons: {
			icon: BookOpenIcon,
			label: "Lessons",
			emptyTitle: "No lessons captured yet",
			emptyDescription: "Lessons record implementation discoveries and prevent recurring mistakes.",
			getItems: () => artifactStore.lessons,
			isLoading: () => artifactStore.lessonsLoading,
		},
	};

	const config = $derived(categoryConfig[category]);
	const items = $derived(config ? config.getItems() : []);
	const loading = $derived(config ? config.isLoading() : false);

	// Auto-select artifact when navigated via cross-link
	$effect(() => {
		const pendingId = navigationStore.pendingArtifactId;
		if (!pendingId || items.length === 0) return;
		const match = items.find((item) => item.name.startsWith(pendingId));
		if (match) {
			navigationStore.pendingArtifactId = null;
			navigationStore.openArtifact(match.rel_path, [match.name]);
		}
	});

	function handleItemClick(item: ArtifactSummary) {
		navigationStore.openArtifact(item.rel_path, [item.name]);
	}
</script>

{#if config}
	<ScrollArea.Root class="min-h-0 flex-1">
		<div class="p-1">
			{#if loading}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner />
				</div>
			{:else if artifactStore.error}
				<div class="px-2 py-4">
					<ErrorDisplay message={artifactStore.error} />
				</div>
			{:else if items.length === 0}
				<div class="px-2 py-8">
					<EmptyState
						icon={config.icon}
						title={config.emptyTitle}
						description={config.emptyDescription}
					/>
				</div>
			{:else}
				{#each items as item (item.rel_path)}
					<button
						class="flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50"
						class:bg-accent={navigationStore.selectedArtifactPath === item.rel_path}
						onclick={() => handleItemClick(item)}
					>
						<span class="truncate text-sm font-medium">{item.name}</span>
						{#if item.description}
							<p class="line-clamp-2 text-xs text-muted-foreground">{item.description}</p>
						{/if}
					</button>
				{/each}
			{/if}
		</div>
	</ScrollArea.Root>
{/if}
