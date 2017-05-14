//! Transmit the device ID repeatedly via USB serial.

#![no_std]

extern crate numtoa;
extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use numtoa::NumToA;

const PERIOD: u32 = 1000; // ms
const BAUD_RATE: u32 = 115_200;

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    unsafe {
        hal::USB_USART_Init(BAUD_RATE);
    }
}

fn loop_() {
    let mut device_id: [u8; 12] = [0; 12];
    let mut hex_byte_buf: [u8; 2] = [0; 2];
    unsafe {
        // Get Device ID
        hal::HAL_device_ID(device_id.as_mut_ptr(), 8);

        // Print Device ID via USB serial
        for byte in b"Device ID is " {
            hal::USB_USART_Send_Data(*byte);
        }
        let mut first = true;
        for byte in device_id.iter() {
            if first {
                first = false;
            } else {
                hal::USB_USART_Send_Data(b':');
            }
            byte.numtoa(16, &mut hex_byte_buf);
            hal::USB_USART_Send_Data(hex_byte_buf[0]);
            hal::USB_USART_Send_Data(hex_byte_buf[1]);
        }
        hal::USB_USART_Send_Data(b'\n');
        hal::HAL_Delay_Milliseconds(PERIOD);
    }
}
