#![feature(plugin)]
#![feature(start)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use core::option::Option::Some;

use zinc::hal::cortex_m4::systick;
use zinc::hal::k20::{pin, watchdog, uart};
use zinc::hal::pin::Gpio;
use zinc::hal::uart::Parity;
use zinc::drivers::chario::CharIO;

/// Wait the given number of SysTick ticks
pub fn wait(ticks: u32) {
  let mut n = ticks;
  // Reset the tick flag
  systick::tick();
  loop {
    if systick::tick() {
      n -= 1;
      if n == 0 {
        break;
      }
    }
  }
}

#[zinc_main]
pub fn main() {
  zinc::hal::mem_init::init_stack();
  zinc::hal::mem_init::init_data();
  watchdog::init(watchdog::State::Disabled);

  // Pins for Hologram Dash (http://www.hologram.io/)
  let led1 = pin::Pin::new(pin::Port::PortB, 19, pin::Function::Gpio, Some(zinc::hal::pin::Out));
  pin::Pin::new(pin::Port::PortD, 2, pin::Function::AltFunction3, Some(zinc::hal::pin::In));
  pin::Pin::new(pin::Port::PortD, 3, pin::Function::AltFunction3, Some(zinc::hal::pin::Out));
  let uart2 = uart::UART::new(uart::UARTPeripheral::UART2, 115200, 8, Parity::Disabled, 1);

  systick::setup(systick::ten_ms().unwrap_or(480000));
  systick::enable();
  uart2.puts("Hello, World!\n");
  loop {
    led1.set_high();
    wait(10);
    led1.set_low();
    wait(10);
  }
}
