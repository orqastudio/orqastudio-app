/**
 * Plugin manifest for @orqastudio/plugin-software-project.
 *
 * Declares delivery artifact schemas (milestone, epic, task),
 * the roadmap view, delivery-related widgets, and relationships.
 */

import type { PluginManifest } from "@orqastudio/types";

export const PLUGIN_NAME = "@orqastudio/plugin-software-project";

export const manifest: PluginManifest = {
	name: PLUGIN_NAME,
	version: "0.1.0",
	displayName: "Software Project",
	description:
		"Delivery planning with milestones, epics, tasks, and roadmap views.",
	provides: {
		schemas: [
			{
				key: "milestone",
				label: "Milestone",
				plural: "Milestones",
				icon: "flag",
				defaultPath: ".orqa/delivery/milestones",
				idPrefix: "MS",
				frontmatter: {
					required: ["title", "status"],
					optional: ["description", "priority", "target_date"],
				},
				statusTransitions: {},
			},
			{
				key: "epic",
				label: "Epic",
				plural: "Epics",
				icon: "layers",
				defaultPath: ".orqa/delivery/epics",
				idPrefix: "EPIC",
				frontmatter: {
					required: ["title", "status"],
					optional: ["description", "priority"],
				},
				statusTransitions: {},
			},
			{
				key: "task",
				label: "Task",
				plural: "Tasks",
				icon: "check-square",
				defaultPath: ".orqa/delivery/tasks",
				idPrefix: "TASK",
				frontmatter: {
					required: ["title", "status"],
					optional: ["description", "priority", "assignee"],
				},
				statusTransitions: {},
			},
			{
				key: "research",
				label: "Research",
				plural: "Research",
				icon: "flask-conical",
				defaultPath: ".orqa/discovery/research",
				idPrefix: "RES",
				frontmatter: {
					required: ["title", "status"],
					optional: ["description", "methodology"],
				},
				statusTransitions: {},
			},
			{
				key: "wireframe",
				label: "Wireframe",
				plural: "Wireframes",
				icon: "layout",
				defaultPath: ".orqa/discovery/wireframes",
				idPrefix: "WF",
				frontmatter: {
					required: ["title", "status"],
					optional: ["description"],
				},
				statusTransitions: {},
			},
		],
		views: [
			{ key: "roadmap", label: "Roadmap", icon: "kanban" },
		],
		widgets: [
			{
				key: "pipeline",
				label: "Delivery Pipeline",
				icon: "git-branch",
				defaultPosition: { row: 0, col: 0 },
				defaultSpan: { rows: 1, cols: 2 },
			},
			{
				key: "milestone-context",
				label: "Milestone Context",
				icon: "flag",
				defaultPosition: { row: 0, col: 2 },
				defaultSpan: { rows: 1, cols: 1 },
			},
		],
		relationships: [
			{
				key: "delivers",
				inverse: "delivered-by",
				label: "Delivers",
				inverseLabel: "Delivered By",
				from: ["task", "epic"],
				to: ["epic", "milestone"],
				description:
					"Work delivers to a parent planning artifact",
			},
		],
	},
	defaultNavigation: [
		{
			key: "delivery",
			type: "group",
			icon: "rocket",
			label: "Delivery",
			children: [
				{
					key: "roadmap",
					type: "plugin",
					icon: "kanban",
					pluginSource: PLUGIN_NAME,
				},
				{
					key: "milestones",
					type: "plugin",
					icon: "flag",
					pluginSource: PLUGIN_NAME,
				},
				{
					key: "epics",
					type: "plugin",
					icon: "layers",
					pluginSource: PLUGIN_NAME,
				},
				{
					key: "tasks",
					type: "plugin",
					icon: "check-square",
					pluginSource: PLUGIN_NAME,
				},
			],
		},
	],
};
