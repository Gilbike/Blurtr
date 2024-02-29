import { writable } from 'svelte/store';

export const blurt = writable<{ question: string; anwser: string }[]>([]);
