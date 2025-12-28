import en from "../locales/en.json";
import zhHans from "../locales/zh-Hans.json";
import zhHant from "../locales/zh-Hant.json";

const messages: Record<string, Record<string, string>> = {
  en: en,
  "zh-Hans": zhHans,
  "zh-Hant": zhHant,
};

export const i18n = $state({
  locale: "zh-Hans",
  fallback: "en",
});

export function changeLocale(lang: string) {
  if (messages[lang]) {
    i18n.locale = lang;
  }
}

export function t(key: string) {
  return messages[i18n.locale]?.[key] || messages["en"]?.[key] || key;
}
