use crate::{
    BenchmarkKind,
    results::{BenchmarkComparisonResult, BenchmarkResult},
};

pub struct BenchmarkAnalysisRow {
    pub kind: BenchmarkKind,
    pub baseline_value: u64,
    pub candidate_value: u64,
    pub delta: i64,
    pub delta_percentage: f64,
    pub status: ComparisonStatus,
}

pub enum ComparisonStatus {
    Improved,
    Regressed,
    Unchanged,
}

pub fn analyze_comparison_results(
    results: Vec<BenchmarkComparisonResult>,
) -> Vec<BenchmarkAnalysisRow> {
    let mut rows: Vec<BenchmarkAnalysisRow> = Vec::new();

    for result in results {
        match result.baseline {
            BenchmarkResult::PerftSpeed {
                nodes: _,
                duration_ms: _,
                nps: baseline_nps,
            } => match result.candidate {
                BenchmarkResult::PerftSpeed {
                    nodes: _,
                    duration_ms: _,
                    nps: candidate_nps,
                } => {
                    let delta = candidate_nps as i64 - baseline_nps as i64;
                    let delta_percentage = delta as f64 / baseline_nps as f64;

                    let status = {
                        if delta_percentage > 5 as f64 {
                            ComparisonStatus::Improved
                        } else if delta_percentage < -5 as f64 {
                            ComparisonStatus::Regressed
                        } else {
                            ComparisonStatus::Unchanged
                        }
                    };

                    let row = BenchmarkAnalysisRow {
                        kind: BenchmarkKind::PerftSpeed,
                        baseline_value: baseline_nps,
                        candidate_value: candidate_nps,
                        delta,
                        delta_percentage,
                        status,
                    };
                    rows.push(row);
                }
                _ => {}
            },
            _ => {}
        }
    }
    rows
}
