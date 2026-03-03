export type ActivityView =
	| "chat"
	| "project"
	| "docs"
	| "agents"
	| "rules"
	| "skills"
	| "hooks"
	| "settings";

class NavigationStore {
	activeActivity = $state<ActivityView>("chat");
	navPanelCollapsed = $state(false);

	setActivity(view: ActivityView) {
		this.activeActivity = view;
		// Auto-expand nav panel when switching activities
		if (this.navPanelCollapsed) {
			this.navPanelCollapsed = false;
		}
	}

	toggleNavPanel() {
		this.navPanelCollapsed = !this.navPanelCollapsed;
	}
}

export const navigationStore = new NavigationStore();
