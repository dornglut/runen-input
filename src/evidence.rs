use crate::identity::InputContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationOrigin {
    SourceReport,
    BackendSyntheticReconciliation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    ObservedConfirmed,
    EstimatedRevisable,
    PredictedProvisional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryRole {
    OrdinaryCurrent,
    HistoricalCoalesced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceTimeUnit {
    Microseconds,
    Milliseconds,
    Nanoseconds,
    NativeTicks { ticks_per_second: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceTime {
    pub context: InputContext,
    pub value: u64,
    pub unit: SourceTimeUnit,
}

impl SourceTime {
    pub const fn new(context: InputContext, value: u64, unit: SourceTimeUnit) -> Self {
        Self {
            context,
            value,
            unit,
        }
    }
}

