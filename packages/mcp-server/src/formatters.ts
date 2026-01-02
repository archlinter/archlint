import { JsScanResult, JsSmellWithExplanation, JsDetectorInfo } from '@archlinter/core';

export function formatResult<T>(
  data: T,
  format: 'json' | 'markdown',
  markdownFormatter: (data: T) => string
) {
  if (format === 'json') {
    return JSON.stringify(data, null, 2);
  }
  return markdownFormatter(data);
}

export function formatScanResultMd(result: JsScanResult): string {
  let md = `# archlint Scan Result\n\n`;
  md += `**Project Path:** \`${result.projectPath}\`\n`;
  md += `**Architecture Grade:** ${result.grade.level} (${result.grade.score}/100)\n\n`;

  md += `## Summary\n`;
  md += `- Files Analyzed: ${result.summary.filesAnalyzed}\n`;
  md += `- Total Smells: ${result.summary.totalSmells}\n`;
  md += `- Cyclic Dependencies: ${result.summary.cyclicDependencies}\n`;
  md += `- God Modules: ${result.summary.godModules}\n`;
  md += `- Dead Code: ${result.summary.deadCode}\n\n`;

  md += `## Top Smells\n`;
  const topSmells = result.smells.slice(0, 10);
  topSmells.forEach((s, i) => {
    md += `${i + 1}. **${s.smell.smellType}** [${s.smell.severity}]\n`;
    md += `   - Problem: ${s.explanation.problem}\n`;
    const rec = s.explanation.recommendations[0] || 'No specific recommendation';
    md += `   - Recommendation: ${rec}\n`;
    md += `   - Files: ${s.smell.files.map((f) => `\`${f}\``).join(', ')}\n\n`;
  });

  if (result.smells.length > 10) {
    md += `*...and ${result.smells.length - 10} more smells. Use \`archlint_get_smells\` to see them all.*\n`;
  }

  return md;
}

export function formatSmellsMd(
  smells: JsSmellWithExplanation[],
  total: number,
  offset: number,
  limit: number
): string {
  let md = `## archlint Smells (${offset + 1}-${Math.min(offset + limit, total)} of ${total})\n\n`;

  if (smells.length === 0) {
    md += 'No smells found matching your filters.\n';
    return md;
  }

  smells.forEach((s, i) => {
    md += `### ${offset + i + 1}. ${s.smell.smellType} [${s.smell.severity}]\n`;
    md += `- **Problem:** ${s.explanation.problem}\n`;
    md += `- **Reason:** ${s.explanation.reason}\n`;
    md += `- **Recommendation:** ${s.explanation.recommendations.join('; ')}\n`;
    md += `- **Files:** ${s.smell.files.map((f) => `\`${f}\``).join(', ')}\n\n`;
  });

  return md;
}

export function formatStatsMd(result: JsScanResult): string {
  let md = `## archlint Statistics\n\n`;
  md += `**Grade:** ${result.grade.level} (${result.grade.score}/100)\n`;
  md += `**Density:** ${result.grade.density.toFixed(4)} smells per file\n\n`;

  md += `| Metric | Value |\n`;
  md += `|--------|-------|\n`;
  md += `| Files Analyzed | ${result.summary.filesAnalyzed} |\n`;
  md += `| Total Smells | ${result.summary.totalSmells} |\n`;
  md += `| Cyclic Dependencies | ${result.summary.cyclicDependencies} |\n`;
  md += `| Cycle Clusters | ${result.summary.cycleClusters} |\n`;
  md += `| Files In Cycles | ${result.summary.filesInCycles} |\n`;
  md += `| God Modules | ${result.summary.godModules} |\n`;
  md += `| Dead Code | ${result.summary.deadCode} |\n`;
  md += `| Dead Symbols | ${result.summary.deadSymbols} |\n`;
  md += `| High Complexity Functions | ${result.summary.highComplexityFunctions} |\n`;
  md += `| Unstable Interfaces | ${result.summary.unstableInterfaces} |\n`;
  md += `| Feature Envy | ${result.summary.featureEnvy} |\n`;
  md += `| Shotgun Surgery | ${result.summary.shotgunSurgery} |\n`;
  md += `| Hub Dependencies | ${result.summary.hubDependencies} |\n`;

  return md;
}

export function formatDetectorsMd(detectors: JsDetectorInfo[]): string {
  let md = `## Available archlint Detectors\n\n`;
  md += `| ID | Name | Deep | Default |\n`;
  md += `|----|------|------|---------|\n`;
  detectors.forEach((d) => {
    md += `| \`${d.id}\` | ${d.name} | ${d.isDeep ? '✅' : '❌'} | ${d.defaultEnabled ? '✅' : '❌'} |\n`;
    md += `| | *${d.description}* | | |\n`;
  });
  return md;
}
