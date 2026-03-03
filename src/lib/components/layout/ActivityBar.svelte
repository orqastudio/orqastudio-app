<script lang="ts">
	import MessageSquareIcon from "@lucide/svelte/icons/message-square";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import WebhookIcon from "@lucide/svelte/icons/webhook";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import ActivityBarItem from "./ActivityBarItem.svelte";
	import type { Component } from "svelte";

	const items: { view: ActivityView; icon: Component; label: string }[] = [
		{ view: "chat", icon: MessageSquareIcon, label: "Chat" },
		{ view: "project", icon: FolderOpenIcon, label: "Project" },
		{ view: "docs", icon: FileTextIcon, label: "Docs" },
		{ view: "agents", icon: UsersIcon, label: "Agents" },
		{ view: "rules", icon: ShieldIcon, label: "Rules" },
		{ view: "skills", icon: ZapIcon, label: "Skills" },
		{ view: "hooks", icon: WebhookIcon, label: "Hooks" },
	];
</script>

<div class="flex w-12 flex-col items-center border-r border-border bg-muted/30 py-2">
	{#each items as item}
		<ActivityBarItem
			icon={item.icon}
			label={item.label}
			active={navigationStore.activeActivity === item.view}
			onclick={() => navigationStore.setActivity(item.view)}
		/>
	{/each}

	<div class="flex-1"></div>

	<ActivityBarItem
		icon={SettingsIcon}
		label="Settings"
		active={navigationStore.activeActivity === "settings"}
		onclick={() => navigationStore.setActivity("settings")}
	/>
</div>
