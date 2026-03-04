export interface SetupStatus {
	setup_complete: boolean;
	current_version: number;
	stored_version: number;
	steps: SetupStepStatus[];
}

export interface SetupStepStatus {
	id: string;
	label: string;
	status: StepStatus;
	detail: string | null;
}

export type StepStatus = "pending" | "checking" | "complete" | "error" | "action_required";

export interface ClaudeCliInfo {
	installed: boolean;
	version: string | null;
	path: string | null;
	authenticated: boolean;
	subscription_type: string | null;
}
