<script lang="ts">
	import Breadcrumb from "./Breadcrumb.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	const artifact = $derived(artifactStore.activeArtifact);
	const breadcrumbs = $derived(navigationStore.breadcrumbs);

	function handleContentClick(event: MouseEvent) {
		const anchor = (event.target as HTMLElement).closest("a");
		if (!anchor) return;

		const href = anchor.getAttribute("href");
		if (!href) return;

		// Internal doc links start with / and don't have a protocol
		if (href.startsWith("/") && !href.includes("://")) {
			event.preventDefault();
			// Strip leading / and trailing /
			const docPath = href.replace(/^\/+/, "").replace(/\/+$/, "");
			if (!docPath) return;

			// Build breadcrumbs from path segments
			const breadcrumbs = docPath.split("/").map((seg) =>
				seg
					.split("-")
					.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
					.join(" ")
			);
			navigationStore.openArtifact(docPath, breadcrumbs);
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- Breadcrumb bar -->
	<div class="flex items-center justify-between border-b border-border px-4 py-2">
		<Breadcrumb items={breadcrumbs} />
	</div>

	<!-- Content -->
	{#if artifactStore.loading}
		<div class="flex flex-1 items-center justify-center">
			<LoadingSpinner />
		</div>
	{:else if artifactStore.error}
		<div class="flex flex-1 items-center justify-center px-4">
			<ErrorDisplay message={artifactStore.error} />
		</div>
	{:else if artifact}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="min-h-0 flex-1 overflow-y-auto" onclick={handleContentClick}>
			<div class="p-6">
				<MarkdownRenderer content={artifact.content} />
			</div>
		</div>
	{:else}
		<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
			Select an artifact to view its contents
		</div>
	{/if}
</div>
