import { forgeInvoke } from "$lib/ipc/invoke";
import type { Project, ProjectSummary } from "$lib/types";

class ProjectStore {
	activeProject = $state<Project | null>(null);
	projects = $state<ProjectSummary[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	get hasProject(): boolean {
		return this.activeProject !== null;
	}

	get artifactCounts(): Record<string, number> {
		if (!this.activeProject) return {};
		const summary = this.projects.find((p) => p.id === this.activeProject?.id);
		return summary ? { total: summary.artifact_count } : {};
	}

	/** Try to restore the last active project on app startup. */
	async loadActiveProject() {
		this.loading = true;
		this.error = null;
		try {
			const project = await forgeInvoke<Project | null>("project_get_active");
			if (project) {
				this.activeProject = project;
			}
		} catch {
			// No active project — not an error, user just needs to open one
		} finally {
			this.loading = false;
		}
	}

	/** Open a project by its directory path. Creates a DB record if new. */
	async openProject(path: string) {
		this.loading = true;
		this.error = null;
		try {
			const project = await forgeInvoke<Project>("project_open", { path });
			this.activeProject = project;
			await this.loadProjects();
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to open project: ${message}`;
		} finally {
			this.loading = false;
		}
	}

	/** Load all known projects. */
	async loadProjects() {
		try {
			const projects = await forgeInvoke<ProjectSummary[]>("project_list");
			this.projects = projects;
		} catch {
			// Non-critical
		}
	}

	/** Close the current project, returning to the welcome screen. */
	closeProject() {
		this.activeProject = null;
		this.error = null;
	}

	setActiveProject(project: Project | null) {
		this.activeProject = project;
		this.error = null;
	}

	setProjects(projects: ProjectSummary[]) {
		this.projects = projects;
	}

	setLoading(loading: boolean) {
		this.loading = loading;
	}

	setError(error: string | null) {
		this.error = error;
		this.loading = false;
	}

	clear() {
		this.activeProject = null;
		this.projects = [];
		this.loading = false;
		this.error = null;
	}
}

export const projectStore = new ProjectStore();
