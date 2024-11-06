// Copyright (C) 2024 OpenTier FZCO
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use chrono::Local;
use clap::Parser;
use log::{error, trace};
use slint::*;
use std::fmt::format as fmt_format;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use vehicle_dashboard::events::battery::BatteryData;
use vehicle_dashboard::events::exterior::Exterior;
use vehicle_dashboard::events::speed::Speed;
use vehicle_dashboard::events::state::{LockState, State};
use vehicle_dashboard::events::trip_data::TripData;
use vehicle_dashboard::led_manager::LedManager;
use vehicle_dashboard::subscribers::SubscriberTaskSpawner;
use vehicle_dashboard::topics::*;
use zenoh::Config;

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {}

slint::include_modules!();

// Define the Model struct to hold the latest data
struct Model {
    battery_data: Option<BatteryData>,
    lock_state: Option<LockState>,
    exterior: Option<Exterior>,
    speed: Option<Speed>,
    trip_data: Option<TripData>,
}

fn minutes_to_ddhhmm(minutes: f32) -> String {
    let total_minutes: u64 = minutes as u64;
    let days = total_minutes / 1440;
    let hours = (total_minutes % 1440) / 60;
    let mins = total_minutes % 60;

    fmt_format(format_args!("{:02}:{:02}:{:02}", days, hours, mins))
}

fn setup(window: &MainWindow, model: Arc<RwLock<Model>>, led_manager: Arc<LedManager>) -> Timer {
    let update_timer = Timer::default();

    let messages = [
        "Heads up! Your package is more fragile than your last relationship. Handle with care!",
        "Destination in sight. Just 12 more turns, 3 speed bumps, and 1 curious squirrel",
        "Remember, speed limits are real, not just suggestions. Keep it cool!",
        "The customer's cat might try to escape. You've been warned!",
        "Today's challenge: deliver faster than the neighbor’s gossip",
        "No pressure, but this package is more anticipated than a pizza on Friday night",
        "We’ve heard there's a cookie waiting for you at the destination. You got this!",
        "If you can deliver this one on time, we'll consider making you our favorites",
        "Beware: The customer is expecting the package like it's the last piece of cake at a partys",
        "We believe in you! Now just get this to the right address, okay?",
        "Package onboard! Keep it safe, it's got more miles to go than your average marathon",
        "Almost there! Just remember not to call it quits one street too early this time",
        "Heads up: This package is allergic to potholes. Drive carefully!",
        "They say it's a jungle out there. Deliver, dodge, and make it back in one piece!",
        "Plot twist: This address is correct. We triple-checked!",
        "The destination is near! Remember, ring the bell, don't yell",
        "This one's fragile, but we believe you can deliver it without any drama!",
        "If you hear barking, just remember – the package can't save you!",
        "Today's mission: No 'Sorry we missed you' slips. We’re counting on you!",
        "You've got the package, now make sure it gets there faster than your last excuse",
    ];

    update_timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(300),
        {
            let weak_window = window.as_weak();
            let model_clone = model.clone();
            let mut counter = 0;
            let mut msg_id: usize = 0;
            let led_manager_clone = led_manager.clone();

            move || {
                if let Some(main_window) = weak_window.upgrade() {
                    // Update time and date
                    let time_data_adapter = &main_window.global::<TimeDateAdapter>();
                    let now = Local::now();

                    time_data_adapter.set_date(slint::format!("{}", now.format("%A %e %B %Y")));
                    time_data_adapter.set_time(slint::format!("{}", now.format("%I:%M")));
                    time_data_adapter.set_time_suffix(slint::format!("{}", now.format("%p")));

                    // Update other UI elements from the model
                    let model = model_clone.read().unwrap();

                    if let Some(ref battery) = model.battery_data {
                        let battery_status_adapter = &main_window.global::<BatteryGaugeAdapter>();
                        battery_status_adapter.set_batteryLevel(battery.battery_level.round());
                        battery_status_adapter.set_isCharging(battery.is_charging);
                        battery_status_adapter.set_estimatedRange(battery.estimated_range as i32);
                        battery_status_adapter
                            .set_timeToFullCharge(battery.time_to_fully_charge as i32);
                    }
                    let mut is_lock: bool = false;
                    if let Some(ref state) = model.lock_state {
                        is_lock = state.state == State::Lock as i32;
                        let state_adapter = &main_window.global::<StateAdapter>();
                        state_adapter.set_isLocked(is_lock);
                        if !is_lock {
                            if let Err(e) = led_manager_clone.lock_light() {
                                error!("Failed to lock light {:?}", e);
                            }
                        } else {
                            if let Err(e) = led_manager_clone.unlock_light() {
                                error!("Failed to unlock light {:?}", e);
                            }
                        }
                    }

                    if let Some(ref exterior) = model.exterior {
                        let temperature_adapter =
                            &main_window.global::<AmbientTemperatureAdapter>();
                        temperature_adapter.set_temperature(exterior.air_temperature as i32);
                    }

                    if let Some(ref speed) = model.speed {
                        let speedometer_adapter = &main_window.global::<SpeedometerAdapter>();
                        speedometer_adapter.set_speed(speed.value as i32);
                    }

                    if let Some(ref trip_data) = model.trip_data {
                        let trip_data_adapter = &main_window.global::<TripDataAdapter>();
                        trip_data_adapter.set_distance(trip_data.traveled_distance);
                        trip_data_adapter.set_sinceStart(trip_data.traveled_distance_since_start);
                        trip_data_adapter.set_averageSpeed(trip_data.average_speed);
                        trip_data_adapter
                            .set_time(minutes_to_ddhhmm(trip_data.trip_duration as f32).into());
                    }

                    let tell_tales_adapter_adapter: &TellTalesAdapter<'_> =
                        &main_window.global::<TellTalesAdapter>();
                    let notifications_adapter: &CourrierNotificationsAdpater<'_> =
                        &main_window.global::<CourrierNotificationsAdpater>();

                    if counter % 2 == 0 {
                        let state = tell_tales_adapter_adapter.get_left_signal();
                        tell_tales_adapter_adapter.set_left_signal(!state);
                        tell_tales_adapter_adapter.set_right_signal(!state);
                        if is_lock {
                            if let Err(e) = led_manager_clone.blinker_led(state) {
                                error!("Failed to lock light {:?}", e);
                            }
                        }
                    } else if counter % 5 == 0 {
                        notifications_adapter.set_message(messages[msg_id].into());
                        msg_id = if msg_id < messages.len() - 1 {
                            msg_id + 1
                        } else {
                            0
                        };
                    } else if counter % 12 == 0 {
                        tell_tales_adapter_adapter
                            .set_highbeam(!tell_tales_adapter_adapter.get_highbeam());
                    } else if counter % 15 == 0 {
                        tell_tales_adapter_adapter.set_fog(!tell_tales_adapter_adapter.get_fog());
                    } else if counter % 17 == 0 {
                        tell_tales_adapter_adapter
                            .set_bendbeam(!tell_tales_adapter_adapter.get_bendbeam());
                        tell_tales_adapter_adapter
                            .set_brake(!tell_tales_adapter_adapter.get_brake());
                    } else if counter % 11 == 0 {
                        tell_tales_adapter_adapter.set_park(!tell_tales_adapter_adapter.get_park());
                    } else if counter % 25 == 0 {
                        tell_tales_adapter_adapter.set_tire(!tell_tales_adapter_adapter.get_tire());
                    }

                    counter += 1;
                } else {
                    error!("Failed to update main window!");
                }
            }
        },
    );

    update_timer
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), slint::PlatformError> {
    // Parse command line arguments
    let _args = Args::parse();
    env_logger::init();

    // Initialize the UI
    let main_window = MainWindow::new().unwrap();

    // Create the shared model
    let model = Arc::new(RwLock::new(Model {
        battery_data: None,
        lock_state: None,
        exterior: None,
        speed: None,
        trip_data: None,
    }));

    let led_manager = Arc::new(LedManager::default());

    // Setup the timer
    let _timer = setup(&main_window, model.clone(), led_manager.clone());

    // Create a Zenoh session and wrap it in Arc
    let session = Arc::new(zenoh::open(Config::default()).await.unwrap());

    // Spawn subscriber tasks
    let (state_tx, mut state_rx) = mpsc::channel::<LockState>(32);
    SubscriberTaskSpawner::spawn_task(session.clone(), LOCK_STATE_TOPIC, state_tx);

    let (battery_tx, mut battery_rx) = mpsc::channel::<BatteryData>(100);
    SubscriberTaskSpawner::spawn_task(session.clone(), BATTERY_STATE_TOPIC, battery_tx);

    let (exterior_tx, mut exterior_rx) = mpsc::channel::<Exterior>(100);
    SubscriberTaskSpawner::spawn_task(session.clone(), EXTERIOR_TOPIC, exterior_tx);

    let (speed_tx, mut speed_rx) = mpsc::channel::<Speed>(100);
    SubscriberTaskSpawner::spawn_task(session.clone(), SPEED_TOPIC, speed_tx);

    let (trip_data_tx, mut trip_data_rx) = mpsc::channel::<TripData>(100);
    SubscriberTaskSpawner::spawn_task(session.clone(), TRIP_DATA_TOPIC, trip_data_tx);

    // Spawn tasks to receive data and update the model
    tokio::spawn({
        let model_clone = model.clone();
        async move {
            while let Some(battery) = battery_rx.recv().await {
                trace!("Received BatteryData: {:?}", battery);
                let mut model = model_clone.write().unwrap();
                model.battery_data = Some(battery);
            }
        }
    });

    tokio::spawn({
        let model_clone = model.clone();
        async move {
            while let Some(state) = state_rx.recv().await {
                trace!("Received LockState: {:?}", state);
                let mut model = model_clone.write().unwrap();
                model.lock_state = Some(state);
            }
        }
    });

    tokio::spawn({
        let model_clone = model.clone();
        async move {
            while let Some(exterior) = exterior_rx.recv().await {
                trace!("Received Exterior: {:?}", exterior);
                let mut model = model_clone.write().unwrap();
                model.exterior = Some(exterior);
            }
        }
    });

    tokio::spawn({
        let model_clone = model.clone();
        async move {
            while let Some(speed) = speed_rx.recv().await {
                trace!("Received Speed: {:?}", speed);
                let mut model = model_clone.write().unwrap();
                model.speed = Some(speed);
            }
        }
    });

    tokio::spawn({
        let model_clone = model.clone();
        async move {
            while let Some(trip_data) = trip_data_rx.recv().await {
                trace!("Received TripData: {:?}", trip_data);
                let mut model = model_clone.write().unwrap();
                model.trip_data = Some(trip_data);
            }
        }
    });

    // Run the Slint UI event loop on the current thread
    main_window.run()
}
