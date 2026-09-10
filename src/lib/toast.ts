import { ref } from "vue";

const message = ref("");
let timer: number | undefined;

/** 弹出一条自动消失的提示 */
export function showToast(msg: string, duration = 2500) {
  message.value = msg;
  window.clearTimeout(timer);
  timer = window.setTimeout(() => (message.value = ""), duration);
}

export function useToast() {
  return { message };
}
