use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6};

use bevy::prelude::*;
use directories::ProjectDirs;

pub mod name_list;
pub mod network;
pub mod save_system;

pub const NAME: &str = "Awesome Vehicle Builder";
pub const DIR_NAME: &str = "awesome_vehicle_builder";

pub const DEFAULT_PORT: u16 = 16565;

pub type Name = String;

#[derive(Debug)]
pub struct CommonPlugins;

impl Plugin for CommonPlugins {
    fn build(&self, app: &mut App) {}
}

pub fn ip_addr_into_socket_addr(ip: IpAddr, port: u16) -> SocketAddr {
    match ip {
        IpAddr::V4(ipv4) => SocketAddr::V4(SocketAddrV4::new(ipv4, port)),
        IpAddr::V6(ipv6) => SocketAddr::V6(SocketAddrV6::new(ipv6, port, 0, 0)),
    }
}

pub enum Paths {
    GameSave(Name),
    VehicleSave(Name),
    SettingsSave,
}

impl Into<std::path::PathBuf> for Paths {
    fn into(self) -> std::path::PathBuf {
        let game_dirs = get_game_dirs();
        match self {
            Self::GameSave(name) => {
                let mut dir = game_dirs.data_dir().to_path_buf();
                dir.push(name);

                dir
            }
            Self::VehicleSave(name) => {
                let mut dir = game_dirs.data_dir().to_path_buf();
                dir.push(name);

                dir
            }
            Self::SettingsSave => game_dirs.config_dir().to_path_buf(),
        }
    }
}

pub fn get_game_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", DIR_NAME).expect("Failed to get a valid home directory")
}
