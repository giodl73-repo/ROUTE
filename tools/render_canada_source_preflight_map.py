#!/usr/bin/env python3
"""Render the Canada source-custody preflight overlay map.

This map is a preflight surface. It shows candidate source coverage for the
adapter fields, not an official Canadian network or validated ROUTE output.
"""

from __future__ import annotations

import csv
import html
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
NODES = ROOT / "data" / "international-portability-pilot-nodes.csv"
LINKS = ROOT / "data" / "international-portability-pilot-inference.csv"
COVERAGE = ROOT / "data" / "international-canada-adapter-coverage-preflight.csv"
OUT = ROOT / "maps" / "international" / "canada-source-custody-preflight.svg"
W = 1600
H = 980
MARGIN = 150

ROLE_STYLE = {
    "T1": {"color": "#1d4ed8", "width": 8},
    "T2": {"color": "#059669", "width": 6},
    "T3": {"color": "#d97706", "width": 5},
    "T4": {"color": "#7c3aed", "width": 5},
}

COVERAGE_STYLE = {
    "preflight-ready": "#22c55e",
    "source-candidate-found": "#f59e0b",
    "source-needed": "#ef4444",
    "held": "#94a3b8",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def text(x: float, y: float, value: str, size: int, fill: str = "#e5e7eb", anchor: str = "start") -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" font-family="Arial,sans-serif" '
        f'font-size="{size}" fill="{fill}" text-anchor="{anchor}">{html.escape(value)}</text>'
    )


def project(nodes: list[dict[str, str]]) -> dict[str, tuple[float, float]]:
    lons = [float(n["lon"]) for n in nodes]
    lats = [float(n["lat"]) for n in nodes]
    min_lon, max_lon = min(lons), max(lons)
    min_lat, max_lat = min(lats), max(lats)
    scale = min((W - 2 * MARGIN - 420) / (max_lon - min_lon), (H - 2 * MARGIN) / (max_lat - min_lat))
    x0 = 80
    y0 = (H - (max_lat - min_lat) * scale) / 2
    return {
        n["node_id"]: (x0 + (float(n["lon"]) - min_lon) * scale, y0 + (max_lat - float(n["lat"])) * scale)
        for n in nodes
    }


def main() -> None:
    nodes = [n for n in read_csv(NODES) if n["pilot_id"] == "canada-service-network"]
    links = [l for l in read_csv(LINKS) if l["pilot_id"] == "canada-service-network"]
    coverage = read_csv(COVERAGE)
    coords = project(nodes)

    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
        f'<rect width="{W}" height="{H}" fill="#0f172a"/>',
        '<rect x="28" y="28" width="1544" height="924" rx="20" fill="#111827" stroke="#334155" stroke-width="2"/>',
        text(60, 78, "Canada source-custody preflight overlay", 30, "#f8fafc"),
        text(60, 112, "Candidate source coverage for adapter fields; not an official network, SLA, construction, ROI, or validation claim.", 16, "#cbd5e1"),
    ]

    for link in links:
        style = ROLE_STYLE[link["inferred_role"]]
        x1, y1 = coords[link["from_node"]]
        x2, y2 = coords[link["to_node"]]
        svg.append(
            f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" '
            f'stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round" opacity="0.78">'
            f'<title>{html.escape(link["link_id"])} {html.escape(link["inferred_role"])} {html.escape(link["evidence_label"])}</title></line>'
        )

    for node in nodes:
        x, y = coords[node["node_id"]]
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="13" fill="#f8fafc" stroke="#0f172a" stroke-width="3"/>')
        svg.append(text(x + 18, y - 8, node["label"], 15, "#f8fafc"))
        svg.append(text(x + 18, y + 11, node["node_class"], 12, "#94a3b8"))

    px, py = 1070, 168
    svg.append(f'<rect x="{px-24}" y="{py-42}" width="470" height="560" rx="12" fill="#020617" stroke="#334155"/>')
    svg.append(text(px, py - 14, "Adapter source coverage", 18, "#f8fafc"))
    for idx, row in enumerate(coverage):
        y = py + 26 + idx * 58
        color = COVERAGE_STYLE.get(row["coverage_result"], "#94a3b8")
        svg.append(f'<circle cx="{px}" cy="{y-4}" r="8" fill="{color}"/>')
        svg.append(text(px + 18, y, row["adapter_field"], 14, "#e5e7eb"))
        svg.append(text(px + 18, y + 19, row["coverage_result"], 12, "#94a3b8"))
        svg.append(text(px + 18, y + 37, row["candidate_source_ids"], 11, "#cbd5e1"))

    lx, ly = 60, H - 180
    svg.append(f'<rect x="{lx-16}" y="{ly-28}" width="510" height="120" rx="12" fill="#020617" stroke="#334155"/>')
    svg.append(text(lx, ly - 6, "Line roles", 16, "#f8fafc"))
    for idx, role in enumerate(["T1", "T2", "T3", "T4"]):
        y = ly + 22 + idx * 22
        style = ROLE_STYLE[role]
        svg.append(f'<line x1="{lx}" y1="{y}" x2="{lx+54}" y2="{y}" stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round"/>')
        svg.append(text(lx + 66, y + 5, f"{role} candidate role", 13, "#cbd5e1"))

    svg.append(text(60, H - 42, "Held claims: official Canadian network, agency approval, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, external validation.", 14, "#fbbf24"))
    svg.append("</svg>")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(svg) + "\n", encoding="utf-8", newline="\n")
    print(f"rendered {OUT}")


if __name__ == "__main__":
    main()
