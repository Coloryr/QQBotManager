/** 按接口口径计算字符数：汉字等全角字符算 2，ASCII 算 1 */
export function weightedLen(s: string): number {
  let n = 0;
  for (const ch of s) n += ch.charCodeAt(0) > 255 ? 2 : 1;
  return n;
}

/** 按加权长度截断字符串 */
export function truncateWeighted(s: string, max: number): string {
  let out = "";
  let n = 0;
  for (const ch of s) {
    const w = ch.charCodeAt(0) > 255 ? 2 : 1;
    if (n + w > max) break;
    out += ch;
    n += w;
  }
  return out;
}

export function inputValue(e: Event): string {
  return (e.target as HTMLInputElement).value;
}
