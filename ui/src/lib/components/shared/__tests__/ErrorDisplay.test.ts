import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import ErrorDisplay from "../ErrorDisplay.svelte";

describe("ErrorDisplay", () => {
	it("renders the error message", () => {
		render(ErrorDisplay, { props: { message: "Something went wrong" } });
		expect(screen.getByText("Something went wrong")).toBeInTheDocument();
	});

	it("renders retry button when onRetry is provided", () => {
		const onRetry = vi.fn();
		render(ErrorDisplay, {
			props: { message: "Failed", onRetry },
		});
		expect(screen.getByRole("button", { name: "Retry" })).toBeInTheDocument();
	});

	it("does not render retry button when onRetry is omitted", () => {
		render(ErrorDisplay, { props: { message: "Failed" } });
		expect(screen.queryByRole("button")).not.toBeInTheDocument();
	});

	it("calls onRetry when retry button is clicked", async () => {
		const onRetry = vi.fn();
		render(ErrorDisplay, {
			props: { message: "Failed", onRetry },
		});
		await fireEvent.click(screen.getByRole("button", { name: "Retry" }));
		expect(onRetry).toHaveBeenCalledOnce();
	});
});
