export interface Artifact {
	id: number;
	project_id: number;
	artifact_type: ArtifactType;
	rel_path: string;
	name: string;
	description: string | null;
	content: string;
	file_hash: string | null;
	file_size: number | null;
	file_modified_at: string | null;
	compliance_status: ComplianceStatus;
	relationships: ArtifactRelationship[] | null;
	metadata: Record<string, unknown> | null;
	created_at: string;
	updated_at: string;
}

export interface ArtifactSummary {
	id: number;
	artifact_type: ArtifactType;
	rel_path: string;
	name: string;
	description: string | null;
	compliance_status: ComplianceStatus;
	file_modified_at: string | null;
}

export type ArtifactType = "agent" | "rule" | "skill" | "hook" | "doc";
export type ComplianceStatus = "compliant" | "non_compliant" | "unknown" | "error";

export interface ArtifactRelationship {
	type: "references" | "extends" | "depends_on";
	target: string;
}
