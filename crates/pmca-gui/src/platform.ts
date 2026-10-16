/** Facts about the host the window runs on. */

import { invoke } from "@tauri-apps/api/core";

// ---------------------------------------------------------------------------
// Host platform
// ---------------------------------------------------------------------------

type HostPlatform = "windows" | "macos" | "unix";

/**
 * Which operating system this window runs on, asked once at start-up.
 *
 * It defaults to `windows` because that is the only platform where skipping the
 * driver paragraph leaves someone stuck: on Linux and macOS the same text is
 * merely irrelevant, on Windows its absence is a dead end. If the question ever
 * fails to be answered, showing one paragraph too many is the cheaper mistake.
 */
export let hostPlatform: HostPlatform = "windows";

export async function detectHostPlatform(): Promise<void> {
  try {
    hostPlatform = await invoke<HostPlatform>("host_platform");
  } catch {
    // Keep the default; see above.
  }
}
