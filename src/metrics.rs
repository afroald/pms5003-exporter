use crate::pms5003::Pms5003Frame;
use prometheus_client::{encoding::text::encode, metrics::gauge::Gauge, registry::Registry};
use std::time::{Duration, Instant};

pub const METRICS_TTL: Duration = Duration::from_millis(10_000);

pub struct Metrics {
    registry: Registry,
    pub last_update: Instant,

    pm10: Gauge,
    pm25: Gauge,
    pm100: Gauge,
    pm10_atmos: Gauge,
    pm25_atmos: Gauge,
    pm100_atmos: Gauge,
    pm03_count: Gauge,
    pm05_count: Gauge,
    pm10_count: Gauge,
    pm25_count: Gauge,
    pm50_count: Gauge,
    pm100_count: Gauge,
}

impl Metrics {
    pub fn new() -> Self {
        let mut registry = <Registry>::with_prefix("pm");

        let pm10: Gauge = Default::default();
        registry.register(
            "pm10",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm10.clone(),
        );

        let pm25: Gauge = Default::default();
        registry.register(
            "pm25",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm25.clone(),
        );

        let pm100: Gauge = Default::default();
        registry.register(
            "pm100",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm100.clone(),
        );

        let pm10_atmos: Gauge = Default::default();
        registry.register(
            "pm10_atmos",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm10_atmos.clone(),
        );

        let pm25_atmos: Gauge = Default::default();
        registry.register(
            "pm25_atmos",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm25_atmos.clone(),
        );

        let pm100_atmos: Gauge = Default::default();
        registry.register(
            "pm100_atmos",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm100_atmos.clone(),
        );

        let pm03_count: Gauge = Default::default();
        registry.register(
            "pm03_count",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm03_count.clone(),
        );

        let pm05_count: Gauge = Default::default();
        registry.register(
            "pm05_count",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm05_count.clone(),
        );

        let pm10_count: Gauge = Default::default();
        registry.register(
            "pm10_count",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm10_count.clone(),
        );

        let pm25_count: Gauge = Default::default();
        registry.register(
            "pm25_count",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm25_count.clone(),
        );

        let pm50_count: Gauge = Default::default();
        registry.register(
            "pm50_count",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm50_count.clone(),
        );

        let pm100_count: Gauge = Default::default();
        registry.register(
            "pm100_count",
            "PM1.0 concentration in µg/m³, corrected for standard atmosphere",
            pm100_count.clone(),
        );

        Self {
            registry,
            pm10,
            pm25,
            pm100,
            pm10_atmos,
            pm25_atmos,
            pm100_atmos,
            pm03_count,
            pm05_count,
            pm10_count,
            pm25_count,
            pm50_count,
            pm100_count,
            last_update: Instant::now() - METRICS_TTL,
        }
    }

    pub fn update(&mut self, frame: &Pms5003Frame) {
        self.pm10.set(frame.pm10.into());
        self.pm25.set(frame.pm25.into());
        self.pm100.set(frame.pm100.into());
        self.pm10_atmos.set(frame.pm10_atmos.into());
        self.pm25_atmos.set(frame.pm25_atmos.into());
        self.pm100_atmos.set(frame.pm100_atmos.into());
        self.pm03_count.set(frame.pm03_count.into());
        self.pm05_count.set(frame.pm05_count.into());
        self.pm10_count.set(frame.pm10_count.into());
        self.pm25_count.set(frame.pm25_count.into());
        self.pm50_count.set(frame.pm50_count.into());
        self.pm100_count.set(frame.pm100_count.into());
        self.last_update = Instant::now();
    }

    pub fn encode(&self) -> Result<String, std::fmt::Error> {
        let mut buffer = String::new();
        encode(&mut buffer, &self.registry)?;
        Ok(buffer)
    }
}
