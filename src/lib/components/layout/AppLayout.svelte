<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import ActivityBar from "./ActivityBar.svelte";
	import NavSubPanel from "./NavSubPanel.svelte";
	import Toolbar from "./Toolbar.svelte";
	import StatusBar from "./StatusBar.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import ArtifactBrowser from "$lib/components/artifact/ArtifactBrowser.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import ConversationView from "$lib/components/conversation/ConversationView.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { projectStore } from "$lib/stores/project.svelte";

	const hasProject = $derived(projectStore.hasProject);
	const isConfiguring = $derived(navigationStore.activeActivity === "configure");

	onMount(() => {
		settingsStore.initialize();
		projectStore.loadActiveProject();
	});

	onDestroy(() => {
		settingsStore.destroy();
	});

	// When a project becomes active, switch to the project dashboard
	$effect(() => {
		if (hasProject && navigationStore.activeActivity === "chat") {
			navigationStore.setActivity("project");
		}
	});

	// Load doc tree when switching to docs activity (and project is loaded)
	$effect(() => {
		if (
			hasProject &&
			navigationStore.activeActivity === "docs" &&
			artifactStore.docTree.length === 0
		) {
			artifactStore.loadDocTree();
		}
	});

	// Auto-load doc content when the selected artifact path changes
	$effect(() => {
		const path = navigationStore.selectedArtifactPath;
		if (path && navigationStore.activeActivity === "docs") {
			artifactStore.loadDoc(path);
		}
	});
</script>

<div class="flex h-screen flex-col bg-background text-foreground">
	<!-- Toolbar -->
	<Toolbar />

	<!-- Main Content Area -->
	<div class="flex flex-1 overflow-hidden">
		{#if isConfiguring}
			<!-- Forge Configuration — no activity bar, nav panel for config categories -->
			{#if navigationStore.showNavPanel}
				<NavSubPanel />
			{/if}
			<div class="flex flex-1 overflow-hidden">
				<div class="flex-1 overflow-hidden">
					<SettingsView />
				</div>
			</div>
		{:else if hasProject}
			<!-- Activity Bar (48px fixed width) — project only -->
			<ActivityBar />

			<!-- Nav Sub-Panel (collapsible, 200px) -->
			{#if navigationStore.showNavPanel}
				<NavSubPanel />
			{/if}

			<!-- Explorer + Chat panels -->
			<div class="flex flex-1 overflow-hidden">
				<!-- Explorer Panel -->
				<div class="flex-1 overflow-hidden border-r border-border">
					{#if navigationStore.activeActivity === "project"}
						<ProjectDashboard />
					{:else if navigationStore.activeActivity === "settings"}
						<SettingsView />
					{:else if navigationStore.activeActivity === "chat"}
						<WelcomeScreen />
					{:else if navigationStore.isArtifactActivity}
						{#if navigationStore.explorerView === "artifact-viewer"}
							<ArtifactViewer />
						{:else}
							<ArtifactBrowser category={navigationStore.activeActivity} />
						{/if}
					{:else}
						<WelcomeScreen />
					{/if}
				</div>

				<!-- Chat Panel -->
				<div class="flex min-w-[360px] flex-1 flex-col border-l border-border">
					<ConversationView />
				</div>
			</div>
		{:else}
			<!-- No project loaded — welcome screen, no sidebar -->
			<div class="flex flex-1 overflow-hidden">
				<div class="flex-1 overflow-hidden">
					<WelcomeScreen />
				</div>
			</div>
		{/if}
	</div>

	<!-- Status Bar -->
	<StatusBar />
</div>
