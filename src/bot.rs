use core::time::Duration;

use vex_rt::{peripherals::Peripherals, robot, rtos::{Loop, Mutex}, select};
use crate::{context::Context, port::PortManager};

/// A safe translation layer to convert a user defiend Bot into a vex competition Robot struct
pub struct Robot<UserBot: Bot> {
    /// The user defined robot code
    custom: Mutex<UserBot>,
    /// Manages the ports of the robot for safety
    port_manager: Mutex<PortManager>,
    /// The peripherals of the robot
    peripherals: Peripherals,
}

/// Represents a user implemented, Vex VRC robot
///
/// *Your robot should implement this*
pub trait Bot: Sync + Send + 'static{
    /// The tickspeed of the robot in **milliseconds** (how long does each cycle lasts)
    const TICK_SPEED: u64;
    
    /// Creates a new instance of your bot
    fn new(peripherals: &Peripherals, port_manager: &mut PortManager) -> Self;

    /// Run each tick (runtime cycle) of `opcontrol` and returns if it has completed
    #[allow(unused_variables)]
    fn opcontrol(&mut self, context: Context) -> bool { true }
    /// Run each tick (runtime cycle) of `autonomous` and returns if it has completed
    #[allow(unused_variables)]
    fn autonomous(&mut self, context: Context) -> bool { true }
    /// Run each tick (runtime cycle) of `disabled` and returns if it has completed
    #[allow(unused_variables)]
    fn disabled(&mut self, context: Context) -> bool { true }
}

macro_rules! cycled {
    ($name:ident, $bot:ty) => {
        #[inline]
        fn $name(&mut self, context: vex_rt::prelude::Context) {
            let mut l = Loop::new(Duration::from_millis(<$bot>::TICK_SPEED));
            let mut tick = 0;

            loop {
                tick += 1;

                if let (Some(ref mut user_defined), Some(ref mut port_manager)) = (self.custom.poll(), self.port_manager.poll()) {
                    if user_defined.$name(Context::new(
                        tick,
                        &self.peripherals,
                        port_manager,
                    )) { break };
                } else { continue }

                select! {
                    _ = context.done() => continue,
                    _ = l.select() => continue,
                }
            }
        }
    }
}

impl<UserBot: Bot> robot::Robot for Robot<UserBot> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let mut port_manager = PortManager::new();
        
        Self {
            custom: Mutex::new(Bot::new(&peripherals, &mut port_manager)),
            port_manager: Mutex::new(port_manager),
            peripherals,
        }
    }

    cycled!(opcontrol, UserBot);
    cycled!(autonomous, UserBot);
    cycled!(disabled, UserBot);
}
