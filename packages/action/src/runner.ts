import * as exec from '@actions/exec';
import { JsDiffResult } from '@archlinter/core';

export interface RunOptions {
  baseline: string;
  failOn: string;
  workingDirectory: string;
}

export async function runArchlintDiff(options: RunOptions): Promise<JsDiffResult> {
  let stdout = '';
  let stderr = '';

  const args = [
    'diff',
    options.baseline,
    '--fail-on', options.failOn,
    '--explain',
    '--json'
  ];

  try {
    await exec.exec('npx', ['@archlinter/cli', ...args], {
      cwd: options.workingDirectory,
      listeners: {
        stdout: (data: Buffer) => {
          stdout += data.toString();
        },
        stderr: (data: Buffer) => {
          stderr += data.toString();
        }
      },
      ignoreReturnCode: true // We handle failure based on the JSON result or exit code manually
    });

    // Try to find JSON in stdout (in case there are other logs)
    const jsonStart = stdout.indexOf('{');
    const jsonEnd = stdout.lastIndexOf('}');
    
    if (jsonStart === -1 || jsonEnd === -1) {
      throw new Error(`Failed to parse archlint output. Raw output:\n${stdout}\n${stderr}`);
    }

    const jsonStr = stdout.substring(jsonStart, jsonEnd + 1);
    return JSON.parse(jsonStr) as JsDiffResult;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`Archlint execution failed: ${error.message}`);
    }
    throw error;
  }
}
