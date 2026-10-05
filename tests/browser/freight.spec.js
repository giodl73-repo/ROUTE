import {test,expect} from '@playwright/test';
import fs from 'node:fs/promises';
test('real WASM responds to keyboard and exports a reproducible scenario',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto('/ROUTE/');await expect(page.locator('#status')).toContainText('Ready');
 const before=await page.locator('#hours').textContent();
 await page.locator('#capacity').focus();await page.keyboard.press('ArrowLeft');
 await expect(page.locator('#capacity-value')).toHaveText('95%');
 await expect(page.locator('#hours')).not.toHaveText(before);await expect(page.locator('#status')).toContainText('Ready');
 await page.locator('#outage').fill('8');await expect(page.locator('#status')).toContainText('Ready');
 await expect(page.locator('#missed')).not.toHaveText('0 → 0');
 const downloadPromise=page.waitForEvent('download');await page.locator('#download').click();
 const download=await downloadPromise;const json=JSON.parse(await fs.readFile(await download.path(),'utf8'));
 expect(json.model).toBe('route-fixed-corridor-v1');expect(json.input.capacity_pct).toBe(95);expect(json.scenario.missed_swaps).toBeGreaterThan(0);
 await page.locator('#share').click();await expect(page).toHaveURL(/capacity_pct=95/);
 const hours=await page.locator('#hours').textContent();await page.reload();await expect(page.locator('#status')).toContainText('Ready');
 await expect(page.locator('#hours')).toHaveText(hours);await expect(page.locator('#outage')).toHaveValue('8');
 await page.getByRole('button',{name:'Reset',exact:true}).click();await expect(page.locator('#capacity')).toHaveValue('100');await expect(page.locator('#outage')).toHaveValue('0');
 expect(errors).toEqual([]);
});
test('alternate corridor, invalid target, mobile and model notice',async({page})=>{
 await page.setViewportSize({width:390,height:844});await page.goto('/ROUTE/?corridor=1&demand_pct=150&outage_hours=24&reserve_pct=0&adjacent_pct=0');
 await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#scenario-name')).toHaveText('Urban bottleneck example');
 await expect(page.locator('#retention')).toHaveText('100 → 0%');
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 await page.locator('#target').fill('0');await expect(page.locator('#status')).toContainText('between 1 and 48');await expect(page.locator('#download')).toBeDisabled();
 await page.locator('#target').fill('8');await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#download')).toBeEnabled();
 await page.getByText('What this model can tell you').click();await expect(page.getByText(/hold and narrow/)).toBeVisible();
 const response=await page.request.get('/ROUTE/LICENSE');expect(response.status()).toBe(200);expect(await response.text()).toContain('NonCommercial');
});
test('invalid shared scenarios fall back safely and broken worker disables export',async({page})=>{
 await page.goto('/ROUTE/?capacity_pct=0&corridor=99');await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#capacity')).toHaveValue('100');
 await page.route('**/pkg/route_web_bg.wasm',route=>route.abort());await page.reload();await expect(page.locator('#status')).toContainText(/could not load|Failed|fetch/i);await expect(page.locator('#download')).toBeDisabled();
});

test('whole trip comparison uses WASM, reconciles totals and survives sharing',async({page})=>{
 await page.goto('/ROUTE/?corridor=2&demand_pct=200&capacity_pct=25');
 await expect(page.locator('#status')).toContainText('Ready');
 await expect(page.locator('#trips tr')).toHaveCount(3);
 await expect(page.locator('#trips tr').first()).toContainText('10 h');
 await expect(page.locator('.timebar')).toHaveCount(3);
 const rows=await page.locator('#trips').textContent();
 await page.reload();await expect(page.locator('#status')).toContainText('Ready');
 await expect(page.locator('#trips')).toHaveText(rows);
 await page.setViewportSize({width:390,height:844});
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
});

test('trip breakdown widths represent hours and omit zero durations',async({page})=>{
 await page.goto('/ROUTE/');await expect(page.locator('#status')).toContainText('Ready');
 const bars=await page.locator('.timebar').evaluateAll(bars=>bars.map(bar=>Array.from(bar.children).map(part=>({hours:Number(part.dataset.hours),width:part.getBoundingClientRect().width}))));
 for(const parts of bars){const totalHours=parts.reduce((a,p)=>a+p.hours,0),totalWidth=parts.reduce((a,p)=>a+p.width,0);for(const part of parts){expect(part.hours).toBeGreaterThan(0);expect(Math.abs(part.width/totalWidth-part.hours/totalHours)).toBeLessThan(0.003);}}
});
