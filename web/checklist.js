const $=id=>document.getElementById('checklist-'+id);
const worker=new Worker(new URL('./checklist-worker.js',import.meta.url),{type:'module'});
let ready=false,revision=0,canonical=null,watchdog;
function state(message,error=false){$('status').textContent=message;$('status').classList.toggle('error',error);}
function clear(){canonical=null;$('download').disabled=true;$('results').hidden=true;$('rows').replaceChildren();clearTimeout(watchdog);revision++;}
function fail(message){ready=false;clear();worker.terminate();state(message,true);}
function inspect(){clear();if(!ready)return;state('Inspecting recorded source entries…');worker.postMessage({type:'inspect',id:revision,query:$('query').value});watchdog=setTimeout(()=>fail('Source-checklist engine timed out. Reload to try again.'),15000);}
function field(list,label,value){const dt=document.createElement('dt'),dd=document.createElement('dd');dt.textContent=label;dd.textContent=value||'Not recorded';list.append(dt,dd);}
function row({contract:c,observation:o}){const article=document.createElement('article');article.className='source-entry';article.dataset.sourceId=c.source_id;
 const title=document.createElement('h3');title.textContent=c.source_id+' · recorded '+o.readiness_status;article.append(title);
 const dl=document.createElement('dl');
 for(const[label,value]of [['Artifact',c.artifact],['Publisher',c.publisher],['Contract source year (frozen label)',c.current_source_year],['Contract candidate year (frozen label)',c.candidate_source_year],['Contract acquisition / parser',c.acquisition_status+' / '+c.local_parser_status],['Contract credential mode',c.credential_mode],['Contract command (not executed here)',c.current_command],['Contract gap',c.blocking_gap],['Observed blocker',o.blocking_gap],['Recorded artifact evidence count (units in detail)',o.evidence_count],['Recorded artifact evidence detail',o.evidence_detail],['Acquisition attempt',o.attempt_status],['Next action',c.next_action]])field(dl,label,value);
 const dt=document.createElement('dt'),dd=document.createElement('dd'),a=document.createElement('a');dt.textContent='Publisher reference (mutable)';a.href=c.authoritative_url;a.textContent=c.authoritative_url;a.rel='noreferrer';dd.append(a);dl.append(dt,dd);article.append(dl);return article;
}
function render(raw){const d=JSON.parse(raw);canonical=raw;const s=d.recorded_summary;
 $('summary').textContent=s.source_entries+' recorded source entries: '+s.ready+' ready / '+s.blocked+' blocked / '+s.excluded+' excluded. Showing '+d.selected_source_entries+'. Evidence counts have unlike units and must not be added.';
 $('recording').textContent='Local observation: '+d.recording.recorded_at_utc+' · origin commit '+d.recording.origin_commit+'. No acquisition or primary-hash re-verification was performed. Raw primary bytes are absent. Contract years are not today’s latest source or endpoint-health claims.';
 $('rows').replaceChildren(...d.selected_rows.map(row));$('results').hidden=false;$('download').disabled=false;state('Ready · recorded checklist inspected by Rust');
}
worker.onmessage=({data})=>{if(data.type==='ready'){ready=true;inspect();return;}if(data.id===undefined){fail('Source-checklist engine could not load: '+data.message);return;}if(data.id!==revision)return;clearTimeout(watchdog);if(data.type==='result'){try{render(data.canonical);}catch{clear();state('Invalid source-checklist response.',true);}}else state('Invalid checklist query: '+data.message,true);};
worker.onerror=()=>fail('Source-checklist engine could not load. Reload to try again.');
$('controls').addEventListener('submit',e=>{e.preventDefault();inspect();});
$('query').addEventListener('input',()=>{clear();state('Query changed. Inspect recorded entries to continue.');});
$('filter').addEventListener('change',()=>{$('query').value=JSON.stringify({status:$('filter').value||null});inspect();});
$('reset').addEventListener('click',()=>{$('filter').value='';$('query').value='{"status":null}';inspect();});
$('download').addEventListener('click',()=>{if(!canonical)return;const url=URL.createObjectURL(new Blob([canonical+'\n'],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download='route-recorded-source-checklist.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
watchdog=setTimeout(()=>fail('Source-checklist engine timed out. Reload to try again.'),15000);worker.postMessage({type:'init'});
