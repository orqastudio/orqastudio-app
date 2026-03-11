<script lang="ts">
	import { ScrollArea as ScrollAreaPrimitive } from "bits-ui";
	import { cn, type WithoutChild } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		viewportRef = $bindable(null),
		class: className,
		orientation = "vertical",
		scrollbarXClasses = "",
		scrollbarYClasses = "",
		children,
		...restProps
	}: WithoutChild<ScrollAreaPrimitive.RootProps> & {
		orientation?: "vertical" | "horizontal" | "both" | undefined;
		scrollbarXClasses?: string | undefined;
		scrollbarYClasses?: string | undefined;
		viewportRef?: HTMLElement | null;
	} = $props();
</script>

<ScrollAreaPrimitive.Root
	bind:ref
	data-slot="scroll-area"
	class={cn("relative overflow-auto", className)}
	{...restProps}
>
	<ScrollAreaPrimitive.Viewport
		bind:ref={viewportRef}
		data-slot="scroll-area-viewport"
		class="size-full rounded-[inherit]"
	>
		{@render children?.()}
	</ScrollAreaPrimitive.Viewport>
</ScrollAreaPrimitive.Root>
