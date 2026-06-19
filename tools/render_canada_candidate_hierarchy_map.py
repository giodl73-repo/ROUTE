#!/usr/bin/env python3
"""Render a Canada candidate T1/T2/T3 hierarchy map.

The output is a candidate hierarchy result from ROUTE-held rows. It is not an
official Canadian network, SLA, construction plan, ROI, approval, or validation.
"""

from __future__ import annotations

import csv
import html
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INPUT = ROOT / "data" / "international-canada-candidate-hierarchy.csv"
OUT = ROOT / "maps" / "international" / "canada-candidate-hierarchy.svg"
W = 1800
H = 1080
MARGIN = 130

TIER_STYLE = {
    "T1": {"color": "#2563eb", "width": 10, "label": "T1 candidate national spine"},
    "T2": {"color": "#059669", "width": 7, "label": "T2 candidate regional connector"},
    "T3": {"color": "#d97706", "width": 5, "label": "T3 candidate access feeder"},
}


def read_rows() -> list[dict[str, str]]:
    with INPUT.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def svg_text(x: float, y: float, value: str, size: int, fill: str = "#e5e7eb", anchor: str = "start") -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" font-family="Arial,sans-serif" '
        f'font-size="{size}" fill="{fill}" text-anchor="{anchor}">{html.escape(value)}</text>'
    )


def collect_nodes(rows: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    nodes: dict[str, dict[str, str]] = {}
    for row in rows:
        nodes[row["from_node"]] = {
            "node_id": row["from_node"],
            "label": row["from_label"],
            "lon": row["from_lon"],
            "lat": row["from_lat"],
        }
        nodes[row["to_node"]] = {
            "node_id": row["to_node"],
            "label": row["to_label"],
            "lon": row["to_lon"],
            "lat": row["to_lat"],
        }
    return nodes


def project(nodes: dict[str, dict[str, str]]) -> dict[str, tuple[float, float]]:
    lons = [float(n["lon"]) for n in nodes.values()]
    lats = [float(n["lat"]) for n in nodes.values()]
    min_lon, max_lon = min(lons), max(lons)
    min_lat, max_lat = min(lats), max(lats)
    scale = min((W - 2 * MARGIN - 420) / (max_lon - min_lon), (H - 2 * MARGIN) / (max_lat - min_lat))
    used_w = (max_lon - min_lon) * scale
    used_h = (max_lat - min_lat) * scale
    x0 = 70 + ((W - 490) - used_w) / 2
    y0 = (H - used_h) / 2
    return {
        node_id: (x0 + (float(n["lon"]) - min_lon) * scale, y0 + (max_lat - float(n["lat"])) * scale)
        for node_id, n in nodes.items()
    }


def main() -> None:
    rows = read_rows()
    nodes = collect_nodes(rows)
    coords = project(nodes)
    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
        f'<rect width="{W}" height="{H}" fill="#0f172a"/>',
        '<rect x="30" y="30" width="1740" height="1020" rx="20" fill="#111827" stroke="#334155" stroke-width="2"/>',
        svg_text(70, 82, "Canada candidate service hierarchy", 34, "#f8fafc"),
        svg_text(70, 118, "Result map: T1/T2/T3 candidates from ROUTE readiness rows; all official, SLA, construction, ROI, approval, and validation claims held.", 16, "#cbd5e1"),
    ]

    for row in rows:
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
        svg.append(svg_text(x + 16, y - 8, node["label"], 14, "#f8fafc"))
        svg.append(svg_text(x + 16, y + 10, node_id, 11, "#94a3b8"))

    panel_x, panel_y = 1290, 160
    svg.append(f'<rect x="{panel_x}" y="{panel_y}" width="420" height="520" rx="12" fill="#020617" stroke="#334155"/>')
    svg.append(svg_text(panel_x + 24, panel_y + 38, "Hierarchy result", 20, "#f8fafc"))
    tier_counts = {tier: sum(1 for row in rows if row["candidate_tier"] == tier) for tier in ["T1", "T2", "T3"]}
    for idx, tier in enumerate(["T1", "T2", "T3"]):
        style = TIER_STYLE[tier]
        y = panel_y + 82 + idx * 58
        dash = ' stroke-dasharray="10 8"' if tier == "T3" else ""
        svg.append(f'<line x1="{panel_x+28}" y1="{y}" x2="{panel_x+94}" y2="{y}" stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round"{dash}/>')
        svg.append(svg_text(panel_x + 112, y + 5, f'{tier}: {tier_counts[tier]} links', 15, "#e5e7eb"))
        svg.append(svg_text(panel_x + 112, y + 25, style["label"], 12, "#94a3b8"))

    bullets = [
        "Road graph: parse-ready candidate, not promoted",
        "Needs: parse-ready candidate, not promoted",
        "Nodes: source packs still required",
        "Targets: held planning assumptions",
        "Constraints: source packs still required",
    ]
    svg.append(svg_text(panel_x + 24, panel_y + 280, "Readiness posture", 18, "#f8fafc"))
    for idx, bullet in enumerate(bullets):
        svg.append(svg_text(panel_x + 36, panel_y + 315 + idx * 28, f"- {bullet}", 13, "#cbd5e1"))

    svg.append(svg_text(70, H - 54, "Held claims: official Canadian network, agency/provincial/port approval, guaranteed SLA, construction readiness, ROI, eligibility, compliance, endorsement, external validation.", 14, "#fbbf24"))
    svg.append("</svg>")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(svg) + "\n", encoding="utf-8", newline="\n")
    print(f"rendered {OUT}")


if __name__ == "__main__":
    main()
