<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import * as Card from "$lib/components/ui/card";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import ProviderSettings from "./ProviderSettings.svelte";
	import ModelSettings from "./ModelSettings.svelte";
	import AppearanceSettings from "./AppearanceSettings.svelte";
	import ShortcutsSettings from "./ShortcutsSettings.svelte";
	import ProjectSetupWizard from "./ProjectSetupWizard.svelte";
	import ProjectGeneralSettings from "./ProjectGeneralSettings.svelte";
	import ProjectScanningSettings from "./ProjectScanningSettings.svelte";
	import ProjectArtifactLinksSettings from "./ProjectArtifactLinksSettings.svelte";
	import ProjectDeliverySettings from "./ProjectDeliverySettings.svelte";
	import ProjectStatusSettings from "./ProjectStatusSettings.svelte";

	interface Props {
		activeSection?: string;
	}

	const { activeSection }: Props = $props();

	const section = $derived(activeSection ?? settingsStore.activeSection);
	const project = $derived(projectStore.activeProject);
	const isProjectSection = $derived(
		section === "project-general" ||
		section === "project-scanning" ||
		section === "project-artifact-links" ||
		section === "project-delivery" ||
		section === "project-status",
	);
</script>

<ScrollArea.Root class="h-full">
	<div class="space-y-6 p-6">
		{#if section === "provider"}
			<ProviderSettings />
		{/if}

		{#if section === "model"}
			<ModelSettings />
		{/if}

		{#if section === "appearance"}
			<AppearanceSettings />
		{/if}

		{#if section === "shortcuts"}
			<ShortcutsSettings />
		{/if}

		{#if isProjectSection}
			{#if !project}
				<Card.Root>
					<Card.Content class="py-8">
						<div class="flex items-center gap-2 text-sm text-muted-foreground">
							<CircleXIcon class="h-4 w-4" />
							No project loaded
						</div>
					</Card.Content>
				</Card.Root>
			{:else if !projectStore.settingsLoaded}
				<Card.Root>
					<Card.Content class="flex items-center gap-2 py-8">
						<LoaderCircleIcon class="h-4 w-4 animate-spin" />
						<span class="text-sm text-muted-foreground">Loading project settings...</span>
					</Card.Content>
				</Card.Root>
			{:else if projectStore.hasSettings && projectStore.projectSettings}
				{#if section === "project-general"}
					<ProjectGeneralSettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						iconDataUrl={projectStore.iconDataUrl}
						onUploadIcon={(sourcePath) => projectStore.uploadIcon(sourcePath)}
						onRemoveIcon={() => projectStore.removeIcon()}
					/>
				{:else if section === "project-scanning"}
					<ProjectScanningSettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						onRescan={() => projectStore.scanProject(project.path, projectStore.projectSettings?.excluded_paths)}
						rescanning={projectStore.scanning}
					/>
				{:else if section === "project-artifact-links"}
					<ProjectArtifactLinksSettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
					/>
				{:else if section === "project-delivery"}
					<ProjectDeliverySettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
					/>
				{:else if section === "project-status"}
					<ProjectStatusSettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
					/>
				{/if}
			{:else}
				<ProjectSetupWizard
					projectPath={project.path}
					onComplete={(s) => { projectStore.projectSettings = s; }}
				/>
			{/if}
		{/if}
	</div>
</ScrollArea.Root>
