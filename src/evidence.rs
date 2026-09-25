use crate::identity::InputContext;

/// Provenance of an observation relative to the underlying source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationOrigin {
    /// The source/backend reported ordinary source evidence.
    SourceReport,
    /// The backend synthesized evidence to reconcile previously retained state.
    BackendSyntheticReconciliation,
}

/// Certainty class for one observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    /// Confirmed observed evidence.
    ObservedConfirmed,
    /// Revisable estimate that must not silently become confirmed state.
    EstimatedRevisable,
    /// Provisional prediction that must not mutate confirmed state.
    PredictedProvisional,
}

/// Delivery role independent from observation certainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryRole {
    /// The ordinary current sample for the source stream.
    OrdinaryCurrent,
    /// An earlier confirmed sample delivered later or in a coalesced batch.
    HistoricalCoalesced,
}

/// Unit/domain of a source-provided timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceTimeUnit {
    /// Microseconds in the source clock domain.
    Microseconds,
    /// Milliseconds in the source clock domain.
    Milliseconds,
    /// Nanoseconds in the source clock domain.
    Nanoseconds,
    /// Source-native ticks with an explicit positive frequency.
    NativeTicks {
        /// Number of source ticks per second.
        ticks_per_second: u64,
    },
}

/// Source-provided time associated with one input context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceTime {
    /// Source/device context whose clock produced this value.
    pub context: InputContext,
    /// Timestamp value in the declared unit.
    pub value: u64,
    /// Unit/domain for the timestamp value.
    pub unit: SourceTimeUnit,
}

impl SourceTime {
    /// Creates source-time evidence in the supplied input context and unit.
    pub const fn new(context: InputContext, value: u64, unit: SourceTimeUnit) -> Self {
        Self {
            context,
            value,
            unit,
        }
    }
}
