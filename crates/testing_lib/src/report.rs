use crate::benchmark::{BenchmarkKind, analysis::BenchmarkAnalysisRow};
pub fn print_report(analysis: Vec<BenchmarkAnalysisRow>) {
    println!("Baseline\tvs\tCandidate");
    for row in analysis {
        match row.kind {
            BenchmarkKind::PerftSpeed => {
                println!("=============================");
                println!("Perft Speed (Higher is better)");
                println!("=============================");
                println!("Baseline: {} nodes/second", row.baseline_value);
                println!("Candidate: {} nodes/second", row.candidate_value);
                if row.delta > 0 {
                    println!("Delta: +{}\t+{:.2}%", row.delta, row.delta_percentage);
                } else {
                    println!("Delta: {}\t{:.2}%", row.delta, row.delta_percentage);
                }
                println!("STATUS: {}", row.status.status());
            }
            BenchmarkKind::NodeCount => {
                println!("=============================");
                println!("Node Count (Lower is better)");
                println!("=============================");
                println!(
                    "Baseline: {} nodes, {} ms",
                    row.baseline_value,
                    row.baseline_time.unwrap_or(0)
                );
                println!(
                    "Candidate: {} nodes, {} ms",
                    row.candidate_value,
                    row.candidate_time.unwrap_or(0)
                );
                if row.delta > 0 {
                    println!("Delta: +{} nodes\t+{:.2}%", row.delta, row.delta_percentage);
                } else {
                    println!("Delta: {} nodes\t{:.2}%", row.delta, row.delta_percentage);
                }
                if let (Some(dt), Some(dt_pct)) = (row.delta_time, row.delta_time_percentage) {
                    if dt > 0 {
                        println!("Time Delta: +{} ms\t+{:.2}%", dt, dt_pct);
                    } else {
                        println!("Time Delta: {} ms\t{:.2}%", dt, dt_pct);
                    }
                }
                println!("STATUS: {}", row.status.status());
            }
            BenchmarkKind::MovesPerSecond => {}
        }
    }
}
