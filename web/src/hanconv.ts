import { invoke } from "@tauri-apps/api/core";

/**
 * 汉字转换类
 */
export class Conversion {
  /**
   * 创建一个新的汉字转换实例
   * @param id 函数名，如 "s2t"
   * @param source 源语言/汉字变体
   * @param target 目标语言/汉字变体
   * @param idiom 是否转换常用词
   */
  constructor(
    public id: string,
    public source: string, 
    public target: string, 
    public idiom: boolean
  ) {}
  
  async convert(text: string): Promise<string> {
    return await invoke(this.id, { s: text });
  }
}

/**
 * Simplified Chinese to Traditional Chinese
 * 
 * 简体中文 → 繁体中文
 */
export const S2T = new Conversion("s2t", "简体中文", "繁体中文", false);

/**
 * Traditional Chinese to Simplified Chinese
 * 
 * 繁体中文 → 简体中文
 */
export const T2S = new Conversion("t2s", "繁体中文", "简体中文", false);

/**
 * Simplified Chinese to Traditional Chinese (Taiwan)
 * 
 * 简体中文 → 繁体中文（台湾）
 */
export const S2TW = new Conversion("s2tw", "简体中文", "繁体中文（台湾）", false);

/**
 * Traditional Chinese (Taiwan) to Simplified Chinese
 * 
 * 繁体中文（台湾）→ 简体中文
 */
export const TW2S = new Conversion("tw2s", "繁体中文（台湾）", "简体中文", false);

/**
 * Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
 * 
 * 简体中文 → 繁体中文（台湾），转换为台湾常用词
 */
export const S2TWP = new Conversion("s2twp", "简体中文", "繁体中文（台湾）", true);

/**
 * Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
 * 
 * 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
 */
export const TW2SP = new Conversion("tw2sp", "繁体中文（台湾）", "简体中文", true);

/**
 * Traditional Chinese to Traditional Chinese (Taiwan)
 * 
 * 繁体中文 → 繁体中文（台湾）
 */
export const T2TW = new Conversion("t2tw", "繁体中文", "繁体中文（台湾）", false);

/**
 * Traditional Chinese (Taiwan) to Traditional Chinese
 * 
 * 繁体中文（台湾）→ 繁体中文
 */
export const TW2T = new Conversion("tw2t", "繁体中文（台湾）", "繁体中文", false);

/**
 * Simplified Chinese to Traditional Chinese (Hong Kong)
 * 
 * 简体中文 → 繁体中文（香港）
 */
export const S2HK = new Conversion("s2hk", "简体中文", "繁体中文（香港）", false);

/**
 * Traditional Chinese (Hong Kong) to Simplified Chinese
 * 
 * 繁体中文（香港）→ 简体中文
 */
export const HK2S = new Conversion("hk2s", "繁体中文（香港）", "简体中文", false);

/**
 * Traditional Chinese to Traditional Chinese (Hong Kong)
 * 
 * 繁体中文 → 繁体中文（香港）
 */
export const T2HK = new Conversion("t2hk", "繁体中文", "繁体中文（香港）", false);

/**
 * Traditional Chinese (Hong Kong) to Traditional Chinese
 * 
 * 繁体中文（香港）→ 繁体中文
 */
export const HK2T = new Conversion("hk2t", "繁体中文（香港）", "繁体中文", false);

/**
 * Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
 * 
 * 繁体字 → 日文新字体
 */
export const T2JP = new Conversion("t2jp", "繁体字", "日文新字体", false);

/**
 * New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
 * 
 * 日文新字体 → 繁体字
 */
export const JP2T = new Conversion("jp2t", "日文新字体", "繁体字", false);

/**
 * 所有支持的汉字转换类型
 */
export const conversions: Conversion[] = [
  S2T, T2S, S2TW, TW2S, S2TWP, TW2SP, T2TW, TW2T, S2HK, HK2S, T2HK, HK2T, T2JP, JP2T
];

/**
 * Simplified Chinese to Traditional Chinese
 * 
 * 简体中文 → 繁体中文
 */
export async function s2t(text: string): Promise<string> {
  return await S2T.convert(text);
}

/**
 * Traditional Chinese to Simplified Chinese
 * 
 * 繁体中文 → 简体中文
 */
export async function t2s(text: string): Promise<string> {
  return await T2S.convert(text);
}

/**
 * Simplified Chinese to Traditional Chinese (Taiwan)
 * 
 * 简体中文 → 繁体中文（台湾）
 */
export async function s2tw(text: string): Promise<string> {
  return await S2TW.convert(text);
}

/**
 * Traditional Chinese (Taiwan) to Simplified Chinese
 * 
 * 繁体中文（台湾）→ 简体中文
 */
export async function tw2s(text: string): Promise<string> {
  return await TW2S.convert(text);
}

/**
 * Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
 * 
 * 简体中文 → 繁体中文（台湾），转换为台湾常用词
 */
export async function s2twp(text: string): Promise<string> {
  return await S2TWP.convert(text);
}

/**
 * Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
 * 
 * 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
 */
export async function tw2sp(text: string): Promise<string> {
  return await TW2SP.convert(text);
}

/**
 * Traditional Chinese to Traditional Chinese (Taiwan)
 * 
 * 繁体中文 → 繁体中文（台湾）
 */
export async function t2tw(text: string): Promise<string> {
  return await T2TW.convert(text);
}

/**
 * Traditional Chinese (Taiwan) to Traditional Chinese
 * 
 * 繁体中文（台湾）→ 繁体中文
 */
export async function tw2t(text: string): Promise<string> {
  return await TW2T.convert(text);
}

/**
 * Simplified Chinese to Traditional Chinese (Hong Kong)
 * 
 * 简体中文 → 繁体中文（香港）
 */
export async function s2hk(text: string): Promise<string> {
  return await S2HK.convert(text);
}

/**
 * Traditional Chinese (Hong Kong) to Simplified Chinese
 * 
 * 繁体中文（香港）→ 简体中文
 */
export async function hk2s(text: string): Promise<string> {
  return await HK2S.convert(text);
}

/**
 * Traditional Chinese to Traditional Chinese (Hong Kong)
 * 
 * 繁体中文 → 繁体中文（香港）
 */
export async function t2hk(text: string): Promise<string> {
  return await T2HK.convert(text);
}

/**
 * Traditional Chinese (Hong Kong) to Traditional Chinese
 * 
 * 繁体中文（香港）→ 繁体中文
 */
export async function hk2t(text: string): Promise<string> {
  return await HK2T.convert(text);
}

/**
 * Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
 * 
 * 繁体字 → 日文新字体
 */
export async function t2jp(text: string): Promise<string> {
  return await T2JP.convert(text);
}

/**
 * New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
 * 
 * 日文新字体 → 繁体字
 */
export async function jp2t(text: string): Promise<string> {
  return await JP2T.convert(text);
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
  S2T,
  T2S,
  S2TW,
  TW2S,
  S2TWP,
  TW2SP,
  T2TW,
  TW2T,
  S2HK,
  HK2S,
  T2HK,
  HK2T,
  T2JP,
  JP2T,
  conversions
};
