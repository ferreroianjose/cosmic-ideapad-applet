use cosmic::{
    Element, app,
    iced::{
        Length, Limits, Task,
        platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup},
        widget, window,
    },
};

use crate::ideapad_laptop;

const ID: &str = "com.ferreroianojose.ideapad-applet";

#[derive(Default)]
pub struct Window {
    core: app::Core,
    popup: Option<window::Id>,
    // options documented here
    // https://github.com/torvalds/linux/blob/master/Documentation/ABI/testing/sysfs-platform-ideapad-laptop
    camera_power: Option<bool>,      // Power of camera module
    conservation_mode: Option<bool>, // Limit the maximum battery charge
    fan_mode: Option<u8>,            // Change fan mode 0-4
    fn_lock: Option<bool>,           // Control fn-lock mode
    usb_charging: Option<bool>,      // Always on USB charging
}

#[derive(Clone, Debug)]
pub enum Message {
    TogglePopup,
    CloseRequested(window::Id),
    CameraPower,
    ConservationMode,
    FanMode,
    FnLock,
    UsbCharging,
    SetCameraPower(bool),
    SetConservationMode(bool),
    SetFanMode(u8),
    SetFnLock(bool),
    SetUsbCharging(bool),
}

impl cosmic::Application for Window {
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    const APP_ID: &'static str = ID;

    fn core(&self) -> &app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut app::Core {
        &mut self.core
    }

    //fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
    //Some(cosmic::applet::style())
    //}

    fn init(core: app::Core, _flags: ()) -> (Self, app::Task<Message>) {
        let camera_power: Option<bool> = ideapad_laptop::get_camera_power().ok();
        let conservation_mode: Option<bool> = ideapad_laptop::get_conservation_mode().ok();
        let fan_mode: Option<u8> = ideapad_laptop::get_fan_mode().ok();
        let fn_lock: Option<bool> = ideapad_laptop::get_fn_lock().ok();
        let usb_charging: Option<bool> = ideapad_laptop::get_usb_charging().ok();

        (
            Self {
                core,
                popup: None,
                camera_power,
                conservation_mode,
                fan_mode,
                fn_lock,
                usb_charging,
            },
            Task::none(),
        )
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::CloseRequested(id))
    }

    fn update(&mut self, message: Self::Message) -> app::Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = window::Id::unique();
                    self.popup = Some(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(250.0)
                        .min_width(100.0)
                        .min_height(100.0)
                        .max_height(250.0);
                    popup_settings.positioner.size = None;

                    get_popup(popup_settings)
                }
            }
            Message::CloseRequested(id) => {
                if Some(id) == self.popup {
                    self.popup = None;
                }
                Task::none()
            }
            Message::CameraPower => {
                self.camera_power = ideapad_laptop::get_camera_power().ok();
                Task::none()
            }
            Message::ConservationMode => {
                self.conservation_mode = ideapad_laptop::get_conservation_mode().ok();
                Task::none()
            }
            Message::FanMode => {
                self.fan_mode = ideapad_laptop::get_fan_mode().ok();
                Task::none()
            }
            Message::FnLock => {
                self.fn_lock = ideapad_laptop::get_fn_lock().ok();
                Task::none()
            }
            Message::UsbCharging => {
                self.usb_charging = ideapad_laptop::get_usb_charging().ok();
                Task::none()
            }
            Message::SetCameraPower(b) => {
                match ideapad_laptop::set_camera_power(b) {
                    Ok(_) => self.camera_power = ideapad_laptop::get_camera_power().ok(),
                    Err(e) => {
                        eprintln!("Error while setting camera power: {e}")
                    }
                }
                Task::none()
            }
            Message::SetConservationMode(b) => {
                match ideapad_laptop::set_conservation_mode(b) {
                    Ok(_) => self.conservation_mode = ideapad_laptop::get_conservation_mode().ok(),
                    Err(e) => {
                        eprintln!("Error while setting conservation mode: {e}")
                    }
                }
                self.conservation_mode = Some(b);
                Task::none()
            }
            Message::SetFanMode(i) => {
                match ideapad_laptop::set_fan_mode(i) {
                    Ok(_) => self.fan_mode = ideapad_laptop::get_fan_mode().ok(),
                    Err(e) => {
                        eprintln!("Error while setting fan mode: {e}")
                    }
                }
                Task::none()
            }
            Message::SetFnLock(b) => {
                match ideapad_laptop::set_fn_lock(b) {
                    Ok(_) => self.fn_lock = ideapad_laptop::get_fn_lock().ok(),
                    Err(e) => {
                        eprintln!("Error while setting fn lock: {e}")
                    }
                }
                Task::none()
            }
            Message::SetUsbCharging(b) => {
                match ideapad_laptop::set_fn_lock(b) {
                    Ok(_) => self.usb_charging = ideapad_laptop::get_usb_charging().ok(),
                    Err(e) => {
                        eprintln!("Error while setting usb charging: {e}")
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: window::Id) -> cosmic::Element<Message> {
        let cosmic::cosmic_theme::Spacing { space_s, .. } =
            cosmic::theme::active().cosmic().spacing;

        if matches!(self.popup, Some(p) if p == id) {
            let mut content = widget::column![].spacing(space_s).padding([8, 0]);

            if let Some(camera_power) = self.camera_power {
                content = content.push(cosmic::applet::padded_control(
                    cosmic::iced::widget::row![
                        cosmic::widget::text("Camera Power:").width(Length::Fill),
                        cosmic::widget::toggler(camera_power)
                            .label("")
                            .on_toggle(Message::SetCameraPower),
                    ]
                    .width(Length::Fill),
                ));
            }

            if let Some(conservation_mode) = self.conservation_mode {
                content = content.push(cosmic::applet::padded_control(
                    cosmic::iced::widget::row![
                        cosmic::widget::text("Conservation Mode:").width(Length::Fill),
                        cosmic::widget::toggler(conservation_mode)
                            .label("")
                            .on_toggle(Message::SetConservationMode),
                    ]
                    .width(Length::Fill),
                ));
            }

            if let Some(fan_mode) = self.fan_mode {
                content = content.push(cosmic::applet::padded_control(
                    cosmic::iced::widget::row![
                        cosmic::widget::text("Fan Mode:").width(Length::Fill),
                        cosmic::iced::widget::row![
                            cosmic::widget::slider(0..=4, fan_mode, Message::SetFanMode),
                            cosmic::widget::text(fan_mode.to_string()).width(Length::Shrink),
                        ]
                        .spacing(8),
                    ]
                    .width(Length::Fill),
                ));
            }

            if let Some(fn_lock) = self.fn_lock {
                content = content.push(cosmic::applet::padded_control(
                    cosmic::iced::widget::row![
                        cosmic::widget::text("Fn Lock:").width(Length::Fill),
                        cosmic::widget::toggler(fn_lock)
                            .label("")
                            .on_toggle(Message::SetFnLock),
                    ]
                    .width(Length::Fill),
                ));
            }

            if let Some(usb_charging) = self.usb_charging {
                content = content.push(cosmic::applet::padded_control(
                    cosmic::iced::widget::row![
                        cosmic::widget::text("USB Charging:").width(Length::Fill),
                        cosmic::widget::toggler(usb_charging)
                            .label("")
                            .on_toggle(Message::SetUsbCharging),
                    ]
                    .width(Length::Fill),
                ));
            }

            self.core.applet.popup_container(content).into()
        } else {
            cosmic::widget::text("").into()
        }
    }
}
