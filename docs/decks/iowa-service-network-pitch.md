---
marp: true
theme: default
paginate: false
---

<!--
Title: Iowa Service Network Pitch
Slug: iowa-service-network-pitch
Kind: deck
Status: draft
Truth label: concept / evidence-bounded. Client-facing state service-network
pitch with official-plan, legal-SLA, construction, funding, ROI, clearance,
validation, endorsement, and public-readiness claims held.
Sources:
  - docs/briefs/iowa-state-service-network-goals.md
  - docs/briefs/iowa-state-service-network-offer.md
  - docs/briefs/iowa-service-network-discovery-workshop.md
  - docs/briefs/state-value-brief.md
-->

<style>
section {
  background: #0f172a;
  color: #f8fafc;
  font-family: "Aptos", "Segoe UI", sans-serif;
  padding: 36px 56px;
}
h1 {
  color: #f8fafc;
  font-size: 60px;
  letter-spacing: 0;
  line-height: 1.04;
}
h2 {
  color: #f8fafc;
  font-size: 37px;
  letter-spacing: 0;
}
h3 {
  color: #bfdbfe;
  margin: 0 0 8px;
}
strong {
  color: #fbbf24;
}
section.title {
  display: grid;
  grid-template-columns: 0.9fr 1.1fr;
  gap: 34px;
  align-items: center;
}
section.title p {
  color: #cbd5e1;
  font-size: 25px;
  line-height: 1.25;
}
.eyebrow {
  display: inline-block;
  color: #111827;
  background: #fbbf24;
  border-radius: 999px;
  padding: 7px 12px;
  font-size: 16px;
  font-weight: 800;
  margin-bottom: 16px;
}
.hero-map img,
.route-card img {
  width: 100%;
  border-radius: 8px;
  border: 1px solid #334155;
  background: #ffffff;
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.35);
}
.caption,
.card p,
.rail p,
.panel p {
  color: #cbd5e1;
  font-size: 18px;
  line-height: 1.32;
  margin: 0;
}
.grid2,
.grid3,
.grid4 {
  display: grid;
  gap: 16px;
  margin-top: 22px;
}
.grid2 { grid-template-columns: repeat(2, 1fr); }
.grid3 { grid-template-columns: repeat(3, 1fr); }
.grid4 { grid-template-columns: repeat(4, 1fr); }
.card,
.rail,
.panel {
  background: #162033;
  border: 1px solid #334155;
  border-radius: 8px;
  padding: 18px;
}
.card.good { border-color: #34d399; }
.card.warn { border-color: #fbbf24; }
.card.blue { border-color: #60a5fa; }
.takeaway {
  margin-top: 20px;
  background: rgba(251, 191, 36, 0.12);
  border: 1px solid rgba(251, 191, 36, 0.55);
  border-radius: 8px;
  color: #fde68a;
  padding: 13px 16px;
  font-size: 24px;
  font-weight: 900;
  text-align: center;
}
.compare {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 22px;
  margin-top: 24px;
}
.compare .old {
  border-color: #f97316;
}
.compare .new {
  border-color: #34d399;
}
.quote {
  border-left: 6px solid #fbbf24;
  padding-left: 20px;
  font-size: 34px;
  line-height: 1.18;
  color: #f8fafc;
}
.tier {
  color: #93c5fd;
  font-size: 28px;
  font-weight: 900;
}
.role {
  color: #fbbf24;
  font-size: 24px;
  font-weight: 900;
  margin: 8px 0;
}
.rail {
  min-height: 188px;
}
.flow {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 10px;
  margin-top: 24px;
}
.step {
  min-height: 120px;
  border-radius: 8px;
  border: 1px solid #60a5fa;
  background: #162033;
  padding: 13px;
}
.step b {
  display: block;
  color: #fbbf24;
  font-size: 19px;
  margin-bottom: 8px;
}
.step span {
  color: #cbd5e1;
  font-size: 15px;
  line-height: 1.22;
}
.route-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 14px;
  margin-top: 18px;
}
.route-card {
  background: #162033;
  border: 1px solid #334155;
  border-radius: 8px;
  padding: 12px;
}
.route-card img {
  height: 154px;
  object-fit: cover;
  object-position: center;
  box-shadow: none;
}
.route-card h3 {
  font-size: 19px;
  margin: 10px 0 5px;
}
.route-card p {
  color: #cbd5e1;
  font-size: 14px;
  line-height: 1.22;
  margin: 0;
}
table {
  background: #f8fafc;
  color: #111827;
  border-radius: 8px;
  overflow: hidden;
  font-size: 17px;
}
th {
  background: #fde68a;
  color: #111827;
}
td, th {
  padding: 9px 11px;
}
.cta {
  margin-top: 22px;
  border-radius: 8px;
  background: linear-gradient(90deg, #fbbf24, #f97316);
  color: #111827;
  padding: 18px 24px;
  font-size: 27px;
  font-weight: 900;
  text-align: center;
}
</style>

<!-- _class: title -->

<div>
  <div class="eyebrow">Iowa service-network diagnostic</div>
  <h1>Iowa already has roads. What should they promise?</h1>
  <p>ROUTE helps Iowa turn top cities, rural production, terminals, hospitals, campuses, and failures into a service hierarchy and 90-day diagnostic package.</p>
</div>
<div class="hero-map">
  <img src="../../maps/i80.png" alt="I-80 structural map" />
  <p class="caption">Map level: structural route view. Use for service discussion only; not proof of official plan, legal SLA, construction, funding, ROI, or validation.</p>
</div>

---

## The opening question

<div class="compare">
  <div class="panel old">
    <h3>Traditional question</h3>
    <div class="quote">Which roads are on the map?</div>
    <p>That discussion quickly becomes route labels, project lists, and local priority fights.</p>
  </div>
  <div class="panel new">
    <h3>ROUTE question</h3>
    <div class="quote">What must the system deliver?</div>
    <p>The conversation moves to service roles, unacceptable failures, and what evidence is needed next.</p>
  </div>
</div>

<div class="takeaway">The first meeting is about Iowa's priorities, not approval of a finished map.</div>

---

## The offer

<div class="grid3">
  <div class="card good">
    <h3>Name the places</h3>
    <p>Top cities, regional centers, freight nodes, hospitals, campuses, airports, terminals, and production regions.</p>
  </div>
  <div class="card good">
    <h3>Name the failures</h3>
    <p>Flood, snow, work-zone, bridge, incident, detour, interchange, terminal, and rural access failures.</p>
  </div>
  <div class="card good">
    <h3>Name the first package</h3>
    <p>Operations, asset fixes, terminal access, rural access, resilience hardening, or long-range capital questions.</p>
  </div>
</div>

<div class="takeaway">ROUTE turns those answers into a service network Iowa can refine.</div>

---

## Service roles for Iowa

<div class="grid4">
  <div class="rail">
    <div class="tier">T1</div>
    <div class="role">State spine</div>
    <p>Top Iowa city pairs and national gateways with the strongest reliability target.</p>
  </div>
  <div class="rail">
    <div class="tier">T2</div>
    <div class="role">Regional connectors</div>
    <p>Secondary cities, campuses, hospitals, job centers, and cross-state links.</p>
  </div>
  <div class="rail">
    <div class="tier">T3</div>
    <div class="role">Rural production</div>
    <p>Agriculture, energy, manufacturing, county-seat, and rural hospital access.</p>
  </div>
  <div class="rail">
    <div class="tier">T4 / R</div>
    <div class="role">Terminals + resilience</div>
    <p>Airports, river terminals, rail yards, industrial parks, detours, and recovery overlays.</p>
  </div>
</div>

---

## Iowa starter views

<div class="route-grid">
  <div class="route-card">
    <img src="../../maps/i80.png" alt="I-80 structural route map" />
    <h3>I-80</h3>
    <p>Candidate statewide and national-gateway spine discussion.</p>
  </div>
  <div class="route-card">
    <img src="../../maps/i35.png" alt="I-35 structural route map" />
    <h3>I-35</h3>
    <p>North-south spine, metro access, and interchange resilience discussion.</p>
  </div>
  <div class="route-card">
    <img src="../../maps/us30.png" alt="US 30 structural route map" />
    <h3>US 30</h3>
    <p>Regional connector and state-system redundancy discussion.</p>
  </div>
  <div class="route-card">
    <img src="../../maps/us69.png" alt="US 69 structural route map" />
    <h3>US 69</h3>
    <p>Rural and regional access prompt, not a promotion claim.</p>
  </div>
  <div class="route-card">
    <img src="../../maps/us83.png" alt="US 83 structural route map" />
    <h3>US 83</h3>
    <p>Cross-state structural example for rural access comparison.</p>
  </div>
  <div class="route-card">
    <img src="../../maps/all-tiers-v2.png" alt="All tiers structural map" />
    <h3>Service hierarchy</h3>
    <p>Use the map as a structural surface, never as proof.</p>
  </div>
</div>

---

## The 90-day package

<div class="flow">
  <div class="step"><b>1. Intake</b><span>Places, failures, source owners, and political constraints.</span></div>
  <div class="step"><b>2. Tier design</b><span>Draft T1/T2/T3/T4/R service hierarchy.</span></div>
  <div class="step"><b>3. Promise menu</b><span>Candidate reliability, access, recovery, and terminal targets.</span></div>
  <div class="step"><b>4. Resilience</b><span>Critical failures, alternates, and recovery target menu.</span></div>
  <div class="step"><b>5. Packaging</b><span>Operations, asset, access, resilience, and capital sequence.</span></div>
  <div class="step"><b>6. Dashboard</b><span>Executive measures for the promise after adoption.</span></div>
</div>

<div class="takeaway">Ninety days should produce better decisions, not pretend adoption.</div>

---

## What leaders get

<div class="grid3">
  <div class="card blue"><h3>Clear story</h3><p>A service promise for Iowa's people, economy, regions, and gateways.</p></div>
  <div class="card blue"><h3>Practical sequence</h3><p>Operational fixes, asset fixes, access packages, resilience, and capital options separated.</p></div>
  <div class="card blue"><h3>Better tradeoffs</h3><p>Tiers and promise labels show what gets priority and why.</p></div>
  <div class="card blue"><h3>Resilience posture</h3><p>Unacceptable failures and backup expectations are named before crisis.</p></div>
  <div class="card blue"><h3>Rural visibility</h3><p>Production and emergency access do not disappear behind traffic-volume-only logic.</p></div>
  <div class="card blue"><h3>Management view</h3><p>Dashboard questions are designed before adoption.</p></div>
</div>

---

## What this is not

<div class="grid3">
  <div class="card warn"><h3>Not an official DOT plan</h3><p>No route designation, endorsement, approval, validation, or public-readiness claim.</p></div>
  <div class="card warn"><h3>Not a legal SLA</h3><p>Service targets are planning promises until ownership, funding, operating rules, and review close.</p></div>
  <div class="card warn"><h3>Not construction or ROI proof</h3><p>No funding commitment, environmental clearance, right-of-way clearance, construction program, or numeric ROI.</p></div>
</div>

<div class="takeaway">ROUTE asks Iowa to define the service network it wants to evaluate.</div>

---

## The first meeting

<table>
  <thead><tr><th>Question</th><th>ROUTE output</th></tr></thead>
  <tbody>
    <tr><td>Which places must stay connected?</td><td>Priority place list and first-pass service hierarchy.</td></tr>
    <tr><td>Which movements deserve the strongest promise?</td><td>Candidate T1/T2/T3/T4 roles and promise backlog.</td></tr>
    <tr><td>Which failures would be unacceptable?</td><td>Resilience stressor list and recovery-target menu.</td></tr>
    <tr><td>Which bottlenecks limit the promise?</td><td>Access, terminal, interchange, bridge, incident, and local-constraint backlog.</td></tr>
    <tr><td>Which package should move first?</td><td>30/60/90-day action list and candidate investment sequence.</td></tr>
  </tbody>
</table>

---

## Close

<div class="grid3">
  <div class="card good"><h3>Bring priorities</h3><p>Cities, regions, freight nodes, rural access, terminals, hospitals, campuses, and failures.</p></div>
  <div class="card good"><h3>Bring owners</h3><p>Planning, operations, freight, rural/access, local/MPO, and finance/program voices.</p></div>
  <div class="card good"><h3>Bring boundaries</h3><p>What Iowa cannot claim yet and what evidence would change that.</p></div>
</div>

<div class="cta">Start with one decision: what places in Iowa must stay connected?</div>

---

<!-- _class: title -->

<div>
  <div class="eyebrow">ROUTE</div>
  <h1>Iowa can buy a better decision process before buying a bigger project.</h1>
  <p>ROUTE turns the first answer into a service hierarchy, promise backlog, resilience agenda, investment sequence, and dashboard questions.</p>
</div>
<div class="hero-map">
  <img src="../../maps/i35.png" alt="I-35 structural map" />
  <p class="caption">Map level: structural route view. Excluded claims: official plan, legal SLA, construction readiness, funding, ROI, validation, public readiness.</p>
</div>
