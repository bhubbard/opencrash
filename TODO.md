# OpenCrash (`opencrash`) — Engineering Roadmap & TODOs

This document tracks the phase-by-phase engineering checklist to build `opencrash` into a single-file, pure-Rust traffic collision, road hazard, and police report jurisdiction engine.

---

## Phase 1: Binary `.pack` Format & Columnar Spatial Index (`crates/opencrash-pack`)

- [ ] **1.1 Header & 64-Byte Aligned Section Table**
  - [ ] Finalize `PackHeader` (`OCRS` magic, version, `spatial_rtree_section`, `h3_agg_section`, `collisions_columnar_section`, `intersections_fst_section`, `police_agency_section`).
  - [ ] Add `memmap2` zero-copy wrapper and XXH3-64 checksum verification.
- [ ] **1.2 Bit-Packed Columnar Collision Storage**
  - [ ] Encode each collision record into a compact **24-byte** fixed struct:
    - `lat_e7: i32`, `lon_e7: i32` (`8 bytes`, `1.1 cm` precision)
    - `epoch_minutes: u32` (`4 bytes`)
    - `primary_rd_id: u32`, `secondary_rd_id: u32` (`8 bytes` into string table)
    - `severity_pcf_weather_lighting_flags: u16` (`2 bytes`)
    - `agency_ncic_id: u16` (`2 bytes`)
- [ ] **1.3 Hilbert-Sorted Spatial R-Tree & H3 Pre-Aggregates**
  - [ ] Sort all collision records along a Hilbert space-filling curve so spatial radius queries read contiguous OS memory pages.
  - [ ] Precompute H3 Resolution 9/10 hex-cell 5-year and 10-year crash aggregates + county percentile rankings.

---

## Phase 2: Public Crash & Road Network ETL (`opencrash build`)

- [ ] **2.1 California SWITRS / CCITRS & TIMS Ingestor**
  - [ ] Stream-parse California CHP SWITRS / CCITRS `Collisions`, `Parties`, and `Victims` tables (15+ years, ~7.5M crashes).
  - [ ] Map NCIC jurisdiction codes (`1942` LAPD, `9xxx` CHP Area Offices, Sheriff substations) to exact TCR request addresses, phone numbers, and online portal URLs.
- [ ] **2.2 NHTSA FARS National Fatal Crash Ingestor**
  - [ ] Ingest NHTSA Fatality Analysis Reporting System (FARS) national datasets across all 50 states.
- [ ] **2.3 State DOT Open Crash Feeds**
  - [ ] Add connectors for TX (CRIS), FL (FLHSMV/Signal4), NY (AIS), IL, AZ, NV.
- [ ] **2.4 OpenStreetMap Intersection & Highway Postmile Index**
  - [ ] Extract named street intersections (`Wilshire Blvd & Western Ave`) and state highway postmiles/interchanges from regional `.osm.pbf` extracts into an FST lookup index.

---

## Phase 3: Hazard Scoring, Intake Matching & TCR Agency Router (`crates/opencrash-core`)

- [ ] **3.1 Dangerous Intersection Hazard Brief Generator (`/hazard`)**
  - [ ] Given `(lat, lon)` or `"Primary St & Cross St, City"` and a radius (`50m..500m`), compute:
    - Total collisions, injury collisions, severe injury collisions, and fatalities (5-year and 10-year windows)
    - Top Primary Collision Factors (e.g., `Cal. Veh. Code § 21453(a)` red light, `§ 22350` unsafe speed, `§ 21801` left-turn yield)
    - Pedestrian / bicyclist / motorcycle involvement rates
    - County and statewide hazard percentile (`0..100`)
    - Markdown / JSON narrative snippet ready to drop into a Personal Injury **Demand Letter**.
- [ ] **3.2 Responsible Police Report (TCR) Agency Resolver (`/agency`)**
  - [ ] Determine whether an incident occurred on a state highway/freeway (CHP Area Office jurisdiction) vs. surface street (Municipal PD or County Sheriff contract city) and return exact report-ordering instructions.

---

## Phase 4: CLI, Interactive TUI & Batch Mode (`crates/opencrash-cli`)

- [ ] **4.1 Ratatui Terminal UI (`$ opencrash`)**
  - [ ] Type any intersection or freeway exit -> view 5-year crash heatmap, severity breakdown, peak collision hours, and reporting police department.
- [ ] **4.2 Batch CSV Lead & Demand Enricher (`$ opencrash batch`)**
  - [ ] Enrich a CSV of intake leads or litigation files with `5yr_injury_crashes`, `county_hazard_percentile`, `top_pcf_violation`, and `tcr_police_agency`.

---

## Phase 5: Stateless HTTP & MCP Server (`opencrash serve`)

- [ ] **5.1 HTTP Endpoints**
  - [ ] `GET /hazard?lat=...&lon=...&radius=150` — Return GeoJSON FeatureCollection + `IntersectionHazardBrief`.
  - [ ] `GET /intersection?q=Wilshire+and+Western,+Los+Angeles` — Geocode intersection and return hazard brief.
  - [ ] `GET /agency?lat=...&lon=...&highway=true` — Return responsible police agency for ordering the Traffic Collision Report.
- [ ] **5.2 Model Context Protocol (`--mcp`) Server**
  - [ ] Expose `get_intersection_hazard_brief`, `resolve_police_report_agency`, and `search_nearby_collisions` MCP tools for AI intake (`phone-agent`, `intake-engine`) and demand-letter drafting agents (`calljacob`, `firmpilot`).

---

## Phase 6: Open Benchmark Suite (`MESSY INTERSECTIONS`)

- [ ] **6.1 Curate `MESSY INTERSECTIONS` Dataset (`benches/messy-intersections/`)**
  - [ ] 1,000 colloquial verbal crash descriptions from real intake calls (`"on the 405 South right past Sunset"`, `"corner of Ventura and Sepulveda in Sherman Oaks"`).
- [ ] **6.2 Benchmark Harness**
  - [ ] Measure intersection resolution accuracy, radius aggregation latency (`p50 < 0.4 ms`), and memory efficiency.

---

## Phase 7: Astro Website & Live Map Demo (`site/`)

- [ ] **7.1 Build `opencrash` Astro Landing Page (OpenGeocode style)**
  - [ ] Interactive hero `<oc-intersection-morph>` + `<oc-crash-map>` zooming to any intersection and rendering its 5-year crash density and police jurisdiction live.
  - [ ] Benchmark tabs (`Accuracy`, `Throughput`, `Latency`, `Memory`) + 5-step terminal walkthrough.
