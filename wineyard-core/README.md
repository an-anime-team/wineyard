# wineyard-core

Standard abstractions provider.

## Archives

Archive formats support: `archives`.

| Format | Feature        |
| ------ | -------------- |
| All    | `archives-all` |
| `tar`  | `archives-tar` |
| `zip`  | `archives-zip` |
| `7z`   | `archives-7z`  |

## Network

Network support: `network`.

| Component  | Feature              |
| ---------- | -------------------- |
| All        | `network-all`        |
| Downloader | `network-downloader` |

## Hashes

Hashes support: `hashes`.

| Algorithm                 | Feature          |
| ------------------------- | ---------------- |
| All                       | `hashes-all`     |
| `seahash`                 | `hashes-seahash` |
| `crc32`                   | `hashes-crc32`   |
| `crc32c`                  | `hashes-crc32c`  |
| `siphash`                 | `hashes-siphash` |
| `xxh`                     | `hashes-xxh`     |
| `md5`                     | `hashes-md5`     |
| `sha1`                    | `hashes-sha1`    |
| `sha2`                    | `hashes-sha2`    |
| `sha3`, `keccak`, `shake` | `hashes-sha3`    |
| `blake2s`, `blake2b`      | `hashes-blake2`  |
| `blake3`                  | `hashes-blake3`  |
