#![no_main]
#![no_std]

mod layout;

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;
use rtic::app;
use stm32f3xx_hal::gpio::{Input, Output, PXx, PushPull};
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::usb::{Peripheral, UsbBus, UsbBusType};
use stm32f3xx_hal::{pac, timer};
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;
use usb_device::device::UsbVidPid;

use keyberon::debounce::Debouncer;
use keyberon::key_code::{KbHidReport, KeyCode};
use keyberon::layout::Layout;
use keyberon::matrix::{Matrix, PressedKeys};

use crate::layout::{BASE_LAYER, FUNCTION_LAYER, MACRO_LAYER};

// Same values that Clueboard QMK firmware uses
const VID: u16 = 0xC1ED;
const PID: u16 = 0x2391;

type UsbClass = keyberon::Class<'static, UsbBusType, ()>;
type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;

// pub struct Leds {
//     caps_lock: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
// }
// impl keyberon::keyboard::Leds for Leds {
//     fn caps_lock(&mut self, status: bool) {
//         if status {
//             self.caps_lock.set_low().unwrap()
//         } else {
//             self.caps_lock.set_high().unwrap()
//         }
//     }
// }

pub static LAYERS: keyberon::layout::Layers = &[BASE_LAYER, FUNCTION_LAYER, MACRO_LAYER];

#[app(device = stm32f3xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
        matrix: Matrix<PXx<Output<PushPull>>, PXx<Input>, 8, 10>,
        debouncer: Debouncer<PressedKeys<8, 10>>,
        layout: Layout,
        timer: timer::Timer<pac::TIM3>,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        let mut flash = c.device.FLASH.constrain();
        let mut rcc = c.device.RCC.constrain();

        // set 0x424C in DR10 for dfu on reset
        // let bkp = rcc
        //     .bkp
        //     .constrain(c.device.BKP, &mut rcc.apb1, &mut c.device.PWR);
        // bkp.write_data_register_low(9, 0x424C);

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(48.MHz()) // can also run at 72MHz
            .pclk1(24.MHz())
            .pclk2(24.MHz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        let mut gpioa = c.device.GPIOA.split(&mut rcc.ahb);
        let mut gpiob = c.device.GPIOB.split(&mut rcc.ahb);
        let mut gpioc = c.device.GPIOC.split(&mut rcc.ahb);

        // USB is on PA11 (D-) and PA12 (D+)
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        let mut usb_dp = gpioa
            .pa12
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        usb_dp.set_low().unwrap();
        cortex_m::asm::delay(clocks.sysclk().0 / 100);

        // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        // led.set_high().unwrap();
        let leds = (); // Leds { caps_lock: led };

        let usb_dm =
            gpioa
                .pa11
                .into_af14_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
        let usb_dp =
            usb_dp.into_af14_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

        let usb = Peripheral {
            usb: c.device.USB,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        *USB_BUS = Some(UsbBus::new(usb));
        let usb_bus = USB_BUS.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, leds);
        let usb_dev = keyberon::new_device(
            usb_bus,
            UsbVidPid(VID, PID),
            "Clueboard",
            "66% HotSwap Keyboard",
        );

        // Set up the matrix scan timer, polls at 1kHz (1000 times a second/every 1ms)
        let mut timer = timer::Timer::new(c.device.TIM3, clocks, &mut rcc.apb1);
        timer.start(1.milliseconds());
        timer.enable_interrupt(timer::Event::Update);

        let matrix = Matrix::new(
            [
                gpiob
                    .pb10
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb2
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb1
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb0
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
                gpioa
                    .pa7
                    .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb4
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb3
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb7
                    .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper)
                    .downgrade()
                    .downgrade(),
            ],
            [
                gpiob
                    .pb11
                    .into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioa
                    .pa6
                    .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioa
                    .pa3
                    .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioa
                    .pa2
                    .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioa
                    .pa1
                    .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb5
                    .into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr)
                    .downgrade()
                    .downgrade(),
                gpiob
                    .pb6
                    .into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioc
                    .pc15
                    .into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioc
                    .pc14
                    .into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr)
                    .downgrade()
                    .downgrade(),
                gpioc
                    .pc13
                    .into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr)
                    .downgrade()
                    .downgrade(),
            ],
        );

        init::LateResources {
            usb_dev,
            usb_class,
            timer,
            debouncer: Debouncer::new(PressedKeys::default(), PressedKeys::default(), 5),
            matrix: matrix.unwrap(),
            layout: Layout::new(LAYERS),
        }
    }

    #[task(binds = USB_HP_CAN_TX, priority = 2, resources = [usb_dev, usb_class])]
    fn usb_tx(mut c: usb_tx::Context) {
        usb_poll(&mut c.resources.usb_dev, &mut c.resources.usb_class);
    }

    #[task(binds = USB_LP_CAN_RX0, priority = 2, resources = [usb_dev, usb_class])]
    fn usb_rx(mut c: usb_rx::Context) {
        usb_poll(&mut c.resources.usb_dev, &mut c.resources.usb_class);
    }

    #[task(binds = TIM3, priority = 1, resources = [usb_class, matrix, debouncer, layout, timer])]
    fn tick(mut c: tick::Context) {
        c.resources.timer.clear_event(timer::Event::Update);

        for event in c
            .resources
            .debouncer
            .events(c.resources.matrix.get().unwrap())
        {
            c.resources.layout.event(event);
        }
        c.resources.layout.tick();
        send_report(c.resources.layout.keycodes(), &mut c.resources.usb_class);
    }
};

fn send_report(iter: impl Iterator<Item = KeyCode>, usb_class: &mut resources::usb_class<'_>) {
    use rtic::Mutex;
    let report: KbHidReport = iter.collect();
    if usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
        while let Ok(0) = usb_class.lock(|k| k.write(report.as_bytes())) {}
    }
}

fn usb_poll(usb_dev: &mut UsbDevice, keyboard: &mut UsbClass) {
    if usb_dev.poll(&mut [keyboard]) {
        keyboard.poll();
    }
}
