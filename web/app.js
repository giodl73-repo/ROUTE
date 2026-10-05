const $=id=>document.getElementById(id);
const fields={corridor:'corridor',demand_pct:'demand',capacity_pct:'capacity',outage_hours:'outage',reserve_pct:'reserve',adjacent_pct:'adjacent',target_hours:'target'};
const defaults={corridor:0,demand_pct:100,capacity_pct:100,outage_hours:0,reserve_pct:15,adjacent_pct:35,target_hours:8};
let latest=0,ready=false,result=null,timer;
const worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});
const number=(value,digits=1)=>value.toLocaleString(undefined,{maximumFractionDigits:digits});
function status(text,error=false){$('status').textContent=text;$('status').classList.toggle('error',error);}
function disable(){result=null;$('share').disabled=true;$('download').disabled=true;}
function input(){return Object.fromEntries(Object.entries(fields).map(([key,id])=>[key,Number($(id).value)]));}
function labels(){for(const [key,id] of Object.entries(fields)){const output=$(id+'-value');if(output)output.textContent=$(id).value+(key==='outage_hours'?' h':'%');}}
function compute(){clearTimeout(timer);labels();disable();latest++;if(!ready)return;
 if(!$('controls').reportValidity()){status('Enter a travel target between 1 and 48 hours.',true);return;}
 status('Computing scenario…');worker.postMessage({type:'evaluate',id:latest,input:input()});
}
function render(data,ms){result=data;const b=data.baseline,s=data.scenario;
 $('scenario-name').textContent=data.corridor_name;
 $('hours').textContent=number(b.hours)+' → '+number(s.hours)+' h';
 $('hours-detail').textContent=number(s.delay_hours)+' h above free flow; baseline '+number(b.delay_hours)+' h';
 $('gap').textContent=number(b.target_gap_hours)+' → '+number(s.target_gap_hours)+' h';
 $('gap-detail').textContent='Above the '+number(data.input.target_hours)+' h driving-only target';
 $('missed').textContent=number(b.missed_swaps,0)+' → '+number(s.missed_swaps,0);
 $('missed-detail').textContent=number(s.affected_swaps,0)+' of '+number(s.daily_swaps,0)+' daily swaps affected at Relay A';
 $('retention').textContent=number(b.affected_retention*100)+' → '+number(s.affected_retention*100)+'%';
 $('retention-detail').textContent=s.affected_swaps===0?'No outage: no swaps affected':'Retention among outage-affected swaps only';
 $('legs').replaceChildren(...s.legs.map(leg=>{const row=document.createElement('tr');for(const v of [leg.name,number(leg.miles,0),number(leg.flow_vph,0),number(leg.capacity_vph,0),number(leg.vc_ratio,2),number(leg.hours,2)]){const cell=document.createElement('td');cell.textContent=v;row.append(cell);}return row;}));
 $('trips').replaceChildren(...s.trips.map(trip=>{const row=document.createElement('tr');for(const v of [trip.mode,number(trip.typical.elapsed_hours)+' h',number(trip.delayed.elapsed_hours)+' h',number(trip.typical.driving_hours)+' h',number(trip.typical.rest_swap_hours)+' h',number(trip.typical.delay_hours)+' h',number(trip.typical.fixed_overhead_hours)+' h']){const cell=document.createElement('td');cell.textContent=v;row.append(cell);}return row;}));
 $('timelines').replaceChildren(...s.trips.map(trip=>{const block=document.createElement('div');block.className='timeline';const title=document.createElement('p');title.textContent=trip.mode;block.append(title);const bar=document.createElement('div');bar.className='timebar';for(const [key,label] of [['driving_hours','Driving'],['rest_swap_hours','Rest / swaps'],['delay_hours','Incidents'],['fixed_overhead_hours','Other stops']]){if(trip.typical[key]===0)continue;const part=document.createElement('span');part.dataset.hours=trip.typical[key];part.style.flex=trip.typical[key]+' 1 0';part.title=label+': '+number(trip.typical[key])+' h';part.textContent=label;bar.append(part);}block.append(bar);return block;}));
 $('trip-note').textContent='Invented incident assumptions; simplified rest and staffing rules. Relay assumes fresh drivers at both hubs. Hub outage is separate and does not change these arrival times. Each timeline shows one median trip, not a chronological stop schedule.';
 $('staffing').textContent='Relay A staffing proxy: '+number(b.freight_drivers,0)+' baseline → '+number(s.freight_drivers,0)+' scenario freight drivers. Staffing depends on swaps, not road capacity.';
 $('share').disabled=false;$('download').disabled=false;status('Ready · Rust calculation '+number(ms,2)+' ms');
}
worker.onmessage=({data})=>{if(data.type==='ready'){ready=true;compute();}else if(data.type==='error'&&data.id===undefined){ready=false;disable();status('The Rust engine could not load: '+data.message,true);}else if(data.id===latest){if(data.type==='result')render(data.result,data.ms);else status(data.message,true);}};
worker.onerror=()=>{ready=false;disable();status('The Rust engine could not load. Reload to try again.',true);};
$('controls').addEventListener('submit',e=>e.preventDefault());
$('controls').addEventListener('input',()=>{disable();latest++;labels();clearTimeout(timer);timer=setTimeout(compute,100);});
$('controls').addEventListener('reset',()=>{setTimeout(()=>{for(const[key,id]of Object.entries(fields))$(id).value=defaults[key];history.replaceState(null,'',location.pathname);compute();},0);});
$('share').addEventListener('click',async()=>{if(!result)return;const url=new URL(location.href);url.search='';url.hash='';for(const[key,value]of Object.entries(result.input))url.searchParams.set(key,String(value));history.replaceState(null,'',url);try{await navigator.clipboard.writeText(url.href);status('Scenario link copied.');}catch{status('Scenario link is in the address bar; copy it from there.');}});
$('download').addEventListener('click',()=>{if(!result)return;const url=URL.createObjectURL(new Blob([JSON.stringify(result,null,2)+'\n'],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download='route-scenario.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
const params=new URLSearchParams(location.search);
let invalidLink=false;
for(const[key,id]of Object.entries(fields)){if(params.has(key)){const raw=params.get(key);const value=Number(raw);const el=$(id);const min=Number(el.min||0),max=Number(el.max||(id==='corridor'?2:Infinity));if(!raw.trim()||!Number.isFinite(value)||value<min||value>max||(id==='corridor'&&!Number.isInteger(value))){invalidLink=true;break;}el.value=value;}}
if(invalidLink){for(const[key,id]of Object.entries(fields))$(id).value=defaults[key];status('Invalid scenario link. Loading the default scenario.',true);}
labels();worker.postMessage({type:'init'});
