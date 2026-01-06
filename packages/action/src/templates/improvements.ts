import { JsImprovement } from '@archlinter/core';

export function improvements(list: JsImprovement[]): string {
  if (list.length === 0) return '';

  let md = `### 🟢 Architectural Improvements\n\n`;
  
  for (const imp of list) {
    md += `- ${imp.message}\n`;
  }

  md += `\n`;
  return md;
}
