import init,{inspect_source_checklist_json} from './pkg/route_web.js';
self.onmessage=async({data})=>{try{if(data.type==='init'){await init();self.postMessage({type:'ready'});}else if(data.type==='inspect'){self.postMessage({type:'result',id:data.id,canonical:inspect_source_checklist_json(data.query)});}}catch(e){self.postMessage({type:'error',id:data.id,message:String(e)});}};
