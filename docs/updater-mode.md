# Updater entry and timeout recovery

The updater runs separately from the camera's normal application. A black
screen after confirming an initialization prompt can be expected: use the USB
mode to determine whether the transition succeeded. The program allows two
minutes of polling for confirmation and reboot; individual operating-system USB
queries can add time. Keep the battery and cable connected during the transition.

If that wait expires, inspect the camera's USB mode before restarting it. A
host timeout is not evidence that the camera froze. On a camera already detected
in updater mode, the existing updater flow continues on that connection rather
than requesting another mode switch. The payload is still guard-checked.

For the updater shell, rerun the command with the same payload selection. Model
autodetection is unavailable in updater mode, so supply `--model` or the original
`--fdat-file`. For example, when the locally available payload supports this model:

```sh
pmca-console updatershell --model DSC-HX50V --flash
```

Use the same `--fdat-dir` if the first command supplied one. This command sends
and executes a payload; it is not just a USB status query. For official firmware,
resume only with the same intended firmware file and its required confirmations.
These observations do not establish a general recovery method for interrupted
firmware writes. Do not remove power while a transfer or write is in progress.

## DSC-HX50V firmware 1.00: observed sequence

A Linux USB test on 2026-11-20 used the Python Sony-PMCA-RE reference at commit
`a82f5baaa8e9c3d9f28f94699e860fb2e48cc8e0`. These observations motivate the
wait/guidance change; they are **not a hardware validation of the Rust code**.
No firmware images, keys, serial numbers or camera dumps accompany this report.

| Step | Observed USB identity |
| --- | --- |
| Normal mass storage | `054c:06f0` |
| Confirm initialization on camera | Disconnect, screen goes black |
| Updater ready | `054c:03e2`, mass-storage class |
| Shell exit and updater completion | Normal `054c:06f0` returns |

Disconnect to updater enumeration took approximately 8.4 seconds. Mode-switch
acknowledgement to enumeration took approximately 26.4 seconds, including the
user's response time. A delayed confirmation caused a previous 30-second host
wait to expire. Resuming against the updater interface successfully ran the
exact selected package, whose firmware-data length was zero. Updater init and
driver hashes matched the original updater files; full logical partition reads
before/after matched the original backups.

The payload body mounts settings read/write, so zero firmware bytes do not mean
the whole operation is strictly read-only. No restoration write was tested.

A separate research test pre-created an empty `/setting/updater/mode` and synced
settings, then disconnected USB and removed the battery before booting. Updater
USB appeared directly without another OK prompt. Original updater startup removes
`mode*` when its `comp` marker exists; the mode file was absent afterwards. Native
protocol selection without that file instead returned to normal firmware after
battery removal. These findings are specific to this firmware, not instructions
to stage a persistent boot flag on other cameras, and not proof of recovery from
a damaged main image. A flag consumed during startup does not remain armed.
