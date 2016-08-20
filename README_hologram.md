Running Zinc on the Hologram Dash
=================================

These instructions are a step-by-step guide to running rust code on the Hologram Dash. These instructions are tested on OSX 10.11.6 and Debian GNU Linux 8. A guide for Windows users will be added later.

## Installing rustup
Zinc works with an older version of Rust Nightly (2016-06-08). To install that version along with other versions of rust, we'll install rustup. The rustup tool allows multiple versions of rust to be installed, and you can choose which one to use. Install rustup from https://www.rustup.rs or enter this in the terminal:

`curl https://sh.rustup.rs -sSf | sh`

Choose the default installation (option 1) and the most recent release of rust stable will be installed. Add cargo to your path by either restarting your terminal or

`$ source $HOME/.cargo/env`

Check that cargo is installed

`$ cargo -V`

## Installing Rust Nightly (2016-06-08)
We need a specific version of rust nightly, so we'll use rustup to install and use that version.

```
$ rustup install nightly-2016-06-08
$ rustup default nightly-2016-06-08
```

## Installing ARM toolchain
The Dash uses a Kinetis MCU with an ARM Cortex-M4 core. The arm gcc toolchain will be used by cargo to build the target code. Follow the instructions for your host machine http://gnuarmeclipse.github.io/toolchain/install/ here. It is recommended to not add the ARM gcc toolchain to your path, but this is the only way cargo will find it. Add gcc to you path temporarily, but this needs to be called each time you open a terminal to build Zinc, replacing the gcc path with the actual version installed.

`$ export PATH=$PATH:/usr/local/gcc-arm-none-eabi-5_4-2016q2/bin`

## Install the Hologram Dash Updater Tool
Download and install the Updater from https://hologram.io/docs/guides-tutorials/using-our-platform/tutorials/upgrade-dash-user-program-and-firmware/#hologram-dash-updater-utility. Here is an example installation for Linux, which needs a udev rules file to have the correct USB permissions and to prevent modem manager from sending AT commands through the Dash.

```
$ cd $HOME/Downloads
$ wget wget http://downloads.hologram.io/dash/updater/dashupdater-0.7.1-x86_64-pc-linux-gnu.tar.bz2
$ wget https://raw.githubusercontent.com/hologram-io/hologram-dash-arduino-integration/master/85-hologram.rules
$ cd /usr/local
$ sudo tar xjf ~/Downloads/dashupdater-0.7.1-x86_64-pc-linux-gnu.tar.bz2
$ export PATH=$PATH:/usr/local/dashupdater-0.7.1-x86_64-pc-linux-gnu/bin
$ sudo cp 85-hologram.rules /etc/udev/rules.d/
$ sudo udevadm control --reload-rules
```

## Clone Zinc
Clone the Hologram fork of Zinc into your GitHub or code directory and build the blink demo from the branch with the modifications for the Dash.

```
$ cd $HOME/GitHub
$ git clone https://github.com/HologramErik/zinc.git
$ cd zinc
$ git checkout demo-hologram-dash
$ cd examples/blink_k20
$ cargo build --target=thumbv7em-none-eabi --features mcu_k20 --release
$ arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabi/release/blink blink.bin
```

## Program the Dash
Press the program button on the Dash, connected to a USB port on the host computer. If you have a Serial/USB converter (like an FT232), connect it to the Dash Serial2 port (pins L6 and L8) and open a terminal at 57600.

```
$ dashupdater --imagetype user --method usb --imagefile blink.bin
```

The updater will load the rust blink program and the LED will begin to blink. If the Serial/USB converter is connected, `Hello, World!` will be printed in the terminal.
