/*
 *  AICENT STACK - v1.2.5-ALPHA TOTALITY SMOKE TEST
 */

use aicent_stack::{AicentOrganism, epoekie, SovereignOrchestration};
// Directly import the trait to ensure the metabolic pulse method is in scope.
use epoekie::SovereignLifeform; 
use std::time::Instant;

#[tokio::test]
async fn verify_imperial_totality_v125() {
    println!("\n--- [CI_IGNITION] Initiating 17-Pillar Suture Audit ---");

    let seed = b"imperial_genesis_node_2026_ignite_v125";
    let node_aid = epoekie::AID::derive_from_entropy(seed);
    
    let is_radiant = true;
    let mut organism = AicentOrganism::ignite_organism_128(node_aid, is_radiant).await;

    let start = Instant::now();
    organism.diagnostic_eye.update_imperial_vision_128(0.9999, 0.9999, 12).await;
    
    // Now accessible due to SovereignLifeform being in scope.
    organism.execute_metabolic_pulse();

    let reflex_ns = start.elapsed().as_nanos();
    
    assert!(organism.verify_total_suture_integrity());
    println!("    AICENT STACK v1.2.5-ALPHA: GREEN LIGHT CONFIRMED");
    println!("    VERIFIED REFLEX: {} ns", reflex_ns);
}
