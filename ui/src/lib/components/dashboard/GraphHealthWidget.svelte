<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, CardAction } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { getGraphViz } from "$lib/graph-viz.svelte";
	import type { IntegrityCheck } from "@orqastudio/types";

	const graphViz = getGraphViz();

	interface Props {
		checks: IntegrityCheck[];
		loading: boolean;
		fixing?: boolean;
		scanned: boolean;
		onScan: () => void;
		onAutoFix?: () => void;
	}

	const { checks, loading, fixing = false, scanned, onScan, onAutoFix }: Props = $props();

	// Graph-theoretic metrics — reactive, no scan needed.
	const health = $derived(graphViz.graphHealth);

	// Score: percentage of graph in the largest connected component.
	const healthScore = $derived(Math.round(health.largestComponentRatio * 100));

	// Traffic light based on largestComponentRatio thresholds.
	type HealthStatus = "green" | "amber" | "red" | "empty";
	const status = $derived.by((): HealthStatus => {
		if (health.totalNodes === 0) return "empty";
		if (health.largestComponentRatio > 0.9) return "green";
		if (health.largestComponentRatio > 0.7) return "amber";
		return "red";
	});

	const circleClass = $derived.by(() => {
		if (status === "green") return "bg-green-500";
		if (status === "amber") return "bg-amber-500";
		if (status === "red") return "bg-destructive";
		return "bg-muted-foreground/30";
	});

	const scoreLabel = $derived.by(() => {
		if (health.totalNodes === 0) return "—";
		return `${healthScore}%`;
	});

	// Integrity scan counters (complementary to graph metrics).
	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const fixableCount = $derived(checks.filter((c) => c.auto_fixable).length);

	// Orphan severity: green <5%, amber 5-15%, red >15%
	const orphanSeverity = $derived.by(() => {
		if (health.orphanPercentage > 15) return "text-destructive";
		if (health.orphanPercentage > 5) return "text-warning";
		return "text-emerald-500";
	});

	// Avg degree: green >=4, cyan 3-4, amber 2-3, red <2
	const degreeSeverity = $derived.by(() => {
		if (health.avgDegree >= 4) return "text-emerald-500";
		if (health.avgDegree >= 3) return "text-cyan-500";
		if (health.avgDegree >= 2) return "text-warning";
		return "text-destructive";
	});
</script>

<CardRoot class="gap-2 flex flex-col h-full">
	<CardHeader class="pb-2">
		<CardTitle class="flex items-center gap-1.5 text-sm font-semibold">
			<Icon name="eye" size="md" />
			Clarity
		</CardTitle>
		<CardDescription class="text-xs">Where You Are</CardDescription>
		<CardAction>
			{#if loading}
				<LoadingSpinner size="sm" />
			{:else}
				<div class="flex items-center gap-2">
					<span class="text-sm font-semibold tabular-nums">{scoreLabel}</span>
					<span class="relative flex h-3 w-3 shrink-0 items-center justify-center">
						<span class="absolute h-3 w-3 rounded-full {circleClass} opacity-30"></span>
						<span class="h-1.5 w-1.5 rounded-full {circleClass}"></span>
					</span>
				</div>
			{/if}
		</CardAction>
	</CardHeader>
	<CardContent class="flex flex-1 flex-col gap-3 pt-0">
		{#if health.totalNodes > 0}
			<div class="grid grid-cols-2 gap-2 flex-1 text-center text-xs">
				<!-- Clusters -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="network" size="sm" />
						<span class="{health.componentCount > 1 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
							{health.componentCount}
						</span>
						<span class="text-muted-foreground">Cluster{health.componentCount !== 1 ? "s" : ""}</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Connected Components</p>
						<p class="text-muted-foreground">The number of disconnected subgraphs. A healthy graph has 1 cluster — all artifacts are reachable from each other. Multiple clusters indicate orphaned groups of artifacts that aren't connected to the main knowledge graph.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Orphans -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="unlink" size="sm" />
						<span class="{orphanSeverity} font-semibold tabular-nums">
							{health.orphanCount} <span class="font-normal">({health.orphanPercentage}%)</span>
						</span>
						<span class="text-muted-foreground">Orphans</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Orphaned Artifacts</p>
						<p class="text-muted-foreground">Artifacts with no incoming references — nothing points to them. They exist in isolation and won't be discovered through graph traversal. Each orphan should either be connected via relationships or removed if no longer relevant.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Avg Degree -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="git-branch" size="sm" />
						<span class="{degreeSeverity} font-semibold tabular-nums">{health.avgDegree}</span>
						<span class="text-muted-foreground">Avg Degree</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Average Connection Degree</p>
						<p class="text-muted-foreground">The average number of relationships per artifact. Higher means a more interconnected knowledge graph. A well-connected graph has an average degree of 4+ — each artifact relates to multiple others.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Scan Results -->
				{#if scanned}
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
							{#if errorCount > 0}
								<Icon name="circle-alert" size="sm" />
								<span class="text-destructive font-semibold tabular-nums">{errorCount}E / {warningCount}W</span>
							{:else if warningCount > 0}
								<Icon name="triangle-alert" size="sm" />
								<span class="text-warning font-semibold tabular-nums">{warningCount}W</span>
							{:else}
								<Icon name="circle-alert" size="sm" />
								<span class="text-emerald-500 font-semibold">Clean</span>
							{/if}
							<span class="text-muted-foreground">Integrity</span>
						</TooltipTrigger>
						<TooltipContent side="bottom" class="w-64 text-xs">
							<p class="font-medium mb-1">Integrity Scan Results</p>
							<p class="text-muted-foreground">File-level checks: broken references, invalid statuses, missing required fields, schema violations. Errors must be fixed. Warnings indicate potential issues. Use Auto-fix for machine-fixable problems.</p>
						</TooltipContent>
					</TooltipRoot>
				{:else}
					<div class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 text-muted-foreground">
						<Icon name="scan" size="sm" />
						<span class="tabular-nums">—</span>
						<span>Integrity</span>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Actions -->
		<div class="grid grid-cols-2 gap-2 mt-auto">
			<Button variant="outline" size="sm" onclick={onScan} disabled={loading || fixing}>
				{#if loading}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<Icon name="scan" size="sm" />
				{/if}
				Scan
			</Button>
			<Button variant="outline" size="sm" onclick={onAutoFix} disabled={loading || fixing || !scanned || fixableCount === 0 || !onAutoFix}>
				{#if fixing}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<Icon name="wrench" size="sm" />
				{/if}
				Auto-fix{scanned && fixableCount > 0 ? ` (${fixableCount})` : ""}
			</Button>
		</div>
	</CardContent>
</CardRoot>
