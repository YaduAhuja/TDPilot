# TDPilot
TDPilot, an open-source Linux Ryzen CPU power and performance tuning utility

## Configuration

TDPilot reads `config.json` from the current working directory. The checked-in
defaults are:

```json
{
  "profileDirectory": "./profiles",
  "ryzenAdjExecPath": "./external/ryzenadj-manylinux_2_28-x86_64/ryzenadj"
}
```

If `config.json` is missing, TDPilot creates it with these defaults. Both paths
are resolved relative to the current working directory unless they are
absolute.

The `profiles` directory includes these presets (all power values are in mW):

| Profile | Sustained (STAPM and slow) | Boost (fast) |
|---|---:|---:|
| `power-saving` | 6,000 (6 W) | 12,000 (12 W) |
| `balanced` | 12,000 (12 W) | 18,000 (18 W) |
| `performance` | 18,000 (18 W) | 24,000 (24 W) |

## Commands

```text
tdpilot profile list
tdpilot profile show balanced
tdpilot profile apply balanced
tdpilot profile save custom --json '{"stapmLimit":10000,"fastLimit":16000}'
tdpilot profile save custom --json '{"stapmLimit":12000}' --force
tdpilot profile delete custom --yes
tdpilot status
```

Applying a profile and querying status run the configured RyzenAdj executable.
RyzenAdj may require elevated privileges on your system. Profile names are file
names (not paths); saving over an existing profile requires `--force`.
