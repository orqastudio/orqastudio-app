/**
 * Plugin loader — discovers and registers installed plugins from IPC.
 *
 * Reads installed plugins from the Rust backend (which scans the plugins/
 * directory), loads each manifest, and registers with the plugin registry.
 * View components are resolved from the compiled component registry for
 * first-party plugins.
 */

import type { PluginRegistry } from "@orqastudio/sdk";
import type { PluginManifest } from "@orqastudio/types";
import { invoke } from "@tauri-apps/api/core";
import { PLUGIN_COMPONENTS } from "./registry.js";

interface DiscoveredPlugin {
	name: string;
	version: string;
	display_name: string | null;
	description: string | null;
	path: string;
	source: string;
}

/**
 * Discover and register all installed plugins.
 *
 * Called once during app startup. Loads manifests via IPC and binds
 * any available view components from the compiled registry.
 */
export async function registerInstalledPlugins(registry: PluginRegistry): Promise<void> {
	let plugins: DiscoveredPlugin[];
	try {
		plugins = await invoke<DiscoveredPlugin[]>("plugin_list_installed");
	} catch {
		// No plugins directory or IPC not ready — skip silently on startup
		return;
	}

	for (const plugin of plugins) {
		try {
			const manifest = await invoke<PluginManifest>("plugin_get_manifest", {
				name: plugin.name,
			});

			// Resolve view components from the compiled registry
			const components = PLUGIN_COMPONENTS[plugin.name] ?? {};

			registry.register(manifest, components);
		} catch (err) {
			console.error(`Failed to register plugin "${plugin.name}":`, err);
		}
	}
}
