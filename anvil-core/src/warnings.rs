use chrono::Utc;

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
    pub damage_threat: SevereThunderStormDamageThreat,
}

#[derive(PartialEq, Clone, Copy)]
pub enum WarningType {
    SevereThunderStorm(SevereThunderStormWarning),
    Tornado(TornadoWarning),
    TornadoPDS(TornadoWarning),
    TornadoEmergency(TornadoWarning),
}

pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    LightBlue,
    Blue,
    Magenta,
    Purple,
    Black,
    White,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Red => "#FF0000",
            Color::Orange => "#FF6700",
            Color::Yellow => "#FFFF00",
            Color::Green => "#00FF00",
            Color::LightBlue => "#00ffea",
            Color::Blue => "#0000FF",
            Color::Magenta => "#FF00FF",
            Color::Purple => "#7a007a",
            Color::Black => "#000000",
            Color::White => "#FFFFFF",
        }
        .to_string()
    }
}

pub struct WarningColor {
    pub inner: Color,
    pub outer: Color,
}

impl WarningType {
    pub fn get_color(&self) -> WarningColor {
        match self {
            Self::Tornado(TornadoWarning {
                tornado_status: TornadoStatus::Observed,
            }) => WarningColor {
                inner: Color::Black,
                outer: Color::Red,
            },
            Self::TornadoEmergency(TornadoWarning {
                tornado_status: TornadoStatus::Observed,
            }) => WarningColor {
                inner: Color::Black,
                outer: Color::Purple,
            },
            Self::TornadoPDS(TornadoWarning {
                tornado_status: TornadoStatus::Observed,
            }) => WarningColor {
                inner: Color::Black,
                outer: Color::Magenta,
            },
            Self::SevereThunderStorm(SevereThunderStormWarning {
                damage_threat: SevereThunderStormDamageThreat::Destructive,
            }) => WarningColor {
                inner: Color::Red,
                outer: Color::Yellow,
            },
            _ => WarningColor {
                inner: Color::Magenta,
                outer: Color::Magenta,
            },
        }
    }
}

pub type WarningPolygonPoint = (f32, f32);

#[derive(PartialEq, Clone)]
pub struct WarningPolygon {
    pub points: Vec<WarningPolygonPoint>,
}

#[derive(PartialEq, Clone)]
pub struct Warning {
    pub warning_type: WarningType,
    pub polygon: WarningPolygon,
    pub valid_until: chrono::DateTime<Utc>,
}
