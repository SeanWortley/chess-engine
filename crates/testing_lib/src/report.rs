use crate::benchmark::{BenchmarkKind, analysis::BenchmarkAnalysisRow};
pub fn print_report(analysis: Vec<BenchmarkAnalysisRow>) {
    println!("Baseline\tvs\tCandidate");
    for row in analysis {
        match row.kind {
            BenchmarkKind::PerftSpeed => {
                println!("=============================");
                println!("Perft Speed");
                println!("=============================");
                println!("Baseline: {} nodes/second", row.baseline_value);
                println!("Candidate: {} nodes/second", row.candidate_value);
                if row.delta > 0 {
                    println!("Delta: +{}\t+{:.2}%", row.delta, row.delta_percentage);
                } else {
                    println!("Delta: {}\t{:.2}%", row.delta, row.delta_percentage);
                }
            }
            BenchmarkKind::NodesEvaluated => {}
            BenchmarkKind::MovesPerSecond => {}
        }
    }
}
