#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    // Initalazing the framework
    info!("Embassy initialized!");

    // Initalazition of the led
    let blinky_led: Output<'static> = Output::new(peripherals.GPIO2, esp_hal::gpio::Level::High, OutputConfig::default()); // We are using GPIO2 since it is the builtin led of most Esp32 Devboards

    // In embassy spawner handles the calling of the functions but it is a good practice that all our functions are async supported and non blocking
    spawner.must_spawn(blinky(blinky_led)); // We call the function here we spawn it 



    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

#[embassy_executor::task]
async fn blinky(mut led: Output<'static> ){ // make sure led is mutable  or we won't be able to change its state in the future 
    // Creating a loop and inside i will blink it every 500 mili secounds.
    loop {
        Timer::after(Duration::from_millis(500)).await; // We need await here or it will be a bloking timer and we don't want anything to be bloking
        led.toggle(); // We could also do a led.sethigh() delay(500) led.setlow() delay(500) but we can shorten it with just a toggle 
    }
}
