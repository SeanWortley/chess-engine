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

impl ComparisonStatus {
    pub fn status(&self) -> &str {
        match self {
            Self::Improved => "Improved",
            Self::Regressed => "Regressed",
            Self::Unchanged => "Unchanged",
        }
    }
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
                    let delta_percentage = (delta as f64 * 100.0) / baseline_nps as f64;

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
            BenchmarkResult::NodesEvaluated {
                nodes: baseline_nodes,
                time_ms: baseline_time,
            } => match result.candidate {
                BenchmarkResult::NodesEvaluated {
                    nodes: candidate_nodes,
                    time_ms: candidate_time,
                } => {
                    let delta_nodes = candidate_nodes as i64 - baseline_nodes as i64;
                    let delta_nodes_percentage =
                        (delta_nodes as f64 * 100.0) / baseline_nodes as f64;

                    let delta_time = candidate_time as i64 - baseline_time as i64;
                    let delta_time_percentage = (delta_time as f64 * 100.0) / baseline_time as f64;

                    let status = {
                        if delta_nodes_percentage > 5 as f64 {
                            ComparisonStatus::Regressed
                        } else if delta_nodes_percentage < -5 as f64 {
                            ComparisonStatus::Improved
                        } else {
                            ComparisonStatus::Unchanged
                        }
                    };

                    let row = BenchmarkAnalysisRow {
                        kind: BenchmarkKind::NodeCount,
                        baseline_value: baseline_nodes,
                        candidate_value: candidate_nodes,
                        delta_nodes,
                        delta_nodes_percentage,
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
