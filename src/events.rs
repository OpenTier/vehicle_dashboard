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

pub mod state {
    include!(concat!(env!("OUT_DIR"), "/intra.lock_state.rs"));
}

pub mod speed {
    include!(concat!(env!("OUT_DIR"), "/intra.speed.rs"));
}

pub mod trip_data {
    include!(concat!(env!("OUT_DIR"), "/intra.trip_data.rs"));
}

pub mod battery {
    include!(concat!(env!("OUT_DIR"), "/intra.battery.rs"));
}

pub mod exterior {
    include!(concat!(env!("OUT_DIR"), "/intra.exterior.rs"));
}
