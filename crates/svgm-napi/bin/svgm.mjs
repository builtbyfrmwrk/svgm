#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'node:fs';
import { basename } from 'node:path';
import { parseArgs } from 'node:util';
import { optimize, version } from '../index.js';

const { values: flags, positionals: files } = parseArgs({
  allowPositionals: true,
  options: {
    output:    { type: 'string',  short: 'o' },
    preset:    { type: 'string' },
    precision: { type: 'string' },
    stdout:    { type: 'boolean' },
    'dry-run': { type: 'boolean' },
    quiet:     { type: 'boolean', short: 'q' },
    version:   { type: 'boolean', short: 'v' },
    help:      { type: 'boolean', short: 'h' },
  },
});

if (flags.version) {
  console.log(version());
  process.exit(0);
}

if (flags.help || files.length === 0) {
  console.log(`svgm ${version()} — SVG optimizer

Usage: svgm [options] <file...>

Options:
  -o, --output <path>   Write output to path instead of overwriting
      --preset <name>   Optimization preset (safe | default)
      --precision <n>   Decimal digits for numeric rounding [default: 3]
      --stdout          Write result to stdout
      --dry-run         Preview size reduction without writing
  -q, --quiet           Suppress all output except errors
  -v, --version         Show version
  -h, --help            Show this help

Examples:
  svgm icon.svg                   Optimize in place
  svgm icon.svg -o icon.min.svg   Write to different file
  svgm icon.svg --stdout          Print to stdout
  svgm icon.svg --dry-run         Preview without writing`);
  process.exit(0);
}

if (files.length > 1 && flags.output) {
  error('cannot use -o with multiple input files');
}

if (files.length > 1 && flags.stdout) {
  error('cannot use --stdout with multiple input files');
}

const opts = {};
if (flags.preset) opts.preset = flags.preset;
if (flags.precision) opts.precision = parseInt(flags.precision, 10);

const piped = !process.stdout.isTTY;
let exitCode = 0;

for (const file of files) {
  try {
    const input = readFileSync(file, 'utf-8');
    const result = optimize(input, opts);
    const writeToStdout = flags.stdout || (piped && !flags.output && files.length === 1);

    if (flags['dry-run']) {
      if (!flags.quiet) printSummary(file, input.length, result.data.length, result.iterations);
      continue;
    }

    if (writeToStdout) {
      process.stdout.write(result.data);
    } else {
      const target = flags.output || file;
      writeFileSync(target, result.data);
      if (!flags.quiet) printSummary(file, input.length, result.data.length, result.iterations);
    }
  } catch (e) {
    process.stderr.write(`error: ${file}: ${e.message}\n`);
    exitCode = 1;
  }
}

if (exitCode) process.exit(exitCode);

function printSummary(path, inputSize, outputSize, iterations) {
  const name = basename(path);
  const reduction = inputSize > 0 ? ((inputSize - outputSize) / inputSize) * 100 : 0;
  const passWord = iterations === 1 ? 'pass' : 'passes';
  process.stderr.write(
    `\n  ${name}\n  ${formatBytes(inputSize)} -> ${formatBytes(outputSize)} (${reduction.toFixed(1)}% smaller)  ${iterations} ${passWord}\n\n`
  );
}

function formatBytes(bytes) {
  if (bytes >= 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MiB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  return `${bytes} B`;
}

function error(msg) {
  process.stderr.write(`error: ${msg}\n`);
  process.exit(1);
}
