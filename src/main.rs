use std::{thread::sleep, time::Duration};

use dht_embedded::{Dht22, DhtSensor, NoopInterruptControl};
use gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::{CdevPin, Delay};
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;

fn main() {
    let builder = PrometheusBuilder::new();
    // Defaults to enabled, listening at 0.0.0.0:9000
    // TODO: change the port.
    builder
        .install()
        .expect("failed to install recorder/exporter");

    let humidity = gauge!("humidity_percent_2");
    let temperature_c = gauge!("temperature_celsius_2");
    let temperature_f = gauge!("temperature_fahrenheit_2");

    // TODO: figure out the right device.
    let mut gpiochip = Chip::new("/dev/gpiochip0").unwrap();
    let line = gpiochip.get_line(24).unwrap();
    let handle = line
        .request(
            LineRequestFlags::INPUT | LineRequestFlags::OUTPUT,
            1,
            "dht-sensor",
        )
        .unwrap();
    let pin = CdevPin::new(handle).unwrap();
    // TODO: Note that, if your hardware supports it, you should set the GPIO pin to "open drain" mode.
    let mut sensor = Dht22::new(NoopInterruptControl, Delay, pin);

    loop {
        match sensor.read() {
            Ok(reading) => {
                /*
                print!("{}{}{}", termion::clear::All, termion::cursor::Goto(0, 0),
                println!(
                    "{}°C {}°F {}% RH",
                    reading.temperature(),
                    (reading.temperature() * 1.8) + 32.0,
                    reading.humidity()
                );
                */

                humidity.set(reading.humidity());
                temperature_c.set(reading.temperature());
                temperature_f.set((reading.temperature() * 1.8) + 32.0);

                sleep(Duration::from_millis(2_100));
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
