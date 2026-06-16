---
marp: true
theme: default
paginate: false
---

<!--
Status: draft
Kind: deck
Owner: route-comms

Truth label: concept. This deck sells the ROUTE / Interstate 2.0 vision for
politicians, funders, states, and industry. It does not claim final corridor
rankings, official endorsement, construction readiness, or ROI values.
-->

<style>
section {
  background: #111827;
  color: #f9fafb;
  font-family: "Aptos", "Segoe UI", sans-serif;
  padding: 34px 54px;
}
h1 {
  color: #fbbf24;
  font-size: 52px;
  letter-spacing: -1px;
}
h2 {
  color: #f9fafb;
  font-size: 34px;
  letter-spacing: -0.4px;
}
h3 {
  color: #93c5fd;
}
strong {
  color: #fbbf24;
}
em {
  color: #a7f3d0;
  font-style: normal;
}
section.title {
  display: flex;
  flex-direction: column;
  justify-content: center;
}
section.title h1 {
  font-size: 64px;
  max-width: 900px;
}
section.title p {
  font-size: 26px;
  color: #d1d5db;
}
section.title::after {
  content: "";
  width: 68%;
  height: 8px;
  margin-top: 34px;
  border-radius: 999px;
  background: linear-gradient(90deg, #fbbf24, #60a5fa, #34d399);
}
.cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 18px;
  margin-top: 22px;
}
.card {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 18px;
  padding: 20px;
  min-height: 145px;
}
.card h3 {
  margin: 0 0 10px;
  font-size: 23px;
}
.card p {
  color: #d1d5db;
  font-size: 18px;
  margin: 0;
}
.good {
  border-color: #34d399;
}
.warn {
  border-color: #fbbf24;
}
.blue {
  border-color: #60a5fa;
}
.big-number {
  font-size: 58px;
  color: #fbbf24;
  font-weight: 800;
}
.ladder {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 14px;
  margin-top: 26px;
}
.rung {
  background: linear-gradient(180deg, #1f2937, #111827);
  border: 1px solid #4b5563;
  border-radius: 18px;
  padding: 18px;
}
.rung .tier {
  color: #93c5fd;
  font-weight: 800;
  font-size: 26px;
}
.rung .promise {
  color: #fbbf24;
  font-size: 31px;
  font-weight: 800;
  margin: 8px 0;
}
.flow {
  display: flex;
  align-items: stretch;
  gap: 10px;
  margin-top: 26px;
}
.step {
  flex: 1;
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 16px;
  padding: 16px;
  text-align: center;
  font-size: 20px;
}
.arrow {
  display: flex;
  align-items: center;
  color: #fbbf24;
  font-size: 28px;
  font-weight: 800;
}
.split {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 28px;
  align-items: center;
}
.quote {
  border-left: 6px solid #fbbf24;
  padding-left: 20px;
  color: #e5e7eb;
  font-size: 30px;
  line-height: 1.25;
}
table {
  background: #f9fafb;
  color: #111827;
  border-radius: 12px;
  overflow: hidden;
  font-size: 18px;
}
th {
  background: #fde68a;
  color: #111827;
}
td, th {
  padding: 10px 12px;
}
.map-hero {
  display: grid;
  grid-template-columns: 1.25fr 0.75fr;
  gap: 26px;
  align-items: center;
}
.map-hero img,
.map-grid img {
  width: 100%;
  border-radius: 18px;
  border: 2px solid #374151;
  background: #ffffff;
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.35);
}
.caption {
  color: #d1d5db;
  font-size: 18px;
  line-height: 1.35;
}
.map-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 18px;
}
.map-card {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 18px;
  padding: 14px;
}
.map-card h3 {
  margin: 8px 0 4px;
}
.map-card p {
  color: #d1d5db;
  margin: 0;
  font-size: 15px;
}
.map-grid.compact {
  gap: 14px;
}
.map-grid.compact .map-card {
  padding: 10px;
}
.map-grid.compact img {
  height: 145px;
  object-fit: cover;
  object-position: center;
}
.map-grid.compact h3 {
  font-size: 18px;
}
.map-grid.compact p {
  font-size: 13px;
}
.cta {
  margin-top: 28px;
  border-radius: 18px;
  background: linear-gradient(90deg, #fbbf24, #f97316);
  color: #111827;
  padding: 18px 24px;
  font-size: 27px;
  font-weight: 800;
  text-align: center;
}
.takeaway {
  margin-top: 22px;
  border-radius: 16px;
  background: rgba(251, 191, 36, 0.12);
  border: 1px solid rgba(251, 191, 36, 0.55);
  color: #fde68a;
  padding: 14px 20px;
  font-size: 25px;
  font-weight: 800;
  text-align: center;
}
.mini-flow {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 18px;
  margin-top: 26px;
}
.mini-flow div {
  background: #1f2937;
  border: 1px solid #60a5fa;
  border-radius: 18px;
  padding: 22px;
  text-align: center;
  font-size: 24px;
  font-weight: 800;
}
.feature-wheel {
  display: grid;
  grid-template-columns: 1fr 1.1fr 1fr;
  grid-template-rows: repeat(2, 1fr);
  gap: 16px;
  align-items: stretch;
  margin-top: 22px;
}
.feature-wheel .hub {
  grid-column: 2;
  grid-row: 1 / span 2;
  background: radial-gradient(circle at center, #fbbf24, #f97316);
  color: #111827;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 28px;
  font-size: 31px;
  font-weight: 900;
  min-height: 250px;
}
.feature {
  background: #1f2937;
  border: 1px solid #4b5563;
  border-radius: 18px;
  padding: 18px;
}
.feature h3 {
  margin: 0 0 8px;
  font-size: 22px;
}
.feature p {
  color: #d1d5db;
  margin: 0;
  font-size: 17px;
}
.wide-map {
  display: grid;
  grid-template-columns: 1.45fr 0.55fr;
  gap: 22px;
  align-items: center;
}
.wide-map img {
  width: 100%;
  max-height: 500px;
  object-fit: contain;
  border-radius: 20px;
  border: 2px solid #374151;
  background: #ffffff;
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.35);
}
.side-list {
  display: grid;
  gap: 12px;
}
.side-list div {
  background: #1f2937;
  border-left: 5px solid #34d399;
  border-radius: 12px;
  padding: 13px 15px;
  color: #e5e7eb;
  font-size: 17px;
}
.before-after {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-top: 24px;
}
.before-after .panel {
  border-radius: 22px;
  padding: 24px;
  min-height: 330px;
}
.before-after .before {
  background: #2a1f1f;
  border: 2px solid #ef4444;
}
.before-after .after {
  background: #13251e;
  border: 2px solid #34d399;
}
.before-after h3 {
  font-size: 30px;
  margin-top: 0;
}
.network-stack {
  display: grid;
  gap: 12px;
  margin-top: 18px;
}
.network-stack div {
  border-radius: 14px;
  padding: 12px 16px;
  color: #111827;
  font-weight: 800;
}
.network-stack .flat {
  background: #fecaca;
}
.network-stack .t1 {
  background: #fbbf24;
}
.network-stack .t2 {
  background: #93c5fd;
}
.network-stack .t3 {
  background: #a7f3d0;
}
.network-stack .t4 {
  background: #ddd6fe;
}
.panel p {
  color: #e5e7eb;
  font-size: 20px;
}
.t1-road {
  display: grid;
  grid-template-columns: 1fr 1.2fr 1fr;
  gap: 20px;
  align-items: center;
  margin-top: 18px;
}
.road-card {
  background: #1f2937;
  border: 1px solid #4b5563;
  border-radius: 18px;
  padding: 18px;
  min-height: 115px;
  margin-bottom: 16px;
}
.road-card h3 {
  margin: 0 0 8px;
  font-size: 22px;
}
.road-card p {
  color: #d1d5db;
  margin: 0;
  font-size: 16px;
}
.road-core {
  min-height: 360px;
  border-radius: 26px;
  background:
    linear-gradient(90deg, transparent 48%, #fbbf24 48%, #fbbf24 52%, transparent 52%),
    repeating-linear-gradient(180deg, #0f172a 0 42px, #1f2937 42px 84px);
  border: 3px solid #60a5fa;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: inset 0 0 0 10px rgba(255, 255, 255, 0.04);
}
.road-core div {
  background: #fbbf24;
  color: #111827;
  border-radius: 999px;
  padding: 24px 32px;
  font-size: 34px;
  font-weight: 900;
  text-align: center;
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.35);
}
.relay-board {
  display: grid;
  grid-template-columns: 0.9fr 1.1fr;
  gap: 26px;
  align-items: center;
}
.hub-ring {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 14px;
}
.hub-ring div {
  background: #1f2937;
  border: 1px solid #60a5fa;
  border-radius: 18px;
  padding: 18px;
  min-height: 100px;
  color: #d1d5db;
}
.hub-ring strong {
  display: block;
  margin-bottom: 8px;
  font-size: 21px;
}
.roi-board {
  display: grid;
  grid-template-columns: 1fr 1fr 0.9fr;
  gap: 18px;
  margin-top: 24px;
}
.roi-panel {
  background: #1f2937;
  border: 1px solid #4b5563;
  border-radius: 20px;
  padding: 20px;
  min-height: 310px;
}
.roi-panel h3 {
  margin-top: 0;
  font-size: 26px;
}
.roi-panel ul {
  margin: 14px 0 0;
  padding-left: 22px;
  color: #d1d5db;
  font-size: 18px;
  line-height: 1.35;
}
.roi-gate {
  background: linear-gradient(180deg, #fbbf24, #f97316);
  color: #111827;
}
.roi-gate h3,
.roi-gate strong {
  color: #111827;
}
.roi-gate p {
  font-size: 21px;
  font-weight: 800;
}
</style>

<!-- _class: title -->

# Interstate 2.0 should be a promise, not a map.

ROUTE turns national ambition into a plan states, industry, and funders can refine together.

---

<!-- _class: title -->

# America built the first interstate system for another era.

**The next one has to solve freight reliability, rural access, resilience, electrification, automation, and regional growth.**

---

## The problem politicians recognize

<div class="feature-wheel">
  <div class="feature">
    <h3>Gridlock</h3>
    <p>Ports, metros, mountain passes, and interchanges can break national freight reliability.</p>
  </div>
  <div class="hub">The system has no national service standard</div>
  <div class="feature">
    <h3>No organized resiliency</h3>
    <p>Closures and shocks are handled locally, not as a designed national redundancy system.</p>
  </div>
  <div class="feature">
    <h3>Inconsistent freight lanes</h3>
    <p>Freight priority appears corridor by corridor instead of as a national operating layer.</p>
  </div>
  <div class="feature">
    <h3>Future tech has nowhere to land</h3>
    <p>EV charging, autonomous trunking, relay labor, and hubs need planned nodes.</p>
  </div>
</div>

<div class="takeaway">ROUTE gives politicians, states, and industry the same planning language.</div>

---

## The answer: a national service promise ladder

<div class="ladder">
  <div class="rung">
    <div class="tier">T1</div>
    <div class="promise">48h / 36h</div>
    <p>National timed-freight spine</p>
  </div>
  <div class="rung">
    <div class="tier">T2</div>
    <div class="promise">24h / 12h</div>
    <p>Regional connector layer</p>
  </div>
  <div class="rung">
    <div class="tier">T3</div>
    <div class="promise">6h</div>
    <p>Feeder and access mesh</p>
  </div>
  <div class="rung">
    <div class="tier">T4</div>
    <div class="promise">1h</div>
    <p>Terminal and last-mile access</p>
  </div>
</div>

<div class="takeaway">The question changes: what should the network promise?</div>

---

## Roads need the hierarchy rail already has

<div class="before-after">
  <div class="panel before">
    <h3>Today: one flat category</h3>
    <p>Interstates are mostly treated as the same kind of asset. That makes the
    country argue route by route instead of by service role.</p>
    <div class="network-stack">
      <div class="flat">Interstate = Interstate = Interstate</div>
      <div class="flat">Same label, different national value</div>
      <div class="flat">Priorities blur together</div>
    </div>
  </div>
  <div class="panel after">
    <h3>ROUTE: a service network</h3>
    <p>Rail, metro, and transit systems already use hierarchy: express,
    regional, feeder, terminal. ROUTE applies that logic to roads.</p>
    <div class="network-stack">
      <div class="t1">T1 national spine — 48h / 36h</div>
      <div class="t2">T2 regional connectors — 24h / 12h</div>
      <div class="t3">T3 feeder access — 6h</div>
      <div class="t4">T4 terminal access — 1h</div>
    </div>
  </div>
</div>

---

## T1: the national promise spine

<div class="t1-road">
  <div>
    <div class="road-card">
      <h3>Roadway needs</h3>
      <p>Surface, safety, capacity, and reliability work where the national promise depends on it.</p>
    </div>
    <div class="road-card">
      <h3>Managed lanes</h3>
      <p>Premium freight lanes where service windows justify the investment.</p>
    </div>
  </div>
  <div class="road-core"><div>48h / 36h<br/>national service</div></div>
  <div>
    <div class="road-card">
      <h3>Interchange fixes</h3>
      <p>Targeted bottleneck and junction repairs before giant corridor spending.</p>
    </div>
    <div class="road-card">
      <h3>Resilience + multiple routes</h3>
      <p>Redundancy for closures, weather, mountain passes, ports, and national shocks.</p>
    </div>
  </div>
</div>

<div class="takeaway">T1 is where the country buys reliability.</div>

---

## The maps make the promise visible

<div class="map-hero">
  <div>
    <img src="../../maps/beck-schematic.png" alt="ROUTE Beck schematic map" />
  </div>
  <div>
    <h3>From highway map to service map</h3>
    <p class="caption">
      ROUTE has already built national schematic maps that make Interstate 2.0
      legible: trunks, connectors, stops, transfers, and service layers can be
      shown like a metro system for freight and access.
    </p>
    <p class="caption">
      Use them as the visual centerpiece: the promise is visible before the
      investment package is final.
    </p>
  </div>
</div>

---

## T2: the regional engine

<div class="wide-map">
  <div>
    <img src="../../maps/beck-schematic-t2.png" alt="ROUTE T2 Beck schematic map" />
  </div>
  <div class="side-list">
    <div><strong>24h / 12h service:</strong> regional freight promises that feed the T1 spine.</div>
    <div><strong>Real contacts:</strong> routes must connect to meaningful system nodes, not just pass nearby.</div>
    <div><strong>Relief value:</strong> T2 can carry resilience, bypass, port, and border value when T1 is stressed.</div>
    <div><strong>Demotion discipline:</strong> useful routes can fall to T3/T4 instead of cluttering the regional map.</div>
  </div>
</div>

---

## One system, multiple views

<div class="map-grid compact">
  <div class="map-card">
    <img src="../../maps/all-tiers.png" alt="All tiers map" />
    <h3>National tier picture</h3>
    <p>Shows the system as a layered national service network.</p>
  </div>
  <div class="map-card">
    <img src="../../maps/beck-schematic-t2-only.png" alt="T2-only Beck schematic map" />
    <h3>Regional connector view</h3>
    <p>Inspect regional service without the national trunk dominating the story.</p>
  </div>
  <div class="map-card">
    <img src="../../maps/t3-great-lakes.png" alt="T3 Great Lakes regional map" />
    <h3>Regional access view</h3>
    <p>Turns feeder obligations into something a region can discuss.</p>
  </div>
  <div class="map-card">
    <img src="../../maps/t3-texas-border.png" alt="T3 Texas Border regional map" />
    <h3>Border and production access</h3>
    <p>Connects industry, agriculture, border flow, and access.</p>
  </div>
</div>

---

## ROUTE is the refinement engine

<div class="flow">
  <div class="step">Public goal</div>
  <div class="arrow">→</div>
  <div class="step">Requirement</div>
  <div class="arrow">→</div>
  <div class="step">Model change</div>
  <div class="arrow">→</div>
  <div class="step">Plan refinement</div>
  <div class="arrow">→</div>
  <div class="step">Staged option</div>
</div>

<div class="cards">
  <div class="card blue"><h3>Freight reliability</h3><p>Re-rank corridors, hubs, relays, and bottlenecks around service windows.</p></div>
  <div class="card blue"><h3>Rural access</h3><p>Keep agricultural and smaller-market access visible when metro volume dominates.</p></div>
  <div class="card blue"><h3>Funding window</h3><p>Break the vision into pilots, studies, hubs, upgrades, and corridor packages.</p></div>
</div>

---

## What states get

<div class="cards">
  <div class="card good">
    <h3>A national case</h3>
    <p>State priorities become part of a national freight, access, resilience, and competitiveness story.</p>
  </div>
  <div class="card good">
    <h3>A refinement loop</h3>
    <p>State objections can change the plan instead of being handled after the announcement.</p>
  </div>
  <div class="card good">
    <h3>A staged ask</h3>
    <p>Planning, hubs, charging, safety, pavement, and corridor upgrades can be funded in practical steps.</p>
  </div>
</div>

**Not Washington handing states a finished map. A way for states to shape the national plan.**

---

## What industry gets

<div class="cards">
  <div class="card good">
    <h3>Freight reliability</h3>
    <p>Timed promises make service quality concrete for shippers, carriers, ports, and warehouses.</p>
  </div>
  <div class="card good">
    <h3>Operational nodes</h3>
    <p>Relay hubs, charging depots, transfer points, and maintenance bases become planned infrastructure.</p>
  </div>
  <div class="card good">
    <h3>A voice in requirements</h3>
    <p>Industry pain points can become route, stop, staging, and investment refinements.</p>
  </div>
</div>

---

## Relay hubs: the aviation model for freight

<div class="relay-board">
<div class="quote">

Pilots use crew bases, duty rules, handoffs, maintenance systems, and hub operations.

Premium freight can evolve the same way.

</div>
<div class="hub-ring">
  <div><strong>Crew base</strong>Regional shifts and professional freight chauffeurs.</div>
  <div><strong>Charge + service</strong>Known EV, maintenance, and staging nodes.</div>
  <div><strong>Handoff point</strong>Scheduled load, driver, and future AV transfers.</div>
  <div><strong>State asset</strong>Jobs, hubs, and explainable infrastructure packages.</div>
</div>
</div>

---

## Why the relay layer matters

<div class="cards">
  <div class="card blue">
    <h3>Human today</h3>
    <p>Regional relay shifts can improve driver quality of life and widen the labor pool.</p>
  </div>
  <div class="card blue">
    <h3>Electric next</h3>
    <p>Heavy-duty charging and maintenance can live at predictable relay nodes.</p>
  </div>
  <div class="card blue">
    <h3>Autonomous later</h3>
    <p>Driverless trunk segments can phase in between staffed hubs when rules and technology allow.</p>
  </div>
</div>

<div class="mini-flow">
  <div>Plan hubs</div>
  <div>Pilot service + charging</div>
  <div>Automate trunk segments</div>
</div>

---

## The policy backbone already exists

| Paper track | Plain-English role in the story |
|---|---|
| Freight reliability | Why predictable windows matter to shippers and carriers. |
| National max-flow | Where the system binds or fails under stress. |
| 48-hour economy | Why fast highway freight changes the market for high-value goods. |
| Empty backhaul / relay | Why hubs can improve utilization and scheduling. |
| Interstate 2.0 framework | How managed lanes, missing links, resilience, charging, and transit fit together. |
| Relay marketplace | How the operating model can mature toward EV and autonomous freight. |

---

## ROI without fake numbers

<div class="roi-board">
  <div class="roi-panel">
    <h3>Value stack</h3>
    <ul>
      <li>freight reliability</li>
      <li>bottleneck relief</li>
      <li>resilience recovery</li>
      <li>rural + production access</li>
      <li>driver workforce quality</li>
      <li>EV / AV readiness</li>
    </ul>
  </div>
  <div class="roi-panel">
    <h3>Cost stack</h3>
    <ul>
      <li>planning + evidence</li>
      <li>right-of-way + delivery</li>
      <li>capital construction</li>
      <li>operations + maintenance</li>
      <li>technology + controls</li>
      <li>community mitigation</li>
    </ul>
  </div>
  <div class="roi-panel roi-gate">
    <h3>The rule</h3>
    <p>No ROI claim until sources, price year, uncertainty, exclusions, and review lanes are visible.</p>
  </div>
</div>

---

## The win-win

<div class="cards">
  <div class="card good"><h3>States</h3><p>A stronger case for nationally relevant projects.</p></div>
  <div class="card good"><h3>Industry</h3><p>Better service windows and a place to put operational requirements.</p></div>
  <div class="card good"><h3>Funders</h3><p>Staged packages instead of a single impossible mega-ask.</p></div>
</div>

<div class="cards">
  <div class="card good"><h3>Drivers</h3><p>A premium regional career path, not just punishing long-haul work.</p></div>
  <div class="card good"><h3>Communities</h3><p>Concerns enter the plan early, before positions harden.</p></div>
  <div class="card good"><h3>Technology</h3><p>A real deployment corridor for EV and autonomous freight.</p></div>
</div>

---

## What we are asking for now

<div class="cards">
  <div class="card warn">
    <h3>Story package</h3>
    <p>Decks, one-pagers, state views, industry views, and public visuals.</p>
  </div>
  <div class="card warn">
    <h3>Decision package</h3>
    <p>Research conclusions, service promise ladder, and staged investment logic.</p>
  </div>
  <div class="card warn">
    <h3>Demo package</h3>
    <p>Show a requirement entering the system and changing the plan.</p>
  </div>
</div>

<div class="cta">Fund the story, decision, and demo package so ROUTE becomes legible to leaders.</div>

---

<!-- _class: title -->

# Interstate 2.0 is the vision.

**ROUTE is how the vision becomes a plan people can fund, refine, and believe in.**
