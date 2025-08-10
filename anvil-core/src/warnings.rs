use std::time;

use chrono::{TimeZone, Utc};

#[derive(PartialEq, Clone, Copy)]
pub enum TornadoStatus {
    RadarIndicated,
    Observed,
}

#[derive(PartialEq, Clone, Copy)]
pub struct TornadoWarning {
    pub tornado_status: TornadoStatus,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SevereThunderStormDamageThreat {
    Base,
    Considerable,
    Destructive,
}

#[derive(PartialEq, Clone, Copy)]
pub struct SevereThunderStormWarning {
    pub damage_threat: SevereThunderStormDamageThreat
}

#[derive(PartialEq, Clone, Copy)]
pub enum WarningType {
    SevereThunderStorm(SevereThunderStormWarning),
    Tornado(TornadoWarning),
    TornadoPDS(TornadoWarning),
    TornadoEmergency(TornadoWarning),
}

#[derive(PartialEq, Clone, Copy)]
pub struct WarningPolygon {}

#[derive(PartialEq, Clone, Copy)]
pub struct Warning {
    pub warning_type: WarningType,
    pub polygon: WarningPolygon,
    pub valid_until: chrono::DateTime<Utc>,
}
