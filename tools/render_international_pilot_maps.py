#!/usr/bin/env python3
"""Render schematic international portability pilot maps.

The inputs are fixture rows, not official network data. The renderer exists to
test whether ROUTE's role inference and held-claim map pattern can be repeated
across jurisdictions.
"""

from __future__ import annotations

import csv
import html
from collections import defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
NODES = ROOT / "data" / "international-portability-pilot-nodes.csv"
LINKS = ROOT / "data" / "international-portability-pilot-links.csv"
OUT_DIR = ROOT / "maps" / "international"
INFERENCE = ROOT / "data" / "international-portability-pilot-inference.csv"
W = 1600
H = 980
MARGIN = 130

ROLE_STYLE = {
    "T1": {"label": "T1 candidate spine", "color": "#1d4ed8", "width": 8},
    "T2": {"label": "T2 candidate connector", "color": "#059669", "width": 6},
    "T3": {"label": "T3 candidate access", "color": "#d97706", "width": 5},
    "T4": {"label": "T4 candidate terminal/local", "color": "#7c3aed", "width": 5},
}

PILOT_TITLE = {
    "canada-service-network": "Canada service-network adapter fixture",
    "eu-rhine-alpine-region": "EU Rhine-Alpine style regional adapter fixture",
    "india-logistics-spine": "India logistics-spine adapter fixture",
    "japan-pacific-belt": "Japan Pacific Belt adapter fixture",
    "china-logistics-spine": "China logistics-spine adapter fixture",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def infer_role(row: dict[str, str]) -> str:
    road_class = row["road_class_hint"]
    need = row["need_class"]
    target = float(row["service_target_hours"])
    if road_class == "terminal_connector" or target <= 1.0 or "port" in need:
        return "T4"
    if road_class == "national_spine" and target <= 48.0:
        return "T1"
    if road_class == "regional_connector" and target <= 36.0:
        return "T2"
    return "T3"


def confidence_posture(row: dict[str, str], role: str) -> str:
    if row["source_status"] != "source-backed":
        return "heuristic-held"
    if role in {"T1", "T2"} and row["confidence"] == "high":
        return "source-backed-review-needed"
    return "source-backed-held"


def project(nodes: list[dict[str, str]]) -> dict[str, tuple[float, float]]:
    lons = [float(n["lon"]) for n in nodes]
    lats = [float(n["lat"]) for n in nodes]
    min_lon, max_lon = min(lons), max(lons)
    min_lat, max_lat = min(lats), max(lats)
    lon_span = max(max_lon - min_lon, 0.1)
    lat_span = max(max_lat - min_lat, 0.1)
    scale = min((W - 2 * MARGIN) / lon_span, (H - 2 * MARGIN) / lat_span)
    used_w = lon_span * scale
    used_h = lat_span * scale
    x0 = (W - used_w) / 2
    y0 = (H - used_h) / 2
    coords = {}
    for n in nodes:
        x = x0 + (float(n["lon"]) - min_lon) * scale
        y = y0 + (max_lat - float(n["lat"])) * scale
        coords[n["node_id"]] = (x, y)
    return coords


def svg_text(x: float, y: float, text: str, size: int, fill: str = "#e5e7eb", anchor: str = "start") -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" font-family="Arial,sans-serif" '
        f'font-size="{size}" fill="{fill}" text-anchor="{anchor}">{html.escape(text)}</text>'
    )


def render_pilot(pilot: str, nodes: list[dict[str, str]], links: list[dict[str, str]]) -> str:
    coords = project(nodes)
    node_by_id = {n["node_id"]: n for n in nodes}
    title = PILOT_TITLE.get(pilot, pilot)
    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
        f'<rect width="{W}" height="{H}" fill="#0f172a"/>',
        '<rect x="28" y="28" width="1544" height="924" rx="20" fill="#111827" stroke="#334155" stroke-width="2"/>',
        svg_text(60, 78, title, 30, "#f8fafc"),
        svg_text(60, 112, "Fixture output: inferred service roles from local adapter rows; not an official network or SLA proof.", 16, "#cbd5e1"),
    ]
    for link in links:
        role = link["inferred_role"]
        style = ROLE_STYLE[role]
        x1, y1 = coords[link["from_node"]]
        x2, y2 = coords[link["to_node"]]
        svg.append(
            f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" '
            f'stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round" opacity="0.9">'
            f'<title>{html.escape(link["link_id"])} {role}: {html.escape(link["need_class"])}</title></line>'
        )
    for node in nodes:
        x, y = coords[node["node_id"]]
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="13" fill="#f8fafc" stroke="#0f172a" stroke-width="3"/>')
        svg.append(svg_text(x + 18, y - 8, node["label"], 15, "#f8fafc"))
        svg.append(svg_text(x + 18, y + 11, node["node_class"], 12, "#94a3b8"))
    lx, ly = 60, H - 190
    svg.append(f'<rect x="{lx-16}" y="{ly-28}" width="510" height="148" rx="12" fill="#020617" stroke="#334155"/>')
    svg.append(svg_text(lx, ly - 6, "Legend", 16, "#f8fafc"))
    for idx, (role, style) in enumerate(ROLE_STYLE.items()):
        y = ly + 22 + idx * 26
        svg.append(f'<line x1="{lx}" y1="{y}" x2="{lx+54}" y2="{y}" stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round"/>')
        svg.append(svg_text(lx + 66, y + 5, f'{role}: {style["label"]}', 13, "#cbd5e1"))
    caption = "Held claims: official plan, construction, guaranteed SLA, ROI, eligibility, compliance, endorsement, external validation."
    svg.append(svg_text(60, H - 42, caption, 14, "#fbbf24"))
    svg.append("</svg>")
    return "\n".join(svg) + "\n"


def main() -> None:
    nodes = read_csv(NODES)
    links = read_csv(LINKS)
    nodes_by_pilot: dict[str, list[dict[str, str]]] = defaultdict(list)
    links_by_pilot: dict[str, list[dict[str, str]]] = defaultdict(list)
    for node in nodes:
        nodes_by_pilot[node["pilot_id"]].append(node)
    inference_rows: list[dict[str, str]] = []
    for link in links:
        row = dict(link)
        row["inferred_role"] = infer_role(row)
        row["evidence_label"] = confidence_posture(row, row["inferred_role"])
        links_by_pilot[row["pilot_id"]].append(row)
        inference_rows.append(row)

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for pilot, pilot_nodes in sorted(nodes_by_pilot.items()):
        svg = render_pilot(pilot, pilot_nodes, links_by_pilot[pilot])
        (OUT_DIR / f"{pilot}.svg").write_text(svg, encoding="utf-8", newline="\n")

    fieldnames = [
        "pilot_id",
        "link_id",
        "from_node",
        "to_node",
        "road_class_hint",
        "need_class",
        "service_target_hours",
        "confidence",
        "source_status",
        "inferred_role",
        "evidence_label",
        "boundary",
    ]
    with INFERENCE.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(inference_rows)

    print(f"rendered {len(nodes_by_pilot)} international pilot maps to {OUT_DIR}")
    print(f"wrote inference table to {INFERENCE}")


if __name__ == "__main__":
    main()
