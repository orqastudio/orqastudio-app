import { forgeInvoke } from "$lib/ipc/invoke";
import type {
	Project,
	ProjectSummary,
	ProjectSettings,
	ProjectScanResult,
} from "$lib/types";

class ProjectStore {
	activeProject = $state<Project | null>(null);
	projects = $state<ProjectSummary[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	// File-based project settings (.forge/project.json)
	projectSettings = $state<ProjectSettings | null>(null);
	settingsLoaded = $state(false);
	scanning = $state(false);

	get hasProject(): boolean {
		return this.activeProject !== null;
	}

	get hasSettings(): boolean {
		return this.projectSettings !== null;
	}

	get projectPath(): string | null {
		return this.activeProject?.path ?? null;
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
				await this.loadProjectSettings(project.path);
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
			await this.loadProjectSettings(path);
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

	/** Load project settings from .forge/project.json */
	async loadProjectSettings(path: string) {
		this.settingsLoaded = false;
		try {
			const settings = await forgeInvoke<ProjectSettings | null>(
				"project_settings_read",
				{ path },
			);
			this.projectSettings = settings;
		} catch {
			this.projectSettings = null;
		} finally {
			this.settingsLoaded = true;
		}
	}

	/** Save project settings to .forge/project.json */
	async saveProjectSettings(path: string, settings: ProjectSettings) {
		try {
			const saved = await forgeInvoke<ProjectSettings>(
				"project_settings_write",
				{ path, settings },
			);
			this.projectSettings = saved;
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to save project settings: ${message}`;
		}
	}

	/** Scan the project filesystem for stack and governance info */
	async scanProject(
		path: string,
		excludedPaths?: string[],
	): Promise<ProjectScanResult | null> {
		this.scanning = true;
		try {
			const result = await forgeInvoke<ProjectScanResult>("project_scan", {
				path,
				excluded_paths: excludedPaths ?? null,
			});
			return result;
		} catch (err: unknown) {
			const message = err instanceof Error ? err.message : String(err);
			this.error = `Failed to scan project: ${message}`;
			return null;
		} finally {
			this.scanning = false;
		}
	}

	/** Close the current project, returning to the welcome screen. */
	closeProject() {
		this.activeProject = null;
		this.projectSettings = null;
		this.settingsLoaded = false;
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
		this.projectSettings = null;
		this.settingsLoaded = false;
		this.scanning = false;
		this.loading = false;
		this.error = null;
	}
}

export const projectStore = new ProjectStore();
