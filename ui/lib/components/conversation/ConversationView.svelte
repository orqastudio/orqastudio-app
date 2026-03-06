<script lang="ts">
	import MessageBubbleIcon from "@lucide/svelte/icons/message-square";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import SessionHeader from "./SessionHeader.svelte";
	import MessageBubble from "./MessageBubble.svelte";
	import MessageInput from "./MessageInput.svelte";
	import StreamingIndicator from "./StreamingIndicator.svelte";
	import ToolCallCard from "$lib/components/tool/ToolCallCard.svelte";
	import ToolCallGroup from "$lib/components/tool/ToolCallGroup.svelte";
	import ToolApprovalDialog from "$lib/components/tool/ToolApprovalDialog.svelte";
	import { stripToolName } from "$lib/utils/tool-display";
	import { conversationStore } from "$lib/stores/conversation.svelte";
	import { sessionStore } from "$lib/stores/session.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { onMount } from "svelte";

	let scrollViewportRef = $state<HTMLElement | null>(null);
	let userScrolledUp = $state(false);
	let initialized = $state(false);
	let showResumeBanner = $state(false);

	const session = $derived(sessionStore.activeSession);
	const sessions = $derived(sessionStore.sessions);
	const messages = $derived(conversationStore.messages);
	const isStreaming = $derived(conversationStore.isStreaming);
	const isLoading = $derived(conversationStore.isLoading);
	const error = $derived(conversationStore.error);
	const streamingContent = $derived(conversationStore.streamingContent);
	const streamingThinking = $derived(conversationStore.streamingThinking);
	const activeToolCalls = $derived(conversationStore.activeToolCalls);
	const pendingApproval = $derived(conversationStore.pendingApproval);
	const processViolations = $derived(conversationStore.processViolations);

	// Restore last session on mount
	onMount(() => {
		restoreLastSession();

		// Keyboard shortcut: Ctrl+N for new session
		function handleKeydown(event: KeyboardEvent) {
			if ((event.ctrlKey || event.metaKey) && event.key === "n") {
				event.preventDefault();
				handleNewSession();
			}
		}
		window.addEventListener("keydown", handleKeydown);
		return () => window.removeEventListener("keydown", handleKeydown);
	});

	async function restoreLastSession() {
		const project = projectStore.activeProject;
		if (!project) {
			initialized = true;
			return;
		}

		// Load sessions list for the dropdown
		await sessionStore.loadSessions(project.id);

		// Try to restore the last active session
		if (!sessionStore.hasActiveSession) {
			const lastSessionId = settingsStore.lastSessionId;
			if (lastSessionId !== null) {
				try {
					await sessionStore.restoreSession(lastSessionId);
					if (sessionStore.hasActiveSession) {
						showResumeBanner = true;
					}
				} catch {
					// Non-critical — proceed without restoring
				}
			}
		}

		// Auto-create a session if none was restored
		if (!sessionStore.hasActiveSession) {
			await sessionStore.createSession(project.id);
		}

		initialized = true;
	}

	// Load messages when session changes
	$effect(() => {
		if (session) {
			conversationStore.loadMessages(session.id);
		} else {
			conversationStore.clear();
		}
	});

	// Auto-scroll to bottom when new content arrives, unless user scrolled up
	$effect(() => {
		// Track dependencies: messages, streamingContent, and activeToolCalls
		void messages.length;
		void streamingContent;
		void activeToolCalls.size;

		if (!userScrolledUp && scrollViewportRef) {
			requestAnimationFrame(() => {
				if (scrollViewportRef) {
					scrollViewportRef.scrollTop = scrollViewportRef.scrollHeight;
				}
			});
		}
	});

	function handleScroll() {
		if (!scrollViewportRef) return;
		const { scrollTop, scrollHeight, clientHeight } = scrollViewportRef;
		const distanceFromBottom = scrollHeight - scrollTop - clientHeight;
		userScrolledUp = distanceFromBottom > 100;
	}

	function scrollToBottom() {
		userScrolledUp = false;
		if (scrollViewportRef) {
			scrollViewportRef.scrollTop = scrollViewportRef.scrollHeight;
		}
	}

	function handleSend(content: string) {
		if (!session) return;
		userScrolledUp = false;
		showResumeBanner = false;
		conversationStore.sendMessage(session.id, content);
	}

	function handleStop() {
		if (!session) return;
		conversationStore.stopStreaming(session.id);
	}

	async function handleNewSession() {
		const project = projectStore.activeProject;
		if (!project) return;
		conversationStore.clear();
		await sessionStore.createSession(project.id);
	}

	async function handleSelectSession(sessionId: number) {
		conversationStore.clear();
		await sessionStore.selectSession(sessionId);
	}

	async function handleDeleteSession(sessionId: number) {
		await sessionStore.deleteSession(sessionId);
	}

	function handleUpdateTitle(title: string) {
		if (!session) return;
		sessionStore.updateTitle(session.id, title);
	}

	function handleSelectModel(model: string) {
		conversationStore.selectedModel = model;
	}

	// Determine if the last message is a streaming assistant message
	const lastMessage = $derived(messages.length > 0 ? messages[messages.length - 1] : null);
	const isLastMessageStreaming = $derived(
		lastMessage !== null &&
			lastMessage.role === "assistant" &&
			lastMessage.stream_status === "pending"
	);

	// Convert active tool calls map to array for display
	const toolCallsArray = $derived(Array.from(activeToolCalls.values()));

	interface ToolCallInfo {
		toolCallId: string;
		toolName: string;
		input: string | null;
		output: string | null;
		isError: boolean;
		isComplete: boolean;
	}

	type GroupedEntry =
		| { kind: "single"; toolCall: ToolCallInfo }
		| { kind: "group"; toolName: string; toolCalls: ToolCallInfo[]; key: string };

	function groupConsecutiveToolCalls(calls: ToolCallInfo[]): GroupedEntry[] {
		const result: GroupedEntry[] = [];
		let i = 0;
		while (i < calls.length) {
			const current = calls[i];
			const strippedName = stripToolName(current.toolName);

			// Don't group incomplete/running calls
			if (!current.isComplete) {
				result.push({ kind: "single", toolCall: current });
				i++;
				continue;
			}

			// Collect consecutive completed calls of the same type
			let j = i + 1;
			while (
				j < calls.length &&
				calls[j].isComplete &&
				stripToolName(calls[j].toolName) === strippedName
			) {
				j++;
			}

			const count = j - i;
			if (count >= 2) {
				result.push({
					kind: "group",
					toolName: current.toolName,
					toolCalls: calls.slice(i, j),
					key: current.toolCallId,
				});
			} else {
				result.push({ kind: "single", toolCall: current });
			}
			i = j;
		}
		return result;
	}

	const groupedToolCalls = $derived(groupConsecutiveToolCalls(toolCallsArray));
</script>

<div class="flex h-full flex-col">
	{#if !initialized}
		<div class="flex h-full items-center justify-center">
			<LoadingSpinner />
		</div>
	{:else if session}
		<!-- Session header -->
		<SessionHeader
			{session}
			{sessions}
			resolvedModel={conversationStore.currentModel}
			onNewSession={handleNewSession}
			onUpdateTitle={handleUpdateTitle}
			onSelectModel={handleSelectModel}
			onSelectSession={handleSelectSession}
			onDeleteSession={handleDeleteSession}
		/>

		<!-- Resume notification banner -->
		{#if showResumeBanner}
			<div class="flex items-center justify-between border-b border-border bg-muted/50 px-4 py-2 text-sm text-muted-foreground">
				<span>Session resumed after restart. Send a message to continue.</span>
				<button
					class="ml-2 text-xs hover:text-foreground"
					onclick={() => { showResumeBanner = false; }}
				>
					Dismiss
				</button>
			</div>
		{/if}

		<!-- Message area -->
		<div class="relative flex-1 overflow-hidden">
			{#if isLoading}
				<div class="flex h-full items-center justify-center">
					<LoadingSpinner />
				</div>
			{:else if error}
				<div class="flex h-full items-center justify-center p-4">
					<ErrorDisplay
						message={error}
						onRetry={() => {
							if (session) conversationStore.loadMessages(session.id);
						}}
					/>
				</div>
			{:else if messages.length === 0 && !isStreaming}
				<div class="flex h-full items-center justify-center">
					<EmptyState
						icon={MessageBubbleIcon}
						title="No messages yet"
						description="Send a message to start the conversation."
					/>
				</div>
			{:else}
				<ScrollArea class="h-full" bind:viewportRef={scrollViewportRef}>
					<div class="space-y-4 p-4" onscroll={handleScroll}>
						{#each messages as message (message.id)}
							<MessageBubble
								{message}
								streamingContent={isLastMessageStreaming && message.id === lastMessage?.id
									? streamingContent
									: undefined}
							/>
						{/each}

						<!-- Active tool calls during streaming -->
						{#if isStreaming && toolCallsArray.length > 0}
							<div class="space-y-2 px-4">
								{#each groupedToolCalls as entry (entry.kind === "group" ? entry.key : entry.toolCall.toolCallId)}
									{#if entry.kind === "group"}
										<ToolCallGroup
											toolName={entry.toolName}
											toolCalls={entry.toolCalls}
										/>
									{:else}
										<ToolCallCard
											toolName={entry.toolCall.toolName}
											toolInput={entry.toolCall.input || null}
											toolOutput={entry.toolCall.output}
											isError={entry.toolCall.isError}
											isComplete={entry.toolCall.isComplete}
										/>
									{/if}
								{/each}
							</div>
						{/if}

						<!-- Streaming indicator -->
						{#if isStreaming && !streamingContent && toolCallsArray.length === 0}
							<StreamingIndicator isThinking={streamingThinking.length > 0} />
						{/if}

						<!-- Tool approval dialog — rendered inline in the message stream -->
						{#if pendingApproval}
							<div class="px-4">
								<ToolApprovalDialog
									approval={pendingApproval}
									onApprove={() => conversationStore.respondToApproval(true)}
									onDeny={() => conversationStore.respondToApproval(false)}
								/>
							</div>
						{/if}

						<!-- Process violation warnings -->
						{#if processViolations.length > 0}
							<div class="space-y-1 px-4">
								{#each processViolations as violation (violation.check)}
									<div class="rounded-md border border-yellow-500/30 bg-yellow-500/10 px-3 py-2 text-sm text-yellow-700 dark:text-yellow-400">
										<span class="font-medium">Process:</span> {violation.message}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</ScrollArea>

				<!-- Scroll to bottom button -->
				{#if userScrolledUp}
					<button
						class="absolute bottom-2 left-1/2 -translate-x-1/2 rounded-full border border-border bg-background px-3 py-1 text-xs text-muted-foreground shadow-md transition-colors hover:bg-muted"
						onclick={scrollToBottom}
					>
						Scroll to bottom
					</button>
				{/if}
			{/if}
		</div>

		<!-- Input area -->
		<MessageInput
			{isStreaming}
			selectedModel={conversationStore.selectedModel}
			onsend={handleSend}
			onstop={handleStop}
			onmodelchange={(model) => { conversationStore.selectedModel = model; }}
		/>
	{:else}
		<!-- No session selected -->
		<div class="flex h-full items-center justify-center">
			<EmptyState
				icon={MessageBubbleIcon}
				title="No session active"
				description="Select or create a session to begin chatting."
				action={{
					label: "New Session",
					onclick: handleNewSession,
				}}
			/>
		</div>
	{/if}
</div>
