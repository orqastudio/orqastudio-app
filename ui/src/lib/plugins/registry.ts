/**
 * Plugin component registry — maps plugin names to their Svelte view components.
 *
 * Each first-party plugin that ships UI components registers them here.
 * The plugin loader reads this registry to bind components when registering
 * plugins from project.json.
 *
 * Community/user plugins that provide views will need a different mechanism
 * (web components, iframes, or a plugin build step) — this covers first-party
 * plugins whose components are compiled into the app.
 */

import type { Component } from "svelte";

// First-party plugin view components
import RoadmapView from "$lib/components/roadmap/RoadmapView.svelte";

/**
 * Map of plugin name → { viewKey → Svelte component }.
 *
 * When a new first-party plugin with views is created, add its
 * components here. The plugin loader uses this to register views
 * with the plugin registry.
 */
export const PLUGIN_COMPONENTS: Record<string, Record<string, Component>> = {
	"@orqastudio/plugin-software-project": {
		roadmap: RoadmapView,
	},
	// Add future first-party plugin components here:
	// "@orqastudio/plugin-research": { explorer: ResearchExplorer },
};
