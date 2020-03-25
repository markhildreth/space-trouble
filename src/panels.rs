use st_data::control_values::{FourSwitchValue, PushButtonValue, ToggleSwitchValue};
use st_device::controls::{Control, FourSwitch, PushButton, StatefulControl, ToggleSwitch};
use st_device::Pin;

pub struct PanelOneControls {
    eigenthrottle: StatefulControl<ToggleSwitch, ToggleSwitchValue>,
    gelatinous_darkbucket: StatefulControl<ToggleSwitch, ToggleSwitchValue>,
    vent_hydrogen: StatefulControl<PushButton, PushButtonValue>,
    vent_water_vapor: StatefulControl<PushButton, PushButtonValue>,
    vent_waste: StatefulControl<PushButton, PushButtonValue>,
    vent_frustrations: StatefulControl<PushButton, PushButtonValue>,
    newtonian_fibermist: StatefulControl<FourSwitch, FourSwitchValue>,
}

impl Default for PanelOneControls {
    fn default() -> PanelOneControls {
        PanelOneControls {
            eigenthrottle: ToggleSwitch::new(Pin::D5).stateful(),
            gelatinous_darkbucket: ToggleSwitch::new(Pin::D6).stateful(),
            vent_hydrogen: PushButton::new(Pin::A2).stateful(),
            vent_water_vapor: PushButton::new(Pin::A3).stateful(),
            vent_waste: PushButton::new(Pin::A4).stateful(),
            vent_frustrations: PushButton::new(Pin::A5).stateful(),
            newtonian_fibermist: FourSwitch::new(Pin::D10, Pin::D11, Pin::D12).stateful(),
        }
    }
}

/*
impl PanelOneControls {
    fn update(&mut self, device: &Device) -> PanelOneReadResult {}
}
*/

pub struct PanelOneResult {
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
