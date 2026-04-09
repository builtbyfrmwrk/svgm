import { describe, it, expect } from 'vitest';
import { execSync } from 'node:child_process';
import { writeFileSync, readFileSync, mkdtempSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { optimize, version } from '../index.js';

const bin = join(import.meta.dirname, '..', 'bin', 'svgm.mjs');
const run = (args: string) => execSync(`node ${bin} ${args}`, { encoding: 'utf-8' });
const testSvg = '<svg xmlns="http://www.w3.org/2000/svg"><g><rect width="10" height="10"/></g></svg>';

describe('optimize', () => {
  it('optimizes SVG with defaults', () => {
    const svg = '<svg xmlns="http://www.w3.org/2000/svg"><g><rect width="10" height="10"/></g></svg>';
    const result = optimize(svg);
    expect(result.data).toBeTruthy();
    expect(result.iterations).toBeGreaterThanOrEqual(1);
  });

  it('accepts safe preset', () => {
    const svg = '<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10"/></svg>';
    const result = optimize(svg, { preset: 'safe' });
    expect(result.data).toBeTruthy();
  });

  it('accepts pass overrides', () => {
    const svg = '<svg xmlns="http://www.w3.org/2000/svg"><desc>Created with Figma</desc><rect width="10" height="10"/></svg>';
    const result = optimize(svg, { passes: { removeDesc: true } });
    expect(result.data).not.toContain('<desc>');
  });

  it('throws on invalid SVG', () => {
    expect(() => optimize('<not valid xml')).toThrow();
  });

  it('throws on unknown preset', () => {
    expect(() => optimize('<svg/>', { preset: 'unknown' })).toThrow('unknown preset');
  });

  it('throws on unknown pass', () => {
    expect(() => optimize('<svg/>', { passes: { nonExistent: true } })).toThrow('unknown pass');
  });
});

describe('version', () => {
  it('returns version string', () => {
    const v = version();
    expect(v).toMatch(/^\d+\.\d+\.\d+/);
  });
});

describe('cli', () => {
  it('--version prints version', () => {
    const out = run('--version').trim();
    expect(out).toMatch(/^\d+\.\d+\.\d+/);
  });

  it('--help prints usage', () => {
    const out = run('--help');
    expect(out).toContain('Usage:');
    expect(out).toContain('--preset');
  });

  it('--stdout outputs optimized SVG', () => {
    const tmp = mkdtempSync(join(tmpdir(), 'svgm-'));
    const file = join(tmp, 'test.svg');
    writeFileSync(file, testSvg);
    const out = run(`${file} --stdout`);
    expect(out).toContain('<svg');
    expect(out).toContain('<path');
    rmSync(tmp, { recursive: true });
  });

  it('optimizes file in place with -o', () => {
    const tmp = mkdtempSync(join(tmpdir(), 'svgm-'));
    const file = join(tmp, 'test.svg');
    writeFileSync(file, testSvg);
    run(`${file} -o ${file} --quiet`);
    const result = readFileSync(file, 'utf-8');
    expect(result).toContain('<path');
    expect(result.length).toBeLessThan(testSvg.length);
    rmSync(tmp, { recursive: true });
  });

  it('--dry-run does not modify file', () => {
    const tmp = mkdtempSync(join(tmpdir(), 'svgm-'));
    const file = join(tmp, 'test.svg');
    writeFileSync(file, testSvg);
    run(`${file} --dry-run --quiet`);
    const result = readFileSync(file, 'utf-8');
    expect(result).toBe(testSvg);
    rmSync(tmp, { recursive: true });
  });

  it('-o writes to output path', () => {
    const tmp = mkdtempSync(join(tmpdir(), 'svgm-'));
    const input = join(tmp, 'in.svg');
    const output = join(tmp, 'out.svg');
    writeFileSync(input, testSvg);
    run(`${input} -o ${output} --quiet`);
    const result = readFileSync(output, 'utf-8');
    expect(result).toContain('<path');
    // original unchanged
    expect(readFileSync(input, 'utf-8')).toBe(testSvg);
    rmSync(tmp, { recursive: true });
  });
});
