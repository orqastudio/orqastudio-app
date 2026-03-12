import { describe, it, expect } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import SearchInput from "../SearchInput.svelte";

describe("SearchInput", () => {
	it("renders with default placeholder", () => {
		render(SearchInput);
		expect(screen.getByPlaceholderText("Search...")).toBeInTheDocument();
	});

	it("renders with custom placeholder", () => {
		render(SearchInput, { props: { placeholder: "Find items..." } });
		expect(screen.getByPlaceholderText("Find items...")).toBeInTheDocument();
	});

	it("accepts user input", async () => {
		render(SearchInput);
		const input = screen.getByPlaceholderText("Search...");
		await fireEvent.input(input, { target: { value: "test query" } });
		expect(input).toHaveValue("test query");
	});

	it("renders as an input element", () => {
		render(SearchInput);
		const input = screen.getByPlaceholderText("Search...");
		expect(input.tagName).toBe("INPUT");
	});
});
