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
	import { conversationStore } from "$lib/stores/conversation.svelte";
	import { sessionStore } from "$lib/stores/session.svelte";
	import { projectStore } from "$lib/stores/project.svelte";

	let scrollViewportRef = $state<HTMLElement | null>(null);
	let userScrolledUp = $state(false);

	const session = $derived(sessionStore.activeSession);
	const messages = $derived(conversationStore.messages);
	const isStreaming = $derived(conversationStore.isStreaming);
	const isLoading = $derived(conversationStore.isLoading);
	const error = $derived(conversationStore.error);
	const streamingContent = $derived(conversationStore.streamingContent);
	const streamingThinking = $derived(conversationStore.streamingThinking);
	const activeToolCalls = $derived(conversationStore.activeToolCalls);

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
		conversationStore.sendMessage(session.id, content);
	}

	function handleStop() {
		if (!session) return;
		conversationStore.stopStreaming(session.id);
	}

	async function handleNewSession() {
		const project = projectStore.activeProject;
		if (!project) return;
		await sessionStore.createSession(project.id);
		if (sessionStore.activeSession) {
			conversationStore.clear();
		}
	}

	function handleUpdateTitle(title: string) {
		if (!session) return;
		sessionStore.updateTitle(session.id, title);
	}

	function handleSelectModel(_model: string) {
		// Model selection will be wired to backend in a future phase
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
</script>

<div class="flex h-full flex-col">
	{#if session}
		<!-- Session header -->
		<SessionHeader
			{session}
			resolvedModel={conversationStore.currentModel}
			onNewSession={handleNewSession}
			onUpdateTitle={handleUpdateTitle}
			onSelectModel={handleSelectModel}
		/>

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
								{#each toolCallsArray as toolCall (toolCall.toolCallId)}
									<ToolCallCard
										toolName={toolCall.toolName}
										toolInput={toolCall.input || null}
										toolOutput={toolCall.output}
										isError={toolCall.isError}
										isComplete={toolCall.isComplete}
									/>
								{/each}
							</div>
						{/if}

						<!-- Streaming indicator -->
						{#if isStreaming && !streamingContent && toolCallsArray.length === 0}
							<StreamingIndicator isThinking={streamingThinking.length > 0} />
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
		<MessageInput {isStreaming} onsend={handleSend} onstop={handleStop} />
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
