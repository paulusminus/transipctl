use transip::{Client, api::vps::Vps};
use transip::api::vps::TransipApiVps;

use crate::{error::Error, Execute, Result};

#[derive(Debug)]
pub(crate) struct VpsListCommand;

impl Execute for VpsListCommand {
    type ApiResult = Vec<Vps>;
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps_list().map_err(Error::from)
    }
}

#[derive(Debug)]
pub(crate) struct VpsStartCommand(pub(crate) String);

impl Execute for VpsStartCommand {
    type ApiResult = ();
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps_start(self.0.as_str()).map_err(Error::from)
    }
}

#[derive(Debug)]
pub(crate) struct VpsStopCommand(pub(crate) String);

impl Execute for VpsStopCommand {
    type ApiResult = ();
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps_stop(self.0.as_str()).map_err(Error::from)
    }
}

#[derive(Debug)]
pub(crate) struct VpsResetCommand(pub(crate) String);

impl Execute for VpsResetCommand {
    type ApiResult = ();
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps_reset(self.0.as_str()).map_err(Error::from)
    }
}

#[derive(Debug)]
pub(crate) struct VpsItemCommand(pub(crate) String);

impl Execute for VpsItemCommand {
    type ApiResult = Vps;
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps(self.0.as_str()).map_err(Error::from)
    }
}

#[derive(Debug)]
pub(crate) struct VpsLockCommand(pub(crate) String);

impl Execute for VpsLockCommand {
    type ApiResult = ();
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps_set_is_locked(self.0.as_str(), true).map_err(Error::from)
    }
}

#[derive(Debug)]
pub(crate) struct VpsUnlockCommand(pub(crate) String);

impl Execute for VpsUnlockCommand {
    type ApiResult = ();
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult> {
        client.vps_set_is_locked(self.0.as_str(), false).map_err(Error::from)
    }
}
