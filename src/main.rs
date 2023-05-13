#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::{Executor, _export::StaticCell};
use embassy_time::{Duration, Timer};
use esp32c3_hal::{
    clock::ClockControl,
    embassy, entry,
    gpio::{AnyPin, Output, PushPull, IO},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc,
};
// use esp_println println! implementation for printing to the serial monitor
use esp_println::println;

// use esp_backtrace as panic handler for printing the backtrace
use esp_backtrace as _;

// define StaticCell for the embassy Executor, because executor::run requires a static reference
static EXECUTOR: StaticCell<Executor> = StaticCell::new();

// based on https://github.com/esp-rs/esp-hal/blob/fa3627c1fda334ac89181ae293f762eab82bb307/esp32c3-hal/examples/embassy_hello_world.rs

#[entry]
fn main() -> ! {
    // get peripherals from HAL
    let peripherals = Peripherals::take();
    // retrieve clocks
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // get timers and watchdogs
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // initialize embassy with the clocks and the systimer
    embassy::init(
        &clocks,
        esp32c3_hal::systimer::SystemTimer::new(peripherals.SYSTIMER),
    );

    // get GPIO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // configure builtin LED to pass to the blink task
    let builtin_led = io.pins.gpio5.into_push_pull_output();

    // initialize executor
    let executor = EXECUTOR.init(Executor::new());

    // run the embassy executor and spawn the blink task
    executor.run(|spawner| {
        spawner.spawn(led_blink(builtin_led.into())).ok();
    });
}

#[embassy_executor::task]
async fn led_blink(mut pin: AnyPin<Output<PushPull>>) -> () {
    loop {
        pin.toggle().unwrap();
        println!("Toggle pin!");
        Timer::after(Duration::from_millis(1000)).await;
    }
}
