<script lang="ts">
	import SvelteMarkdown from "@humanspeak/svelte-markdown";
	import { parseFrontmatter } from "$lib/utils/frontmatter";

	let { content }: { content: string } = $props();

	// Strip YAML frontmatter defensively so callers that pass raw file content
	// don't render the --- block as markdown text.
	const body = $derived(parseFrontmatter(content).body);
</script>

<div class="prose prose-sm dark:prose-invert max-w-none">
	<SvelteMarkdown source={body} />
</div>
