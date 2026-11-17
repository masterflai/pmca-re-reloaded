# Sony WBI1 warm-boot RAM images (offline)

`pmca-console wbi` inspects **existing local files**, never contacts the camera,
and never changes a boot or flash setting. The decoder follows the documented
Sony layout in [ma1co/fwtool.py](https://github.com/ma1co/fwtool.py/tree/master/fwtool/sony)
and Sony's LZ77 packet variant. Unknown checksum fields are exposed verbatim;
their semantics have not been established, so they are **not** claimed verified.

```sh
pmca-console wbi inspect snapshot.wbi
pmca-console wbi inspect snapshot.wbi --json --out snapshot.report.json
pmca-console wbi unpack snapshot.wbi --out snapshot-sections
pmca-console wbi diff snapshot-before.wbi snapshot-after.wbi --json
```

The `unpack` directory must not already exist. It contains binary RAM segments
indexed by physical and virtual address. The `diff` operation compares decoded
segment SHA-256 hashes at physical addresses and reports added, removed, and
modified memory regions. A header change is shown separately in JSON.

WBI1 does not contain a known creation timestamp. `resume_vector` is an ARM
resume address, **not** the creation time, and `uname -a` reports a kernel build
date, not the snapshot time. `created_at` and `captured_at` are therefore null
in offline reports. Capture tools may use a host-side manifest with UTC time,
image SHA-256, file size, selected camera path, and optional operator-provided
provenance. Such metadata must never be silently injected into a firmware image.

**Privacy:** the extracted files are kernel/user process memory and may
contain passwords, Wi-Fi secrets, addresses, and identifiers. Keep them private;
do not check images, decoded sections or full reports into this repository.

**Scope:** no image generation, no WBI restore, no firmware writing, and no
provenance claims beyond what the data itself supports. The `WBI1` source image
must be obtained through a separately qualified camera readback flow.
