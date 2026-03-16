<script lang="ts">
	import FolderPlusIcon from "@lucide/svelte/icons/folder-plus";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import FolderXIcon from "@lucide/svelte/icons/folder-x";
	import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
	import InfoIcon from "@lucide/svelte/icons/info";
	import LogOutIcon from "@lucide/svelte/icons/log-out";
	import { DropdownMenuRoot, DropdownMenuTrigger, DropdownMenuItem, DropdownMenuContent, DropdownMenuSeparator, DropdownMenuGroup, DropdownMenuSub, DropdownMenuSubTrigger, DropdownMenuSubContent } from "@orqastudio/svelte-components/pure";

	interface Props {
		hasProject: boolean;
		onNewProject: () => void;
		onOpenProject: () => void;
		onCloseProject: () => void;
		onSettings: () => void;
		onAbout: () => void;
		onExit: () => void;
	}

	const {
		hasProject,
		onNewProject,
		onOpenProject,
		onCloseProject,
		onSettings,
		onAbout,
		onExit,
	}: Props = $props();

	let activeMenu = $state<string | null>(null);
	const menuMode = $derived(activeMenu !== null);

	const triggerClass =
		"flex h-7 items-center rounded px-2.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground data-[state=open]:bg-accent data-[state=open]:text-foreground";

	function handleMenuClick(menu: string): void {
		activeMenu = activeMenu === menu ? null : menu;
	}

	function handleMenuHover(menu: string): void {
		if (menuMode && activeMenu !== menu) {
			activeMenu = menu;
		}
	}

	function handleItem(action: () => void): void {
		activeMenu = null;
		action();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="flex items-center px-1" data-menu-bar>
	<!-- File menu -->
	<DropdownMenuRoot
		open={activeMenu === "file"}
		onOpenChange={(isOpen) => { if (!isOpen) activeMenu = null; }}
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div onmouseenter={() => handleMenuHover("file")}>
			<DropdownMenuTrigger
				class={triggerClass}
				onclick={(e: MouseEvent) => { e.preventDefault(); handleMenuClick("file"); }}
			>
				File
			</DropdownMenuTrigger>
		</div>
		<DropdownMenuContent align="start" class="w-52">
			<DropdownMenuItem onclick={() => handleItem(onNewProject)}>
				<FolderPlusIcon class="mr-2 h-4 w-4" />
				New Project...
			</DropdownMenuItem>
			<DropdownMenuItem onclick={() => handleItem(onOpenProject)}>
				<FolderOpenIcon class="mr-2 h-4 w-4" />
				Open Project...
			</DropdownMenuItem>
			{#if hasProject}
				<DropdownMenuSeparator />
				<DropdownMenuItem onclick={() => handleItem(onCloseProject)}>
					<FolderXIcon class="mr-2 h-4 w-4" />
					Close Project
				</DropdownMenuItem>
			{/if}
			<DropdownMenuSeparator />
			<DropdownMenuItem onclick={() => handleItem(onExit)}>
				<LogOutIcon class="mr-2 h-4 w-4" />
				Exit
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenuRoot>

	<!-- Edit menu -->
	<DropdownMenuRoot
		open={activeMenu === "edit"}
		onOpenChange={(isOpen) => { if (!isOpen) activeMenu = null; }}
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div onmouseenter={() => handleMenuHover("edit")}>
			<DropdownMenuTrigger
				class={triggerClass}
				onclick={(e: MouseEvent) => { e.preventDefault(); handleMenuClick("edit"); }}
			>
				Edit
			</DropdownMenuTrigger>
		</div>
		<DropdownMenuContent align="start" class="w-52">
			<DropdownMenuItem onclick={() => handleItem(onSettings)}>
				<SlidersHorizontalIcon class="mr-2 h-4 w-4" />
				Settings
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenuRoot>

	<!-- Help menu -->
	<DropdownMenuRoot
		open={activeMenu === "help"}
		onOpenChange={(isOpen) => { if (!isOpen) activeMenu = null; }}
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div onmouseenter={() => handleMenuHover("help")}>
			<DropdownMenuTrigger
				class={triggerClass}
				onclick={(e: MouseEvent) => { e.preventDefault(); handleMenuClick("help"); }}
			>
				Help
			</DropdownMenuTrigger>
		</div>
		<DropdownMenuContent align="start" class="w-52">
			<DropdownMenuItem onclick={() => handleItem(onAbout)}>
				<InfoIcon class="mr-2 h-4 w-4" />
				About
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenuRoot>
</div>
