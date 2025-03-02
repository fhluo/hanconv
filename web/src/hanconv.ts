import { invoke } from "@tauri-apps/api/core";

/**
 * Simplified Chinese to Traditional Chinese
 * 
 * 简体中文 → 繁体中文
 */
export async function s2t(text: string): Promise<string> {
	return await invoke("s2t", { s: text });
}

/**
 * Traditional Chinese to Simplified Chinese
 * 
 * 繁体中文 → 简体中文
 */
export async function t2s(text: string): Promise<string> {
	return await invoke("t2s", { s: text });
}

/**
 * Simplified Chinese to Traditional Chinese (Taiwan)
 * 
 * 简体中文 → 繁体中文（台湾）
 */
export async function s2tw(text: string): Promise<string> {
	return await invoke("s2tw", { s: text });
}

/**
 * Traditional Chinese (Taiwan) to Simplified Chinese
 * 
 * 繁体中文（台湾）→ 简体中文
 */
export async function tw2s(text: string): Promise<string> {
	return await invoke("tw2s", { s: text });
}

/**
 * Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
 * 
 * 简体中文 → 繁体中文（台湾），转换为台湾常用词
 */
export async function s2twp(text: string): Promise<string> {
	return await invoke("s2twp", { s: text });
}

/**
 * Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
 * 
 * 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
 */
export async function tw2sp(text: string): Promise<string> {
	return await invoke("tw2sp", { s: text });
}

/**
 * Traditional Chinese to Traditional Chinese (Taiwan)
 * 
 * 繁体中文 → 繁体中文（台湾）
 */
export async function t2tw(text: string): Promise<string> {
	return await invoke("t2tw", { s: text });
}

/**
 * Traditional Chinese (Taiwan) to Traditional Chinese
 * 
 * 繁体中文（台湾）→ 繁体中文
 */
export async function tw2t(text: string): Promise<string> {
	return await invoke("tw2t", { s: text });
}

/**
 * Simplified Chinese to Traditional Chinese (Hong Kong)
 * 
 * 简体中文 → 繁体中文（香港）
 */
export async function s2hk(text: string): Promise<string> {
	return await invoke("s2hk", { s: text });
}

/**
 * Traditional Chinese (Hong Kong) to Simplified Chinese
 * 
 * 繁体中文（香港）→ 简体中文
 */
export async function hk2s(text: string): Promise<string> {
	return await invoke("hk2s", { s: text });
}

/**
 * Traditional Chinese to Traditional Chinese (Hong Kong)
 * 
 * 繁体中文 → 繁体中文（香港）
 */
export async function t2hk(text: string): Promise<string> {
	return await invoke("t2hk", { s: text });
}

/**
 * Traditional Chinese (Hong Kong) to Traditional Chinese
 * 
 * 繁体中文（香港）→ 繁体中文
 */
export async function hk2t(text: string): Promise<string> {
	return await invoke("hk2t", { s: text });
}

/**
 * Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
 * 
 * 繁体字 → 日文新字体
 */
export async function t2jp(text: string): Promise<string> {
	return await invoke("t2jp", { s: text });
}

/**
 * New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
 * 
 * 日文新字体 → 繁体字
 */
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
