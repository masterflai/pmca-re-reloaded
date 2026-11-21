/** Byte maps: how one image divides up, where a comparison's findings sit, and how the sources of a setting relate. */

import { invoke } from "@tauri-apps/api/core";
import type { CompareMap } from "../bindings/CompareMap";
import type { DumpSummary } from "../bindings/DumpSummary";
import type { IndexImage } from "../bindings/IndexImage";
import { make } from "../dom";
import { tr, trFill } from "../i18n";
import { revealSensitive } from "../privacy";

/** A byte range in the data region of the first source; `end` is exclusive. */
export type ByteRange = { start: number; end: number };

const SVG_NS = "http://www.w3.org/2000/svg";
/** The bar is stretched to its box, so only the proportions of this grid matter. */
const BAR_WIDTH = 1000;
const BAR_HEIGHT = 28;
const COMPARE_HEIGHT = 40;
/** Narrower than this a segment would vanish, so it is drawn at this width. */
const MIN_BAR_SEGMENT = 1.5;
/** Starting zoom of the magnifier: a 318-byte gap in a 1 MB image becomes about 16 of 1000 units wide. */
const LENS_ZOOM = 50;
const LENS_MIN = 2;
const LENS_MAX = 4096;
/** A single column is too thin to hit with a mouse, so a click selects this many either side. */
const CLICK_SPREAD = 8;
/** Columns drawn this wide (of 1024), so one differing value stays visible at about 3 px. */
const COLUMN_MIN_WIDTH = 3;
/** Which finding a column is painted with when it holds several. */
const DOMINANT = [
  "different",
  "structural",
  "similar",
  "absent",
  "noise",
  "identical",
  "empty",
] as const;

/** One piece of a bar: where it starts, how big it is, how it is painted and named. */
interface Segment {
  start: number;
  size: number;
  cls: string;
  label: string;
}

function svgEl<K extends keyof SVGElementTagNameMap>(
  tag: K,
  attrs: Record<string, string | number> = {},
): SVGElementTagNameMap[K] {
  const node = document.createElementNS(SVG_NS, tag);
  for (const [name, value] of Object.entries(attrs)) node.setAttribute(name, String(value));
  return node;
}

function hex(n: number): string {
  return `0x${n.toString(16).padStart(6, "0")}`;
}

/** Hex as the backend writes it, with or without a `0x` prefix. */
function parseHex(text: string): number {
  return parseInt(text.replace(/^0x/i, ""), 16);
}

function bytes(n: number): string {
  return `${n.toLocaleString()} B`;
}

/** Draws the part `[from, to)` of the segments across the whole width of `svg`. */
function drawSegments(svg: SVGSVGElement, segments: Segment[], from: number, to: number): void {
  const span = to - from;
  if (span <= 0) return;
  for (const seg of segments) {
    const start = Math.max(seg.start, from);
    const end = Math.min(seg.start + seg.size, to);
    if (end <= start) continue;
    const rect = svgEl("rect", {
      x: ((start - from) / span) * BAR_WIDTH,
      y: 0,
      width: Math.max(((end - start) / span) * BAR_WIDTH, MIN_BAR_SEGMENT),
      height: BAR_HEIGHT,
      class: seg.cls,
    });
    const title = svgEl("title");
    title.textContent = describeSegment(seg);
    rect.append(title);
    svg.append(rect);
  }
}

function describeSegment(seg: Segment): string {
  return `${seg.label}\n${trFill("map-range", {
    start: hex(seg.start),
    end: hex(seg.start + seg.size),
    bytes: seg.size.toLocaleString(),
  })}`;
}

/** The segment drawn on top at byte `p`; later segments lie over earlier ones. */
function segmentAt(segments: Segment[], p: number): Segment | undefined {
  for (let i = segments.length - 1; i >= 0; i--) {
    const seg = segments[i];
    if (p >= seg.start && p < seg.start + seg.size) return seg;
  }
  return undefined;
}

/** What lies at file byte `p`, as one line, or null where no value does. */
type Lookup = (p: number) => string | null;

function valueAt(index: IndexImage[], p: number): string | null {
  let image: IndexImage | undefined;
  for (const candidate of index) {
    if (candidate.start <= p && (image === undefined || candidate.start > image.start)) image = candidate;
  }
  if (image === undefined) return null;
  const q = p - image.start;
  const entries = image.entries;
  let low = 0;
  let high = entries.length - 1;
  let found = -1;
  while (low <= high) {
    const mid = (low + high) >> 1;
    if (entries[mid].offset <= q) {
      found = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  const e = found < 0 ? undefined : entries[found];
  if (e === undefined || q >= e.offset + Math.max(e.size, 1)) return null;
  const prefix = image.start > 0 ? `${index.indexOf(image) + 1}:` : "";
  const name = e.name === null ? "" : ` ${e.name}`;
  return `${prefix}${e.id}${name} · ${e.size} B · ${e.redacted ? "•••" : e.value}`;
}

/**
 * A proportional bar over `total` bytes with a magnifier below it.
 *
 * Pointing shows the surroundings enlarged; a click holds the magnifier so its
 * own segments can be pointed at; the wheel changes the zoom.
 */
function bar(total: number, segments: Segment[], label: string, lookup: Lookup): HTMLElement {
  const svg = svgEl("svg", {
    viewBox: `0 0 ${BAR_WIDTH} ${BAR_HEIGHT}`,
    preserveAspectRatio: "none",
    role: "img",
    "aria-label": label,
    class: "imgmap-zoomable",
  });
  if (total <= 0) return make("div", {}, svg);
  drawSegments(svg, segments, 0, total);
  const frame = svgEl("rect", { y: 0, height: BAR_HEIGHT, class: "imgmap-window" });
  frame.style.display = "none";
  svg.append(frame);

  const lens = svgEl("svg", {
    viewBox: `0 0 ${BAR_WIDTH} ${BAR_HEIGHT}`,
    preserveAspectRatio: "none",
    class: "imgmap-lens",
  });
  const info = make("div", { className: "hint cmp-mono" });
  const lensBox = make("div", { className: "imgmap-lens-box", hidden: true }, lens, info);

  let zoom = LENS_ZOOM;
  let pinned: number | null = null;

  const show = (p: number): void => {
    const span = Math.max(1, total / zoom);
    const from = Math.min(Math.max(0, p - span / 2), total - span);
    const to = from + span;
    lens.replaceChildren();
    drawSegments(lens, segments, from, to);
    const x = ((p - from) / span) * BAR_WIDTH;
    lens.append(
      svgEl("line", { x1: x, x2: x, y1: 0, y2: BAR_HEIGHT, class: "imgmap-cursor" }),
      svgEl("polygon", { points: `${x - 7},0 ${x + 7},0 ${x},7`, class: "imgmap-cursor-tip" }),
      svgEl("polygon", {
        points: `${x - 7},${BAR_HEIGHT} ${x + 7},${BAR_HEIGHT} ${x},${BAR_HEIGHT - 7}`,
        class: "imgmap-cursor-tip",
      }),
    );
    frame.setAttribute("x", String((from / total) * BAR_WIDTH));
    frame.setAttribute("width", String(Math.max((span / total) * BAR_WIDTH, MIN_BAR_SEGMENT)));
    frame.style.display = "";
    const value = lookup(Math.floor(p));
    const seg = segmentAt(segments, p);
    const here = value ?? (seg === undefined ? "–" : describeSegment(seg).replace("\n", " · "));
    info.textContent = `${hex(Math.floor(p))} · ${here} · ×${zoom}`;
    lensBox.hidden = false;
  };
  const hide = (): void => {
    lensBox.hidden = true;
    frame.style.display = "none";
  };
  const at = (event: MouseEvent): number => {
    const box = svg.getBoundingClientRect();
    return Math.min(total - 1, Math.max(0, ((event.clientX - box.left) / box.width) * total));
  };

  svg.addEventListener("pointermove", (event) => {
    if (pinned === null) show(at(event));
  });
  svg.addEventListener("pointerleave", () => {
    if (pinned === null) hide();
  });
  svg.addEventListener("click", (event) => {
    pinned = pinned === null ? at(event) : null;
    svg.classList.toggle("imgmap-pinned", pinned !== null);
    show(pinned ?? at(event));
  });
  svg.addEventListener(
    "wheel",
    (event) => {
      event.preventDefault();
      const next = event.deltaY < 0 ? zoom * 2 : zoom / 2;
      zoom = Math.min(LENS_MAX, Math.max(LENS_MIN, next));
      show(pinned ?? at(event));
    },
    { passive: false },
  );
  return make("div", { className: "imgmap-bar" }, svg, lensBox);
}

/** The map of one image, and of the file around it when the file holds more; `folded` puts it in a closed fold. */
export function layoutMap(summary: DumpSummary, folded = false): HTMLElement {
  const { layout, furtherImages } = summary;
  const free = layout.free;
  const block = make("div", { className: "imgmap" });
  // Loaded once per map; until it arrives, or if it cannot be read, pointing names the stretch only.
  let index: IndexImage[] | null = null;
  invoke<IndexImage[]>("compare_index", { path: summary.path, reveal: revealSensitive() })
    .then((loaded) => {
      index = loaded;
    })
    .catch(() => {});
  const lookup: Lookup = (p) => (index === null ? null : valueAt(index, p));

  if (furtherImages.length > 0 || free.trailing > 0) {
    const total = layout.imageSize + free.trailing;
    const heading = tr("map-file-heading");
    const segments: Segment[] = [
      {
        start: 0,
        size: layout.imageSize,
        cls: "imgmap-image",
        label: trFill("map-image-n", { n: 1 }),
      },
    ];
    let cursor = layout.imageSize;
    const padding = (start: number, size: number): Segment => ({
      start,
      size,
      cls: "imgmap-padding",
      label: tr("map-padding"),
    });
    furtherImages.forEach((span, index) => {
      if (span.start > cursor) segments.push(padding(cursor, span.start - cursor));
      segments.push({
        start: span.start,
        size: span.size,
        cls: "imgmap-further",
        label: trFill("map-image-n", { n: index + 2 }),
      });
      cursor = Math.max(cursor, span.start + span.size);
    });
    if (cursor < total) segments.push(padding(cursor, total - cursor));
    block.append(make("h3", { i18n: "map-file-heading" }), bar(total, segments, heading, lookup));
  }

  const segments: Segment[] = [];
  let cursor = 0;
  const part = (cls: string, key: string, size: number): void => {
    if (size > 0) segments.push({ start: cursor, size, cls, label: tr(key) });
    cursor += size;
  };
  part("imgmap-header", "compare-free-header", free.header);
  part("imgmap-subsystem", "compare-free-subsystem-table", free.subsystemTable);
  part("imgmap-property", "compare-free-property-table", free.propertyTable);
  part("imgmap-reserve", "compare-free-table-reserve", free.tableReserve);
  const dataStart = parseHex(layout.dataOffset);
  segments.push({
    start: dataStart,
    size: Math.max(0, layout.imageSize - dataStart),
    cls: "imgmap-data",
    label: tr("compare-free-data-size"),
  });
  for (const gap of free.gaps) {
    segments.push({
      start: parseHex(gap.offset),
      size: gap.size,
      cls: "imgmap-gap",
      label: tr("map-gaps"),
    });
  }

  const legend = make("ul", { className: "imgmap-legend" });
  const kinds: [string, string, number][] = [
    ["imgmap-header", "compare-free-header", free.header],
    ["imgmap-subsystem", "compare-free-subsystem-table", free.subsystemTable],
    ["imgmap-property", "compare-free-property-table", free.propertyTable],
    ["imgmap-reserve", "compare-free-table-reserve", free.tableReserve],
    ["imgmap-data", "compare-free-data-used", free.dataUsed],
    ["imgmap-gap", "compare-free-data-free", free.dataFree],
    ["imgmap-further", "compare-free-trailing", free.trailing],
  ];
  for (const [cls, key, size] of kinds) {
    if (size <= 0) continue;
    legend.append(
      make(
        "li",
        {},
        make("span", { className: `imgmap-swatch ${cls}` }),
        ` ${tr(key)}: ${bytes(size)}`,
      ),
    );
  }

  block.append(
    make("h3", { i18n: "map-image-heading" }),
    bar(layout.imageSize, segments, tr("map-image-heading"), lookup),
    legend,
    make("p", { className: "hint", i18n: "map-lens-hint" }),
    make("p", {
      className: "hint",
      textContent: trFill("map-hint", {
        shown: free.gaps.length,
        count: free.gapCount.toLocaleString(),
        free: free.dataFree.toLocaleString(),
      }),
    }),
  );
  if (!folded) return block;
  return make("details", { className: "imgmap-fold" }, make("summary", { i18n: "map-toggle" }), block);
}

/**
 * The columns of a comparison along the data region, with the selected range marked.
 *
 * Clicking a column selects a window around it; dragging selects the columns between.
 */
export function compareMap(
  map: CompareMap,
  camera: string,
  selected: ByteRange | null,
  onPick: (range: ByteRange | null) => void,
  visible: ReadonlySet<string>,
): HTMLElement {
  const n = map.columns.length;
  const clamp = (c: number): number => Math.min(n - 1, Math.max(0, c));
  const block = make("div", { className: "imgmap" });
  block.append(
    make("h3", { i18n: "map-compare-heading" }),
    make("p", {
      className: "hint",
      textContent: trFill("map-compare-hint", { camera, columns: n }),
    }),
  );

  const svg = svgEl("svg", {
    viewBox: `0 0 ${Math.max(n, 1)} ${COMPARE_HEIGHT}`,
    preserveAspectRatio: "none",
    role: "img",
    "aria-label": tr("map-compare-heading"),
    class: "imgmap-columns",
  });
  svg.append(svgEl("rect", { x: 0, y: 0, width: Math.max(n, 1), height: COMPARE_HEIGHT, class: "imgmap-bg" }));
  const shown = DOMINANT.filter((cls) => visible.has(cls));
  const drawn: { index: number; rank: number; rect: SVGRectElement }[] = [];
  map.columns.forEach((counts, index) => {
    const present = shown.filter((cls) => (counts[cls] ?? 0) > 0);
    if (present.length === 0) return;
    const all = DOMINANT.filter((cls) => (counts[cls] ?? 0) > 0);
    const total = all.reduce((sum, cls) => sum + (counts[cls] ?? 0), 0);
    const colStart = map.dataStart + index * map.columnBytes;
    const colEnd = Math.min(map.dataEnd, colStart + map.columnBytes);
    const title = svgEl("title");
    title.textContent = [
      trFill("map-compare-column", { start: hex(colStart), end: hex(colEnd), count: total }),
      ...all.map((cls) => `${tr(`compare-class-${cls}`)}: ${counts[cls] ?? 0}`),
    ].join("\n");
    const rect = svgEl("rect", {
      x: Math.max(0, index - (COLUMN_MIN_WIDTH - 1) / 2),
      y: 0,
      width: COLUMN_MIN_WIDTH,
      height: COMPARE_HEIGHT,
      class: `imgmap-cls-${present[0]}`,
    });
    rect.append(title);
    drawn.push({ index, rank: DOMINANT.indexOf(present[0]), rect });
  });
  // The most notable findings are drawn last, so a wide neighbour never hides them.
  drawn.sort((a, b) => b.rank - a.rank || a.index - b.index);
  for (const { rect } of drawn) svg.append(rect);

  if (selected !== null && n > 0) {
    const first = clamp(Math.floor((selected.start - map.dataStart) / map.columnBytes));
    const last = clamp(Math.ceil((selected.end - map.dataStart) / map.columnBytes) - 1);
    svg.append(
      svgEl("rect", {
        x: first,
        y: 0,
        width: last - first + 1,
        height: COMPARE_HEIGHT,
        class: "imgmap-selection",
      }),
    );
  }

  // -1 means no column, so a pointer outside the bar or a bar with no columns picks nothing.
  const columnAt = (event: PointerEvent): number => {
    const box = svg.getBoundingClientRect();
    if (n === 0 || box.width === 0) return -1;
    return clamp(Math.floor(((event.clientX - box.left) / box.width) * n));
  };
  let anchor = -1;
  svg.addEventListener("pointerdown", (event) => {
    anchor = columnAt(event);
    if (anchor >= 0) svg.setPointerCapture(event.pointerId);
  });
  svg.addEventListener("pointercancel", () => {
    anchor = -1;
  });
  svg.addEventListener("pointerup", (event) => {
    const from = anchor;
    anchor = -1;
    const to = columnAt(event);
    if (from < 0 || to < 0) return;
    const [c0, c1] =
      from === to
        ? [Math.max(0, from - CLICK_SPREAD), Math.min(n - 1, from + CLICK_SPREAD)]
        : [Math.min(from, to), Math.max(from, to)];
    onPick({
      start: map.dataStart + c0 * map.columnBytes,
      end: Math.min(map.dataEnd, map.dataStart + (c1 + 1) * map.columnBytes),
    });
  });
  block.append(svg);
  if (shown.length > 0) {
    block.append(
      make(
        "ul",
        { className: "imgmap-legend" },
        ...shown.map((cls) =>
          make("li", {}, make("span", { className: `cmp-dot cmp-${cls}` }), ` ${tr(`compare-class-${cls}`)}`),
        ),
      ),
    );
  }

  if (selected !== null) {
    const clear = make("button", { i18n: "map-compare-all" });
    clear.addEventListener("click", () => onPick(null));
    block.append(
      make(
        "div",
        { className: "row" },
        make("span", {
          className: "cmp-mono",
          textContent: trFill("map-compare-range", { start: hex(selected.start), end: hex(selected.end) }),
        }),
        clear,
      ),
    );
  }
  return block;
}

/** How card file, settings memory and firmware names share one number; boxes, so translations can wrap. */
export function linksExplanation(): HTMLDetailsElement {
  const box = (titleKey: string, detailKey: string): HTMLElement =>
    make(
      "div",
      { className: "links-box" },
      make("strong", { i18n: titleKey }),
      make("div", { className: "cmp-mono", i18n: detailKey }),
    );
  const arrow = (): HTMLElement =>
    make("span", { className: "links-arrow", textContent: "⟷", ariaHidden: "true" });

  const cells = ["0", "1", "2", "…", "n"].map((label, index) =>
    make("span", {
      className: index === 0 ? "links-cell cmp-mono links-cell-work" : "links-cell cmp-mono",
      textContent: label,
    }),
  );

  return make(
    "details",
    { className: "imgmap-links" },
    make("summary", { i18n: "links-toggle" }),
    make("p", { i18n: "links-intro" }),
    make(
      "div",
      { className: "links-row" },
      box("links-card", "links-card-detail"),
      arrow(),
      box("links-store", "links-store-detail"),
      arrow(),
      box("links-fw", "links-fw-detail"),
    ),
    make(
      "div",
      { className: "links-box links-sets" },
      make("strong", { i18n: "links-sets" }),
      make("div", { className: "links-cells" }, ...cells),
      make("p", { i18n: "links-sets-detail" }),
    ),
    make("p", { className: "hint", i18n: "links-note" }),
  );
}
