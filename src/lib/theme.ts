import { ref, watchEffect } from "vue";

export type ThemeMode = "light" | "dark" | "system";

const STORAGE_KEY = "theme";

const saved = localStorage.getItem(STORAGE_KEY) as ThemeMode | null;
const mode = ref<ThemeMode>(
  saved === "light" || saved === "dark" || saved === "system" ? saved : "system",
);

const systemDark = window.matchMedia("(prefers-color-scheme: dark)");
const resolved = ref<"light" | "dark">(systemDark.matches ? "dark" : "light");

// 跟随系统时监听系统主题变化
systemDark.addEventListener("change", (e) => {
  if (mode.value === "system") {
    resolved.value = e.matches ? "dark" : "light";
  }
});

watchEffect(() => {
  resolved.value =
    mode.value === "system" ? (systemDark.matches ? "dark" : "light") : mode.value;
  document.documentElement.dataset.theme = resolved.value;
});

watchEffect(() => {
  localStorage.setItem(STORAGE_KEY, mode.value);
});

export function useTheme() {
  return { mode, resolved };
}
