use st_device::controls::{Debounce, FourSwitch, PushButton, ToggleSwitch};
use st_device::Pin;

struct PanelOneControls {
    eigenthrottle: ToggleSwitch,
    gelatinous_darkbucket: ToggleSwitch,
    vent_hydrogen: PushButton,
    vent_water_vapor: PushButton,
    vent_waste: PushButton,
    vent_frustrations: PushButton,
    newtonian_fibermist: Debounce<FourSwitch>,
}

impl Default for PanelOneControls {
    fn default() -> PanelOneControls {
        PanelOneControls {
            eigenthrottle: ToggleSwitch::new(Pin::D5),
            gelatinous_darkbucket: ToggleSwitch::new(Pin::D6),
            vent_hydrogen: PushButton::new(Pin::A2),
            vent_water_vapor: PushButton::new(Pin::A3),
            vent_waste: PushButton::new(Pin::A4),
            vent_frustrations: PushButton::new(Pin::A5),
            // TODO: See if controls can derive Debounceable so that we can do
            // FourSwitch::new(...).debounce(400)
            newtonian_fibermist: Debounce::new(FourSwitch::new(Pin::D10, Pin::D11, Pin::D12), 400),
        }
    }
}

struct PanelOneResult {
    eigenthrottle: ToggleSwitch,
    gelatinous_darkbucket: ToggleSwitch,
    vent_hydrogen: PushButton,
    vent_water_vapor: PushButton,
    vent_waste: PushButton,
    vent_frustrations: PushButton,
    newtonian_fibermist: FourSwitch,
}

/*
impl<T> DeviceReader<T>
    where T: Device
{

    fn new(device: &mut T) -> Self {
        PanelOneControls {
            eigenthrottle: self.eigenthrottle.update(device),
            ...
    }
}


struct ToggleSwitch {
    pin: Pin
}

trait Control<TP: PinSource, TC: Control> {
    fn update(&mut self, &mut TP) -> PinChange<TC>;

    fn debounce(self: TC) -> Debounce<TC> {
        Debounce::new(self)
    }
}

impl<T> Control<T> for ToggleSwitch
    where T: PinSource
{
    fn update(&mut T) -> PinChange<ToggleSwitch>

    }
}

struct Debounce<TP: PinSource, TC: Control<TP>> {
    inner: TC
}

impl<TP: PinSource, TC: Control<TP>> Control<TP> for Debounce {

}
*/
