#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use cortex_m_rt::entry;

use core::{mem::MaybeUninit, panic::PanicInfo};

use embedded_rust_h2al::{events, Component, ComponentsBuilder};
use embedded_rust_hardware_init::device_config;

#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // log::error!("panic: {}", info);
    cortex_m_semihosting::hprintln!("panic: {}", info).unwrap();
    cortex_m::interrupt::disable();
    loop {}
}
struct BluePill;
impl BluePill {
    fn init() -> (
        &'static mut (
            stm32f1xx_hal::gpio::gpioa::PA0<
                stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::PullUp>,
            >,
        ),
        &'static mut (
            stm32f1xx_hal::gpio::gpioc::PC13<
                stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>,
            >,
        ),
        &'static mut (),
    ) {
        use core::mem::MaybeUninit;
        use stm32f1xx_hal::gpio::{self, Edge, ExtiPin};
        use stm32f1xx_hal::pac;
        use stm32f1xx_hal::prelude::*;
        use stm32f1xx_hal::pwm::{self, PwmChannel};
        use stm32f1xx_hal::serial::{self, Config};
        use stm32f1xx_hal::timer::{self, Timer};
        let peripherals = stm32f1xx_hal::pac::Peripherals::take().unwrap();
        let mut flash = peripherals.FLASH.constrain();
        let mut rcc = peripherals.RCC.constrain();
        let cfgr = rcc.cfgr;
        let cfgr = cfgr.sysclk(36000000u32.hz());
        let clocks = cfgr.freeze(&mut flash.acr);
        let mut afio = peripherals.AFIO.constrain(&mut rcc.apb2);
        let mut gpioa = peripherals.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = peripherals.GPIOC.split(&mut rcc.apb2);
        let mut pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
        pa0.make_interrupt_source(&mut afio);
        pa0.trigger_on_edge(&peripherals.EXTI, Edge::FALLING);
        pa0.enable_interrupt(&peripherals.EXTI);
        let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        static mut INPUT_PINS: MaybeUninit<(
            stm32f1xx_hal::gpio::gpioa::PA0<
                stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::PullUp>,
            >,
        )> = MaybeUninit::uninit();
        static mut OUTPUT_PINS: MaybeUninit<(
            stm32f1xx_hal::gpio::gpioc::PC13<
                stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>,
            >,
        )> = MaybeUninit::uninit();
        static mut PWM_PINS: MaybeUninit<()> = MaybeUninit::uninit();
        static mut CHANNELS: MaybeUninit<()> = MaybeUninit::uninit();
        static mut TIMERS: MaybeUninit<()> = MaybeUninit::uninit();
        unsafe {
            (
                (INPUT_PINS.write((pa0,))),
                (OUTPUT_PINS.write((pc13,))),
                (PWM_PINS.write(())),
            )
        }
    }
    #[inline]
    fn enable_interrupts() {
        unsafe {
            stm32f1xx_hal::pac::NVIC::unmask(stm32f1xx_hal::pac::Interrupt::EXTI0);
        }
    }
}
#[entry]
fn main() -> ! {
    static mut ca: [MaybeUninit<Component>; 2] = ComponentsBuilder::allocate_array();
    let components = BluePill::init();
    let mut cb = ComponentsBuilder::new(ca);
    let c = unsafe { cb.finalize() };
    loop {}
}

pub async fn test_task() {
    loop {}
}

// macro_rules! to_target_endianess {
//     ($int:expr) => {
//         if cfg!(target_endian = "big") {
//             $int.to_be_bytes()
//         } else {
//             $int.to_le_bytes()
//         }
//     };
// }

// enum Level {
//     Full,
//     High,
//     Half,
//     Low,
//     Off,
// }

// struct Brightness {
//     level: Level,
// }

// impl Brightness {
//     fn next(&mut self) -> f32 {
//         match self.level {
//             Level::Full => {
//                 self.level = Level::High;
//                 0.75f32
//             }
//             Level::High => {
//                 self.level = Level::Half;
//                 0.5f32
//             }
//             Level::Half => {
//                 self.level = Level::Low;
//                 0.25f32
//             }
//             Level::Low => {
//                 self.level = Level::Off;
//                 0.0f32
//             }
//             Level::Off => {
//                 self.level = Level::Full;
//                 1.0f32
//             }
//         }
//     }
// }

// pub async fn test_task() {
//     let mut button_events = BluePill::get_resource("event:gpio/pa0").unwrap();
//     let mut led = BluePill::get_resource("digital:gpio/pc13").unwrap();
//     let mut brightness = Brightness { level: Level::Off };
//     let mut pwm = BluePill::get_resource("percent:pwm/pa1").unwrap();
//     let mut usart1 = BluePill::get_resource("bus:serial/usart1").unwrap();
//     pwm.write(&to_target_endianess!(brightness.next()))
//         .await
//         .unwrap();
//     let mut led_state = false;
//     let mut buf = [0; 6];
//     loop {
//         usart1.write_all("ABCDEF".as_bytes()).await.unwrap();
//         log::info!("written");
//         usart1.read(&mut buf).await.unwrap();
//         log::info!("{:?}", buf);
//     }
// }
