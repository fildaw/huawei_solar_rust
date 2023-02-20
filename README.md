# huawei_solar_rust
Small CLI app for retrieving information from Huawei SUN2000-KTL-M solar inverters. Written in Rust.

# Usage

## Basic command
```./huawei_solar_cli query ip[:port] <query_params> <output_destination> [--slave_id <id>] [--output_format <json/pretty_print>]```
### Required arguments
IP and port depends on the method of connecting (see below). Default port used by the app is 502.

`query_params` are parameters you want to get from the inverter seperated by commas, eg. **active_power,day_active_power_peak,daily_yield_energy**
You can simply type **all** to retrieve all available parameters from the inverter that this app can get. (only majority of basic parameters are available at the moment, this will be updated in the future).
Type `./huawei_solar_cli query --help` to get a list of all possible parameters.

`output_destination` is location where retrieved params will be saved in a file. You can type **-** to print params to stdout.
### Optional arguments
`--slave_id` this is the id used by modbus to identify device. See below on usage of this option. If not specified, default value is **0**.
`--output_format` this is format in which data will be saved/printed by the app. Available options are **json** and **pretty_print**. Defaults to **json**.

## Connection methods

There are basically two ways to connect to Huawei inverter.

### Connecting directly to the inverter
First one uses internal Wi-Fi network exposed by inverter itself. Typically, it is named `SUN2000-xxxx`.
To connect to the inverter this way, you will need a device with a Wi-Fi card (for example Raspberry Pi) and configure it to connect to inverter's Wi-Fi.
In this configuration, inverter's IP address is **192.168.200.1** and port is **6607**. Slave id is 0, which is default used by this app.

Example:
```./huawei_solar_cli query 192.168.200.1:6607 all data.json```

### Connecting through S-Dongle
This method uses Huawei Smart Dongle to talk with the inverter. It's a small device which enables inverter's internet connection.
Connecting through it is often more convienient, because you can interact with the inverter using your LAN without having seperate wireless network card near the inverter
to connect to it (in particular case completely without using Wi-Fi).
In this configuration, inverter's IP address is specified by your router's DHCP server and port is **502** (which is default). 
Slave id is 1, so you must specify it using `--slave_id 1`.

**NOTE 1:** This method requires you to configurate your inverter through SUN2000 Android app, specifically turning on Modbus TCP option (typically *unrestricted* one).
To do this, you must login to the invterted through the app with installer password. See https://forum.huawei.com/enterprise/en/modbus-tcp-guide/thread/789585-100027

**NOTE 2:** Slave id seems to depend on the COM setting related to S-Dongle in above app (untested).

Example:
```./huawei_solar_cli query <s_dongle_ip> --slave_id 1 all data.json```

### Additional notes
Presented port numbers seem to depend on firmware version of your inverter and s-dongle. If you cannot connect, you can use `nmap` tool to find out ports exposed by your inverter, example: `nmap -p- <inverters_ip>`.
Known to me values for ports are: *502, 6607, 6606*. You can try them all with different slave ids if needed.

# Compiling
Firstly, install Rust on your Linux/Windows system. Preffered way to do it is using rustup tool https://rustup.rs.
When installed, you can start compiling process.
1. Clone this git repository:
`git clone https://github.com/fildaw/huawei_solar_rust`
or download ZIP file using GitHub.
2. Navigate to `huawei_solar_cli` folder: `cd huawei_solar_cli`
3. Build CLI utility:
`cargo build --release`
4. If all went well, this app is successfully compiled and ready to use! 
You can find executable binary `huawei_solar_cli` (or `huawei_solar_cli.exe` on Windows) in `huawei_solar_cli/target/release`.

# Cross-compiling for Raspberry Pi (instruction for Linux)
Rust compiler is very slow on small boards like Raspberry Pi. But, you can compile Rust code on your local machine and run result binary on some other platform.

## Compiling for Raspberry Pi 2/3/4
Install ARM compiler toolchain:
```
sudo apt install gcc-arm-linux-gnueabihf
```

Add ARMv7 arch support for Rust:
```
rustup target add armv7-unknown-linux-gnueabihf
```

Configure cargo by creating file named `config` in ~/.cargo and writing below:
```
[target.armv7-unknown-linux-gnueabihf]
linker = "/usr/bin/arm-linux-gnueabihf-gcc"
```

To compile, use this command in `huawei_solar_cli/`:
```
cargo build --target=armv7-unknown-linux-gnueabihf --release
```

## Compiling for Raspberry Pi Zero W
Clone rpi-tools repo to home dir:
```
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
```

Add ARM arch support for Rust:
```
rustup target add arm-unknown-linux-gnueabihf
```

Configure cargo by creating file named `config` in ~/.cargo and writing below:
```
[target.arm-unknown-linux-gnueabihf]
linker = "/home/<user>/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

To compile, use this command in `huawei_solar_cli/`:
```
cargo build --target=arm-unknown-linux-gnueabihf --release
```

# Author
&copy; 2023 fildaw

Inspired by: https://github.com/wlcrs/huawei-solar-lib

