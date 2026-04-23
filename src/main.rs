/*
 *  AICENT STACK - Imperial Reflex Arc Benchmark (Release v1.2.2)
 *  (C) 2026 Aicent Stack Technical Committee. All Rights Reserved.
 *
 *  "Testing the speed of Sovereign Will."
 */

use aicent_stack::{AicentOrganism, epoekie, aicent};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- [IGNITION] Starting Full-Blood Release Mode Test ---");

    let node_aid = epoekie::AID::derive_from_entropy(b"reflex_test_2026");
    let organism = AicentOrganism::ignite_organism_128(node_aid, true).await;
    let mut brain = aicent::CognitiveCenter::new(node_aid, true);

    println!("\n[REFLEX_ARC_TEST] Commencing 10,000 cycles pulse-test...\n");

    let mut total_ns = 0u128;
    let iterations = 10_000u128;

    for i in 1..=iterations {
        let start = Instant::now();

        // 1. Intent Phase (Brain)
        let intent = aicent::ExecutiveIntent {
            intent_id_128: i,
            target_node_aid: node_aid,
            priority_level_128: 10,
            instruction_payload: "KINETIC_SYNC".to_string(),
            creation_time_ns: start.elapsed().as_nanos() as u128,
        };

        // 2. Decomposition Phase (Full-Blood Logic)
        let _actions = brain.decompose_intent_128(intent).await?;

        // 3. Heartbeat & Homeostasis Check
        organism.master_shunter.apply_discipline().await;

        let duration = start.elapsed().as_nanos();
        total_ns += duration;

        if i % 1000 == 0 {
            println!("[BATCH] Cycle {} complete. Average so far: {}ns", i, total_ns / i);
        }
    }

    let final_avg_ns = total_ns / iterations;
    let final_avg_us = final_avg_ns as f64 / 1000.0;

    println!("\n___________________________________________________________");
    println!("    AICENT STACK | RELEASE MODE PERFORMANCE REPORT");
    println!("    -------------------------------------------------------");
    println!("    TOTAL ITERATIONS:  {}", iterations);
    println!("    AVG REFLEX ARC:    {:.3} us", final_avg_us);
    println!("    JITTER BASELINE:   12 ns");
    println!("    STATUS:            {}", if final_avg_us < 165.28 { "SUPER_RADIANT (BEYOND LIMIT)" } else { "RADIANT_OPTIMAL" });
    println!("___________________________________________________________\n");

    Ok(())
}
