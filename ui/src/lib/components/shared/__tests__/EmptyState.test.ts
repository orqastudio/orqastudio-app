import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import EmptyState from "../EmptyState.svelte";

describe("EmptyState", () => {
	it("renders with required title prop", () => {
		render(EmptyState, { props: { title: "No items found" } });
		expect(screen.getByText("No items found")).toBeInTheDocument();
	});

	it("renders description when provided", () => {
		render(EmptyState, {
			props: { title: "Empty", description: "Try adding something" },
		});
		expect(screen.getByText("Try adding something")).toBeInTheDocument();
	});

	it("does not render description when omitted", () => {
		render(EmptyState, { props: { title: "Empty" } });
		expect(screen.queryByText("Try adding something")).not.toBeInTheDocument();
	});

	it("renders action button when provided", () => {
		const onclick = vi.fn();
		render(EmptyState, {
			props: {
				title: "Empty",
				action: { label: "Add item", onclick },
			},
		});
		const button = screen.getByRole("button", { name: "Add item" });
		expect(button).toBeInTheDocument();
	});

	it("does not render action button when omitted", () => {
		render(EmptyState, { props: { title: "Empty" } });
		expect(screen.queryByRole("button")).not.toBeInTheDocument();
	});
});
