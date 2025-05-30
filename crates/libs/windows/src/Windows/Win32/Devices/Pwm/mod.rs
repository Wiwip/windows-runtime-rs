pub const GUID_DEVINTERFACE_PWM_CONTROLLER: windows_core::GUID = windows_core::GUID::from_u128(0x60824b4c_eed1_4c9c_b49c_1b961461a819);
pub const GUID_DEVINTERFACE_PWM_CONTROLLER_WSZ: windows_core::PCWSTR = windows_core::w!("{60824B4C-EED1-4C9C-B49C-1B961461A819}");
pub const IOCTL_PWM_CONTROLLER_GET_ACTUAL_PERIOD: u32 = 262148u32;
pub const IOCTL_PWM_CONTROLLER_GET_INFO: u32 = 262144u32;
pub const IOCTL_PWM_CONTROLLER_SET_DESIRED_PERIOD: u32 = 294920u32;
pub const IOCTL_PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 262544u32;
pub const IOCTL_PWM_PIN_GET_POLARITY: u32 = 262552u32;
pub const IOCTL_PWM_PIN_IS_STARTED: u32 = 262568u32;
pub const IOCTL_PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 295316u32;
pub const IOCTL_PWM_PIN_SET_POLARITY: u32 = 295324u32;
pub const IOCTL_PWM_PIN_START: u32 = 295331u32;
pub const IOCTL_PWM_PIN_STOP: u32 = 295335u32;
pub const PWM_ACTIVE_HIGH: PWM_POLARITY = PWM_POLARITY(0i32);
pub const PWM_ACTIVE_LOW: PWM_POLARITY = PWM_POLARITY(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_CONTROLLER_INFO {
    pub Size: usize,
    pub PinCount: u32,
    pub MinimumPeriod: u64,
    pub MaximumPeriod: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    pub DesiredPeriod: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
pub const PWM_IOCTL_ID_CONTROLLER_GET_ACTUAL_PERIOD: i32 = 1i32;
pub const PWM_IOCTL_ID_CONTROLLER_GET_INFO: i32 = 0i32;
pub const PWM_IOCTL_ID_CONTROLLER_SET_DESIRED_PERIOD: i32 = 2i32;
pub const PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 100i32;
pub const PWM_IOCTL_ID_PIN_GET_POLARITY: i32 = 102i32;
pub const PWM_IOCTL_ID_PIN_IS_STARTED: i32 = 106i32;
pub const PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 101i32;
pub const PWM_IOCTL_ID_PIN_SET_POLARITY: i32 = 103i32;
pub const PWM_IOCTL_ID_PIN_START: i32 = 104i32;
pub const PWM_IOCTL_ID_PIN_STOP: i32 = 105i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    pub Percentage: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_PIN_GET_POLARITY_OUTPUT {
    pub Polarity: PWM_POLARITY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_PIN_IS_STARTED_OUTPUT {
    pub IsStarted: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    pub Percentage: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWM_PIN_SET_POLARITY_INPUT {
    pub Polarity: PWM_POLARITY,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PWM_POLARITY(pub i32);
