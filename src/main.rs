use crossbeam_channel::bounded;
use esp_idf_hal::cpu::Core;
use esp_idf_hal::task::watchdog::{TWDTConfig, TWDTDriver};
use esp_idf_hal::{
    gpio::{OutputPin, PinDriver},
    peripherals::Peripherals,
};
use ultrasonic::startup::App;
use ultrasonic::{threads, ultrasonic::Ultrasonic};

static ULTRASONIC_STACK_SIZE: usize = 2000;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // This sets the wifi and creates an http client
    let mut app = App::spawn()?;

    //ultrasonic sensor
    let ultrasonic = Ultrasonic::new()?;

    //reset button
    let peripherals = Peripherals::take().unwrap();

    // Configure the watchdog timer
    let config = TWDTConfig {
        duration: std::time::Duration::from_secs(10),
        panic_on_trigger: false, // this tells the esp not to Panic if the watchdog triggers
        subscribed_idle_tasks: enumset::enum_set!(Core::Core1), // Subscribe to idle tasks on Core1 (core reading the ultarsonic sensor)
    };

    // Create the TWDT driver
    let mut driver = TWDTDriver::new(peripherals.twdt, &config)?;

    //application hardware
    let button = PinDriver::input(peripherals.pins.gpio15)?;
    let mut led = PinDriver::output(peripherals.pins.gpio7.downgrade_output())?;

    let (tx, rx) = bounded::<f32>(1);

    let _ultrasonic_thread = std::thread::Builder::new()
        .stack_size(ULTRASONIC_STACK_SIZE)
        .spawn(move || threads::ultrasonic_thread_function(&mut driver, ultrasonic, tx))?;

    let mut distance = -1.0;
    let mut ready = false;
    let mut already_sent = false;

    loop {
        match rx.try_recv() {
            Ok(x) => {
                distance = x;
                ready = true;
                println!("distance: {}", distance);
            }
            Err(_) => {}
        }

        if ready && distance <= 10.0 && !already_sent && distance > 0.0 {
            //envío petición LLENO
            app.client.process_request(1, &mut led)?;
            already_sent = true;
        }

        if ready && distance > 10.0 && already_sent && button.is_low() {
            //envío petición VACIO
            app.client.process_request(0, &mut led)?;
            already_sent = false;
        }
    }
}
