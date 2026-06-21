#!/usr/bin/env python3
"""Render presentation-grade international market-system maps.

These maps are designed to show whether ROUTE can tell a richer country story:
spines, ports, inland hubs, lateral connectors, terminal feeders, and held proof
gaps. They remain candidate planning surfaces, not official networks.
"""

from __future__ import annotations

import csv
import html
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INPUT = ROOT / "data" / "international-market-system-map-v1.csv"
OUT_DIR = ROOT / "maps" / "international"
W = 2200
H = 1320
MAP_X = 70
MAP_Y = 150
MAP_W = 1420
MAP_H = 980

TIER_STYLE = {
    "T1": {"color": "#16a34a", "width": 12, "label": "T1 candidate trunk promise"},
    "T2": {"color": "#2563eb", "width": 8, "label": "T2 candidate market connector"},
    "T3": {"color": "#d97706", "width": 5, "label": "T3 candidate access / terminal feeder"},
}

CONFIG = {
    "china": {
        "title": "China Candidate Market-System Map",
        "subtitle": "2D service-market view: coastal export belts, central inland spines, Yangtze access, western connectors, and terminal feeders.",
        "output": "china-market-system-v1.svg",
        "outline": "M230 165 L520 118 L845 160 L1105 245 L1320 430 L1280 682 L1145 842 L876 915 L692 840 L486 870 L326 760 L190 548 L150 330 Z",
        "zones": [
            ("North / capital-port", 610, 280, 420, 210, "#1d4ed8"),
            ("Yangtze Delta", 910, 565, 370, 200, "#0f766e"),
            ("Central inland", 650, 650, 360, 310, "#16a34a"),
            ("Pearl River export", 780, 930, 360, 180, "#c2410c"),
            ("Western inland", 330, 700, 380, 300, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows China as multiple service markets, not one coast-to-coast line."),
            ("Client workshop", "Ask which promises matter first: export gateways, inland reliability, terminal access, or resilience."),
            ("Still held", "Official route roles, policy alignment, legal SLAs, construction, cost, ROI, eligibility, and validation."),
        ],
        "label_offsets": {
            "GZ": (16, -18),
            "SZX": (16, 18),
            "YTN": (18, 34),
            "TJN": (18, 20),
        },
    },
    "india": {
        "title": "India Candidate Market-System Map",
        "subtitle": "2D service-market view: northwest industrial spine, western ports, central sorting hubs, east coast loop, south market access, and northeast branch.",
        "output": "india-market-system-v1.svg",
        "outline": "M570 120 L760 205 L892 350 L860 550 L945 760 L815 1035 L650 1155 L540 1015 L470 810 L335 725 L280 555 L365 370 Z",
        "zones": [
            ("Northwest industrial", 390, 220, 360, 270, "#16a34a"),
            ("Western port spine", 360, 500, 310, 300, "#0f766e"),
            ("Central sorting", 570, 560, 300, 290, "#2563eb"),
            ("East coast loop", 760, 570, 360, 420, "#c2410c"),
            ("South market", 530, 850, 360, 270, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows India as a portfolio of market promises, not a single diagonal corridor."),
            ("Client workshop", "Rank industrial spine, port access, east-coast loop, central sorting, northeast access, and monsoon resilience."),
            ("Still held", "Official route roles, legal SLAs, construction, cost, ROI, eligibility, external validation, and public readiness."),
        ],
        "label_offsets": {
            "MUM": (16, -24),
            "MUN": (-120, 6),
            "PUN": (20, 30),
            "CHN": (22, 26),
            "ENN": (22, -24),
            "HYD": (22, 20),
            "DEL": (20, -16),
            "JAI": (20, 18),
        },
    },
}


def esc(value: str) -> str:
    return html.escape(value, quote=True)


def text(x: float, y: float, value: str, size: int, fill: str = "#172033", weight: int = 400, anchor: str = "start") -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" font-family="Segoe UI, Arial, sans-serif" '
        f'font-size="{size}" font-weight="{weight}" fill="{fill}" text-anchor="{anchor}">{esc(value)}</text>'
    )


def multiline(x: float, y: float, value: str, size: int, width: int, fill: str = "#4b5563", line_gap: int = 1) -> list[str]:
    words = value.split()
    lines: list[str] = []
    current: list[str] = []
    max_chars = max(24, int(width / (size * 0.55)))
    for word in words:
        if current and len(" ".join(current + [word])) > max_chars:
            lines.append(" ".join(current))
            current = [word]
        else:
            current.append(word)
    if current:
        lines.append(" ".join(current))
    return [text(x, y + idx * (size + 7 + line_gap), line, size, fill) for idx, line in enumerate(lines)]


def read_rows() -> dict[str, list[dict[str, str]]]:
    by_country: dict[str, list[dict[str, str]]] = defaultdict(list)
    with INPUT.open(newline="", encoding="utf-8") as f:
        for row in csv.DictReader(f):
            by_country[row["country"]].append(row)
    return by_country


def collect_nodes(rows: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    nodes: dict[str, dict[str, str]] = {}
    for row in rows:
        nodes[row["from_node"]] = {"label": row["from_label"], "lon": row["from_lon"], "lat": row["from_lat"]}
        nodes[row["to_node"]] = {"label": row["to_label"], "lon": row["to_lon"], "lat": row["to_lat"]}
    return nodes


def project(nodes: dict[str, dict[str, str]]) -> dict[str, tuple[float, float]]:
    lons = [float(n["lon"]) for n in nodes.values()]
    lats = [float(n["lat"]) for n in nodes.values()]
    min_lon, max_lon = min(lons), max(lons)
    min_lat, max_lat = min(lats), max(lats)
    scale = min(MAP_W / (max_lon - min_lon), MAP_H / (max_lat - min_lat)) * 0.9
    used_w = (max_lon - min_lon) * scale
    used_h = (max_lat - min_lat) * scale
    x0 = MAP_X + (MAP_W - used_w) / 2
    y0 = MAP_Y + (MAP_H - used_h) / 2
    return {
        node_id: (x0 + (float(n["lon"]) - min_lon) * scale, y0 + (max_lat - float(n["lat"])) * scale)
        for node_id, n in nodes.items()
    }


def path_between(x1: float, y1: float, x2: float, y2: float, bend: float) -> str:
    mx = (x1 + x2) / 2
    my = (y1 + y2) / 2
    dx = x2 - x1
    dy = y2 - y1
    length = max((dx * dx + dy * dy) ** 0.5, 1)
    nx = -dy / length
    ny = dx / length
    cx = mx + nx * bend
    cy = my + ny * bend
    return f"M{x1:.1f} {y1:.1f} Q{cx:.1f} {cy:.1f} {x2:.1f} {y2:.1f}"


def render(country: str, rows: list[dict[str, str]]) -> Path:
    cfg = CONFIG[country]
    nodes = collect_nodes(rows)
    coords = project(nodes)
    counts = Counter(row["tier"] for row in rows)
    layers = Counter(row["market_layer"] for row in rows)
    source_needed = sum(1 for row in rows if row["evidence_label"] == "source-needed")
    out = OUT_DIR / cfg["output"]

    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
        f'<rect width="{W}" height="{H}" fill="#f8fafc"/>',
        f'<rect x="28" y="28" width="{W-56}" height="{H-56}" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>',
        text(70, 86, cfg["title"], 38, "#111827", 700),
        *multiline(70, 124, cfg["subtitle"], 17, 1300, "#475569"),
        f'<path d="{cfg["outline"]}" fill="#eef6f0" stroke="#94a3b8" stroke-width="2" opacity="0.92"/>',
    ]

    for label, x, y, w, h, color in cfg["zones"]:
        svg.append(f'<ellipse cx="{x}" cy="{y}" rx="{w/2}" ry="{h/2}" fill="{color}" opacity="0.08" stroke="{color}" stroke-width="2" stroke-dasharray="8 8"/>')
        svg.append(text(x - w / 2 + 18, max(y - h / 2 + 30, 175), label, 17, color, 700))

    # Draw lower tiers first so trunk promises remain visually dominant.
    ordered = sorted(rows, key=lambda row: {"T3": 0, "T2": 1, "T1": 2}[row["tier"]])
    for idx, row in enumerate(ordered):
        x1, y1 = coords[row["from_node"]]
        x2, y2 = coords[row["to_node"]]
        style = TIER_STYLE[row["tier"]]
        bend = ((idx % 5) - 2) * 18
        dash = ' stroke-dasharray="10 9"' if row["tier"] == "T3" else ""
        svg.append(
            f'<path d="{path_between(x1, y1, x2, y2, bend)}" fill="none" stroke="{style["color"]}" '
            f'stroke-width="{style["width"]}" stroke-linecap="round" opacity="0.78"{dash}>'
            f'<title>{esc(row["tier"])} {esc(row["from_label"])} to {esc(row["to_label"])}: {esc(row["service_promise"])}</title></path>'
        )

    important = {row["from_node"] for row in rows if row["tier"] == "T1"} | {row["to_node"] for row in rows if row["tier"] == "T1"}
    for node_id, node in nodes.items():
        x, y = coords[node_id]
        is_major = node_id in important
        r = 15 if is_major else 10
        label_dx, label_dy = cfg.get("label_offsets", {}).get(node_id, (16, -8))
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="#ffffff" stroke="#0f172a" stroke-width="{4 if is_major else 2}"/>')
        svg.append(text(x + label_dx, y + label_dy, node["label"], 15 if is_major else 12, "#111827", 700 if is_major else 500))
        svg.append(text(x + label_dx, y + label_dy + 18, node_id, 11, "#64748b"))

    panel_x = 1540
    svg.append(f'<rect x="{panel_x}" y="150" width="580" height="980" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>')
    svg.append(text(panel_x + 34, 205, "What this module demonstrates", 25, "#111827", 700))
    svg.append(text(panel_x + 34, 250, "Market layers", 17, "#334155", 700))
    for idx, (layer, count) in enumerate(layers.most_common(8)):
        svg.extend(multiline(panel_x + 52, 284 + idx * 42, f"{layer}: {count} candidate promises", 14, 480, "#475569"))

    svg.append(text(panel_x + 34, 650, "Tier mix", 17, "#334155", 700))
    for idx, tier in enumerate(["T1", "T2", "T3"]):
        style = TIER_STYLE[tier]
        y = 690 + idx * 54
        dash = ' stroke-dasharray="10 9"' if tier == "T3" else ""
        svg.append(f'<line x1="{panel_x+52}" y1="{y}" x2="{panel_x+130}" y2="{y}" stroke="{style["color"]}" stroke-width="{style["width"]}" stroke-linecap="round"{dash}/>')
        svg.append(text(panel_x + 152, y + 5, f'{tier}: {counts[tier]} links - {style["label"]}', 15, "#334155"))

    svg.append(text(panel_x + 34, 880, "Proof posture", 17, "#334155", 700))
    proof = f"{len(nodes)} nodes, {len(rows)} candidate links, {source_needed} source-needed access links. Review surface only."
    svg.extend(multiline(panel_x + 52, 914, proof, 15, 470, "#475569"))
    for idx, (title, body) in enumerate(cfg["callouts"]):
        y = 1002 + idx * 70
        svg.append(text(panel_x + 52, y, title, 15, "#111827", 700))
        svg.extend(multiline(panel_x + 52, y + 24, body, 13, 470, "#475569"))

    svg.append(text(70, H - 72, "Held claims: official network, legal SLA, construction readiness, costs, numeric ROI, funding eligibility, compliance, endorsement, external validation, and public readiness.", 16, "#92400e", 700))
    svg.append(text(70, H - 42, "Use as a client discovery surface: replace heuristic rows with country-specific source rows before making stronger claims.", 14, "#64748b"))
    svg.append("</svg>")

    out.write_text("\n".join(svg) + "\n", encoding="utf-8", newline="\n")
    return out


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    by_country = read_rows()
    for country in ["china", "india"]:
        out = render(country, by_country[country])
        print(f"rendered {out}")


if __name__ == "__main__":
    main()
