/**
 * The shutter-count report: a printable page drawn on a canvas and saved as a
 * PNG, a JPEG or a PDF built around that JPEG.
 *
 * The page is always light, whatever the window's theme, since it is meant to
 * be printed or passed on.
 */

import { invoke } from "@tauri-apps/api/core";
import type { AboutInfo, PhotoSummary } from "../dto";
import { currentLocale, tr, trFill } from "../i18n";

export type ReportFormat = "jpeg" | "png" | "pdf";
export type Paper = "a4" | "letter";

/** US Letter for US English, A4 everywhere else. */
export function paperFor(locale: string): Paper {
  return locale === "en-US" ? "letter" : "a4";
}

/** Page size in pixels at 150 dpi. */
const PAGES: Record<Paper, [number, number]> = { a4: [1240, 1754], letter: [1275, 1650] };

const FONT = '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif';
const ACCENT = "#0067c0";
const TEXT = "#1b1b1b";
const MUTED = "#5f5f5f";

/** What the page shows; `serial` is `null` when the user left it out. */
export interface ReportInput {
  photo: PhotoSummary;
  serial: string | null;
  evaluated: string;
  notes: string[];
}

/** Split `text` into lines no wider than `width`, breaking words only when they do not fit. */
function wrap(ctx: CanvasRenderingContext2D, text: string, width: number): string[] {
  const lines: string[] = [];
  let line = "";
  for (const word of text.split(" ")) {
    const next = line ? `${line} ${word}` : word;
    if (ctx.measureText(next).width <= width) {
      line = next;
      continue;
    }
    if (line) lines.push(line);
    line = "";
    // Text without spaces (Chinese, Japanese, Thai) is broken by character.
    for (const char of word) {
      if (line && ctx.measureText(line + char).width > width) {
        lines.push(line);
        line = char;
      } else {
        line += char;
      }
    }
  }
  if (line) lines.push(line);
  return lines;
}

/** Draw the report page for `input` on a new canvas. */
export async function drawReport(input: ReportInput, paper: Paper): Promise<HTMLCanvasElement> {
  const about = await invoke<AboutInfo>("about_info");
  const [width, height] = PAGES[paper];
  const canvas = make2d(width, height);
  const ctx = canvas.getContext("2d")!;
  const rtl = document.documentElement.dir === "rtl";
  ctx.direction = rtl ? "rtl" : "ltr";
  const margin = 96;
  const inner = width - 2 * margin;
  const start = rtl ? width - margin : margin;
  const end = rtl ? margin : width - margin;
  const toward = (x: number, by: number): number => (rtl ? x - by : x + by);

  ctx.fillStyle = "#ffffff";
  ctx.fillRect(0, 0, width, height);

  ctx.fillStyle = ACCENT;
  ctx.fillRect(0, 0, width, 170);
  ctx.fillStyle = "#ffffff";
  ctx.textBaseline = "middle";
  ctx.textAlign = "start";
  ctx.font = `600 44px ${FONT}`;
  ctx.fillText("PMCA-RE RELOADED", start, 85);
  ctx.textAlign = "end";
  ctx.font = `400 32px ${FONT}`;
  ctx.fillText(tr("shutter-report-heading"), end, 85);

  const { photo } = input;
  ctx.textAlign = "center";
  ctx.textBaseline = "alphabetic";
  ctx.fillStyle = TEXT;
  if (photo.countUnavailable) {
    // No number to show; say why instead of a bare dash.
    ctx.font = `700 60px ${FONT}`;
    const lines = wrap(ctx, tr(`shutter-report-${photo.countUnavailable}`), inner * 0.8);
    let y = 400 - ((lines.length - 1) * 70) / 2;
    for (const line of lines) {
      ctx.fillText(line, width / 2, y);
      y += 70;
    }
  } else {
    ctx.font = `700 150px ${FONT}`;
    ctx.fillText(photo.shutterCount?.toLocaleString(currentLocale) ?? "–", width / 2, 380);
    ctx.fillStyle = MUTED;
    ctx.font = `400 36px ${FONT}`;
    const kind = photo.shutter ? ` (${tr(`shutter-type-${photo.shutter}`)})` : "";
    ctx.fillText(`${tr("shutter-count")}${kind}`, width / 2, 450);
  }

  let y = 520;
  ctx.strokeStyle = "#d0d0d0";
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(margin, y);
  ctx.lineTo(width - margin, y);
  ctx.stroke();
  y += 70;

  const model = photo.model && photo.salesName ? `${photo.model} (${photo.salesName})` : photo.model;
  const rows: [string, string | null][] = [
    ["info-model", model],
    ["info-serial", input.serial],
    ["info-firmware", photo.software],
    ["info-lens", photo.lens],
    ["shutter-taken", photo.taken],
    ["shutter-file", photo.fileName],
    ["shutter-evaluated", input.evaluated],
  ];
  const labelWidth = inner * 0.32;
  ctx.textAlign = "start";
  for (const [key, value] of rows) {
    if (!value) continue;
    ctx.font = `400 30px ${FONT}`;
    ctx.fillStyle = MUTED;
    ctx.fillText(tr(key), start, y);
    ctx.fillStyle = TEXT;
    const lines = wrap(ctx, value, inner - labelWidth);
    lines.forEach((line, i) => ctx.fillText(line, toward(start, labelWidth), y + i * 42));
    y += lines.length * 42 + 22;
  }

  y += 20;
  ctx.font = `400 26px ${FONT}`;
  ctx.fillStyle = TEXT;
  for (const note of input.notes) {
    for (const line of wrap(ctx, note, inner)) {
      ctx.fillText(line, start, y);
      y += 36;
    }
    y += 16;
  }

  ctx.font = `400 22px ${FONT}`;
  const source = wrap(ctx, tr("shutter-source"), inner);
  let footer = height - margin - source.length * 30 - 40;
  ctx.beginPath();
  ctx.moveTo(margin, footer - 50);
  ctx.lineTo(width - margin, footer - 50);
  ctx.stroke();
  ctx.fillStyle = MUTED;
  ctx.fillText(trFill("shutter-report-footer", { version: about.version }), start, footer);
  footer += 40;
  for (const line of source) {
    ctx.fillText(line, start, footer);
    footer += 30;
  }
  return canvas;
}

function make2d(width: number, height: number): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  return canvas;
}

/**
 * Encode the page and hand it to the backend, which asks where to save it.
 *
 * Resolves to the path written, or `null` when the dialog was dismissed.
 */
export async function saveReport(
  canvas: HTMLCanvasElement,
  format: ReportFormat,
  paper: Paper,
  name: string,
): Promise<string | null> {
  const type = format === "png" ? "image/png" : "image/jpeg";
  const blob = await new Promise<Blob>((resolve, reject) =>
    canvas.toBlob((b) => (b ? resolve(b) : reject(new Error("canvas"))), type, 0.92),
  );
  const bytes = new Uint8Array(await blob.arrayBuffer());
  return invoke<string | null>("save_shutter_report", bytes, {
    headers: { "x-format": format, "x-paper": paper, "x-name": name },
  });
}
