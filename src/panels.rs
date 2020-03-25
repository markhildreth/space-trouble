use st_data::control_values::{
    FourSwitchValue, PushButtonValue, ToggleSwitchValue, VentControlValue,
};
use st_data::{Action, ClientMessage, ClientMessageProducer};
use st_device::controls::{
    Control, FourSwitch, PushButton, StatefulControl, ToggleSwitch, UpdateResult,
};
use st_device::{Device, Pin};

pub struct Panel {
    eigenthrottle: StatefulControl<ToggleSwitch, ToggleSwitchValue>,
    gelatinous_darkbucket: StatefulControl<ToggleSwitch, ToggleSwitchValue>,
    vent_hydrogen: StatefulControl<PushButton, PushButtonValue>,
    vent_water_vapor: StatefulControl<PushButton, PushButtonValue>,
    vent_waste: StatefulControl<PushButton, PushButtonValue>,
    vent_frustrations: StatefulControl<PushButton, PushButtonValue>,
    newtonian_fibermist: StatefulControl<FourSwitch, FourSwitchValue>,
}

impl Default for Panel {
    fn default() -> Panel {
        Panel {
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

impl Panel {
    pub fn update(&mut self, producer: &mut ClientMessageProducer, device: &Device) {
        if let UpdateResult::Change(value) = self.eigenthrottle.update(device) {
            let action = Action::Eigenthrottle(value);
            self.perform(producer, action);
        }

        if let UpdateResult::Change(value) = self.gelatinous_darkbucket.update(device) {
            let action = Action::GelatinousDarkbucket(value);
            self.perform(producer, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_hydrogen.update(device) {
            let action = Action::VentControl(VentControlValue::Hydrogen);
            self.perform(producer, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_water_vapor.update(device)
        {
            let action = Action::VentControl(VentControlValue::WaterVapor);
            self.perform(producer, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_waste.update(device) {
            let action = Action::VentControl(VentControlValue::Waste);
            self.perform(producer, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_frustrations.update(device)
        {
            let action = Action::VentControl(VentControlValue::Frustrations);
            self.perform(producer, action);
        }

        if let UpdateResult::Change(value) = self.newtonian_fibermist.update(device) {
            let action = Action::NewtonianFibermist(value);
            self.perform(producer, action);
        }
    }

    fn perform(&self, producer: &mut ClientMessageProducer, action: Action) {
        let msg = ClientMessage::ActionPerformed(action);
        producer.enqueue(msg).unwrap();
    }
}
