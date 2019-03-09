use evdev_rs::enums::{
    int_to_bus_type, int_to_ev_key, int_to_ev_led, int_to_ev_msc, int_to_ev_syn, int_to_input_prop,
    BusType, EventCode, EventType, InputProp, EV_KEY, EV_LED, EV_MSC, EV_REP, EV_SYN,
};
use evdev_rs::Device;

pub fn device_from_config(device_config: &DeviceConfig) -> Device {
    let dev = Device::new().expect("failed to create device");

    if let Some(name) = &device_config.name {
        dev.set_name(&name);
    }

    if let Some(phys) = &device_config.phys {
        dev.set_phys(&phys);
    }

    if let Some(uniq) = &device_config.uniq {
        dev.set_uniq(&uniq);
    }

    dev.set_bustype(device_config.bustype);
    dev.set_version(device_config.version);
    dev.set_vendor_id(device_config.vendor_id);
    dev.set_product_id(device_config.product_id);

    for input_prop in &device_config.input_properties {
        dev.enable(input_prop)
            .expect("failed to enable input property");
    }

    dev.enable(&EventType::EV_SYN)
        .expect("failed to enable EV_SYN type");
    for syn in &device_config.events.ev_syn {
        let event_code = &EventCode::EV_SYN(syn.clone());
        dev.enable(event_code)
            .expect("failed to enabled EV_SYN events");
    }

    dev.enable(&EventType::EV_KEY)
        .expect("failed to enable EV_KEY type");
    for key in &device_config.events.ev_key {
        let event_code = &EventCode::EV_KEY(key.clone());
        dev.enable(event_code)
            .expect("failed to enabled EV_KEY events");
    }

    dev.enable(&EventType::EV_MSC)
        .expect("failed to enable EV_MSC type");
    for msc in &device_config.events.ev_msc {
        let event_code = &EventCode::EV_MSC(msc.clone());
        dev.enable(event_code)
            .expect("failed to enabled EV_MSC events");
    }

    dev.enable(&EventType::EV_LED)
        .expect("failed to enable EV_LED type");
    for led in &device_config.events.ev_led {
        let event_code = &EventCode::EV_LED(led.clone());
        dev.enable(event_code)
            .expect("failed to enabled EV_LED events");
    }

    dev.enable(&EventType::EV_REP)
        .expect("failed to enable EV_REP type");
    dev.enable_event_code(
        &EventCode::EV_REP(EV_REP::REP_DELAY),
        Some(&device_config.events.ev_rep.repeat_delay),
    )
    .expect("failed to set REP_DELAY");
    dev.enable_event_code(
        &EventCode::EV_REP(EV_REP::REP_MAX),
        Some(&device_config.events.ev_rep.repeat_period),
    )
    .expect("failed to set REP_MAX");

    dev
}

#[derive(Debug)]
pub struct DeviceConfig {
    pub name: Option<String>,
    pub phys: Option<String>,
    pub uniq: Option<String>,
    pub product_id: i32,
    pub vendor_id: i32,
    pub bustype: i32,
    pub bustype_string: Option<BusType>,
    pub version: i32,
    pub driver_version: i32,
    pub input_properties: Vec<InputProp>,
    pub events: DeviceConfigEvents,
}

impl DeviceConfig {
    pub fn from_device(device: &Device) -> DeviceConfig {
        let mut input_properties = vec![];
        for input_prop in int_to_input_prop(0).iter() {
            if device.has(input_prop) {
                input_properties.push(input_prop.clone());
            }
        }

        let bustype = device.bustype();
        DeviceConfig {
            name: device.name().map(|s| s.to_string()),
            phys: device.phys().map(|s| s.to_string()),
            uniq: device.uniq().map(|s| s.to_string()),
            bustype,
            bustype_string: int_to_bus_type(bustype as u32),
            product_id: device.product_id(),
            vendor_id: device.vendor_id(),
            version: device.version(),
            driver_version: device.driver_version(),
            input_properties,
            events: DeviceConfigEvents::from_device(device),
        }
    }
}

#[derive(Debug)]
pub struct DeviceConfigEvents {
    // TODO: ev_abs ?
    pub ev_syn: Vec<EV_SYN>,
    pub ev_key: Vec<EV_KEY>,
    pub ev_msc: Vec<EV_MSC>,
    pub ev_led: Vec<EV_LED>,
    pub ev_rep: DeviceConfigRepeatEvents,
}

impl DeviceConfigEvents {
    pub fn from_device(device: &Device) -> DeviceConfigEvents {
        let mut ev_syn = vec![];
        for i in 0..EventType::get_max(&EventType::EV_SYN).unwrap() {
            if let Some(syn) = int_to_ev_syn(i as u32) {
                if device.has(&EventCode::EV_SYN(syn.clone())) {
                    ev_syn.push(syn);
                }
            }
        }

        let mut ev_key = vec![];
        for i in 0..EventType::get_max(&EventType::EV_KEY).unwrap() {
            if let Some(key) = int_to_ev_key(i as u32) {
                if device.has(&EventCode::EV_KEY(key.clone())) {
                    ev_key.push(key);
                }
            }
        }

        let mut ev_msc = vec![];
        for i in 0..EventType::get_max(&EventType::EV_MSC).unwrap() {
            if let Some(msc) = int_to_ev_msc(i as u32) {
                if device.has(&EventCode::EV_MSC(msc.clone())) {
                    ev_msc.push(msc);
                }
            }
        }

        let mut ev_led = vec![];
        for i in 0..EventType::get_max(&EventType::EV_LED).unwrap() {
            if let Some(led) = int_to_ev_led(i as u32) {
                if device.has(&EventCode::EV_LED(led.clone())) {
                    ev_led.push(led);
                }
            }
        }

        let ev_rep = DeviceConfigRepeatEvents::from_device(device);
        DeviceConfigEvents {
            ev_syn,
            ev_key,
            ev_msc,
            ev_led,
            ev_rep,
        }
    }
}

#[derive(Debug)]
pub struct DeviceConfigRepeatEvents {
    pub repeat_delay: i32,
    pub repeat_period: i32,
}

impl DeviceConfigRepeatEvents {
    pub fn from_device(device: &Device) -> DeviceConfigRepeatEvents {
        DeviceConfigRepeatEvents {
            repeat_delay: device
                .event_value(&EventCode::EV_REP(EV_REP::REP_DELAY))
                .expect("failed to read repeat_delay from device"),
            repeat_period: device
                .event_value(&EventCode::EV_REP(EV_REP::REP_MAX))
                .expect("failed to read repeat_period from device"),
        }
    }
}
