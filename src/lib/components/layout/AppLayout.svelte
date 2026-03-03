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

	onMount(() => {
		settingsStore.initialize();
	});

	onDestroy(() => {
		settingsStore.destroy();
	});
</script>

<div class="flex h-screen flex-col bg-background text-foreground">
	<!-- Toolbar -->
	<Toolbar />

	<!-- Main Content Area -->
	<div class="flex flex-1 overflow-hidden">
		<!-- Activity Bar (48px fixed width) -->
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
	</div>

	<!-- Status Bar -->
	<StatusBar />
</div>
