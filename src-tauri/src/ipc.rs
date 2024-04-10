mod bot_config;
mod frontend_info;

pub use self::{
    bot_config::{BotConfig, BotMode, FarmingConfig, ShoutConfig, SlotType, SupportConfig, CommonConfig},
    frontend_info::FrontendInfo,
};
