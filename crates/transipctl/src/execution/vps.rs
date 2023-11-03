use super::ToJson;
use crate::Result;
use transip::{api::vps::VpsApi, Client};
use transip_command::{VpsAction, VpsCommand};

pub fn execute(command: VpsCommand, client: &mut Client) -> Result<String> {
    match command {
        VpsCommand::Action(name, action) => match action {
            VpsAction::Item => client.vps(&name).and_then_json(),
            VpsAction::Lock => client.vps_set_is_locked(&name, true).and_then_json(),
            VpsAction::Reset => client.vps_reset(&name).and_then_json(),
            VpsAction::Start => client.vps_start(&name).and_then_json(),
            VpsAction::Stop => client.vps_stop(&name).and_then_json(),
            VpsAction::Unlock => client.vps_set_is_locked(&name, false).and_then_json(),
        },
        VpsCommand::List => client.vps_list().and_then_json(),
    }
}
