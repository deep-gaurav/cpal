pub struct AudioDeviceInfo {
    pub id: i32,
    pub device_type: AudioDeviceType,
    pub direction: AudioDeviceDirection,
    pub address: String,
    pub product_name: String,
    pub channel_counts: Vec<i32>,
    pub sample_rates: Vec<i32>,
    pub formats: Vec<AudioFormat>,
}

#[non_exhaustive]
#[repr(i32)]
pub enum AudioDeviceType {
    Unknown,
    AuxLine,
    BluetoothA2DP,
    BluetoothSCO,
    BuiltinEarpiece,
    BuiltinMic,
    BuiltinSpeaker,
    BuiltinSpeakerSafe,
    Bus,
    Dock,
    Fm,
    FmTuner,
    Hdmi,
    HdmiArc,
    HearingAid,
    Ip,
    LineAnalog,
    LineDigital,
    Telephony,
    TvTuner,
    UsbAccessory,
    UsbDevice,
    UsbHeadset,
    WiredHeadphones,
    WiredHeadset,
}

#[repr(i32)]
pub enum AudioDeviceDirection {
    Dumb,
    Input,
    Output,
    InputOutput,
}

#[repr(i32)]
pub enum AudioFormat {
    Invalid,
    Unspecified,
    I16,
    I24,
    I32,
    F32,
}