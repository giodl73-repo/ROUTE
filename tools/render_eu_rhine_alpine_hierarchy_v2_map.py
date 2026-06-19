#!/usr/bin/env python3
"""Render EU Rhine-Alpine candidate hierarchy v2."""

from __future__ import annotations

import csv
import html
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INPUT = ROOT / "data" / "international-eu-rhine-alpine-candidate-hierarchy-v2.csv"
OUT = ROOT / "maps" / "international" / "eu-rhine-alpine-candidate-hierarchy-v2.svg"
W = 1700
H = 1040
MARGIN = 110

TIER_STYLE = {
    "T1": {"color": "#2563eb", "width": 10, "label": "T1 candidate cross-border spine"},
    "T2": {"color": "#059669", "width": 7, "label": "T2 candidate port/regional connector"},
    "T3": {"color": "#d97706", "width": 5, "label": "T3 candidate access feeder"},
}


def rows() -> list[dict[str, str]]:
    with INPUT.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def text(x: float, y: float, value: str, size: int, fill: str = "#e5e7eb", anchor: str = "start") -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" font-family="Arial,sans-serif" '
        f'font-size="{size}" fill="{fill}" text-anchor="{anchor}">{html.escape(value)}</text>'
    )


def collect_nodes(data: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    nodes: dict[str, dict[str, str]] = {}
    for row in data:
        nodes[row["from_node"]] = {"label": row["from_label"], "lon": row["from_lon"], "lat": row["from_lat"]}
        nodes[row["to_node"]] = {"label": row["to_label"], "lon": row["to_lon"], "lat": row["to_lat"]}
    return nodes


def project(nodes: dict[str, dict[str, str]]) -> dict[str, tuple[float, float]]:
    lons = [float(n["lon"]) for n in nodes.values()]
    lats = [float(n["lat"]) for n in nodes.values()]
    min_lon, max_lon = min(lons), max(lons)
    min_lat, max_lat = min(lats), max(lats)
    scale = min((W - 2 * MARGIN - 430) / (max_lon - min_lon), (H - 2 * MARGIN) / (max_lat - min_lat))
    used_w = (max_lon - min_lon) * scale
    used_h = (max_lat - min_lat) * scale
    x0 = 70 + ((W - 500) - used_w) / 2
    y0 = (H - used_h) / 2
    return {
        node_id: (x0 + (float(n["lon"]) - min_lon) * scale, y0 + (max_lat - float(n["lat"])) * scale)
        for node_id, n in nodes.items()
    }


def main() -> None:
    data = rows()
    nodes = collect_nodes(data)
    coords = project(nodes)
    svg = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
        f'<rect width="{W}" height="{H}" fill="#0f172a"/>',
        '<rect x="30" y="30" width="1640" height="980" rx="20" fill="#111827" stroke="#334155" stroke-width="2"/>',
        text(70, 82, "EU Rhine-Alpine candidate hierarchy v2", 32, "#f8fafc"),
        text(70, 116, "Iteration result: cross-border spine, port connectors, access feeders; official, SLA, construction, ROI, approval, and validation claims held.", 15, "#cbd5e1"),
    ]
    for row in data:
        style = TIER_STYLE[row["candidate_tier"]]
        x1, y1 = coords[row["from_node"]]
        x2, y2 = coords[row["to_node"]]
        dash = ' stroke-dasharray="10 8"' if row["candidate_tier"] == "T3" else ""
        svg.append(
            f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" '
            f'stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round" opacity="0.88"{dash}>'
            f'<title>{html.escape(row["hierarchy_id"])} {html.escape(row["candidate_tier"])}: {html.escape(row["need_class"])}</title></line>'
        )
    for node_id, node in nodes.items():
        x, y = coords[node_id]
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="12" fill="#f8fafc" stroke="#0f172a" stroke-width="3"/>')
        svg.append(text(x + 16, y - 8, node["label"], 14, "#f8fafc"))
        svg.append(text(x + 16, y + 10, node_id, 11, "#94a3b8"))
    panel_x, panel_y = 1190, 150
    svg.append(f'<rect x="{panel_x}" y="{panel_y}" width="400" height="590" rx="12" fill="#020617" stroke="#334155"/>')
    svg.append(text(panel_x + 24, panel_y + 38, "Iteration changes", 20, "#f8fafc"))
    changes = ["Basel-Lyon alternate T2 added", "Cologne-Strasbourg T3 added", "Alpine links kept with constraint risk", "governance/source claims held"]
    for idx, change in enumerate(changes):
        svg.append(text(panel_x + 34, panel_y + 78 + idx * 28, f"- {change}", 13, "#cbd5e1"))
    svg.append(text(panel_x + 24, panel_y + 235, "Tier counts", 18, "#f8fafc"))
    for idx, tier in enumerate(["T1", "T2", "T3"]):
        style = TIER_STYLE[tier]
        count = sum(1 for row in data if row["candidate_tier"] == tier)
        y = panel_y + 278 + idx * 52
        dash = ' stroke-dasharray="10 8"' if tier == "T3" else ""
        svg.append(f'<line x1="{panel_x+30}" y1="{y}" x2="{panel_x+95}" y2="{y}" stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round"{dash}/>')
        svg.append(text(panel_x + 112, y + 5, f"{tier}: {count} candidate links", 14, "#e5e7eb"))
    held = ["TEN-T/source-bound parsing", "national road graph source packs", "Alpine constraint evidence", "governance and SLA proof"]
    svg.append(text(panel_x + 24, panel_y + 465, "Still held", 18, "#f8fafc"))
    for idx, item in enumerate(held):
        svg.append(text(panel_x + 34, panel_y + 502 + idx * 26, f"- {item}", 13, "#cbd5e1"))
    svg.append(text(70, H - 54, "Held claims: official EU corridor designation, member-state approval, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, external validation.", 14, "#fbbf24"))
    svg.append("</svg>")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(svg) + "\n", encoding="utf-8", newline="\n")
    print(f"rendered {OUT}")


if __name__ == "__main__":
    main()
