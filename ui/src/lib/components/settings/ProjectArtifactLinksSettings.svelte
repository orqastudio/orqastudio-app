<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Separator } from "$lib/components/ui/separator";
	import type { ProjectSettings, ArtifactLinksConfig, ArtifactLinkDisplayMode } from "$lib/types";
	import { DEFAULT_ARTIFACT_LINK_COLORS } from "$lib/types";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
	}

	const props: Props = $props();

	// Resolve effective config — merge defaults with persisted values.
	const effectiveColors = $derived.by((): Record<string, string> => {
		return { ...DEFAULT_ARTIFACT_LINK_COLORS, ...(props.settings.artifactLinks?.colors ?? {}) };
	});

	const displayMode = $derived<ArtifactLinkDisplayMode>(
		props.settings.artifactLinks?.displayMode ?? "id",
	);

	/** All type prefixes, in display order. */
	const prefixes = Object.keys(DEFAULT_ARTIFACT_LINK_COLORS);

	function buildConfig(): ArtifactLinksConfig {
		return {
			displayMode,
			colors: effectiveColors,
		};
	}

	function handleDisplayModeChange(mode: ArtifactLinkDisplayMode) {
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), displayMode: mode },
		});
	}

	function handleColorChange(prefix: string, color: string) {
		const colors = { ...effectiveColors, [prefix]: color };
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), colors },
		});
	}

	function resetColor(prefix: string) {
		const colors = { ...effectiveColors };
		delete colors[prefix];
		// Restore the default for this prefix only.
		const defaultColor = DEFAULT_ARTIFACT_LINK_COLORS[prefix];
		if (defaultColor) {
			colors[prefix] = defaultColor;
		}
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), colors },
		});
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Artifact Links</Card.Title>
		<Card.Description>Control how artifact link chips are displayed across the app</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-5">
		<!-- Display mode toggle -->
		<div>
			<span class="text-sm font-medium">Display Mode</span>
			<p class="mt-0.5 text-xs text-muted-foreground">
				Choose whether chips show the artifact ID or its resolved title
			</p>
			<div class="mt-2 flex gap-2">
				<button
					class="rounded border px-3 py-1.5 text-sm transition-colors {displayMode === 'id'
						? 'border-primary bg-primary text-primary-foreground'
						: 'border-border bg-background text-muted-foreground hover:bg-accent/50'}"
					onclick={() => handleDisplayModeChange("id")}
				>
					ID
					<span class="ml-1 font-mono text-xs opacity-70">EPIC-001</span>
				</button>
				<button
					class="rounded border px-3 py-1.5 text-sm transition-colors {displayMode === 'title'
						? 'border-primary bg-primary text-primary-foreground'
						: 'border-border bg-background text-muted-foreground hover:bg-accent/50'}"
					onclick={() => handleDisplayModeChange("title")}
				>
					Title
					<span class="ml-1 text-xs opacity-70">My Epic Title</span>
				</button>
			</div>
		</div>

		<Separator />

		<!-- Per-type colour pickers -->
		<div>
			<span class="text-sm font-medium">Chip Colours</span>
			<p class="mt-0.5 text-xs text-muted-foreground">
				Set a background colour for each artifact type prefix
			</p>
			<div class="mt-3 grid grid-cols-2 gap-x-6 gap-y-2 sm:grid-cols-3">
				{#each prefixes as prefix (prefix)}
					{@const color = effectiveColors[prefix] ?? DEFAULT_ARTIFACT_LINK_COLORS[prefix] ?? "#64748b"}
					{@const isDefault = color === DEFAULT_ARTIFACT_LINK_COLORS[prefix]}
					<div class="flex items-center gap-2">
						<!-- Colour swatch + native picker -->
						<label class="flex cursor-pointer items-center gap-1.5" title="Pick colour for {prefix}">
							<span
								class="inline-block h-5 w-5 shrink-0 rounded border border-border"
								style="background-color: {color};"
							></span>
							<input
								type="color"
								class="sr-only"
								value={color}
								oninput={(e) => {
									const target = e.currentTarget;
									handleColorChange(prefix, target.value);
								}}
							/>
						</label>
						<span class="font-mono text-xs font-medium">{prefix}</span>
						{#if !isDefault}
							<button
								class="ml-auto text-[10px] text-muted-foreground hover:text-foreground"
								title="Reset to default"
								onclick={() => resetColor(prefix)}
							>
								↩
							</button>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</Card.Content>
</Card.Root>
