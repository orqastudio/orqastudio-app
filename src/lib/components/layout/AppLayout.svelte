<script lang="ts">
	import ActivityBar from "./ActivityBar.svelte";
	import NavSubPanel from "./NavSubPanel.svelte";
	import Toolbar from "./Toolbar.svelte";
	import StatusBar from "./StatusBar.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
</script>

<div class="flex h-screen flex-col bg-background text-foreground">
	<!-- Toolbar -->
	<Toolbar />

	<!-- Main Content Area -->
	<div class="flex flex-1 overflow-hidden">
		<!-- Activity Bar (48px fixed width) -->
		<ActivityBar />

		<!-- Nav Sub-Panel (collapsible, 200px) -->
		{#if !navigationStore.navPanelCollapsed}
			<NavSubPanel />
		{/if}

		<!-- Explorer + Chat panels -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Explorer Panel -->
			<div class="flex-1 overflow-auto border-r border-border">
				<WelcomeScreen />
			</div>

			<!-- Chat Panel -->
			<div class="flex min-w-[360px] flex-1 flex-col border-l border-border">
				<div class="flex flex-1 items-center justify-center text-muted-foreground">
					<p>Chat panel — will be implemented in Sub-Phase 9</p>
				</div>
			</div>
		</div>
	</div>

	<!-- Status Bar -->
	<StatusBar />
</div>
