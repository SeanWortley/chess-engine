use crate::results::MatchResults;
use crate::results::SprtResult;
pub struct MatchAnalysis {
    pub results: MatchResults,
    pub elo_diff: f64,
    pub elo_error: f64,
    pub los: f64,
    pub draw_ratio: f64,
    pub sprt: SprtResult,
    pub llr: f64,
}
