<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { type Component } from "svelte";

	// Core view components — registered by route key
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import FullGraphView from "$lib/components/graph/FullGraphView.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import ArtifactMasterDetail from "$lib/components/artifact/ArtifactMasterDetail.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import PluginViewContainer from "$lib/components/plugin/PluginViewContainer.svelte";

	const { navigationStore } = getStores();

	/**
	 * Core view registry — maps route keys to components.
	 * Plugin views are handled separately via PluginViewContainer.
	 * New core views are added here, not as if/else branches.
	 */
	const CORE_VIEWS: Record<string, Component> = {
		"project": ProjectDashboard,
		"artifact-graph": FullGraphView,
		"welcome": WelcomeScreen,
	};

	// Resolve what to render
	const resolved = $derived.by(() => {
		const navItem = navigationStore.activeNavItem;

		// Plugin view — loaded at runtime from plugin bundle
		if (navItem?.type === "plugin" && navItem.pluginSource) {
			return {
				type: "plugin" as const,
				pluginName: navItem.pluginSource,
				viewKey: navItem.key,
			};
		}

		// Core view by activity key
		const activity = navigationStore.activeActivity;
		if (CORE_VIEWS[activity]) {
			return { type: "core" as const, component: CORE_VIEWS[activity] };
		}

		// Artifact viewer (specific artifact selected)
		if (navigationStore.explorerView === "artifact-viewer") {
			return { type: "core" as const, component: ArtifactViewer };
		}

		// Artifact browsing (group or artifact activity)
		if (navigationStore.activeGroup !== null || navigationStore.isArtifactActivity) {
			return { type: "artifact-list" as const, activity };
		}

		// Default
		return { type: "core" as const, component: WelcomeScreen };
	});
</script>

<div class="h-full w-full">
	{#if resolved.type === "plugin"}
		<PluginViewContainer
			pluginName={resolved.pluginName}
			viewKey={resolved.viewKey}
		/>
	{:else if resolved.type === "artifact-list"}
		<ArtifactMasterDetail activity={resolved.activity} />
	{:else}
		{@const ViewComponent = resolved.component}
		<ViewComponent />
	{/if}
</div>
