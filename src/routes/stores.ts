import { writable } from "svelte/store";

export const gamePath = writable<string | null>(null);
