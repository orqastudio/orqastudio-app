/**
 * Registration function for the Software Project plugin.
 *
 * Bundled plugins use static imports — this is called during app startup
 * in +layout.svelte after initializeStores().
 */

import type { PluginRegistry } from "@orqastudio/sdk";
import { manifest } from "./manifest.js";

// Static imports of view components
import RoadmapView from "$lib/components/roadmap/RoadmapView.svelte";

/**
 * Register the Software Project plugin with the plugin registry.
 *
 * Components map: view keys → Svelte components.
 * The roadmap view stays in its current location until the full extraction
 * to a separate repo (Step 5 of EPIC-080).
 */
export function registerSoftwareProjectPlugin(registry: PluginRegistry): void {
	registry.register(manifest, {
		// Views
		roadmap: RoadmapView,
		// Widgets — registered by key but rendered by the dashboard
		// (widget rendering happens through the registry, not direct import)
	});
}
