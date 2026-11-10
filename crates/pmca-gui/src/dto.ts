/**
 * Data shapes exchanged with the Rust backend.
 *
 * The shapes of `pmca_app`'s DTOs are generated from the Rust types into
 * `./bindings` by `cargo test -p pmca-app` (ts-rs) and re-exported here, so a
 * changed Rust type turns into a type error in the window. The few shapes
 * defined by other crates or by the Tauri shell itself are still written out
 * below.
 */

export type { AboutInfo } from "./bindings/AboutInfo";
export type { AboutLink } from "./bindings/AboutLink";
export type { AppBackupOutcome } from "./bindings/AppBackupOutcome";
export type { AvailableUpdate } from "./bindings/AvailableUpdate";
export type { CameraPresence } from "./bindings/CameraPresence";
export type { CameraSummary } from "./bindings/CameraSummary";
export type { CompareReport } from "./bindings/CompareReport";
export type { CompareRow } from "./bindings/CompareRow";
export type { CompareSource } from "./bindings/CompareSource";
export type { CompatModel } from "./bindings/CompatModel";
export type { DeviceGroup } from "./bindings/DeviceGroup";
export type { DeviceLists } from "./bindings/DeviceLists";
export type { DumpCamera } from "./bindings/DumpCamera";
export type { DumpLayout } from "./bindings/DumpLayout";
export type { DumpSummary } from "./bindings/DumpSummary";
export type { FreeSpaceView } from "./bindings/FreeSpaceView";
export type { GapView } from "./bindings/GapView";
export type { GpsModel } from "./bindings/GpsModel";
export type { MoreModel } from "./bindings/MoreModel";
export type { Support } from "./bindings/Support";
export type { CommunityApp } from "./bindings/CommunityApp";
export type { CommunityList } from "./bindings/CommunityList";
export type { FirmwareOutcome } from "./bindings/FirmwareOutcome";
export type { InstallOutcome } from "./bindings/InstallOutcome";
export type { InstalledDevice } from "./bindings/InstalledDevice";
export type { KeyClassStatus } from "./bindings/KeyClassStatus";
export type { KeyStatus } from "./bindings/KeyStatus";
export type { PhotoSummary } from "./bindings/PhotoSummary";
export type { UsageLogEntry } from "./bindings/UsageLogEntry";
export type { UsageLogEvent } from "./bindings/UsageLogEvent";
export type { UsageLogView } from "./bindings/UsageLogView";
export type { PluginCompat } from "./bindings/PluginCompat";
export type { PluginEntry } from "./bindings/PluginEntry";
export type { PluginInfo } from "./bindings/PluginInfo";
export type { PluginList } from "./bindings/PluginList";
export type { PluginRunOutcome } from "./bindings/PluginRunOutcome";
export type { ProgressEvent as ProgressPayload } from "./bindings/ProgressEvent";
export type { PinnedProject } from "./bindings/PinnedProject";
export type { ProjectDetails } from "./bindings/ProjectDetails";
export type { ProjectFailure } from "./bindings/ProjectFailure";
export type { ProjectInfo } from "./bindings/ProjectInfo";
export type { ProjectList } from "./bindings/ProjectList";
export type { ProjectRepo } from "./bindings/ProjectRepo";
export type { Source } from "./bindings/Source";
export type { SourceRepo } from "./bindings/SourceRepo";
export type { SourceSuggestion } from "./bindings/SourceSuggestion";
export type { StreamingSettings } from "./bindings/StreamingSettings";
export type { TweakReport } from "./bindings/TweakReport";
export type { WifiAccessPoint } from "./bindings/WifiAccessPoint";

/** Event the Rust backend streams installation progress on. */
export const PROGRESS_EVENT = "pmca://progress";

/** The sources the user has added and the suggestions not yet switched on (Tauri shell). */
export interface SourcesState {
  sources: import("./bindings/Source").Source[];
  suggestions: import("./bindings/SourceSuggestion").SourceSuggestion[];
}

/** A plugin tool's result and where its output was saved (Tauri shell). */
export interface PluginToolResult {
  outcome: import("./bindings/PluginRunOutcome").PluginRunOutcome;
  written: string | null;
}

/** Where the usage log ended up, reported as an absolute path (Tauri shell). */
export interface UsageLogOutcome {
  path: string;
  bytes: number;
}

/** A store app fetched ahead of a batch installation (Tauri shell). */
export interface DownloadedApp {
  package: string;
  version: string;
}

/** One blocking finding in an uploaded translation file (`pmca_i18n`). */
interface CatalogProblem {
  line: number;
  key: string;
  kind: string;
  detail: string;
}

/** The backend's verdict on an uploaded translation file (`pmca_i18n`). */
export interface CatalogReport {
  problems: CatalogProblem[];
  accepted: number;
  missing: number;
  untranslated: number;
}

/** What one shell command produced, on either of the two shells (Tauri shell). */
export interface ShellOutcome {
  message: string;
  dump: string | null;
  finished: boolean;
}

/** What one file operation produced (Tauri shell). */
export interface ToolOutcome {
  message: string;
  report: string | null;
}
