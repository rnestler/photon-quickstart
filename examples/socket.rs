//! Particle.socket API example

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use photon::{App};
use hal::{LED, ll, D7, PinMode, socket};

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    D7.pin_mode(PinMode::Output);
}

fn loop_(ref mut app: App) {
    LED.high();
    
    let tcp_client = socket::TCPClient::new();

    let connected = tcp_client.connect([94, 230, 210, 84], 80).is_ok();
    
    let period = if connected {
        500
    } else {
        100
    };
    for _ in 0..3 {
        LED.high();
        app.delay_ms(period);
        LED.low();
        app.delay_ms(period);
    }

    let data = "PUT /sensors/temperature_room/ HTTP/1.1\r
Host: status.crdmp.ch\r
\r
value=26.0\r\n";

    //tcp_client.send(data.as_bytes());

    app.delay_ms(1000);
}

