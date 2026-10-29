"""Explicitly freeze existing ROUTE cache metadata; never acquire sources."""
import argparse,datetime,hashlib,importlib.util,json,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def blob(repo,path):return subprocess.check_output(['git','-C',str(repo),'show','HEAD:'+path])
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--cache-root',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args();cache=a.cache_root.resolve();dest=a.output.resolve()
 if not dest.is_relative_to(ROOT/'data') or dest.exists():raise ValueError('Choose a new output file inside this checkout data/; existing captures are immutable')
 contract='data/i80-report-source-contract.csv';script='tools/prepare_i80_report_sources.py';packet='docs/packets/i80-flagship-review-packet.md'
 for file in [contract,script,packet]:
  if (cache/file).read_bytes().replace(b'\r\n',b'\n')!=blob(cache,file):raise ValueError('Tracked capture input differs from committed bytes: '+file)
 spec=importlib.util.spec_from_file_location('recorded_readiness',cache/script);m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
 contracts=m.load_contract();observations=m.readiness_rows(contracts,{});rows=[{'contract':c,'observation':o} for c,o in zip(contracts,observations,strict=True)]
 hpms=next(c for c in contracts if c['source_id']=='SRC-I80-HPMS');assert hpms['current_source_year']=='2018' and b'HPMS 2023' in blob(cache,packet)
 d={'schema':'route.i80-recorded-cache-checklist.v1','recorded_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'origin_commit':subprocess.check_output(['git','-C',str(cache),'rev-parse','HEAD'],text=True).strip(),'capture_mode':'existing local cache inspection; attempts={} through unchanged native Python readiness_rows','acquisition_performed':False,'primary_hashes_reverified':False,'raw_primary_bytes_in_bundle':False,'contract_sha256':hashlib.sha256(blob(cache,contract)).hexdigest(),'preparation_script_sha256':hashlib.sha256(blob(cache,script)).hexdigest(),'reviewed_packet_sha256':hashlib.sha256(blob(cache,packet)).hexdigest(),'vintage_discrepancy':{'packet_hpms_label':'HPMS 2023','contract_hpms_year':'2018','status':'unresolved','packet_values_relinked':False},'rows':rows}
 dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes((json.dumps(d,indent=2)+'\n').encode('utf-8'));print(json.dumps({s:sum(o['readiness_status']==s for o in observations) for s in ['ready','blocked','excluded']}))
if __name__=='__main__':main()
