#!/usr/bin/env python3
"""Render presentation-grade country and state market-system maps.

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
OUT_DIR = ROOT / "maps"
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
        "output": "international/china-market-system-v1.svg",
        "outline_lonlat": [(101.5, 41.8), (108.5, 43.0), (117.0, 42.2), (123.8, 39.0), (125.2, 33.8), (123.2, 28.2), (119.0, 23.4), (113.0, 21.6), (106.0, 24.0), (101.8, 29.4), (102.8, 36.0)],
        "zones_lonlat": [
            ("North / capital-port", 117.0, 38.0, 5.8, 3.0, "#1d4ed8"),
            ("Yangtze Delta", 120.0, 31.0, 4.6, 2.3, "#0f766e"),
            ("Central inland", 113.0, 31.0, 5.4, 4.0, "#16a34a"),
            ("Pearl River export", 114.0, 23.4, 4.6, 2.1, "#c2410c"),
            ("Western inland", 105.5, 30.0, 4.8, 4.1, "#7c3aed"),
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
        "output": "international/india-market-system-v1.svg",
        "outline_lonlat": [(74.5, 29.6), (80.8, 29.0), (86.6, 26.4), (91.0, 24.0), (87.2, 20.8), (83.4, 16.2), (80.2, 12.2), (78.0, 8.4), (74.8, 12.0), (72.0, 18.4), (72.5, 23.3)],
        "zones_lonlat": [
            ("Northwest industrial", 75.4, 26.0, 4.3, 3.8, "#16a34a"),
            ("Western port spine", 72.9, 20.5, 3.0, 4.0, "#0f766e"),
            ("Central sorting", 78.5, 20.8, 3.5, 4.2, "#2563eb"),
            ("East coast loop", 84.5, 18.8, 4.6, 6.0, "#c2410c"),
            ("South market", 77.4, 13.0, 4.0, 3.4, "#7c3aed"),
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
    "texas": {
        "title": "Texas Candidate Market-System Map",
        "subtitle": "2D state service-market view: Triangle reliability, Central Texas spine, Gulf ports, border/Rio Grande Valley, Permian, South Plains, Panhandle, and East Texas.",
        "output": "state/texas-market-system-v1.svg",
        "outline_lonlat": [(-106.8, 32.0), (-103.0, 36.6), (-100.0, 36.6), (-99.0, 34.2), (-94.1, 33.0), (-93.6, 29.8), (-96.5, 27.0), (-97.4, 25.8), (-99.7, 27.3), (-102.5, 29.1), (-104.8, 30.3)],
        "zones_lonlat": [
            ("Texas Triangle", -97.0, 30.9, 3.0, 2.2, "#16a34a"),
            ("Central Texas spine", -97.4, 31.3, 1.1, 1.4, "#0f766e"),
            ("Gulf ports", -96.4, 28.6, 2.5, 1.9, "#0891b2"),
            ("Border / RGV", -98.6, 26.7, 2.6, 1.6, "#c2410c"),
            ("Permian energy", -102.2, 32.0, 2.0, 1.5, "#7c3aed"),
            ("South Plains / Panhandle", -101.8, 34.3, 2.7, 2.2, "#2563eb"),
            ("East Texas gateway", -95.1, 31.4, 1.7, 2.0, "#65a30d"),
        ],
        "callouts": [
            ("Why this sells", "Shows Texas as a portfolio of metro, port, border, energy, plains, Gulf, East Texas, and evacuation promises."),
            ("Client workshop", "Rank Triangle reliability, central spine, border/RGV access, Gulf ports, Permian/South Plains, East Texas, evacuation, and rural coverage."),
            ("Still held", "Official TxDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "DFW": (18, -18),
            "WAC": (18, 20),
            "AUS": (18, -18),
            "SAT": (18, 24),
            "HOU": (18, 28),
            "PTX": (-112, 6),
            "BEA": (18, -18),
            "VCT": (-82, 6),
            "COR": (18, 24),
            "BRW": (18, 42),
            "MFE": (-104, -14),
            "LRD": (-92, 8),
            "ELP": (18, 20),
            "LBB": (18, -18),
            "TYL": (18, -18),
            "EGP": (-92, 8),
        },
    },
    "iowa": {
        "title": "Iowa Candidate Market-System Map",
        "subtitle": "2D state service-market view: I-80 spine, Des Moines hub, eastern production corridor, rural access, river gateways, and north-south resilience.",
        "output": "state/iowa-market-system-v1.svg",
        "outline_lonlat": [(-96.8, 43.6), (-90.0, 43.6), (-90.1, 42.5), (-89.9, 41.5), (-90.2, 40.5), (-91.6, 40.3), (-93.6, 40.4), (-95.9, 40.6), (-96.6, 41.5), (-96.7, 42.5)],
        "zones_lonlat": [
            ("I-80 service spine", -93.2, 41.6, 3.0, 0.55, "#16a34a"),
            ("Central Iowa hub", -93.6, 41.8, 1.2, 0.8, "#2563eb"),
            ("Eastern production", -91.5, 42.1, 1.4, 0.9, "#0f766e"),
            ("Northern rural access", -93.3, 43.0, 3.0, 0.6, "#7c3aed"),
            ("River gateways", -95.9, 41.7, 1.0, 1.1, "#c2410c"),
        ],
        "callouts": [
            ("Why this sells", "Shows a state DOT how ROUTE turns a highway map into service promises and proof asks."),
            ("Client workshop", "Rank spine reliability, rural access, terminal links, university/hospital access, flood/snow resilience, and river gateways."),
            ("Still held", "Official state plan, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "DSM": (18, -20),
            "AMES": (18, -18),
            "CB": (-120, 8),
            "QC": (20, 12),
            "IC": (18, 20),
            "CR": (18, -18),
        },
    },
    "florida": {
        "title": "Florida Candidate Market-System Map",
        "subtitle": "2D state service-market view: north-south peninsula spine, I-4/Central Florida, Gulf Coast, South Florida urban belt, Panhandle, ports, and Keys resilience.",
        "output": "state/florida-market-system-v1.svg",
        "outline_lonlat": [(-87.6, 30.8), (-81.2, 30.8), (-80.0, 29.2), (-80.1, 26.5), (-80.2, 25.2), (-81.8, 24.5), (-82.9, 26.3), (-83.1, 28.2), (-84.4, 29.8)],
        "zones_lonlat": [
            ("Peninsula spine", -81.0, 27.4, 0.9, 2.6, "#16a34a"),
            ("I-4 / Central Florida", -82.0, 28.1, 1.2, 0.8, "#2563eb"),
            ("Gulf Coast", -82.4, 27.1, 1.0, 1.4, "#0f766e"),
            ("South Florida urban belt", -80.2, 26.0, 0.7, 1.0, "#c2410c"),
            ("Panhandle", -85.2, 30.2, 2.4, 0.6, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Florida as peninsula, Gulf, South Florida, Panhandle, port, tourism, and resilience promises."),
            ("Client workshop", "Rank I-4, peninsula reliability, South Florida access, Gulf Coast growth, Panhandle continuity, port access, evacuation, and Keys resilience."),
            ("Still held", "Official FDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "JAX": (18, -18),
            "ORL": (18, -18),
            "TPA": (-88, 8),
            "SRQ": (-92, 10),
            "FMY": (18, 20),
            "MIA": (18, 38),
            "PB": (18, -28),
            "PMI": (-108, -12),
            "PCV": (18, 20),
            "EYW": (18, 20),
        },
    },
    "california": {
        "title": "California Candidate Market-System Map",
        "subtitle": "2D state service-market view: Bay Area, capital access, Central Valley logistics, Southern California, ports, north coast, and coastal connectors.",
        "output": "state/california-market-system-v1.svg",
        "outline_lonlat": [(-124.4, 41.8), (-120.0, 42.0), (-114.2, 34.9), (-117.1, 32.4), (-119.9, 34.4), (-122.6, 37.7), (-124.2, 40.4)],
        "zones_lonlat": [
            ("Bay Area", -122.1, 37.6, 0.9, 0.7, "#0f766e"),
            ("Capital / north access", -121.7, 39.1, 1.3, 1.6, "#2563eb"),
            ("Central Valley", -120.0, 36.6, 1.2, 2.3, "#16a34a"),
            ("Southern California", -117.8, 33.8, 1.5, 1.2, "#c2410c"),
            ("North coast resilience", -123.2, 40.4, 1.1, 0.9, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows California as megaregion, port, valley, coastal, and resilience promises."),
            ("Client workshop", "Rank Bay Area, Central Valley logistics, Southern California, ports, north access, coastal connectors, wildfire and seismic resilience."),
            ("Still held", "Official Caltrans priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "SF": (-112, -22),
            "OAK": (18, 28),
            "SJ": (18, 46),
            "SAC": (18, -18),
            "STK": (18, 18),
            "LA": (18, -28),
            "LGB": (-108, 10),
            "IE": (28, 28),
            "SD": (18, 24),
            "RDD": (18, -18),
            "EKA": (18, -18),
        },
    },
    "new-york": {
        "title": "New York Candidate Market-System Map",
        "subtitle": "2D state service-market view: downstate gateway, Hudson spine, Thruway/upstate cities, Southern Tier, North Country, ports, and cross-Hudson access.",
        "output": "state/new-york-market-system-v1.svg",
        "outline_lonlat": [(-79.8, 42.1), (-78.9, 43.5), (-76.0, 44.1), (-73.2, 45.0), (-71.8, 42.8), (-73.9, 40.5), (-76.6, 41.8)],
        "zones_lonlat": [
            ("Downstate gateway", -73.9, 40.9, 0.8, 0.6, "#c2410c"),
            ("Hudson spine", -73.8, 42.0, 0.6, 1.4, "#16a34a"),
            ("Thruway / upstate spine", -76.4, 43.0, 2.6, 0.7, "#2563eb"),
            ("Southern Tier", -76.8, 42.0, 2.0, 0.5, "#0f766e"),
            ("North Country", -74.8, 44.2, 1.6, 0.6, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows New York as downstate gateway, upstate spine, Southern Tier, and North Country promises."),
            ("Client workshop", "Rank downstate access, port/cross-Hudson, Thruway reliability, western gateway, Southern Tier, and northern access."),
            ("Still held", "Official NYSDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "NYC": (18, 30),
            "NWK": (-112, -8),
            "LI": (18, -18),
            "ALB": (18, -18),
            "BUF": (-80, 10),
            "PLB": (18, -18),
            "WAT": (18, -18),
        },
    },
    "illinois": {
        "title": "Illinois Candidate Market-System Map",
        "subtitle": "2D state service-market view: Chicago gateway, northern Illinois, central spine, university markets, Mississippi/Metro East, southern Illinois, and river confluence.",
        "output": "state/illinois-market-system-v1.svg",
        "outline_lonlat": [(-91.6, 42.5), (-87.4, 42.5), (-87.5, 39.0), (-89.1, 37.0), (-91.5, 39.5)],
        "zones_lonlat": [
            ("Chicago gateway", -87.8, 41.7, 1.0, 0.8, "#c2410c"),
            ("Central Illinois spine", -89.2, 40.2, 1.4, 1.3, "#16a34a"),
            ("University markets", -88.7, 40.2, 1.1, 0.8, "#2563eb"),
            ("Mississippi / Metro East", -90.3, 39.0, 1.0, 1.2, "#0f766e"),
            ("Southern Illinois", -89.2, 37.6, 1.0, 0.8, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Illinois as Chicago gateway, central-state, university, Mississippi, and southern river promises."),
            ("Client workshop", "Rank Chicago access, central Illinois spine, university markets, Metro East, western coverage, and southern river access."),
            ("Still held", "Official IDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "CHI": (18, -18),
            "RFD": (-88, -8),
            "STL": (18, 24),
            "CAI": (18, 20),
            "QCY": (-78, 8),
        },
    },
    "georgia": {
        "title": "Georgia Candidate Market-System Map",
        "subtitle": "2D state service-market view: Atlanta hub, central Georgia, Savannah port spine, Augusta/east Georgia, Columbus/west Georgia, south Georgia, and coastal resilience.",
        "output": "state/georgia-market-system-v1.svg",
        "outline_lonlat": [(-85.6, 35.0), (-80.7, 34.9), (-80.8, 32.0), (-81.5, 30.7), (-84.9, 30.6), (-85.5, 32.7)],
        "zones_lonlat": [
            ("Atlanta hub", -84.4, 33.7, 1.0, 0.8, "#16a34a"),
            ("Port spine", -82.3, 32.4, 1.7, 1.0, "#0f766e"),
            ("East Georgia", -82.6, 33.5, 1.4, 0.8, "#2563eb"),
            ("West Georgia", -84.8, 32.7, 1.1, 1.0, "#c2410c"),
            ("South Georgia", -83.4, 31.2, 1.5, 0.8, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Georgia as Atlanta hub, port spine, east/west Georgia, south Georgia, coastal and gateway promises."),
            ("Client workshop", "Rank Atlanta reliability, Savannah port access, Augusta/Columbus connectors, South Georgia, coastal resilience, and terminal access."),
            ("Still held", "Official GDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "ATL": (18, -18),
            "MAC": (18, 18),
            "SAV": (18, -18),
            "PTG": (-112, 6),
            "BRU": (18, 20),
            "CHA": (18, -18),
            "VLD": (18, 24),
        },
    },
    "pennsylvania": {
        "title": "Pennsylvania Candidate Market-System Map",
        "subtitle": "2D state service-market view: Philadelphia gateway, Harrisburg hub, Pittsburgh/Allegheny access, Northeast PA, Erie/Lake Erie, Lehigh Valley, and port feeders.",
        "output": "state/pennsylvania-market-system-v1.svg",
        "outline_lonlat": [(-80.6, 42.3), (-74.7, 42.0), (-74.7, 40.1), (-75.1, 39.7), (-77.0, 39.7), (-80.5, 39.7)],
        "zones_lonlat": [
            ("Philadelphia gateway", -75.2, 40.0, 0.8, 0.5, "#c2410c"),
            ("Harrisburg hub", -76.9, 40.3, 0.9, 0.6, "#16a34a"),
            ("Allegheny / Pittsburgh", -80.0, 40.4, 0.9, 0.7, "#2563eb"),
            ("Northeast / Lehigh", -75.5, 41.0, 1.0, 0.7, "#0f766e"),
            ("Lake Erie access", -80.0, 42.0, 0.8, 0.4, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Pennsylvania as two major metro anchors plus central hub, port, mountain, Northeast, and Lake Erie promises."),
            ("Client workshop", "Rank Philadelphia access, Pittsburgh reliability, Harrisburg sorting, Lehigh/Northeast PA, Erie, port feeders, and mountain resilience."),
            ("Still held", "Official PennDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "PHL": (18, 24),
            "PTP": (-100, -10),
            "PIT": (-90, 8),
            "ERI": (18, -18),
            "OGH": (-98, 8),
            "SCR": (18, -18),
            "ALN": (18, 22),
            "ALT": (18, -18),
        },
    },
    "ohio": {
        "title": "Ohio Candidate Market-System Map",
        "subtitle": "2D state service-market view: 3C urban spine, Lake Erie belt, Dayton logistics, Toledo/Detroit gateway, Appalachian access, river gateways, and manufacturing connectors.",
        "output": "state/ohio-market-system-v1.svg",
        "outline_lonlat": [(-84.9, 41.7), (-80.5, 41.9), (-80.5, 40.6), (-81.7, 39.0), (-82.6, 38.5), (-84.8, 39.1)],
        "zones_lonlat": [
            ("3C urban spine", -82.4, 40.2, 1.4, 1.1, "#16a34a"),
            ("Lake Erie belt", -82.2, 41.4, 1.7, 0.5, "#2563eb"),
            ("Southwest logistics", -84.1, 39.6, 0.9, 0.7, "#c2410c"),
            ("Appalachian access", -81.4, 39.6, 1.0, 0.9, "#7c3aed"),
            ("River gateways", -83.1, 39.0, 1.5, 0.5, "#0f766e"),
        ],
        "callouts": [
            ("Why this sells", "Shows Ohio as a dense statewide service mesh: 3C, Lake Erie, Dayton, Toledo/Detroit, river, and Appalachian access."),
            ("Client workshop", "Rank 3C reliability, Lake Erie manufacturing, Dayton logistics, Toledo/Detroit gateway, river gateways, and Appalachian access."),
            ("Still held", "Official ODOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "CLE": (18, -18),
            "CBU": (18, -18),
            "CIN": (18, 26),
            "DAY": (-92, 6),
            "TOL": (18, -18),
            "AKR": (18, 20),
            "YNG": (18, -18),
            "HUN": (18, 22),
        },
    },
    "north-carolina": {
        "title": "North Carolina Candidate Market-System Map",
        "subtitle": "2D state service-market view: Charlotte/Raleigh growth spine, Triad hub, coastal ports, military/coastal access, Asheville/mountain access, and eastern rural service.",
        "output": "state/north-carolina-market-system-v1.svg",
        "outline_lonlat": [(-84.4, 35.2), (-82.0, 36.4), (-78.5, 36.5), (-75.5, 36.1), (-75.6, 34.6), (-78.0, 33.8), (-81.0, 34.0), (-83.2, 34.9)],
        "zones_lonlat": [
            ("Charlotte / Piedmont", -80.8, 35.3, 1.2, 0.7, "#16a34a"),
            ("Research Triangle", -78.7, 35.9, 0.9, 0.6, "#2563eb"),
            ("Triad hub", -80.0, 36.0, 0.9, 0.6, "#0f766e"),
            ("Coastal / military", -77.4, 35.0, 1.6, 1.0, "#c2410c"),
            ("Mountain access", -82.7, 35.6, 1.0, 0.7, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows North Carolina as growth spine, research, Triad, port/coastal, military, mountain, and eastern access promises."),
            ("Client workshop", "Rank Charlotte-Raleigh reliability, Triad access, Wilmington port, military/coastal access, mountain resilience, and eastern rural coverage."),
            ("Still held", "Official NCDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "CLT": (18, 24),
            "RDU": (18, -18),
            "GSO": (-88, -8),
            "AVL": (-88, 8),
            "WIL": (18, 20),
            "FAY": (18, 22),
            "MHC": (18, 18),
            "NOR": (18, -18),
        },
    },
    "michigan": {
        "title": "Michigan Candidate Market-System Map",
        "subtitle": "2D state service-market view: Detroit gateway, Grand Rapids/Lansing triangle, Lake Michigan belt, Flint/Saginaw, northern access, Upper Peninsula, and bridge/port feeders.",
        "output": "state/michigan-market-system-v1.svg",
        "outline_lonlat": [(-90.4, 46.6), (-84.6, 46.6), (-82.4, 45.1), (-82.5, 42.8), (-83.5, 41.7), (-86.5, 41.8), (-87.2, 43.8), (-88.9, 45.3)],
        "zones_lonlat": [
            ("Detroit gateway", -83.1, 42.4, 0.8, 0.6, "#c2410c"),
            ("Lower Peninsula triangle", -84.7, 43.0, 1.6, 1.2, "#16a34a"),
            ("Lake Michigan belt", -86.5, 43.4, 0.9, 1.2, "#2563eb"),
            ("Northern access", -85.1, 45.0, 1.7, 0.8, "#0f766e"),
            ("Upper Peninsula", -87.4, 46.2, 2.0, 0.6, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Michigan as automotive gateway, west Michigan market, capital hub, northern access, bridge, port, and Upper Peninsula promises."),
            ("Client workshop", "Rank Detroit gateway, Grand Rapids/Lansing reliability, Lake Michigan belt, Flint/Saginaw access, Mackinac continuity, UP coverage, and port feeders."),
            ("Still held", "Official MDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "DET": (18, 24),
            "PTD": (-102, -10),
            "LAN": (18, -18),
            "GRR": (-110, 8),
            "KAL": (-92, 10),
            "TVC": (18, -18),
            "MCK": (18, -18),
            "SSM": (18, -18),
        },
    },
    "washington": {
        "title": "Washington Candidate Market-System Map",
        "subtitle": "2D state service-market view: Puget Sound gateway, I-5 urban belt, Columbia Basin, Spokane inland hub, Tri-Cities, port feeders, and mountain-pass resilience.",
        "output": "state/washington-market-system-v1.svg",
        "outline_lonlat": [(-124.8, 49.0), (-117.0, 49.0), (-117.0, 45.6), (-121.0, 45.6), (-124.0, 46.2)],
        "zones_lonlat": [
            ("Puget Sound gateway", -122.6, 47.5, 1.0, 1.1, "#c2410c"),
            ("I-5 urban belt", -122.5, 47.0, 0.9, 1.6, "#16a34a"),
            ("Columbia Basin", -120.0, 46.8, 1.5, 1.0, "#0f766e"),
            ("Spokane inland hub", -117.5, 47.6, 0.8, 0.7, "#2563eb"),
            ("Mountain-pass resilience", -121.0, 47.2, 1.2, 0.8, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Washington as port gateway, I-5 belt, mountain-pass resilience, agricultural basin, inland hub, and border access promises."),
            ("Client workshop", "Rank Puget Sound ports, I-5 reliability, Cascade pass resilience, Tri-Cities, Spokane, border access, and rural freight coverage."),
            ("Still held", "Official WSDOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "zone_label_offsets": {
            "Puget Sound gateway": (-22, -34),
            "I-5 urban belt": (18, 18),
            "Mountain-pass resilience": (18, -16),
        },
        "label_offsets": {
            "SEA": (18, -18),
            "TAC": (18, 24),
            "PSE": (-118, -16),
            "PTT": (-118, 30),
            "OLY": (-92, 8),
            "VAN": (18, 24),
            "YAK": (18, 20),
            "TRI": (18, 22),
            "SPO": (18, -18),
        },
    },
    "arizona": {
        "title": "Arizona Candidate Market-System Map",
        "subtitle": "2D state service-market view: Phoenix hub, Tucson border spine, northern I-40 access, Yuma/California gateway, Flagstaff mountain resilience, and rural tribal access.",
        "output": "state/arizona-market-system-v1.svg",
        "outline_lonlat": [(-114.8, 37.0), (-109.0, 37.0), (-109.0, 31.3), (-111.0, 31.3), (-114.8, 32.6)],
        "zones_lonlat": [
            ("Phoenix hub", -112.1, 33.5, 1.1, 0.9, "#16a34a"),
            ("Tucson / border spine", -111.2, 32.2, 1.0, 1.0, "#c2410c"),
            ("Northern I-40 access", -112.4, 35.2, 2.1, 0.7, "#2563eb"),
            ("Yuma / California gateway", -114.2, 32.8, 0.8, 0.6, "#0f766e"),
            ("Rural / tribal access", -110.4, 34.6, 1.2, 1.2, "#7c3aed"),
        ],
        "callouts": [
            ("Why this sells", "Shows Arizona as Phoenix hub, Tucson/border spine, I-40 northern access, Yuma gateway, mountain resilience, and rural access promises."),
            ("Client workshop", "Rank Phoenix reliability, Tucson/border access, Flagstaff/I-40, Yuma gateway, rural/tribal access, heat resilience, and California connectors."),
            ("Still held", "Official ADOT priority, legal SLAs, construction, cost, ROI, eligibility, endorsement, and validation."),
        ],
        "label_offsets": {
            "PHX": (18, -18),
            "TUS": (18, 22),
            "NOG": (18, 24),
            "FLG": (18, -18),
            "YUM": (-92, 8),
            "KIN": (-92, -10),
            "GAL": (18, -18),
            "SHOW": (18, 22),
        },
    },
    "canada": {
        "title": "Canada Candidate Market-System Map",
        "subtitle": "2D national service-market view: Pacific gateway, prairie spine, Great Lakes/St. Lawrence, Atlantic access, northern service, and resource connectors.",
        "output": "international/canada-market-system-v1.svg",
        "lat_scale": 1.9,
        "outline_lonlat": [(-132.0, 55.8), (-122.0, 58.0), (-112.0, 56.2), (-100.0, 57.6), (-84.0, 51.0), (-72.0, 48.5), (-62.0, 46.0), (-69.0, 43.0), (-82.0, 42.0), (-97.0, 49.0), (-114.0, 50.0), (-124.0, 48.6)],
        "zones_lonlat": [
            ("Pacific gateway", -124.0, 50.2, 4.5, 2.5, "#0f766e"),
            ("Prairie spine", -106.0, 51.0, 11.0, 2.3, "#16a34a"),
            ("Great Lakes / St. Lawrence", -78.0, 44.8, 7.8, 2.2, "#2563eb"),
            ("Atlantic access", -66.5, 45.0, 4.6, 1.8, "#c2410c"),
            ("Northern service", -105.0, 54.8, 18.0, 2.5, "#7c3aed"),
        ],
        "zone_label_offsets": {
            "Northern service": (90, -42),
            "Pacific gateway": (0, 18),
        },
        "callouts": [
            ("Why this sells", "Shows Canada as gateway, prairie, Great Lakes, Atlantic, and northern service markets."),
            ("Client workshop", "Rank port gateways, prairie reliability, cross-country trunk continuity, northern access, winter resilience, and resource links."),
            ("Still held", "Official Canadian designation, legal SLAs, construction, cost, ROI, eligibility, endorsement, external validation, and public readiness."),
        ],
        "label_offsets": {
            "YVR": (18, 36),
            "SEA": (18, -24),
            "CGY": (18, -18),
            "EDM": (18, -18),
            "REG": (18, 20),
            "WPG": (18, -18),
            "TOR": (18, 20),
            "WSR": (18, 24),
            "OTT": (-78, -16),
            "MTL": (18, -28),
            "QC": (18, -12),
            "HFX": (18, 20),
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


def projection_points(cfg: dict[str, object], nodes: dict[str, dict[str, str]]) -> list[tuple[float, float]]:
    lat_scale = float(cfg.get("lat_scale", 1.0))
    points = [(float(n["lon"]), float(n["lat"]) * lat_scale) for n in nodes.values()]
    points.extend((lon, lat * lat_scale) for lon, lat in cfg.get("outline_lonlat", []))
    for _label, lon, lat, rx, ry, _color in cfg.get("zones_lonlat", []):
        points.extend([(lon - rx, (lat - ry) * lat_scale), (lon + rx, (lat + ry) * lat_scale)])
    return points


def projector(cfg: dict[str, object], nodes: dict[str, dict[str, str]]):
    points = projection_points(cfg, nodes)
    lat_scale = float(cfg.get("lat_scale", 1.0))
    lons = [point[0] for point in points]
    lats = [point[1] for point in points]
    min_lon, max_lon = min(lons), max(lons)
    min_lat, max_lat = min(lats), max(lats)
    scale = min(MAP_W / (max_lon - min_lon), MAP_H / (max_lat - min_lat)) * 0.9
    used_w = (max_lon - min_lon) * scale
    used_h = (max_lat - min_lat) * scale
    x0 = MAP_X + (MAP_W - used_w) / 2
    y0 = MAP_Y + (MAP_H - used_h) / 2

    def project_point(lon: float, lat: float) -> tuple[float, float]:
        scaled_lat = lat * lat_scale
        return x0 + (lon - min_lon) * scale, y0 + (max_lat - scaled_lat) * scale

    return project_point


def project_nodes(cfg: dict[str, object], nodes: dict[str, dict[str, str]]) -> dict[str, tuple[float, float]]:
    project_point = projector(cfg, nodes)
    return {
        node_id: project_point(float(n["lon"]), float(n["lat"]))
        for node_id, n in nodes.items()
    }


def projected_outline(cfg: dict[str, object], nodes: dict[str, dict[str, str]]) -> str:
    points = cfg.get("outline_lonlat", [])
    if not points:
        return str(cfg.get("outline", ""))
    project_point = projector(cfg, nodes)
    projected = [project_point(lon, lat) for lon, lat in points]
    head = projected[0]
    tail = " ".join(f"L{x:.1f} {y:.1f}" for x, y in projected[1:])
    return f"M{head[0]:.1f} {head[1]:.1f} {tail} Z"


def projected_zone(cfg: dict[str, object], nodes: dict[str, dict[str, str]], zone: tuple[str, float, float, float, float, str]) -> tuple[str, float, float, float, float, str]:
    label, lon, lat, rx_lon, ry_lat, color = zone
    project_point = projector(cfg, nodes)
    cx, cy = project_point(lon, lat)
    x1, _ = project_point(lon - rx_lon, lat)
    x2, _ = project_point(lon + rx_lon, lat)
    _, y1 = project_point(lon, lat - ry_lat)
    _, y2 = project_point(lon, lat + ry_lat)
    return label, cx, cy, abs(x2 - x1) / 2, abs(y2 - y1) / 2, color


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
    coords = project_nodes(cfg, nodes)
    counts = Counter(row["tier"] for row in rows)
    layers = Counter(row["market_layer"] for row in rows)
    source_needed = sum(1 for row in rows if row["evidence_label"] == "source-needed")
    out = OUT_DIR / cfg["output"]
    out.parent.mkdir(parents=True, exist_ok=True)

    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}">',
        f'<rect width="{W}" height="{H}" fill="#f8fafc"/>',
        f'<rect x="28" y="28" width="{W-56}" height="{H-56}" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>',
        text(70, 86, cfg["title"], 38, "#111827", 700),
        *multiline(70, 124, cfg["subtitle"], 17, 1300, "#475569"),
        f'<path d="{projected_outline(cfg, nodes)}" fill="#eef6f0" stroke="#94a3b8" stroke-width="2" opacity="0.92"/>',
    ]

    for zone in cfg.get("zones_lonlat", cfg.get("zones", [])):
        label, x, y, rx, ry, color = projected_zone(cfg, nodes, zone) if len(zone) == 6 and isinstance(zone[1], float) else zone
        label_dx, label_dy = cfg.get("zone_label_offsets", {}).get(label, (0, 0))
        svg.append(f'<ellipse cx="{x}" cy="{y}" rx="{rx}" ry="{ry}" fill="{color}" opacity="0.08" stroke="{color}" stroke-width="2" stroke-dasharray="8 8"/>')
        svg.append(text(x - rx + 18 + label_dx, max(y - ry + 30 + label_dy, 175), label, 17, color, 700))

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
        y = 980 + idx * 60
        svg.append(text(panel_x + 52, y, title, 15, "#111827", 700))
        svg.extend(multiline(panel_x + 52, y + 22, body, 12, 470, "#475569"))

    svg.append(text(70, H - 72, "Held claims: official network, legal SLA, construction readiness, costs, numeric ROI, funding eligibility, compliance, endorsement, external validation, and public readiness.", 16, "#92400e", 700))
    svg.append(text(70, H - 42, "Use as a client discovery surface: replace heuristic rows with country-specific source rows before making stronger claims.", 14, "#64748b"))
    svg.append("</svg>")

    out.write_text("\n".join(svg) + "\n", encoding="utf-8", newline="\n")
    return out


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    by_country = read_rows()
    for country in ["china", "india", "texas", "iowa", "florida", "california", "new-york", "illinois", "georgia", "pennsylvania", "ohio", "north-carolina", "michigan", "washington", "arizona", "canada"]:
        out = render(country, by_country[country])
        print(f"rendered {out}")


if __name__ == "__main__":
    main()
