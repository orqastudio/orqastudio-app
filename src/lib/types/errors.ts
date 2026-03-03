export interface ForgeError {
	code:
		| "not_found"
		| "database"
		| "file_system"
		| "sidecar"
		| "validation"
		| "scan"
		| "serialization"
		| "permission_denied";
	message: string;
}
