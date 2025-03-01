import { invoke } from "@tauri-apps/api/core";

export async function s2t(text: string): Promise<string> {
	return await invoke("s2t", { s: text });
}

export async function t2s(text: string): Promise<string> {
	return await invoke("t2s", { s: text });
}

export async function s2tw(text: string): Promise<string> {
	return await invoke("s2tw", { s: text });
}

export async function tw2s(text: string): Promise<string> {
	return await invoke("tw2s", { s: text });
}

export async function s2twp(text: string): Promise<string> {
	return await invoke("s2twp", { s: text });
}

export async function tw2sp(text: string): Promise<string> {
	return await invoke("tw2sp", { s: text });
}

export async function t2tw(text: string): Promise<string> {
	return await invoke("t2tw", { s: text });
}

export async function tw2t(text: string): Promise<string> {
	return await invoke("tw2t", { s: text });
}

export async function s2hk(text: string): Promise<string> {
	return await invoke("s2hk", { s: text });
}

export async function hk2s(text: string): Promise<string> {
	return await invoke("hk2s", { s: text });
}

export async function t2hk(text: string): Promise<string> {
	return await invoke("t2hk", { s: text });
}

export async function hk2t(text: string): Promise<string> {
	return await invoke("hk2t", { s: text });
}

export async function t2jp(text: string): Promise<string> {
	return await invoke("t2jp", { s: text });
}

export async function jp2t(text: string): Promise<string> {
	return await invoke("jp2t", { s: text });
}

export default {
	s2t,
	t2s,
	s2tw,
	tw2s,
	s2twp,
	tw2sp,
	t2tw,
	tw2t,
	s2hk,
	hk2s,
	t2hk,
	hk2t,
	t2jp,
	jp2t,
};
