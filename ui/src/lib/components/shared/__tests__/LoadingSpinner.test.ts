import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import LoadingSpinner from "../LoadingSpinner.svelte";

describe("LoadingSpinner", () => {
	it("renders branded variant by default (shows image)", () => {
		render(LoadingSpinner);
		const img = screen.getByAltText("Loading\u2026");
		expect(img).toBeInTheDocument();
	});

	it("renders minimal spinner when variant is minimal", () => {
		const { container } = render(LoadingSpinner, {
			props: { variant: "minimal" },
		});
		// Minimal renders a div with animate-spin, not an img
		expect(screen.queryByAltText("Loading\u2026")).not.toBeInTheDocument();
		expect(container.querySelector(".animate-spin")).toBeInTheDocument();
	});

	it("renders minimal spinner for sm size even with branded variant", () => {
		const { container } = render(LoadingSpinner, {
			props: { size: "sm", variant: "branded" },
		});
		// sm + branded falls back to minimal (useBranded is false when size=sm)
		expect(screen.queryByAltText("Loading\u2026")).not.toBeInTheDocument();
		expect(container.querySelector(".animate-spin")).toBeInTheDocument();
	});

	it("renders branded image for lg size", () => {
		render(LoadingSpinner, { props: { size: "lg" } });
		const img = screen.getByAltText("Loading\u2026");
		expect(img).toBeInTheDocument();
	});
});
