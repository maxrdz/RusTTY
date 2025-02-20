/*
    This file is part of 'rustty'.

    Copyright (c) 2025 Max Rodriguez <me@maxrdz.com>

    'RusTTY' is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    'RusTTY' is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::fl;

#[derive(Debug, Default)]
pub enum TimeFormat {
    #[default]
    TwelveHour,
    TwentyFourHour,
}

impl std::fmt::Display for TimeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwelveHour => writeln!(f, "{}", fl!("twelve-hour-format")),
            Self::TwentyFourHour => writeln!(f, "{}", fl!("twenty-four-hour-format")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Percentage {
    value: u8,
}

impl From<u8> for Percentage {
    fn from(value: u8) -> Self {
        let mut new = Self::default();
        new.set_percent(value).unwrap();
        new
    }
}

impl Percentage {
    fn set_percent(&mut self, value: u8) -> Result<(), ()> {
        if value > 100 {
            Err(())
        } else {
            self.value = value;
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
pub enum TemperatureUnit {
    #[default]
    Celsius,
    Fahrenheit,
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Celsius => writeln!(f, "{}", fl!("celsius")),
            Self::Fahrenheit => writeln!(f, "{}", fl!("fahrenheit")),
        }
    }
}

#[derive(Debug)]
pub struct BatteryNotifyConfig {
    pub notify_enabled: bool,
    pub battery_critical_percent: Percentage,
    pub battery_low_percent: Percentage,
}

impl Default for BatteryNotifyConfig {
    fn default() -> Self {
        Self {
            notify_enabled: true,
            battery_critical_percent: 10.into(),
            battery_low_percent: 20.into(),
        }
    }
}

#[derive(Debug, Default)]
pub enum PowerProfile {
    Battery,
    #[default]
    Balanced,
    Performance,
}

impl std::fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Battery => writeln!(f, "{}", fl!("battery")),
            Self::Balanced => writeln!(f, "{}", fl!("balanced")),
            Self::Performance => writeln!(f, "{}", fl!("performance")),
        }
    }
}

#[derive(Debug)]
pub struct SensorsNotifyConfig {
    pub notify_enabled: bool,
    /// Temperature in Celsius
    pub cpu_critical_temp: i16,
    /// Temperature in Celsius
    pub cpu_warning_temp: i16,
}

impl Default for SensorsNotifyConfig {
    fn default() -> Self {
        Self {
            notify_enabled: true,
            cpu_critical_temp: 100,
            cpu_warning_temp: 80,
        }
    }
}

#[derive(Debug)]
pub struct DashboardConfig {
    // time
    pub time_format: TimeFormat,
    pub display_seconds: bool,
    pub display_weekday: bool,
    // audio
    pub volume_increments: Percentage,
    // battery
    pub hide_battery: bool,
    pub show_battery_percentage: bool,
    pub battery_notify_config: BatteryNotifyConfig,
    pub screen_brightness: Percentage,
    pub screen_brightness_increments: Percentage,
    // network
    pub hide_network: bool,
    // sensors
    pub hide_cpu_temp: bool,
    pub cpu_temp_unit: TemperatureUnit,
    pub cpu_temp_notify_config: SensorsNotifyConfig,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            // time
            time_format: TimeFormat::default(),
            display_seconds: true,
            display_weekday: true,
            // audio
            volume_increments: 5.into(),
            // battery
            hide_battery: false,
            show_battery_percentage: true,
            battery_notify_config: BatteryNotifyConfig::default(),
            screen_brightness: 50.into(),
            screen_brightness_increments: 5.into(),
            // network
            hide_network: false,
            // sensors
            hide_cpu_temp: false,
            cpu_temp_unit: TemperatureUnit::default(),
            cpu_temp_notify_config: SensorsNotifyConfig::default(),
        }
    }
}
