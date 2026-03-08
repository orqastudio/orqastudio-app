<script lang="ts">
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import SearchInput from "$lib/components/shared/SearchInput.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";
	import {
		navigationStore,
		SUB_CATEGORY_LABELS,
		GROUP_SUB_CATEGORIES,
		type ActivityView,
		type ActivityGroup,
	} from "$lib/stores/navigation.svelte";
	import type { DocNode } from "$lib/types/nav-tree";

	const GROUP_DISPLAY_LABELS: Record<ActivityGroup, string> = {
		documentation: "Docs",
		planning: "Planning",
		team: "Team",
		governance: "Governance",
	};

	let { category }: { category: ActivityView } = $props();

	let filterText = $state("");

	/** Find the NavType for this category in the navTree. */
	const currentNavType = $derived(() => {
		return navigationStore.getNavType(category);
	});

	/** Nodes for this category — either from navTree or empty. */
	const allNodes = $derived(currentNavType() ? currentNavType()!.nodes : []);

	/** Label for this category. */
	const categoryLabel = $derived(
		currentNavType()?.label ?? SUB_CATEGORY_LABELS[category] ?? category,
	);

	/** Whether data is still loading. */
	const loading = $derived(artifactStore.navTreeLoading);

	/** Any error loading the tree. */
	const treeError = $derived(artifactStore.navTreeError);

	function isReadmePath(path: string | null): boolean {
		if (!path) return false;
		const p = path.replace(/\\/g, "/");
		const name = p.split("/").pop() ?? "";
		return name === "README" || name === "README.md";
	}

	/** All nodes from the navTree, with README filtered out. */
	const rawNodes = $derived(
		allNodes.filter((n) => !isReadmePath(n.path)),
	);

	// ---- Filtering ----

	function filterTree(nodes: DocNode[], query: string): DocNode[] {
		if (!query) return nodes;
		const q = query.toLowerCase();
		const result: DocNode[] = [];
		for (const node of nodes) {
			if (node.children) {
				const filteredChildren = filterTree(node.children, query);
				if (filteredChildren.length > 0) {
					result.push({ ...node, children: filteredChildren });
				} else if (node.label.toLowerCase().includes(q)) {
					result.push(node);
				}
			} else if (
				node.label.toLowerCase().includes(q) ||
				(node.description?.toLowerCase().includes(q) ?? false)
			) {
				result.push(node);
			}
		}
		return result;
	}

	const filteredNodes = $derived(filterTree(rawNodes, filterText));

	/** Whether nodes form a tree (have children) or a flat list. */
	const isTree = $derived(rawNodes.some((n) => n.children !== null));

	// ---- Breadcrumb helpers ----

	function humanizeSegment(segment: string): string {
		return segment
			.split("-")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ");
	}

	function buildBreadcrumbs(node: DocNode): string[] {
		const crumbs: string[] = [];

		// Add group label if in a group
		const group = navigationStore.activeGroup;
		if (group) {
			crumbs.push(GROUP_DISPLAY_LABELS[group]);
			// Only add type label if the group has multiple sub-categories.
			const subCategories = GROUP_SUB_CATEGORIES[group];
			if (subCategories.length > 1) {
				crumbs.push(categoryLabel);
			}
		} else {
			crumbs.push(categoryLabel);
		}

		// Add folder hierarchy for tree items
		if (isTree && node.path) {
			const segments = node.path.split("/");
			// All segments except the last are folders
			for (let i = 0; i < segments.length - 1; i++) {
				crumbs.push(humanizeSegment(segments[i]));
			}
		}

		// Add the item name
		crumbs.push(node.label);

		return crumbs;
	}

	function handleLeafClick(node: DocNode) {
		if (!node.path) return;
		navigationStore.openArtifact(node.path, buildBreadcrumbs(node));
	}

	// ---- Rules violation dots ----

	const rulesWithViolations = $derived(
		new Set(enforcementStore.violations.map((v) => v.rule_name)),
	);

	// ---- Cross-link auto-select ----

	$effect(() => {
		const pendingId = navigationStore.pendingArtifactId;
		if (!pendingId || rawNodes.length === 0 || isTree) return;
		const match = rawNodes.find((n) => n.label.startsWith(pendingId));
		if (match?.path) {
			navigationStore.pendingArtifactId = null;
			navigationStore.openArtifact(match.path, [match.label]);
		}
	});
</script>

<div class="flex h-full flex-col">
	<div class="border-b border-border p-2">
		<SearchInput
			bind:value={filterText}
			placeholder="Filter {categoryLabel.toLowerCase()}..."
			size="xs"
		/>
	</div>

	<ScrollArea.Root class="min-h-0 flex-1">
		<div class="p-1">
			{#if loading}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner />
				</div>
			{:else if treeError}
				<div class="px-2 py-4">
					<ErrorDisplay message={treeError} onRetry={() => artifactStore.loadNavTree()} />
				</div>
			{:else if rawNodes.length === 0}
				<div class="px-2 py-8">
					<EmptyState
						icon={FileTextIcon}
						title="No {categoryLabel.toLowerCase()} yet"
						description="No {categoryLabel.toLowerCase()} files found in this project."
					/>
				</div>
			{:else if filteredNodes.length === 0}
				<div class="px-2 py-4 text-center text-xs text-muted-foreground">
					No matching items.
				</div>
			{:else if isTree}
				<div class="space-y-0.5 p-1">
					{#each filteredNodes as node (node.path ?? node.label)}
						{@render treeSection(node, 0)}
					{/each}
				</div>
			{:else}
				{#each filteredNodes as node (node.path)}
					<button
						class="flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50"
						class:bg-accent={navigationStore.selectedArtifactPath === node.path}
						onclick={() => handleLeafClick(node)}
					>
						<span class="flex items-center gap-1.5 truncate text-sm font-medium">
							{node.label}
							{#if category === "rules" && rulesWithViolations.has(node.label)}
								<span
									class="inline-block h-2 w-2 shrink-0 rounded-full bg-destructive"
									title="Has violations"
								></span>
							{/if}
						</span>
						{#if node.description}
							<p class="line-clamp-2 text-xs text-muted-foreground">{node.description}</p>
						{/if}
					</button>
				{/each}
			{/if}
		</div>
	</ScrollArea.Root>
</div>

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
				{#each node.children as child (child.path ?? child.label)}
					{@render treeSection(child, depth + 1)}
				{/each}
			</Collapsible.Content>
		</Collapsible.Root>
	{:else if node.path}
		<button
			class="flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50"
			class:bg-accent={navigationStore.selectedArtifactPath === node.path}
			style="padding-left: {depth * 12 + 8}px"
			onclick={() => handleLeafClick(node)}
		>
			<span class="truncate text-sm font-medium">{node.label}</span>
			{#if node.description}
				<p class="line-clamp-2 text-xs text-muted-foreground">{node.description}</p>
			{/if}
		</button>
	{/if}
{/snippet}
