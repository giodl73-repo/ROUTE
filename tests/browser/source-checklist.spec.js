import {test,expect} from '@playwright/test';
import fs from 'node:fs/promises';
import crypto from 'node:crypto';
const referencePath=process.env.ROUTE_SOURCE_REFERENCES||'.cache/source-checklist-references.json';
async function ready(page){await page.goto('/ROUTE/#source-checklist');await expect(page.locator('#checklist-status')).toContainText('Ready');}
async function download(page){const pending=page.waitForEvent('download');await page.locator('#checklist-download').click();return JSON.parse(await fs.readFile(await(await pending).path(),'utf8'));}
test('source checklist actual WASM matches four native queries and four published pre-change WASM freight results',async({page})=>{
 await ready(page);const refs=JSON.parse(await fs.readFile(referencePath,'utf8'));
 const actual=await page.evaluate(async refs=>{const m=await import('./pkg/route_web.js');await m.default();return {queries:refs.queries.map(q=>m.inspect_source_checklist_json(q.input)),legacy:refs.legacy_wasm.map(q=>m.evaluate_json(q.input))};},refs);
 expect(actual.queries).toEqual(refs.queries.map(q=>q.canonical));expect(actual.legacy).toEqual(refs.legacy_wasm.map(q=>q.canonical));
});
test('recorded status filters preserve original row parity, fixed partition and held vintage',async({page})=>{
 await ready(page);const original=await(await page.request.get('/ROUTE/sources/recorded-checklist.json')).json();
 expect((await download(page)).selected_rows).toEqual(original.rows);
 for(const [status,count] of [['',12],['ready',4],['blocked',2],['excluded',6]]){
  await page.locator('#checklist-filter').selectOption(status);await expect(page.locator('#checklist-status')).toContainText('Ready');await expect(page.locator('.source-entry')).toHaveCount(count);
  await expect(page.locator('#checklist-summary')).toContainText('12 recorded source entries: 4 ready / 2 blocked / 6 excluded');
  await expect(page.locator('#checklist-conflict')).toContainText('HPMS 2023');await expect(page.locator('#checklist-conflict')).toContainText('2018');await expect(page.locator('#checklist-conflict')).toContainText('Hold and narrow');
  const d=await download(page);expect(d.selected_rows).toEqual(original.rows.filter(r=>!status||r.observation.readiness_status===status));expect(Object.values(d.authority)).toEqual(Array(7).fill(false));expect(Object.values(d.unobserved)).toEqual(Array(3).fill(null));expect(d.vintage_discrepancy.packet_values_relinked).toBe(false);
 }
 await page.locator('#checklist-reset').click();await expect(page.locator('#checklist-status')).toContainText('Ready');await expect(page.locator('[data-source-id="SRC-I80-TIGER"]')).toContainText('Contract gap');await expect(page.locator('[data-source-id="SRC-I80-TIGER"]')).toContainText('Observed blocker');
});
test('invalid edits clear stale output, reject unsupported queries and reset independently',async({page})=>{
 await ready(page);const hours=await page.locator('#hours').textContent();
 for(const query of ['{"status":"current"}','{"status":"ready","source_id":"SRC-I80-HPMS"}','{"date":"2026-10-29"}','{"corridor":0}','x'.repeat(65537)]){
  await page.locator('#checklist-query').fill(query);await expect(page.locator('#checklist-download')).toBeDisabled();await expect(page.locator('#checklist-results')).toBeHidden();
  await page.getByRole('button',{name:'Inspect recorded entries',exact:true}).click();await expect(page.locator('#checklist-status')).toContainText('Invalid checklist query');await expect(page.locator('#checklist-download')).toBeDisabled();
 }
 await page.locator('#checklist-reset').click();await expect(page.locator('#checklist-status')).toContainText('Ready');await expect(page.locator('.source-entry')).toHaveCount(12);await expect(page.locator('#hours')).toHaveText(hours);
});
test('mobile keyboard, three committed artifact hashes, no publisher requests and worker failure',async({page})=>{
 const requests=[];page.on('request',r=>{if(/^https?:/.test(r.url())&&!['127.0.0.1','giodl73-repo.github.io'].includes(new URL(r.url()).hostname))requests.push(r.url());});
 await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#checklist-filter').focus();await page.keyboard.press('ArrowDown');await page.keyboard.press('Enter');await expect(page.locator('#checklist-status')).toContainText('Ready');await expect(page.locator('#checklist-filter')).toHaveValue('ready');
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 const custody=await(await page.request.get('/ROUTE/source-custody.json')).json();expect(custody.artifacts).toHaveLength(3);expect(custody.raw_primary_bytes_in_bundle).toBe(false);
 for(const a of custody.artifacts){const response=await page.request.get('/ROUTE/'+a.path);expect(response.ok()).toBe(true);const bytes=await response.body();expect(bytes.length).toBe(a.bytes);expect(crypto.createHash('sha256').update(bytes).digest('hex')).toBe(a.sha256);expect(bytes.includes(Buffer.from('\r\n'))).toBe(false);}
 expect(requests).toEqual([]);
 await page.route('**/checklist-worker.js',r=>r.abort());await page.reload();await expect(page.locator('#checklist-status')).toContainText(/could not load|timed out/);await expect(page.locator('#checklist-download')).toBeDisabled();await expect(page.locator('#checklist-results')).toBeHidden();await expect(page.locator('#status')).toContainText('Ready');
});

test('label-origin trace preserves unverified vintage and exact offline code evidence',async({page})=>{
 await ready(page);await page.locator('#vintage-audit summary').focus();await page.keyboard.press('Enter');
 await expect(page.locator('#vintage-audit')).toContainText('Matched packet vintage remains unverified');
 await expect(page.locator('#vintage-audit')).toContainText('not independent measured segment counts');
 const pending=page.waitForEvent('download');await page.locator('#vintage-json').click();const d=JSON.parse(await fs.readFile(await(await pending).path(),'utf8'));
 expect(d.matched_packet_input_year).toBeNull();expect(d.packet_values_relinked).toBe(false);expect(d.configured_acquisition_year).toBe(2018);expect(d.packet_claimed_year).toBe(2023);expect(d.evidence).toHaveLength(9);
 for(const e of d.evidence){expect(crypto.createHash('sha256').update(e.excerpt).digest('hex')).toBe(e.excerpt_sha256);expect(e.end_line-e.start_line+1).toBe(e.excerpt.split('\n').length-1);}
 expect(d.local_normalized_cache_observation.raw_rows_published).toBe(false);expect(d.local_normalized_cache_observation.estimators_or_populations_matched).toBe(false);
 await page.setViewportSize({width:390,height:844});expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 await expect(page.locator('#checklist-status')).toContainText('Ready');await expect(page.locator('.source-entry')).toHaveCount(12);
});
