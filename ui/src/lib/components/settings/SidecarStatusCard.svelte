<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, CardAction } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import { getStores } from "@orqastudio/sdk";

	const { settingsStore } = getStores();

	function sidecarStatusColor(state: string): string {
		switch (state) {
			case "connected":
				return "text-success";
			case "starting":
				return "text-warning";
			case "error":
				return "text-destructive";
			case "stopped":
			case "not_started":
			default:
				return "text-muted-foreground";
		}
	}

	function handleRestart(): void {
		settingsStore.restartSidecar();
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Provider</CardTitle>
		<CardDescription>Claude Code CLI connection and sidecar status</CardDescription>
	</CardHeader>
	<CardContent class="space-y-4">
		<div class="space-y-3">
			<div class="flex items-center gap-2 text-sm">
				<span class="w-32 text-muted-foreground">Sidecar Status:</span>
				<div class="flex items-center gap-1.5">
					{#if settingsStore.sidecarStatus.state === "connected"}
						<CircleCheckIcon class="h-4 w-4 text-success" />
					{:else if settingsStore.sidecarStatus.state === "starting"}
						<LoaderCircleIcon class="h-4 w-4 animate-spin text-warning" />
					{:else if settingsStore.sidecarStatus.state === "error"}
						<CircleXIcon class="h-4 w-4 text-destructive" />
					{:else}
						<CircleDotIcon class="h-4 w-4 text-muted-foreground" />
					{/if}
					<span class={sidecarStatusColor(settingsStore.sidecarStatus.state)}>
						{settingsStore.sidecarStateLabel}
					</span>
				</div>
			</div>

			{#if settingsStore.sidecarStatus.pid !== null}
				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Process ID:</span>
					<span>{settingsStore.sidecarStatus.pid}</span>
				</div>
			{/if}

			{#if settingsStore.sidecarStatus.uptime_seconds !== null}
				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Uptime:</span>
					<span>{Math.floor(settingsStore.sidecarStatus.uptime_seconds)}s</span>
				</div>
			{/if}

			<div class="flex items-center gap-2 text-sm">
				<span class="w-32 text-muted-foreground">CLI Detected:</span>
				{#if settingsStore.sidecarStatus.cli_detected}
					<div class="flex items-center gap-1.5">
						<CircleCheckIcon class="h-4 w-4 text-success" />
						<span>{settingsStore.sidecarStatus.cli_version ?? "Unknown version"}</span>
					</div>
				{:else}
					<div class="flex items-center gap-1.5">
						<CircleXIcon class="h-4 w-4 text-muted-foreground" />
						<span class="text-muted-foreground">Not found</span>
					</div>
				{/if}
			</div>

			{#if settingsStore.sidecarStatus.error_message}
				<div class="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
					{settingsStore.sidecarStatus.error_message}
				</div>
			{/if}
		</div>

		<Separator />

		<Button variant="outline" size="sm" onclick={handleRestart}>
			<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
			Restart Sidecar
		</Button>
	</CardContent>
</CardRoot>
