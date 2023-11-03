use super::{Execution, ToJson};
use crate::{
    command::vps::{VpsAction, VpsCommand},
    Result,
};
use transip::{api::vps::VpsApi, Client};

impl Execution for VpsCommand {
    fn execute(&self, client: &mut Client) -> Result<String> {
        match self {
            Self::Action(name, action) => match action {
                VpsAction::Item => client.vps(name).and_then_json(),
                VpsAction::Lock => client.vps_set_is_locked(name, true).and_then_json(),
                VpsAction::Reset => client.vps_reset(name).and_then_json(),
                VpsAction::Start => client.vps_start(name).and_then_json(),
                VpsAction::Stop => client.vps_stop(name).and_then_json(),
                VpsAction::Unlock => client.vps_set_is_locked(name, false).and_then_json(),
            },
            Self::List => client.vps_list().and_then_json(),
        }
    }
}
