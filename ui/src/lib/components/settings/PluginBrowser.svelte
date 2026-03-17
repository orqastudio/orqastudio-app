<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { invoke } from "@orqastudio/sdk";

	type Tab = "installed" | "official" | "community";

	interface PluginEntry {
		name: string;
		displayName?: string;
		display_name?: string;
		description?: string;
		version?: string;
		path?: string;
		source?: string;
		repo?: string;
		category?: string;
		icon?: string;
		capabilities?: string[];
	}

	let activeTab = $state<Tab>("installed");
	let installed = $state<PluginEntry[]>([]);
	let official = $state<PluginEntry[]>([]);
	let community = $state<PluginEntry[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let manualSource = $state("");
	let installing = $state(false);

	$effect(() => {
		void loadInstalled();
	});

	async function loadInstalled() {
		try {
			installed = await invoke<PluginEntry[]>("plugin_list_installed");
		} catch {
			installed = [];
		}
	}

	async function loadRegistry(source: "official" | "community") {
		loading = true;
		error = null;
		try {
			const result = await invoke<{ plugins: PluginEntry[] }>("plugin_registry_list", { source });
			if (source === "official") {
				official = result.plugins;
			} else {
				community = result.plugins;
			}
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			loading = false;
		}
	}

	async function handleTabChange(tab: Tab) {
		activeTab = tab;
		if (tab === "official" && official.length === 0) {
			await loadRegistry("official");
		}
		if (tab === "community" && community.length === 0) {
			await loadRegistry("community");
		}
	}

	async function installManual() {
		if (!manualSource.trim()) return;
		installing = true;
		error = null;
		try {
			await invoke("plugin_install_local", { path: manualSource.trim() });
			manualSource = "";
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			installing = false;
		}
	}

	async function uninstallPlugin(name: string) {
		try {
			await invoke("plugin_uninstall", { name });
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		}
	}

	function displayName(plugin: PluginEntry): string {
		return plugin.displayName ?? plugin.display_name ?? plugin.name;
	}
</script>

<div class="space-y-4">
	<CardRoot>
		<CardHeader>
			<CardTitle class="text-sm font-semibold">
				<div class="flex items-center gap-2">
					<Icon name="puzzle" size="md" />
					Plugins
				</div>
			</CardTitle>
			<CardAction>
				<Badge variant="outline" class="text-[10px] px-1.5 py-0">
					{installed.length} installed
				</Badge>
			</CardAction>
		</CardHeader>
	</CardRoot>

	<!-- Tab bar -->
	<div class="flex gap-1 rounded-md border border-border p-1">
		<button
			class="flex-1 rounded px-3 py-1.5 text-xs font-medium transition-colors"
			class:bg-primary={activeTab === "installed"}
			class:text-primary-foreground={activeTab === "installed"}
			class:text-muted-foreground={activeTab !== "installed"}
			onclick={() => handleTabChange("installed")}
		>
			Installed
		</button>
		<button
			class="flex-1 rounded px-3 py-1.5 text-xs font-medium transition-colors"
			class:bg-primary={activeTab === "official"}
			class:text-primary-foreground={activeTab === "official"}
			class:text-muted-foreground={activeTab !== "official"}
			onclick={() => handleTabChange("official")}
		>
			Official
		</button>
		<button
			class="flex-1 rounded px-3 py-1.5 text-xs font-medium transition-colors"
			class:bg-primary={activeTab === "community"}
			class:text-primary-foreground={activeTab === "community"}
			class:text-muted-foreground={activeTab !== "community"}
			onclick={() => handleTabChange("community")}
		>
			Community
		</button>
	</div>

	<!-- Tab content -->
	{#if activeTab === "installed"}
		<div class="space-y-2">
			{#each installed as plugin}
				<CardRoot class="gap-2">
					<CardContent class="py-3">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-xs font-medium">{displayName(plugin)}</p>
								<p class="text-[10px] text-muted-foreground">
									{plugin.version ?? "unknown"} — {plugin.source ?? "local"}
								</p>
								{#if plugin.description}
									<p class="mt-1 text-[10px] text-muted-foreground">{plugin.description}</p>
								{/if}
							</div>
							<Button
								variant="ghost"
								size="sm"
								class="h-7 px-2 text-xs text-destructive"
								onclick={() => uninstallPlugin(plugin.name)}
							>
								Uninstall
							</Button>
						</div>
					</CardContent>
				</CardRoot>
			{:else}
				<p class="py-4 text-center text-xs text-muted-foreground">
					No plugins installed yet.
				</p>
			{/each}
		</div>
	{:else if activeTab === "official"}
		{#if loading}
			<div class="flex items-center justify-center py-8">
				<LoadingSpinner size="md" />
			</div>
		{:else}
			<div class="space-y-2">
				{#each official as plugin}
					<CardRoot class="gap-2">
						<CardContent class="py-3">
							<div class="flex items-center justify-between">
								<div>
									<div class="flex items-center gap-2">
										<Icon name={plugin.icon ?? "puzzle"} size="sm" />
										<p class="text-xs font-medium">{displayName(plugin)}</p>
									</div>
									<p class="mt-1 text-[10px] text-muted-foreground">{plugin.description}</p>
									{#if plugin.capabilities?.length}
										<div class="mt-1 flex gap-1">
											{#each plugin.capabilities as cap}
												<Badge variant="outline" class="text-[9px] px-1 py-0">{cap}</Badge>
											{/each}
										</div>
									{/if}
								</div>
								<Button variant="default" size="sm" class="h-7 px-3 text-xs">
									Install
								</Button>
							</div>
						</CardContent>
					</CardRoot>
				{:else}
					<p class="py-4 text-center text-xs text-muted-foreground">
						No official plugins available yet.
					</p>
				{/each}
			</div>
		{/if}
	{:else if activeTab === "community"}
		{#if loading}
			<div class="flex items-center justify-center py-8">
				<LoadingSpinner size="md" />
			</div>
		{:else}
			<div class="space-y-2">
				{#each community as plugin}
					<CardRoot class="gap-2">
						<CardContent class="py-3">
							<div class="flex items-center justify-between">
								<div>
									<div class="flex items-center gap-2">
										<Icon name={plugin.icon ?? "puzzle"} size="sm" />
										<p class="text-xs font-medium">{displayName(plugin)}</p>
										<Badge variant="destructive" class="text-[9px] px-1 py-0">Unverified</Badge>
									</div>
									<p class="mt-1 text-[10px] text-muted-foreground">{plugin.description}</p>
								</div>
								<Button variant="outline" size="sm" class="h-7 px-3 text-xs">
									Install
								</Button>
							</div>
						</CardContent>
					</CardRoot>
				{:else}
					<p class="py-4 text-center text-xs text-muted-foreground">
						No community plugins available yet.
					</p>
				{/each}
			</div>
		{/if}
	{/if}

	<!-- Manual Install -->
	<CardRoot class="gap-2">
		<CardHeader class="pb-2">
			<CardTitle class="text-xs font-semibold">Manual Install</CardTitle>
		</CardHeader>
		<CardContent class="pt-0">
			<div class="flex gap-2">
				<input
					type="text"
					class="flex-1 rounded border border-border bg-background px-3 py-1.5 text-xs placeholder:text-muted-foreground"
					placeholder="owner/repo, owner/repo@v0.2.0, or local path"
					bind:value={manualSource}
					onkeydown={(e: KeyboardEvent) => { if (e.key === "Enter") installManual(); }}
				/>
				<Button
					variant="default"
					size="sm"
					class="h-8 px-3 text-xs"
					disabled={installing || !manualSource.trim()}
					onclick={installManual}
				>
					{#if installing}
						<LoadingSpinner size="sm" />
					{:else}
						Install
					{/if}
				</Button>
			</div>
		</CardContent>
	</CardRoot>

	{#if error}
		<p class="text-xs text-destructive">{error}</p>
	{/if}
</div>
