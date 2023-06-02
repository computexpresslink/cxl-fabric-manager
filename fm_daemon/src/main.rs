/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM daemon implementation.
 *
 * Copyright (c) 2023 Viacheslav Dubeyko <slava@dubeyko.com>,
 * All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod discover;
mod fm;
mod switch;
mod multi_headed_device;
mod logical_device;
mod pci2pci_bridge;
mod physical_port;
mod mld_port;
mod dynamic_capacity_device;

extern crate daemonize;

use std::fs::File;
use daemonize::Daemonize;
use clap::{Arg, Command};
use std::{
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
};
pub use crate::discover::cxl_fm_discover_command;
pub use crate::fm::cxl_fm_command;
pub use crate::switch::cxl_switch_command;
pub use crate::multi_headed_device::cxl_mh_device_command;
pub use crate::logical_device::cxl_logical_device_command;
pub use crate::pci2pci_bridge::cxl_ppb_command;
pub use crate::physical_port::cxl_physical_port_command;
pub use crate::mld_port::cxl_mld_port_command;
pub use crate::dynamic_capacity_device::cxl_dcd_command;
pub use fm_library::cxl_fm_lib::send_responce;
pub use fm_library::cxl_fm_lib::CxlFmOptions;

/*
 * CXL FM daemon version
 */
const CXL_FM_DAEMON_VERSION: &str = "0.0.1";

/*
 * CXL FM daemon strings
 */
const CXL_FM_DAEMON_NAME: &str = "fm_daemon";
const CXL_FM_DAEMON_DESCRIPTOR: &str = "CXL Fabric Manager (FM) daemon";
const CXL_FM_DAEMON_DEBUG_OPTION: &str = "debug";
const CXL_FM_DAEMON_DEBUG_OPTION_SHORT: char = 'd';
const CXL_FM_DAEMON_IP_ADDRESS_OPTION: &str = "ip";
const CXL_FM_DAEMON_IP_ADDRESS_OPTION_SHORT: char = 'i';
const CXL_FM_DAEMON_PORT_OPTION: &str = "port";
const CXL_FM_DAEMON_PORT_OPTION_SHORT: char = 'p';

const CXL_FM_DAEMON_WORKING_DIRECTORY: &str = "/tmp";
const CXL_FM_DAEMON_LOG_FILE_PATH: &str = "/tmp/fm_daemon.log";
const CXL_FM_DAEMON_ERROR_MESSAGES_FILE_PATH: &str = "/tmp/fm_daemon.err";
const CXL_FM_DAEMON_USER: &str = "nobody";
const CXL_FM_DAEMON_GROUP: &str = "bin";
const CXL_FM_DAEMON_GROUP_ID: u32 = 2;
const CXL_FM_DAEMON_UMASK: u32 = 0o777;

/*
 * Command line interface definition
 */
fn cli() -> Command {
	Command::new(CXL_FM_DAEMON_NAME)
		.about(CXL_FM_DAEMON_DESCRIPTOR)
		.version(CXL_FM_DAEMON_VERSION)
		.arg_required_else_help(true)
		.arg(Arg::new(CXL_FM_DAEMON_DEBUG_OPTION)
			.short(CXL_FM_DAEMON_DEBUG_OPTION_SHORT)
			.long(CXL_FM_DAEMON_DEBUG_OPTION)
			.action(clap::ArgAction::SetTrue))
		.arg(Arg::new(CXL_FM_DAEMON_IP_ADDRESS_OPTION)
			.short(CXL_FM_DAEMON_IP_ADDRESS_OPTION_SHORT)
			.long(CXL_FM_DAEMON_IP_ADDRESS_OPTION)
			.action(clap::ArgAction::Set)
			.required(true))
		.arg(Arg::new(CXL_FM_DAEMON_PORT_OPTION)
			.short(CXL_FM_DAEMON_PORT_OPTION_SHORT)
			.long(CXL_FM_DAEMON_PORT_OPTION)
			.action(clap::ArgAction::Set)
			.required(true))
}

/*
OD * Connection request processing logic
 */
fn handle_connection(mut stream: &TcpStream, env: &CxlFmOptions) {
	if env.is_debug {
		println!("Process request...");
	}

	let buf_reader = BufReader::new(&mut stream);
	let request_line = buf_reader.lines().next().unwrap().unwrap();

	if env.is_debug {
		println!("Request: {:#?}", request_line);
	}

	match request_line.as_str() {
		fm_library::cxl_fm_lib::CXL_FM_DISCOVER_CXL_DEVICE_COMMAND => {
			cxl_fm_discover_command::discover_cxl_devices(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DISCOVER_CXL_SWITCH_COMMAND => {
			cxl_fm_discover_command::discover_cxl_switches(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DISCOVER_LD_COMMAND => {
			cxl_fm_discover_command::discover_logical_devices(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_FM_INFO_COMMAND => {
			cxl_fm_command::get_info(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_FM_CONFIG_COMMAND => {
			cxl_fm_command::get_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_SET_FM_CONFIG_COMMAND => {
			cxl_fm_command::set_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_FM_EVENTS_COMMAND => {
			cxl_fm_command::get_events(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_SWITCH_INFO_COMMAND => {
			cxl_switch_command::get_info(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_SWITCH_CONFIG_COMMAND => {
			cxl_switch_command::get_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_SET_SWITCH_CONFIG_COMMAND => {
			cxl_switch_command::set_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_MHD_INFO_COMMAND => {
			cxl_mh_device_command::get_info(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_BIND_LD_COMMAND => {
			cxl_logical_device_command::bind(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_UNBIND_LD_COMMAND => {
			cxl_logical_device_command::unbind(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_CONNECT_MLD_COMMAND => {
			cxl_logical_device_command::connect(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DISCONNECT_MLD_COMMAND => {
			cxl_logical_device_command::disconnect(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_LD_ALLOCATION_COMMAND => {
			cxl_logical_device_command::get_allocation(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_SET_LD_ALLOCATION_COMMAND => {
			cxl_logical_device_command::set_allocation(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_QOS_CONTROL_COMMAND => {
			cxl_logical_device_command::get_qos_control(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_SET_QOS_CONTROL_COMMAND => {
			cxl_logical_device_command::set_qos_control(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_QOS_STATUS_COMMAND => {
			cxl_logical_device_command::get_qos_status(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_QOS_BANDWIDTH_COMMAND => {
			cxl_logical_device_command::get_qos_bandwidth(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_SET_QOS_BANDWIDTH_COMMAND => {
			cxl_logical_device_command::set_qos_bandwidth(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_QOS_BANDWIDTH_LIMIT_COMMAND => {
			cxl_logical_device_command::get_qos_bandwidth_limit(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_SET_QOS_BANDWIDTH_LIMIT_COMMAND => {
			cxl_logical_device_command::set_qos_bandwidth_limit(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_LD_ERASE => {
			cxl_logical_device_command::erase(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_PPB_CONFIG_COMMAND => {
			cxl_ppb_command::config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_PPB_BIND_COMMAND => {
			cxl_ppb_command::bind(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_PPB_UNBIND_COMMAND => {
			cxl_ppb_command::unbind(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_GET_PHYSICAL_PORT_INFO_COMMAND => {
			cxl_physical_port_command::get_info(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND => {
			cxl_physical_port_command::control(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_BIND_PHYSICAL_PORT_COMMAND => {
			cxl_physical_port_command::bind(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_UNBIND_PHYSICAL_PORT_COMMAND => {
			cxl_physical_port_command::unbind(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_MLD_PORT_TUNNEL_COMMAND => {
			cxl_mld_port_command::tunnel(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND => {
			cxl_mld_port_command::send_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_MLD_PORT_SEND_MEM_REQ_COMMAND => {
			cxl_mld_port_command::send_memory_request(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DCD_GET_INFO_COMMAND => {
			cxl_dcd_command::get_info(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DCD_GET_CONFIG_COMMAND => {
			cxl_dcd_command::get_capacity_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DCD_SET_CONFIG_COMMAND => {
			cxl_dcd_command::set_capacity_config(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DCD_GET_EXTENT_COMMAND => {
			cxl_dcd_command::get_extent_list(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DCD_ADD_CAPACITY_COMMAND => {
			cxl_dcd_command::add_capacity(stream, env);
		},
		fm_library::cxl_fm_lib::CXL_FM_DCD_RELEASE_CAPACITY_COMMAND => {
			cxl_dcd_command::release_capacity(stream, env);
		},
		_ => send_responce(stream, fm_library::cxl_fm_lib::CXL_FM_UNKNOWN_COMMAND, env),
	}
}

/*
 * Main logic of daemon
 */
fn fm_daemon_logic(env: &CxlFmOptions) {
	if env.is_debug {
		println!("{} {}: Daemonized!",
			 CXL_FM_DAEMON_NAME, CXL_FM_DAEMON_VERSION);
	}

	loop {
		let listener = TcpListener::bind(&env.ip_port).unwrap();

		if env.is_debug {
			println!("Ready to accept connections: {}",
				 env.ip_port);
		}

		for stream in listener.incoming() {
			handle_connection(&stream.unwrap(), env);
		}
	};
}

/*
 * Application logic
 */
fn main() {
	let stdout = File::create(CXL_FM_DAEMON_LOG_FILE_PATH).unwrap();
	let stderr = File::create(CXL_FM_DAEMON_ERROR_MESSAGES_FILE_PATH).unwrap();

	let matches = cli().get_matches();

	let ip = matches.get_one::<String>(CXL_FM_DAEMON_IP_ADDRESS_OPTION).unwrap();
	let port = matches.get_one::<String>(CXL_FM_DAEMON_PORT_OPTION).unwrap();
	let ip_port = format!("{ip}:{port}");

	let options = CxlFmOptions {
		ip_port: String::from(ip_port),
		is_debug: matches.get_flag(CXL_FM_DAEMON_DEBUG_OPTION) == true,
	};

	if options.is_debug {
		println!("{} {}", CXL_FM_DAEMON_NAME, CXL_FM_DAEMON_VERSION);
	}

	let daemonize = Daemonize::new()
			// Every method except `new` and `start`
			// is optional, see `Daemonize` documentation
			// for default behaviour.
			.working_directory(CXL_FM_DAEMON_WORKING_DIRECTORY)
			.user(CXL_FM_DAEMON_USER)
			.group(CXL_FM_DAEMON_GROUP)	// Group name
			.group(CXL_FM_DAEMON_GROUP_ID)	// or group id.
			.umask(CXL_FM_DAEMON_UMASK)	// Set umask, `0o027` by default.
			.stdout(stdout)			// Redirect stdout to log file.
			.stderr(stderr)			// Redirect stderr to error messages file.
			.privileged_action(|| "Executed before drop privileges");

	match daemonize.start() {
		Ok(_) => fm_daemon_logic(&options),
		Err(e) => eprintln!("Error, {}", e),
	}
}
