# OpenCrash (`opencrash`)

[![Rust 2021](https://img.shields.io/badge/rust-2021_edition-000000?style=flat-square&logo=rust)](https://github.com/bhubbard/opencrash)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)

**Free, open-source, self-hosted traffic collision, dangerous intersection hazard, and police report jurisdiction engine in pure Rust — served from one memory-mapped `.pack` file.**

Built on the [OpenGeocode](https://opengeocode.com/) "One Binary + One Pack File" architecture, `opencrash` compiles millions of historical traffic collisions (California SWITRS/TIMS, NHTSA FARS, state DOT crash feeds), OpenStreetMap intersection geometries, and law enforcement beat boundaries into a single memory-mapped `.pack` file (`~1.4 GB` for 15+ years of California collisions; `~8.5 GB` for the entire US).

---

## Why OpenCrash?

- **Instant PI Intake Triage (`/match` & `/agency`):** When an injured caller says *"I got rear-ended on I-10 near Robertson Tuesday at 5pm"*, `opencrash` resolves the exact highway postmile/intersection, identifies the responsible **Traffic Collision Report (TCR) agency** (e.g., *CHP West Los Angeles Area 565* vs. *LAPD West Traffic Division*), and locates the nearest Level I/II Trauma Centers in `<0.5 ms`.
- **Automated Demand Letter Hazard Briefs (`/hazard`):** Generate citation-backed **"Dangerous Intersection Briefs"** (e.g., *"47 injury collisions and 6 severe/fatal collisions within 150m in the trailing 5 years — 98th percentile in Los Angeles County"*) to substantiate road hazard and notice in settlement demand packets.
- **One Binary, One File (`mmap`):** Columnar bit-packed collision records + flat R-tree & H3 spatial index run in `<150 MB` RAM with zero PostgreSQL/PostGIS overhead.

---

## Disk & Memory Footprint

| Region Pack | Historical Collisions | Named Intersections | Core `.pack` | + Party/Victim Sub-Records | Resident RAM (`mmap`) |
| :--- | ---: | ---: | ---: | ---: | ---: |
| **`us-california`** | ~7.5M *(15+ yrs SWITRS)* | ~1.1M | **~1.4 GB** | **~2.6 GB** | `~90 MB` |
| **`us-all`** *(NHTSA FARS + State DOTs)* | ~65M | ~9.8M | **~8.5 GB** | **~16.0 GB** | `~150 MB` |

---

## Quickstart

```bash
# 1. Install the binary
cargo install --path crates/opencrash-cli

# 2. Build the California or National Pack
opencrash build --region us/california

# 3. Query 5-year intersection hazard stats & reporting police agency
opencrash hazard --lat 34.0617 --lon -118.3090 --radius 150

# 4. Serve the HTTP + MCP API
opencrash serve us-california.pack --bind 127.0.0.1:8083 --mcp

# 5. Enrich a CSV of intake leads with crash density & TCR agency
opencrash batch intake_leads.csv
```

---

## Workspace Architecture

| Crate | Description |
| :--- | :--- |
| [`crates/opencrash-pack`](./crates/opencrash-pack) | Binary `.pack` file specification (`OCRS` magic), spatial R-tree/H3 section table, and `mmap` reader/writer. |
| [`crates/opencrash-core`](./crates/opencrash-core) | KABCO/SWITRS collision severity models, spatial radius aggregation, intersection hazard scoring, and police agency beat routing. |
| [`crates/opencrash-cli`](./crates/opencrash-cli) | `opencrash` CLI binary (`build`, `hazard`, `serve`, `batch`, `info`). |

---

## Roadmap & Engineering Checklist

See [**TODO.md**](./TODO.md) for the complete phase-by-phase implementation plan.

## License

MIT License — see [LICENSE](./LICENSE).
