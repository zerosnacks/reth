use reth_evm::metrics::ExecutorMetrics;
use reth_metrics::{
    metrics::{Counter, Gauge, Histogram},
    Metrics,
};

/// Metrics for the `EngineApi`.
#[derive(Debug, Default)]
pub(crate) struct EngineApiMetrics {
    /// Engine API-specific metrics.
    pub(crate) engine: EngineMetrics,
    /// Block executor metrics.
    pub(crate) executor: ExecutorMetrics,
    /// Metrics for block validation
    pub(crate) block_validation: BlockValidationMetrics,
}

/// Metrics for the `EngineApi`.
#[derive(Metrics)]
#[metrics(scope = "consensus.engine.beacon")]
pub(crate) struct EngineMetrics {
    /// How many executed blocks are currently stored.
    pub(crate) executed_blocks: Gauge,
    /// The number of times the pipeline was run.
    pub(crate) pipeline_runs: Counter,
    /// The total count of forkchoice updated messages received.
    pub(crate) forkchoice_updated_messages: Counter,
    /// The total count of new payload messages received.
    pub(crate) new_payload_messages: Counter,
    /// Histogram of persistence operation durations (in seconds)
    pub(crate) persistence_duration: Histogram,
    // TODO add latency metrics
}

/// Metrics for non-execution related block validation.
#[derive(Metrics)]
#[metrics(scope = "sync.block_validation")]
pub(crate) struct BlockValidationMetrics {
    /// Histogram of state root duration
    pub(crate) state_root_histogram: Histogram,
    /// Latest state root duration
    pub(crate) state_root_duration: Gauge,
}

impl BlockValidationMetrics {
    /// Records a new state root time, updating both the histogram and state root gauge
    pub(crate) fn record_state_root(&self, elapsed_as_secs: f64) {
        self.state_root_duration.set(elapsed_as_secs);
        self.state_root_histogram.record(elapsed_as_secs);
    }
}
