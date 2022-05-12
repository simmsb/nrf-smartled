//! Smartled drivers for the nrf52 family
//!
//! Currently offers an option using the PWM peripheral, and
//! it is possible in the future to also support the use of the
//! i2s peripheral

#![no_std]

// pub mod i2s;
pub mod pwm;

pub use smart_leds_trait::RGB8;
