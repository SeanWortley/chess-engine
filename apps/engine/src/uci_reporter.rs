use engine_lib::search::SearchReporter;

struct UciReporter;

impl SearchReporter for UciReporter {
    fn report_depth(
        &self,
        depth: u8,
        nodes: u64,
        time_ms: u64,
        score: i16,
        best_move: Option<Move>,
    ) {
        todo!();
    }
}
