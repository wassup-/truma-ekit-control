# Truma E-Kit Control

## Components

The project consists of 2 main components, the [controller](#controller) and (optionally) the [thermostat](#thermostat).

### Controller

The controller is connected directly to the Truma E-Kit, and is responsible for driving the fan and heating coils.
The controller will create a protected Wifi network, and will host an HTTP server that can be used to *request* a specific run mode.

Currently the following 4 run modes are supported:
- **Off** (everything is turned off)
- **Half** (1 heating coil is turned on, fan is turned on)
- **Full** (both heating coils are turned on, fan is turned on)
- **Cool** (only the fan is turned on, both heating coils are turned off)

### Thermostat

The thermostat is connected wirelessly to the controller, and is responsible for steering the controller.
The thermostat will join the protected Wifi network created by the controller, and based on the actual ambient temperature will request the appropriate run mode on the controller.

## Usage

### Configuration

#### Wifi

The Wifi network can be configured by editing the [wifi.rs](truma-ekit-core/src/wifi.rs) file.

```rust
pub const WIFI_SSID: &str = "truma-ekit";
pub const WIFI_PASS: &str = "truma-ekit-pass";
```

#### Temperatures

The treshold for entering and exiting overtemperature protection can be configured by editing the [overtemperature_protection.rs](truma-ekit-controller/src/overtemperature_protection.rs) file.

```rust
/// Cooldown will be entered if the output temperature is greater than or equal to this limit.
const COOLDOWN_ENTER: Temperature = celsius(90.0);
/// Cooldown will be exited if the output temperature is less than than or equal to this limit.
const COOLDOWN_EXIT: Temperature = celsius(50.0);
```

The treshold for running the controller at full capacity can be configured by editing the [thermostat.rs](truma-ekit-thermostat/src/thermostat.rs) file.

```rust
/// The threshold for running the controller at full capacity.
/// If the temperature difference is below this value, the controller will be run at half capacity.
const FULL_CAPACITY_TRESHOLD: Temperature = celsius(1.5);
```

### Flashing the firmware

To run either component, they will have to be flashed onto a suitable microcontroller. At the moment only the **ESP32-C3** is supported.
To flash a component to the microcontroller, connect the microcontroller to your computer and use either of the following commands:
- flash the controller: `cargo run -p truma-ekit-controller`
- flash the thermostat: `cargo run -p truma-ekit-thermostat`


## Hardware

### Controller

The controller consists of the following hardware components:
- **TMP36** (temperature sensor) used for overtemperature protection
- **3 relays** (one for the fan, and one for each heating coil)

The [default configuration](truma-ekit-controller/src/peripherals.rs) assumes the following connections:
- **TMP36** connected to **GPIO2**
- **Fan relay** connected to **GPIO7**
- **Heating coil #1 relay** connected to **GPIO8**
- **Heating coil #2 relay** connected to **GPIO9**

### Thermostat

The thermostat consists of the following hardware components:
- **BME280** (environmental sensor) used to measure the ambient temperature

The [default configuration](truma-ekit-thermostat/src/peripherals.rs) assumes the following connections:
- **BME280 SDA** connected to **GPIO5**
- **BME280 SCL** connected to **GPIO6**
- **BME280 VCC** connected to **GPIO13**

## Contributing

Feel free to create an [issue](https://github.com/wassup-/truma-ekit-control/issues), or create a [pull request](https://github.com/wassup-/truma-ekit-control/pulls).
