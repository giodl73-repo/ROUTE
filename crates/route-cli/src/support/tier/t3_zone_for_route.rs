//! Helper `t3_zone_for_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_for_route(route: &str) -> Option<(&'static str, &'static str)> {
    let key = canonical_route_key(route);
    let zone_id = match key.as_str() {
        "I71" | "I72" | "I74" | "I75" | "I93" | "I96" | "I115" | "I129" | "I176" | "I180"
        | "I190" | "I196" | "I220" | "I235" | "I264" | "I270" | "I271" | "I275" | "I276"
        | "I279" | "I280" | "I294" | "I390" | "I465" | "I471" | "I478" | "I480" | "I496"
        | "I675" | "I691" | "I696" | "I990" | "US7" | "US10" | "US15" | "US22" | "US31"
        | "US35" | "US40" | "US41" | "US42" | "US74" | "US75" | "US223" | "US224" | "US250" => {
            "t3-great-lakes"
        }
        "I16" | "I22" | "I24" | "I37" | "I57" | "I59" | "I65" | "I85" | "I140" | "I175"
        | "I185" | "I464" | "I795" | "US17" | "US45E" | "US45W" | "US74E" | "US80" | "US82"
        | "US84" | "US90Z" | "US119" | "US278" | "US301" => "t3-southeast",
        "I2" | "I10" | "I19" | "I37W" | "I45" | "I69E" | "I110" | "I410" | "I510" | "I610"
        | "US69" | "US77" | "US83" | "US90" | "US96" | "US281" => "t3-texas-border",
        "I8" | "I15" | "I25" | "I70" | "I80" | "I135" | "I205" | "I215" | "I225" | "I335"
        | "I680" | "I705" | "I880" | "US2" | "US6" | "US14" | "US26" | "US76" | "US87" | "US95"
        | "US287" => "t3-mountain-west",
        "I30" | "I40" | "I44" | "I49" | "I55" | "I169" | "I181" | "I240" | "I255" | "I277"
        | "I295" | "I630" | "I635" | "I664" | "I759" | "I840" | "US24" | "US66" | "US69S"
        | "US70" | "US71" | "US167" | "US270" | "US421" => "t3-mid-south",
        _ => return None,
    };
    t3_zone_catalog_entry(zone_id)
}
