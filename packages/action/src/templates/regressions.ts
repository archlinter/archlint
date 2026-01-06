import { JsRegression, JsRegressionType } from '@archlinter/core';

export function regressions(list: JsRegression[]): string {
  if (list.length === 0) return '';

  let md = `### 🔴 Architectural Regressions\n\n`;
  md += `| Severity | Type | Change | Location |\n`;
  md += `|----------|------|--------|----------|\n`;

  for (const reg of list) {
    const severityIcon = getSeverityIcon(reg.smell.severity);
    const changeType = getChangeTypeLabel(reg.regressionType);
    const location = formatLocation(reg);
    
    md += `| ${severityIcon} ${reg.smell.severity.toUpperCase()} | **${reg.smell.smellType}** | ${changeType} | ${location} |\n`;
  }

  md += `\n<details>\n<summary><b>Why is this bad and how to fix?</b></summary>\n\n`;

  for (const reg of list) {
    if (reg.explain) {
      md += `#### ${reg.smell.smellType} at ${reg.smell.files[0]}\n\n`;
      md += `**Why bad:** ${reg.explain.whyBad}\n\n`;
      md += `**How to fix:** ${reg.explain.howToFix}\n\n`;
      md += `--- \n\n`;
    }
  }

  md += `</details>\n\n`;
  return md;
}

function getSeverityIcon(severity: string): string {
  switch (severity.toLowerCase()) {
    case 'critical': return '🔴';
    case 'high': return '🟠';
    case 'medium': return '🟡';
    case 'low': return '🔵';
    default: return '⚪️';
  }
}

function getChangeTypeLabel(type: JsRegressionType): string {
  switch (type.type) {
    case 'newSmell': return '🆕 New';
    case 'severityIncrease': return `⬆️ Severity (${type.from} → ${type.to})`;
    case 'metricWorsening': return `📈 ${type.metric} (+${type.changePercent?.toFixed(0)}%)`;
    default: return '⚠️ Worsened';
  }
}

function formatLocation(reg: JsRegression): string {
  if (reg.smell.files && reg.smell.files.length > 0) {
    return `\`${reg.smell.files[0]}\``;
  }
  return 'unknown';
}
