---
marp: true
theme: default
paginate: false
---

<!--
Status: draft
Kind: deck
Owner: route-comms

Truth label: concept. This deck explains how ROUTE supports the Interstate 2.0
solution story. It does not claim final corridor rankings, official endorsement,
construction readiness, publication-grade SLA proof, or ROI values.
-->

<style>
section {
  background: #111827;
  color: #f9fafb;
  font-family: "Aptos", "Segoe UI", sans-serif;
  padding: 36px 56px;
}
h1 {
  color: #fbbf24;
  font-size: 62px;
  letter-spacing: -1.2px;
}
h2 {
  color: #f9fafb;
  font-size: 36px;
  letter-spacing: -0.4px;
}
h3 {
  color: #bfdbfe;
  margin-top: 0;
}
strong {
  color: #fbbf24;
}
section.title {
  display: flex;
  flex-direction: column;
  justify-content: center;
}
section.title p {
  color: #d1d5db;
  font-size: 27px;
  max-width: 930px;
}
section.title::after {
  content: "";
  width: 70%;
  height: 8px;
  margin-top: 34px;
  border-radius: 999px;
  background: linear-gradient(90deg, #fbbf24, #60a5fa, #34d399);
}
.split {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 26px;
  align-items: center;
}
.quote {
  border-left: 6px solid #fbbf24;
  padding-left: 20px;
  color: #e5e7eb;
  font-size: 30px;
  line-height: 1.25;
}
.cards,
.six {
  display: grid;
  gap: 16px;
  margin-top: 24px;
}
.cards {
  grid-template-columns: repeat(3, 1fr);
}
.six {
  grid-template-columns: repeat(3, 1fr);
}
.card {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 18px;
  padding: 19px;
  min-height: 135px;
}
.card.blue { border-color: #60a5fa; }
.card.good { border-color: #34d399; }
.card.warn { border-color: #fbbf24; }
.card p,
.caption {
  color: #e5e7eb;
  font-size: 18px;
  line-height: 1.35;
  margin: 0;
}
.flow {
  display: flex;
  align-items: stretch;
  gap: 10px;
  margin-top: 28px;
}
.step {
  flex: 1;
  background: #1f2937;
  border: 1px solid #60a5fa;
  border-radius: 16px;
  padding: 16px;
  text-align: center;
  font-size: 20px;
  font-weight: 800;
}
.arrow {
  display: flex;
  align-items: center;
  color: #fbbf24;
  font-size: 28px;
  font-weight: 900;
}
.map-hero {
  display: grid;
  grid-template-columns: 1.35fr 0.65fr;
  gap: 24px;
  align-items: center;
}
.map-hero img {
  width: 100%;
  border-radius: 20px;
  border: 2px solid #374151;
  background: #ffffff;
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.35);
}
.ladder {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 14px;
  margin-top: 28px;
}
.rung {
  background: linear-gradient(180deg, #1f2937, #111827);
  border: 1px solid #4b5563;
  border-radius: 18px;
  padding: 18px;
  min-height: 205px;
}
.rung .tier {
  color: #93c5fd;
  font-weight: 800;
  font-size: 28px;
}
.rung .promise {
  color: #fbbf24;
  font-size: 31px;
  font-weight: 900;
  margin: 8px 0;
}
.stack {
  display: grid;
  gap: 14px;
  margin-top: 24px;
}
.stack div {
  border-radius: 14px;
  padding: 14px 18px;
  background: #1f2937;
  border-left: 6px solid #34d399;
  color: #e5e7eb;
  font-size: 20px;
}
.evidence {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 16px;
  margin-top: 24px;
}
.evidence div {
  border-radius: 18px;
  padding: 20px;
  background: #1f2937;
  border: 1px solid #374151;
  min-height: 180px;
}
.evidence .ok { border-color: #34d399; }
.evidence .risk { border-color: #fbbf24; }
.evidence .hold { border-color: #ef4444; }
.cta {
  margin-top: 24px;
  border-radius: 18px;
  background: linear-gradient(90deg, #fbbf24, #f97316);
  color: #111827;
  padding: 18px 24px;
  font-size: 25px;
  font-weight: 900;
  text-align: center;
}
.note {
  margin-top: 22px;
  border-radius: 14px;
  border: 1px solid rgba(251, 191, 36, 0.5);
  background: rgba(251, 191, 36, 0.1);
  color: #fde68a;
  padding: 12px 16px;
  font-size: 18px;
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
</style>

<!-- _class: title -->

# ROUTE makes Interstate 2.0 refinable.

The solution pitch says what the country should build toward. ROUTE explains how leaders can test, revise, and defend the plan.

---

<!-- _class: title -->

# The product is not a prettier map.

ROUTE is a research system, Rust analysis toolkit, generated map pipeline, VTRACE evidence package, and review process for national infrastructure claims.

---

## The job ROUTE does

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

## What a demo should show

<div class="flow">
  <div class="step">State or industry requirement</div>
  <div class="arrow">→</div>
  <div class="step">Affected service promise</div>
  <div class="arrow">→</div>
  <div class="step">Changed option or hold</div>
  <div class="arrow">→</div>
  <div class="step">Evidence needed</div>
</div>

<div class="cards">
  <div class="card good"><h3>State delivery input</h3><p>Match funding, maintenance, ROW, environmental review, and phasing can reshape the staged ask.</p></div>
  <div class="card good"><h3>Industry operating input</h3><p>OD lanes, dwell, HOS, parking, weights, clearances, WIM, and charging become evidence requests.</p></div>
  <div class="card good"><h3>Community input</h3><p>Noise, air quality, safety, access, runoff, and displacement can hold or modify claims.</p></div>
</div>

---

## Example: stress the service rhythm

<div class="flow">
  <div class="step">Tighten stop rhythm</div>
  <div class="arrow">→</div>
  <div class="step">Run candidate docket</div>
  <div class="arrow">→</div>
  <div class="step">Named candidates</div>
  <div class="arrow">→</div>
  <div class="step">Held midpoint gaps</div>
</div>

<div class="cards">
  <div class="card blue"><h3>250-mile gate</h3><p>Current stop/SLA surface passes with a 248-mile max gap.</p></div>
  <div class="card blue"><h3>225-mile stress</h3><p>South Bend/Elkhart and Montgomery appear as named review candidates.</p></div>
  <div class="card warn"><h3>Evidence hold</h3><p>Midpoint fallbacks become the next source task, not a fake recommendation.</p></div>
</div>

---

## ROUTE turns promises into artifacts

<div class="ladder">
  <div class="rung">
    <div class="tier">T1</div>
    <div class="promise">48h / 36h</div>
    <p>Spine.</p>
  </div>
  <div class="rung">
    <div class="tier">T2</div>
    <div class="promise">24h / 12h</div>
    <p>Regional.</p>
  </div>
  <div class="rung">
    <div class="tier">T3</div>
    <div class="promise">6h</div>
    <p>Feeder.</p>
  </div>
  <div class="rung">
    <div class="tier">T4</div>
    <div class="promise">1h</div>
    <p>Terminal.</p>
  </div>
</div>

<div class="note">Promise windows are planning targets until reliability data, scenarios, uncertainty, and role review close.</div>

---

## Maps are generated artifact surfaces

<div class="map-hero">
  <div>
    <img src="../../maps/beck-schematic.png" alt="ROUTE generated schematic map" />
  </div>
  <div>
    <h3>The point is the pipeline</h3>
    <p class="caption">ROUTE generates structural maps, service maps, and regional views from controlled artifacts.</p>
    <p class="caption"><br/>At deck scale, this is a preview. Detailed labels live in the generated map files and evidence package.</p>
    <p class="caption"><br/>The map is a contract surface, not decoration.</p>
  </div>
</div>

---

## The technical invariant: bundle-first identity

<div class="split">
  <div class="quote">
    ROUTE treats a corridor service as a stable bundle.
    <br/><br/>
    Route labels, map ids, tiers, and zones are presentation fields.
  </div>
  <div class="stack">
    <div><strong>segment_bundle_id</strong> is the service/corridor join key.</div>
    <div><strong>national_segment_id</strong> is the physical member key.</div>
    <div><strong>stitch_group_id</strong> preserves continuity claims.</div>
  </div>
</div>

---

## The optimizer is recursive, not a flat ranker

<div class="flow">
  <div class="step">T1 spine</div>
  <div class="arrow">→</div>
  <div class="step">T2 regions</div>
  <div class="arrow">→</div>
  <div class="step">T3 zones</div>
  <div class="arrow">→</div>
  <div class="step">T4 access</div>
  <div class="arrow">→</div>
  <div class="step">Bubble-up repairs</div>
</div>

<div class="cards">
  <div class="card good"><h3>Fixed point</h3><p>Every lower tier must attach to real contacts or produce a repair witness.</p></div>
  <div class="card good"><h3>Tradeoffs</h3><p>Budget, resilience, access, and freight objectives can be compared without pretending there is one magic score.</p></div>
  <div class="card good"><h3>Audit trail</h3><p>Candidate, selected, rejected, repaired, and held choices remain visible.</p></div>
</div>

---

## Evidence labels keep the pitch honest

<div class="evidence">
  <div class="ok">
    <h3>Implemented</h3>
    <p>Works end-to-end in code or reproducible command output.</p>
  </div>
  <div class="risk">
    <h3>Heuristic</h3>
    <p>Useful model, proxy, scenario, or partial source; not proof-grade yet.</p>
  </div>
  <div class="hold">
    <h3>Held</h3>
    <p>Do not promote until the blocker, source gap, or review concern closes.</p>
  </div>
</div>

<div class="cta">ROUTE can sell vision without pretending every claim is proven.</div>

---

## ROI is a framework before it is a number

<div class="cards">
  <div class="card warn">
    <h3>Value stack</h3>
    <p>Freight reliability, bottleneck relief, resilience, rural access, driver quality, EV/AV readiness.</p>
  </div>
  <div class="card warn">
    <h3>Cost stack</h3>
    <p>Planning, right-of-way, capital work, operations, technology, community mitigation, financing risk.</p>
  </div>
  <div class="card warn">
    <h3>Gate</h3>
    <p>No ROI claim until sources, price year, uncertainty, exclusions, and review lanes are visible.</p>
  </div>
</div>

---

## The review system is part of the product

| Review Lane | What It Protects |
|---|---|
| Parliament | National defense, throughput, equity, freight economics, traffic engineering, climate, rural access. |
| Stakeholders | State DOTs, freight, rural users, communities, non-driving access, environmental/community health. |
| Editorial gates | Citation, numeracy, and scope before public claims. |
| VTRACE | Mission, requirements, specs, evidence, verification, validation, and review. |

---

## What a claim package contains

<div class="cards">
  <div class="card blue">
    <h3>Generated artifact</h3>
    <p>Map, stop/SLA surface, candidate docket, diagnostics, or scenario output.</p>
  </div>
  <div class="card blue">
    <h3>Evidence posture</h3>
    <p>Implemented, heuristic, source-needed, held, or ready for promotion.</p>
  </div>
  <div class="card blue">
    <h3>Role tension</h3>
    <p>State delivery, freight operations, rural access, community health, climate, and numeracy checks.</p>
  </div>
</div>

<div class="cta">A better pitch is not louder. It is easier to inspect.</div>

---

## What to fund next

<div class="cards">
  <div class="card good">
    <h3>Story package</h3>
    <p>Public Interstate 2.0 deck, state and industry briefs, and map visuals.</p>
  </div>
  <div class="card good">
    <h3>Decision package</h3>
    <p>Research conclusions, ROI/cost framework, service promise ladder, and evidence labels.</p>
  </div>
  <div class="card good">
    <h3>Demo package</h3>
    <p>Show a requirement entering ROUTE and visibly changing the plan.</p>
  </div>
</div>

<div class="cta">Fund ROUTE as the refinement engine behind Interstate 2.0.</div>

---

<!-- _class: title -->

# Interstate 2.0 is the vision.

**ROUTE is how the vision becomes a plan people can inspect, challenge, refine, and fund.**
