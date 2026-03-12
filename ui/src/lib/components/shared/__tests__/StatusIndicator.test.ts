import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import StatusIndicator from "../StatusIndicator.svelte";

describe("StatusIndicator", () => {
	it("renders in badge mode by default", () => {
		const { container } = render(StatusIndicator, {
			props: { status: "active" },
		});
		// Badge mode renders a Badge component (span with badge classes)
		// and displays the status text
		expect(screen.getByText("active")).toBeInTheDocument();
	});

	it("renders in dot mode", () => {
		const { container } = render(StatusIndicator, {
			props: { status: "done", mode: "dot" },
		});
		// Dot mode renders a span with rounded-full class
		const dot = container.querySelector(".rounded-full");
		expect(dot).toBeInTheDocument();
	});

	it("renders in inline mode with status text", () => {
		render(StatusIndicator, {
			props: { status: "in-progress", mode: "inline" },
		});
		expect(screen.getByText("in-progress")).toBeInTheDocument();
	});

	it("applies success group styling for done status", () => {
		const { container } = render(StatusIndicator, {
			props: { status: "done", mode: "dot" },
		});
		const dot = container.querySelector(".rounded-full");
		expect(dot?.className).toContain("bg-emerald-500");
	});

	it("falls back to draft group for unknown statuses", () => {
		const { container } = render(StatusIndicator, {
			props: { status: "unknown-status", mode: "dot" },
		});
		const dot = container.querySelector(".rounded-full");
		expect(dot?.className).toContain("bg-muted-foreground");
	});
});
