import { writable } from 'svelte/store';

export type Blurts = {
	question: string;
	answer: string;
	userAnswer: string;
}[];

export const blurt = writable<Blurts>([]);
