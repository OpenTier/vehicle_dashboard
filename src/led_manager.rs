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

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use log::error;
use std::error::Error;
use std::sync::Arc;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use tokio::sync::Mutex;

#[derive(Default)]
pub struct LedManager {
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    is_running: Mutex<bool>,
}

impl LedManager {
    // Method to lock the light
    pub fn lock_light(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let manager = Arc::clone(self);
            tokio::spawn(async move {
                let mut is_running = manager.is_running.lock().await;
                if *is_running {
                    error!("Lock light task is already running");
                    return;
                }
                *is_running = true; // Mark task as running

                if let Err(e) = led_pwm::lock_light().await {
                    error!("Error locking light: {:?}", e);
                }

                *is_running = false; // Mark task as finished
            });
            return Ok(());
        }
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
        {
            return Ok(());
        }
    }

    // Method to unlock the light
    pub fn unlock_light(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let manager = Arc::clone(self);
            tokio::spawn(async move {
                let mut is_running = manager.is_running.lock().await;
                if *is_running {
                    error!("Unlock light task is already running");
                    return;
                }
                *is_running = true; // Mark task as running

                if let Err(e) = led_pwm::unlock_light().await {
                    error!("Error unlocking light: {:?}", e);
                }

                *is_running = false; // Mark task as finished
            });
            return Ok(());
        }
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
        {
            return Ok(());
        }
    }

    // Method for blinker LED
    pub fn blinker_led(self: &Arc<Self>, _state: bool) -> Result<(), Box<dyn Error>> {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let manager = Arc::clone(self);
            tokio::spawn(async move {
                let mut is_running = manager.is_running.lock().await;
                if *is_running {
                    error!("Blinker LED task is already running");
                    return;
                }
                *is_running = true; // Mark task as running

                if let Err(e) = led_pwm::blinker_led(_state).await {
                    error!("Error in blinker_led: {:?}", e);
                }

                *is_running = false; // Mark task as finished
            });
            return Ok(());
        }
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
        {
            return Ok(());
        }
    }
}
