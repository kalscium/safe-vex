use vex_rt::peripherals::Peripherals;
use crate::{controller::Controller, port::PortManager};

/// The context (current state) of the robot
pub struct Context<'a> {
    /// The current state of the controller
    pub controller: Controller,
    /// The current tick of the robot
    pub tick: u16,
    /// A reference to the peripherals of the robot
    pub peripherals: &'a Peripherals,
    /// A mutable reference to the robot's `PortManager`
    pub port_manager: &'a mut PortManager,
}

impl<'a> Context<'a> {
    #[inline]
    pub(crate) fn new(tick: u16, peripherals: &'a Peripherals, port_manager: &'a mut PortManager) -> Self {
        Self {
            controller: Controller::new(peripherals),
            tick,
            peripherals,
            port_manager,
        }
    }
}
