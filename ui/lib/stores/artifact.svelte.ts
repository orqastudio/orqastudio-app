import { invoke, extractErrorMessage } from "$lib/ipc/invoke";
import type { Artifact, ArtifactSummary, ArtifactType, DocNode } from "$lib/types";

class ArtifactStore {
	artifacts = $state<ArtifactSummary[]>([]);
	activeArtifact = $state<Artifact | null>(null);
	docTree = $state<DocNode[]>([]);
	researchTree = $state<DocNode[]>([]);
	planTree = $state<DocNode[]>([]);
	docTreeLoading = $state(false);
	researchTreeLoading = $state(false);
	planTreeLoading = $state(false);
	docTreeError = $state<string | null>(null);
	researchTreeError = $state<string | null>(null);
	planTreeError = $state<string | null>(null);
	loading = $state(false);
	error = $state<string | null>(null);
	filterText = $state("");

	// Orqa artifact lists
	milestones = $state<ArtifactSummary[]>([]);
	milestonesLoading = $state(false);
	epics = $state<ArtifactSummary[]>([]);
	epicsLoading = $state(false);
	tasks = $state<ArtifactSummary[]>([]);
	tasksLoading = $state(false);
	ideas = $state<ArtifactSummary[]>([]);
	ideasLoading = $state(false);
	decisions = $state<ArtifactSummary[]>([]);
	decisionsLoading = $state(false);
	lessons = $state<ArtifactSummary[]>([]);
	lessonsLoading = $state(false);

	get filteredArtifacts(): ArtifactSummary[] {
		if (!this.filterText) return this.artifacts;
		const query = this.filterText.toLowerCase();
		return this.artifacts.filter(
			(a) =>
				a.name.toLowerCase().includes(query) ||
				(a.description?.toLowerCase().includes(query) ?? false),
		);
	}

	artifactsByType(type: ArtifactType): ArtifactSummary[] {
		return this.filteredArtifacts.filter((a) => a.artifact_type === type);
	}

	setArtifacts(artifacts: ArtifactSummary[]) {
		this.artifacts = artifacts;
	}

	setActiveArtifact(artifact: Artifact | null) {
		this.activeArtifact = artifact;
		this.error = null;
	}

	setFilter(text: string) {
		this.filterText = text;
	}

	setLoading(loading: boolean) {
		this.loading = loading;
	}

	setError(error: string | null) {
		this.error = error;
		this.loading = false;
	}

	async loadGovernanceList(artifactType: string) {
		this.loading = true;
		this.error = null;
		try {
			const results = await invoke<ArtifactSummary[]>("governance_list", {
				artifactType,
			});
			// Merge into artifacts (replace entries of this type, keep others)
			const other = this.artifacts.filter((a) => a.artifact_type !== artifactType);
			this.artifacts = [...other, ...results];
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load artifacts: ${message}`;
		} finally {
			this.loading = false;
		}
	}

	async loadGovernanceArtifact(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("governance_read", {
				relPath,
			});
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load artifact: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadDocTree() {
		this.docTreeLoading = true;
		this.docTreeError = null;
		try {
			this.docTree = await invoke<DocNode[]>("doc_tree_scan");
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.docTreeError = `Failed to load documentation tree: ${message}`;
			this.docTree = [];
		} finally {
			this.docTreeLoading = false;
		}
	}

	async loadResearchTree() {
		this.researchTreeLoading = true;
		this.researchTreeError = null;
		try {
			this.researchTree = await invoke<DocNode[]>("research_tree_scan");
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.researchTreeError = `Failed to load research tree: ${message}`;
			this.researchTree = [];
		} finally {
			this.researchTreeLoading = false;
		}
	}

	async loadPlanTree() {
		this.planTreeLoading = true;
		this.planTreeError = null;
		try {
			this.planTree = await invoke<DocNode[]>("plan_tree_scan");
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.planTreeError = `Failed to load plans tree: ${message}`;
			this.planTree = [];
		} finally {
			this.planTreeLoading = false;
		}
	}

	async loadPlan(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("plan_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load plan document: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadResearch(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("research_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load research document: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadDoc(relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>("doc_read", { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			const message = extractErrorMessage(err);
			this.error = `Failed to load document: ${message}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	async loadMilestones() {
		this.milestonesLoading = true;
		this.error = null;
		try {
			this.milestones = await invoke<ArtifactSummary[]>("milestone_list");
		} catch (err: unknown) {
			this.error = `Failed to load milestones: ${extractErrorMessage(err)}`;
			this.milestones = [];
		} finally {
			this.milestonesLoading = false;
		}
	}

	async loadEpics() {
		this.epicsLoading = true;
		this.error = null;
		try {
			this.epics = await invoke<ArtifactSummary[]>("epic_list");
		} catch (err: unknown) {
			this.error = `Failed to load epics: ${extractErrorMessage(err)}`;
			this.epics = [];
		} finally {
			this.epicsLoading = false;
		}
	}

	async loadTasks() {
		this.tasksLoading = true;
		this.error = null;
		try {
			this.tasks = await invoke<ArtifactSummary[]>("task_list");
		} catch (err: unknown) {
			this.error = `Failed to load tasks: ${extractErrorMessage(err)}`;
			this.tasks = [];
		} finally {
			this.tasksLoading = false;
		}
	}

	async loadIdeas() {
		this.ideasLoading = true;
		this.error = null;
		try {
			this.ideas = await invoke<ArtifactSummary[]>("idea_list");
		} catch (err: unknown) {
			this.error = `Failed to load ideas: ${extractErrorMessage(err)}`;
			this.ideas = [];
		} finally {
			this.ideasLoading = false;
		}
	}

	async loadDecisions() {
		this.decisionsLoading = true;
		this.error = null;
		try {
			this.decisions = await invoke<ArtifactSummary[]>("decision_list");
		} catch (err: unknown) {
			this.error = `Failed to load decisions: ${extractErrorMessage(err)}`;
			this.decisions = [];
		} finally {
			this.decisionsLoading = false;
		}
	}

	async loadLessons() {
		this.lessonsLoading = true;
		this.error = null;
		try {
			this.lessons = await invoke<ArtifactSummary[]>("lesson_list");
		} catch (err: unknown) {
			this.error = `Failed to load lessons: ${extractErrorMessage(err)}`;
			this.lessons = [];
		} finally {
			this.lessonsLoading = false;
		}
	}

	async loadArtifactByType(type: string, relPath: string) {
		this.loading = true;
		this.error = null;
		try {
			const artifact = await invoke<Artifact>(`${type}_read`, { relPath });
			this.activeArtifact = artifact;
		} catch (err: unknown) {
			this.error = `Failed to load artifact: ${extractErrorMessage(err)}`;
			this.activeArtifact = null;
		} finally {
			this.loading = false;
		}
	}

	clear() {
		this.artifacts = [];
		this.activeArtifact = null;
		this.docTree = [];
		this.researchTree = [];
		this.planTree = [];
		this.docTreeLoading = false;
		this.researchTreeLoading = false;
		this.planTreeLoading = false;
		this.docTreeError = null;
		this.researchTreeError = null;
		this.planTreeError = null;
		this.loading = false;
		this.error = null;
		this.filterText = "";
		this.milestones = [];
		this.milestonesLoading = false;
		this.epics = [];
		this.epicsLoading = false;
		this.tasks = [];
		this.tasksLoading = false;
		this.ideas = [];
		this.ideasLoading = false;
		this.decisions = [];
		this.decisionsLoading = false;
		this.lessons = [];
		this.lessonsLoading = false;
	}
}

export const artifactStore = new ArtifactStore();
