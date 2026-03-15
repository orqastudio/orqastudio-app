<script lang="ts" module>
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import ClockIcon from "@lucide/svelte/icons/clock";
	import FileEditIcon from "@lucide/svelte/icons/file-edit";
	import ArchiveIcon from "@lucide/svelte/icons/archive";
	import type { Component } from "svelte";
	import type { BadgeVariant } from "$lib/components/ui/badge";

	type StatusGroup = "green" | "blue" | "amber" | "muted";

	const STATUS_GROUP_MAP: Record<string, StatusGroup> = {
		// Green — positive/complete/accepted states
		active: "green",
		accepted: "green",
		done: "green",
		complete: "green",
		promoted: "green",
		shaped: "green",
		// Blue — draft/initial states
		draft: "blue",
		todo: "blue",
		captured: "blue",
		proposed: "blue",
		planning: "blue",
		// Amber — in-flight/transitional states
		"in-progress": "amber",
		exploring: "amber",
		ready: "amber",
		review: "amber",
		recurring: "amber",
		// Muted — inactive/historical states
		inactive: "muted",
		archived: "muted",
		surpassed: "muted",
		superseded: "muted",
		deprecated: "muted",
	};

	const GROUP_ICONS: Record<StatusGroup, Component> = {
		green: CheckCircleIcon,
		blue: FileEditIcon,
		amber: ClockIcon,
		muted: ArchiveIcon,
	};

	/** Dot colour classes per status group (Tailwind bg-* utility). */
	const GROUP_DOT_CLASSES: Record<StatusGroup, string> = {
		green: "bg-emerald-500",
		blue: "bg-blue-500",
		amber: "bg-amber-500",
		muted: "bg-muted-foreground/40",
	};

	/** Badge variant per status group. */
	const GROUP_BADGE_VARIANTS: Record<StatusGroup, BadgeVariant> = {
		green: "outline",
		blue: "secondary",
		amber: "warning",
		muted: "secondary",
	};

	/** Extra Tailwind classes applied to badges that need custom colouring. */
	const GROUP_BADGE_EXTRA: Partial<Record<StatusGroup, string>> = {
		green: "border-emerald-500/30 text-emerald-700 dark:text-emerald-400",
		muted: "text-muted-foreground opacity-70",
	};

	function resolveGroup(status: string): StatusGroup {
		return STATUS_GROUP_MAP[status.toLowerCase()] ?? "blue";
	}

	/** Returns the Badge variant for the given status string. */
	export function statusVariant(status: string): BadgeVariant {
		return GROUP_BADGE_VARIANTS[resolveGroup(status)];
	}

	/** Returns Tailwind dot colour classes for the given status string. */
	export function statusColor(status: string): string {
		return GROUP_DOT_CLASSES[resolveGroup(status)];
	}
</script>

<script lang="ts">
	import { Badge } from "$lib/components/ui/badge";
	import { cn } from "$lib/utils";

	let {
		status,
		mode = "badge",
	}: {
		status: string;
		mode?: "badge" | "dot" | "inline";
	} = $props();

	const group = $derived(resolveGroup(status));
	const Icon = $derived(GROUP_ICONS[group]);
	const dotClass = $derived(GROUP_DOT_CLASSES[group]);
	const badgeVariant = $derived(GROUP_BADGE_VARIANTS[group]);
	const badgeExtra = $derived(GROUP_BADGE_EXTRA[group] ?? "");
</script>

{#if mode === "dot"}
	<span
		class={cn("inline-block h-2 w-2 shrink-0 rounded-full", dotClass)}
		title={status}
	></span>
{:else if mode === "inline"}
	<span class="inline-flex items-center gap-1 text-xs">
		<Icon class="h-3.5 w-3.5 shrink-0" />
		<span class="capitalize">{status}</span>
	</span>
{:else}
	<Badge variant={badgeVariant} class="{badgeExtra} capitalize">
		<Icon class="h-3 w-3 shrink-0" />{status}
	</Badge>
{/if}
