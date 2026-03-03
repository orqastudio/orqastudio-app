<script lang="ts">
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import FolderIcon from "@lucide/svelte/icons/folder";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import type { DocNode } from "$lib/types";

	const tree = $derived(artifactStore.docTree);
	const loading = $derived(artifactStore.docTreeLoading);

	/** Filter out root-level README from the tree (accessible via home icon). */
	const filteredTree = $derived(tree.filter((node) => node.path !== "README"));

	function humanizeSegment(segment: string): string {
		return segment
			.split("-")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ");
	}

	function handleDocClick(path: string) {
		const parts = path.split("/");
		const breadcrumbs = parts.map(humanizeSegment);
		navigationStore.openArtifact(path, breadcrumbs);
	}
</script>

{#if loading}
	<div class="flex h-full items-center justify-center">
		<LoadingSpinner />
	</div>
{:else if tree.length === 0}
	<div class="flex h-full items-center justify-center p-4 text-center text-xs text-muted-foreground">
		No documentation found.
	</div>
{:else}
	<ScrollArea.Root class="h-full">
		<div class="space-y-0.5 p-2">
			{#each filteredTree as node}
				{@render treeSection(node, 0)}
			{/each}
		</div>
	</ScrollArea.Root>
{/if}

{#snippet treeSection(node: DocNode, depth: number)}
	{#if node.children}
		<Collapsible.Root open={true}>
			<Collapsible.Trigger
				class="flex w-full items-center gap-1 rounded px-1 py-1 text-xs font-semibold uppercase tracking-wide text-muted-foreground hover:bg-accent/50"
				style="padding-left: {depth * 12 + 4}px"
			>
				<ChevronRightIcon class="h-3 w-3 transition-transform [[data-state=open]_&]:rotate-90" />
				{node.label}
			</Collapsible.Trigger>
			<Collapsible.Content>
				{#each node.children as child}
					{@render treeSection(child, depth + 1)}
				{/each}
			</Collapsible.Content>
		</Collapsible.Root>
	{:else if node.path}
		<button
			class="flex w-full items-center gap-1.5 rounded px-1 py-1 text-sm text-foreground/80 hover:bg-accent/50"
			class:bg-accent={navigationStore.selectedArtifactPath === node.path}
			class:text-accent-foreground={navigationStore.selectedArtifactPath === node.path}
			style="padding-left: {depth * 12 + 8}px"
			onclick={() => handleDocClick(node.path!)}
		>
			<FileTextIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
			<span class="truncate">{node.label}</span>
		</button>
	{/if}
{/snippet}
