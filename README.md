# Vehicle Dashboard

## Project Overview

Vehicle Dashboard is a SlintUI application designed to function as a vehicle cluster for cars or scooters. It provides real-time monitoring and control of various vehicle parameters, including telltales, speedometer, battery status, and trip data. The application connects to vehicle signals using Zenoh, a protocol for data-centric communication, to gather and display information such as battery status, lock state, exterior conditions, speed, and trip data.

### Key Features

- **Real-Time Data Display**: Monitor vehicle parameters in real-time with a user-friendly interface.
- **Telltales**: Display various telltales such as high beam, fog lights, and turn signals.
- **Speedometer**: Real-time speed display with smooth animations.
- **Battery Status**: Monitor battery level, charging status, and estimated range.
- **Trip Data**: View trip duration, distance traveled, and average speed.
- **Cross-Platform Support**: Runs on multiple platforms, ensuring compatibility with various operating systems.

## Getting Started

1. Clone the Repository

```bash
git clone https://github.com/OpenTier/vehicle-dashboard.git
cd vehicle-dashboard
```

2. Build and run project

```bash
cargo run --bin vehicle-dashboard
```

Ensure that you have Rust and Protocol Buffers compiler installed

## Contributing

We welcome contributions to this template! See [Contribution guidelines](CONTRIBUTING.md) for guidelines on how to contribute.

## License

See the [License file](LICENSE.md) for more details.
