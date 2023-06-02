/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM configuration tool implementation.
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

use clap::{Arg, Command};
pub use crate::discover::cxl_fm_discover_command;
pub use crate::fm::cxl_fm_command;
pub use crate::switch::cxl_switch_command;
pub use crate::multi_headed_device::cxl_mh_device_command;
pub use crate::logical_device::cxl_logical_device_command;
pub use crate::pci2pci_bridge::cxl_ppb_command;
pub use crate::physical_port::cxl_physical_port_command;
pub use crate::mld_port::cxl_mld_port_command;
pub use crate::dynamic_capacity_device::cxl_dcd_command;
pub use fm_library::cxl_fm_lib::CxlFmOptions;

/*
 * CXL FM configuration tool version
 */
const CXL_FM_TOOL_VERSION: &str = "0.0.1";

/*
 * CXL FM configuration tool strings
 */
const CXL_FM_TOOL_NAME: &str = "fm_cli";
const CXL_FM_TOOL_DESCRIPTOR: &str = "CXL Fabric Manager (FM) CLI";
const CXL_FM_TOOL_DEBUG_OPTION: &str = "debug";
const CXL_FM_TOOL_DEBUG_OPTION_SHORT: char = 'd';
const CXL_FM_TOOL_IP_ADDRESS_OPTION: &str = "ip";
const CXL_FM_TOOL_IP_ADDRESS_OPTION_SHORT: char = 'i';
const CXL_FM_TOOL_PORT_OPTION: &str = "port";
const CXL_FM_TOOL_PORT_OPTION_SHORT: char = 'p';

/*
 * Discover command strings
 */
const CXL_FM_DISCOVER_COMMAND: &str = "discover";
const CXL_FM_DISCOVER_COMMAND_DESCRIPTOR: &str = "Discover available CXL agents";
const CXL_FM_DISCOVER_FM_COMMAND: &str = "fm";
const CXL_FM_DISCOVER_FM_COMMAND_DESCRIPTOR: &str = "Discover FM instances";
const CXL_FM_DISCOVER_DEVICES_COMMAND: &str = "cxl_devices";
const CXL_FM_DISCOVER_DEVICES_COMMAND_DESCRIPTOR: &str = "Discover CXL devices";
const CXL_FM_DISCOVER_SWITCHES_COMMAND: &str = "cxl_switch";
const CXL_FM_DISCOVER_SWITCHES_COMMAND_DESCRIPTOR: &str = "Discover CXL switches";
const CXL_FM_DISCOVER_LOGICAL_DEVICES_COMMAND: &str = "logical_devices";
const CXL_FM_DISCOVER_LOGICAL_DEVICES_COMMAND_DESCRIPTOR: &str = "Discover logical devices";

/*
 * FM command strings
 */
const CXL_FM_COMMAND: &str = "fm";
const CXL_FM_COMMAND_DESCRIPTOR: &str = "Manage Fabric Manager (FM)";
const CXL_FM_GET_INFO_COMMAND: &str = "get_info";
const CXL_FM_GET_INFO_COMMAND_DESCRIPTOR: &str = "Get Fabric Manager (FM) status/info";
const CXL_FM_START_COMMAND: &str = "start";
const CXL_FM_START_COMMAND_DESCRIPTOR: &str = "Start Fabric Manager (FM) instance";
const CXL_FM_RESTART_COMMAND: &str = "restart";
const CXL_FM_RESTART_COMMAND_DESCRIPTOR: &str = "Restart Fabric Manager (FM) instance";
const CXL_FM_STOP_COMMAND: &str = "stop";
const CXL_FM_STOP_COMMAND_DESCRIPTOR: &str = "Stop Fabric Manager (FM) instance";
const CXL_FM_GET_CONFIG_COMMAND: &str = "get_config";
const CXL_FM_GET_CONFIG_COMMAND_DESCRIPTOR: &str = "Get Fabric Manager (FM) configuration";
const CXL_FM_SET_CONFIG_COMMAND: &str = "set_config";
const CXL_FM_SET_CONFIG_COMMAND_DESCRIPTOR: &str = "Set Fabric Manager (FM) configuration";
const CXL_FM_GET_EVENTS_COMMAND: &str = "get_events";
const CXL_FM_GET_EVENTS_COMMAND_DESCRIPTOR: &str = "Get Fabric Manager (FM) event records";

/*
 * Switch command strings
 */
const CXL_FM_SWITCH_COMMAND: &str = "switch";
const CXL_FM_SWITCH_COMMAND_DESCRIPTOR: &str = "Manage CXL switch";
const CXL_FM_SWITCH_GET_INFO_COMMAND: &str = "get_info";
const CXL_FM_SWITCH_GET_INFO_COMMAND_DESCRIPTOR: &str = "Get CXL switch status/info";
const CXL_FM_SWITCH_GET_CONFIG_COMMAND: &str = "get_config";
const CXL_FM_SWITCH_GET_CONFIG_COMMAND_DESCRIPTOR: &str = "Get CXL switch configuration";
const CXL_FM_SWITCH_SET_CONFIG_COMMAND: &str = "set_config";
const CXL_FM_SWITCH_SET_CONFIG_COMMAND_DESCRIPTOR: &str = "Set CXL switch configuration";

/*
 * Multi Headed Device (MHD) command strings
 */
const CXL_FM_MH_DEVICE_COMMAND: &str = "mh_device";
const CXL_FM_MH_DEVICE_COMMAND_DESCRIPTOR: &str = "Manage Multi Headed Device (MHD)";
const CXL_FM_MH_DEVICE_GET_INFO_COMMAND: &str = "get_info";
const CXL_FM_MH_DEVICE_GET_INFO_COMMAND_DESCRIPTOR: &str = "Get Multi Headed Device (MHD) status/info";

/*
 * Logical Device (LD) command strings
 */
const CXL_FM_LOGICAL_DEVICE_COMMAND: &str = "logical_device";
const CXL_FM_LOGICAL_DEVICE_COMMAND_DESCRIPTOR: &str = "Manage Logical Device (LD)";
const CXL_FM_LOGICAL_DEVICE_BIND_COMMAND: &str = "bind";
const CXL_FM_LOGICAL_DEVICE_BIND_COMMAND_DESCRIPTOR: &str = "Bind Logical Device (LD)";
const CXL_FM_LOGICAL_DEVICE_UNBIND_COMMAND: &str = "unbind";
const CXL_FM_LOGICAL_DEVICE_UNBIND_COMMAND_DESCRIPTOR: &str = "Unbind Logical Device (LD)";
const CXL_FM_LOGICAL_DEVICE_CONNECT_COMMAND: &str = "connect";
const CXL_FM_LOGICAL_DEVICE_CONNECT_COMMAND_DESCRIPTOR: &str = "Connect Multi Logical Device (MLD) to CXL switch";
const CXL_FM_LOGICAL_DEVICE_DISCONNECT_COMMAND: &str = "disconnect";
const CXL_FM_LOGICAL_DEVICE_DISCONNECT_COMMAND_DESCRIPTOR: &str = "Disconnect Multi Logical Device (MLD) from CXL switch";
const CXL_FM_LOGICAL_DEVICE_GET_ALLOCATION_COMMAND: &str = "get_allocation";
const CXL_FM_LOGICAL_DEVICE_GET_ALLOCATION_COMMAND_DESCRIPTOR: &str = "Get Logical Device (LD) allocations";
const CXL_FM_LOGICAL_DEVICE_SET_ALLOCATION_COMMAND: &str = "set_allocation";
const CXL_FM_LOGICAL_DEVICE_SET_ALLOCATION_COMMAND_DESCRIPTOR: &str = "Set Logical Device (LD) allocations";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_CONTROL_COMMAND: &str = "get_qos_control";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_CONTROL_COMMAND_DESCRIPTOR: &str = "Get QoS control";
const CXL_FM_LOGICAL_DEVICE_SET_QOS_CONTROL_COMMAND: &str = "set_qos_control";
const CXL_FM_LOGICAL_DEVICE_SET_QOS_CONTROL_COMMAND_DESCRIPTOR: &str = "Set QoS control";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_STATUS_COMMAND: &str = "get_qos_status";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_STATUS_COMMAND_DESCRIPTOR: &str = "Get QoS status";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_COMMAND: &str = "get_qos_allocated_bandwidth";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_COMMAND_DESCRIPTOR: &str = "Get QoS allocated bandwidth";
const CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_COMMAND: &str = "set_qos_allocated_bandwidth";
const CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_COMMAND_DESCRIPTOR: &str = "Set QoS allocated bandwidth";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_LIMIT_COMMAND: &str = "get_qos_bandwidth_limit";
const CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_LIMIT_COMMAND_DESCRIPTOR: &str = "Get QoS bandwidth limit";
const CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_LIMIT_COMMAND: &str = "set_qos_bandwidth_limit";
const CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_LIMIT_COMMAND_DESCRIPTOR: &str = "Set QoS bandwidth limit";
const CXL_FM_LOGICAL_DEVICE_ERASE_COMMAND: &str = "erase";
const CXL_FM_LOGICAL_DEVICE_ERASE_COMMAND_DESCRIPTOR: &str = "Secure erase after unbinding";

/*
 * PCI-to-PCI Bridge (PPB) command strings
 */
const CXL_FM_PPB_COMMAND: &str = "ppb";
const CXL_FM_PPB_COMMAND_DESCRIPTOR: &str = "Manage PCI-to-PCI Bridge (PPB)";
const CXL_FM_PPB_CONFIG_COMMAND: &str = "config";
const CXL_FM_PPB_CONFIG_COMMAND_DESCRIPTOR: &str = "Send PCI-to-PCI Bridge (PPB) configuration request";
const CXL_FM_PPB_BIND_COMMAND: &str = "bind";
const CXL_FM_PPB_BIND_COMMAND_DESCRIPTOR: &str = "Bind Virtual PCI-to-PCI Bridge (vPPB) inside a CXL switch";
const CXL_FM_PPB_UNBIND_COMMAND: &str = "unbind";
const CXL_FM_PPB_UNBIND_COMMAND_DESCRIPTOR: &str = "Unbind Virtual PCI-to-PCI Bridge (vPPB) inside a CXL switch";

/*
 * Physical port command strings
 */
const CXL_FM_PHYSICAL_PORT_COMMAND: &str = "physical_port";
const CXL_FM_PHYSICAL_PORT_COMMAND_DESCRIPTOR: &str = "Manage physical ports";
const CXL_FM_PHYSICAL_PORT_GET_INFO_COMMAND: &str = "get_info";
const CXL_FM_PHYSICAL_PORT_GET_INFO_COMMAND_DESCRIPTOR: &str = "Get state of physical port";
const CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND: &str = "control";
const CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND_DESCRIPTOR: &str = "Control physical port";
const CXL_FM_PHYSICAL_PORT_BIND_COMMAND: &str = "bind";
const CXL_FM_PHYSICAL_PORT_BIND_COMMAND_DESCRIPTOR: &str = "Bind physical port to Virtual PCI-to-PCI Bridge (vPPB)";
const CXL_FM_PHYSICAL_PORT_UNBIND_COMMAND: &str = "unbind";
const CXL_FM_PHYSICAL_PORT_UNBIND_COMMAND_DESCRIPTOR: &str = "Unbind physical port from Virtual PCI-to-PCI Bridge (vPPB)";

/*
 * Multi-Logical Device (MLD) ports command strings
 */
const CXL_FM_MLD_PORT_COMMAND: &str = "mld_port";
const CXL_FM_MLD_PORT_COMMAND_DESCRIPTOR: &str = "Manage Multi-Logical Device (MLD) ports";
const CXL_FM_MLD_PORT_TUNNEL_COMMAND: &str = "tunnel";
const CXL_FM_MLD_PORT_TUNNEL_COMMAND_DESCRIPTOR: &str = "Tunnel Management Command";
const CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND: &str = "send_config";
const CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND_DESCRIPTOR: &str = "Send CXL.io configuration request";
const CXL_FM_MLD_PORT_SEND_MEM_REQUEST_COMMAND: &str = "send_memory_request";
const CXL_FM_MLD_PORT_SEND_MEM_REQUEST_COMMAND_DESCRIPTOR: &str = "Send CXL.io memory request";

/*
 * Dynamic Capacity Device (DCD) command strings
 */
const CXL_FM_DCD_COMMAND: &str = "dcd";
const CXL_FM_DCD_COMMAND_DESCRIPTOR: &str = "Manage Dynamic Capacity Device (DCD)";
const CXL_FM_DCD_GET_INFO_COMMAND: &str = "get_info";
const CXL_FM_DCD_GET_INFO_COMMAND_DESCRIPTOR: &str = "Get Dynamic Capacity Device (DCD) info";
const CXL_FM_DCD_GET_CONFIG_COMMAND: &str = "get_capacity_config";
const CXL_FM_DCD_GET_CONFIG_COMMAND_DESCRIPTOR: &str = "Get dynamic capacity region configuration";
const CXL_FM_DCD_SET_CONFIG_COMMAND: &str = "set_capacity_config";
const CXL_FM_DCD_SET_CONFIG_COMMAND_DESCRIPTOR: &str = "Set dynamic capacity region configuration";
const CXL_FM_DCD_GET_EXTENT_COMMAND: &str = "get_extent_list";
const CXL_FM_DCD_GET_EXTENT_COMMAND_DESCRIPTOR: &str = "Get Dynamic Capacity Device (DCD) extent list";
const CXL_FM_DCD_ADD_CAPACITY_COMMAND: &str = "add_capacity";
const CXL_FM_DCD_ADD_CAPACITY_COMMAND_DESCRIPTOR: &str = "Initiate dynamic capacity add";
const CXL_FM_DCD_RELEASE_CAPACITY_COMMAND: &str = "release_capacity";
const CXL_FM_DCD_RELEASE_CAPACITY_COMMAND_DESCRIPTOR: &str = "Initiate dynamic capacity release";

/*
 * Command line interface definition
 */
fn cli() -> Command {
	Command::new(CXL_FM_TOOL_NAME)
		.about(CXL_FM_TOOL_DESCRIPTOR)
		.version(CXL_FM_TOOL_VERSION)
		.subcommand_required(true)
		.arg_required_else_help(true)
		.allow_external_subcommands(true)
		.arg(Arg::new(CXL_FM_TOOL_DEBUG_OPTION)
			.short(CXL_FM_TOOL_DEBUG_OPTION_SHORT)
			.long(CXL_FM_TOOL_DEBUG_OPTION)
			.action(clap::ArgAction::SetTrue))
		.arg(Arg::new(CXL_FM_TOOL_IP_ADDRESS_OPTION)
			.short(CXL_FM_TOOL_IP_ADDRESS_OPTION_SHORT)
			.long(CXL_FM_TOOL_IP_ADDRESS_OPTION)
			.action(clap::ArgAction::Set)
			.required(true))
		.arg(Arg::new(CXL_FM_TOOL_PORT_OPTION)
			.short(CXL_FM_TOOL_PORT_OPTION_SHORT)
			.long(CXL_FM_TOOL_PORT_OPTION)
			.action(clap::ArgAction::Set)
			.required(true))
		.subcommand(
			Command::new(CXL_FM_DISCOVER_COMMAND)
				.about(CXL_FM_DISCOVER_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_DISCOVER_FM_COMMAND)
						.about(CXL_FM_DISCOVER_FM_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DISCOVER_DEVICES_COMMAND)
						.about(CXL_FM_DISCOVER_DEVICES_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DISCOVER_SWITCHES_COMMAND)
						.about(CXL_FM_DISCOVER_SWITCHES_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DISCOVER_LOGICAL_DEVICES_COMMAND)
						.about(CXL_FM_DISCOVER_LOGICAL_DEVICES_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_COMMAND)
				.about(CXL_FM_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_GET_INFO_COMMAND)
						.about(CXL_FM_GET_INFO_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_START_COMMAND)
						.about(CXL_FM_START_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_RESTART_COMMAND)
						.about(CXL_FM_RESTART_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_STOP_COMMAND)
						.about(CXL_FM_STOP_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_GET_CONFIG_COMMAND)
						.about(CXL_FM_GET_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_SET_CONFIG_COMMAND)
						.about(CXL_FM_SET_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_GET_EVENTS_COMMAND)
						.about(CXL_FM_GET_EVENTS_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_SWITCH_COMMAND)
				.about(CXL_FM_SWITCH_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_SWITCH_GET_INFO_COMMAND)
						.about(CXL_FM_SWITCH_GET_INFO_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_SWITCH_GET_CONFIG_COMMAND)
						.about(CXL_FM_SWITCH_GET_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_SWITCH_SET_CONFIG_COMMAND)
						.about(CXL_FM_SWITCH_SET_CONFIG_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_MH_DEVICE_COMMAND)
				.about(CXL_FM_MH_DEVICE_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_MH_DEVICE_GET_INFO_COMMAND)
						.about(CXL_FM_MH_DEVICE_GET_INFO_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_LOGICAL_DEVICE_COMMAND)
				.about(CXL_FM_LOGICAL_DEVICE_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_BIND_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_BIND_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_UNBIND_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_UNBIND_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_CONNECT_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_CONNECT_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_DISCONNECT_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_DISCONNECT_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_GET_ALLOCATION_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_GET_ALLOCATION_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_SET_ALLOCATION_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_SET_ALLOCATION_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_GET_QOS_CONTROL_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_GET_QOS_CONTROL_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_SET_QOS_CONTROL_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_SET_QOS_CONTROL_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_GET_QOS_STATUS_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_GET_QOS_STATUS_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_LIMIT_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_LIMIT_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_LIMIT_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_LIMIT_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_LOGICAL_DEVICE_ERASE_COMMAND)
						.about(CXL_FM_LOGICAL_DEVICE_ERASE_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_PPB_COMMAND)
				.about(CXL_FM_PPB_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_PPB_CONFIG_COMMAND)
						.about(CXL_FM_PPB_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_PPB_BIND_COMMAND)
						.about(CXL_FM_PPB_BIND_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_PPB_UNBIND_COMMAND)
						.about(CXL_FM_PPB_UNBIND_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_PHYSICAL_PORT_COMMAND)
				.about(CXL_FM_PHYSICAL_PORT_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_PHYSICAL_PORT_GET_INFO_COMMAND)
						.about(CXL_FM_PHYSICAL_PORT_GET_INFO_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND)
						.about(CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_PHYSICAL_PORT_BIND_COMMAND)
						.about(CXL_FM_PHYSICAL_PORT_BIND_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_PHYSICAL_PORT_UNBIND_COMMAND)
						.about(CXL_FM_PHYSICAL_PORT_UNBIND_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_MLD_PORT_COMMAND)
				.about(CXL_FM_MLD_PORT_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_MLD_PORT_TUNNEL_COMMAND)
						.about(CXL_FM_MLD_PORT_TUNNEL_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND)
						.about(CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_MLD_PORT_SEND_MEM_REQUEST_COMMAND)
						.about(CXL_FM_MLD_PORT_SEND_MEM_REQUEST_COMMAND_DESCRIPTOR)
				)
		)
		.subcommand(
			Command::new(CXL_FM_DCD_COMMAND)
				.about(CXL_FM_DCD_COMMAND_DESCRIPTOR)
				.subcommand_required(true)
				.arg_required_else_help(true)
				.allow_external_subcommands(true)
				.subcommand(
					Command::new(CXL_FM_DCD_GET_INFO_COMMAND)
						.about(CXL_FM_DCD_GET_INFO_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DCD_GET_CONFIG_COMMAND)
						.about(CXL_FM_DCD_GET_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DCD_SET_CONFIG_COMMAND)
						.about(CXL_FM_DCD_SET_CONFIG_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DCD_GET_EXTENT_COMMAND)
						.about(CXL_FM_DCD_GET_EXTENT_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DCD_ADD_CAPACITY_COMMAND)
						.about(CXL_FM_DCD_ADD_CAPACITY_COMMAND_DESCRIPTOR)
				)
				.subcommand(
					Command::new(CXL_FM_DCD_RELEASE_CAPACITY_COMMAND)
						.about(CXL_FM_DCD_RELEASE_CAPACITY_COMMAND_DESCRIPTOR)
				)
		)
}

/*
 * Application logic
 */
fn main() {
	let matches = cli().get_matches();

	let ip = matches.get_one::<String>(CXL_FM_TOOL_IP_ADDRESS_OPTION).unwrap();
	let port = matches.get_one::<String>(CXL_FM_TOOL_PORT_OPTION).unwrap();
	let ip_port = format!("{ip}:{port}");

	let options = CxlFmOptions {
		ip_port: String::from(ip_port),
		is_debug: matches.get_flag(CXL_FM_TOOL_DEBUG_OPTION) == true,
	};

	if options.is_debug {
		println!("{} {}", CXL_FM_TOOL_NAME, CXL_FM_TOOL_VERSION);
	}

	match matches.subcommand() {
		Some((CXL_FM_DISCOVER_COMMAND, discover)) => {
			match discover.subcommand() {
				Some((CXL_FM_DISCOVER_FM_COMMAND, fm)) => {
					cxl_fm_discover_command::discover_fms(&fm,
									      &options);
				},
				Some((CXL_FM_DISCOVER_DEVICES_COMMAND, devices)) => {
					cxl_fm_discover_command::discover_cxl_devices(&devices,
										      &options);
				},
				Some((CXL_FM_DISCOVER_SWITCHES_COMMAND, switch)) => {
					cxl_fm_discover_command::discover_cxl_switches(&switch,
											&options);
				},
				Some((CXL_FM_DISCOVER_LOGICAL_DEVICES_COMMAND, logical_devices)) => {
					cxl_fm_discover_command::discover_logical_devices(&logical_devices,
											  &options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_COMMAND, fm)) => {
			match fm.subcommand() {
				Some((CXL_FM_GET_INFO_COMMAND, get_info)) => {
					cxl_fm_command::get_info(&get_info,
								 &options);
				},
				Some((CXL_FM_START_COMMAND, start)) => {
					cxl_fm_command::start(&start,
							      &options);
				},
				Some((CXL_FM_RESTART_COMMAND, restart)) => {
					cxl_fm_command::restart(&restart,
								&options);
				},
				Some((CXL_FM_STOP_COMMAND, stop)) => {
					cxl_fm_command::stop(&stop,
							     &options);
				},
				Some((CXL_FM_GET_CONFIG_COMMAND, get_config)) => {
					cxl_fm_command::get_config(&get_config,
								   &options);
				},
				Some((CXL_FM_SET_CONFIG_COMMAND, set_config)) => {
					cxl_fm_command::set_config(&set_config,
								   &options);
				},
				Some((CXL_FM_GET_EVENTS_COMMAND, get_events)) => {
					cxl_fm_command::get_events(&get_events,
								   &options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_SWITCH_COMMAND, switch)) => {
			match switch.subcommand() {
				Some((CXL_FM_SWITCH_GET_INFO_COMMAND, get_info)) => {
					cxl_switch_command::get_info(&get_info,
								     &options);
				},
				Some((CXL_FM_SWITCH_GET_CONFIG_COMMAND, get_config)) => {
					cxl_switch_command::get_config(&get_config,
								       &options);
				},
				Some((CXL_FM_SWITCH_SET_CONFIG_COMMAND, set_config)) => {
					cxl_switch_command::set_config(&set_config,
									&options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_MH_DEVICE_COMMAND, mh_device)) => {
			match mh_device.subcommand() {
				Some((CXL_FM_MH_DEVICE_GET_INFO_COMMAND, get_info)) => {
					cxl_mh_device_command::get_info(&get_info,
									&options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_LOGICAL_DEVICE_COMMAND, logical_device)) => {
			match logical_device.subcommand() {
				Some((CXL_FM_LOGICAL_DEVICE_BIND_COMMAND, bind)) => {
					cxl_logical_device_command::bind(&bind,
									 &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_UNBIND_COMMAND, unbind)) => {
					cxl_logical_device_command::unbind(&unbind,
									   &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_CONNECT_COMMAND, connect)) => {
					cxl_logical_device_command::connect(&connect,
									    &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_DISCONNECT_COMMAND, disconnect)) => {
					cxl_logical_device_command::disconnect(&disconnect,
										&options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_GET_ALLOCATION_COMMAND, get_allocation)) => {
					cxl_logical_device_command::get_allocation(&get_allocation,
										   &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_SET_ALLOCATION_COMMAND, set_allocation)) => {
					cxl_logical_device_command::set_allocation(&set_allocation,
										   &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_GET_QOS_CONTROL_COMMAND, get_qos_control)) => {
					cxl_logical_device_command::get_qos_control(&get_qos_control,
										    &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_SET_QOS_CONTROL_COMMAND, set_qos_control)) => {
					cxl_logical_device_command::set_qos_control(&set_qos_control,
										    &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_GET_QOS_STATUS_COMMAND, get_qos_status)) => {
					cxl_logical_device_command::get_qos_status(&get_qos_status,
										   &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_COMMAND, get_qos_bandwidth)) => {
					cxl_logical_device_command::get_qos_bandwidth(&get_qos_bandwidth,
										      &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_COMMAND, set_qos_bandwidth)) => {
					cxl_logical_device_command::set_qos_bandwidth(&set_qos_bandwidth,
										      &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_GET_QOS_BANDWIDTH_LIMIT_COMMAND, get_qos_bandwidth_limit)) => {
					cxl_logical_device_command::get_qos_bandwidth_limit(&get_qos_bandwidth_limit,
											    &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_SET_QOS_BANDWIDTH_LIMIT_COMMAND, set_qos_bandwidth_limit)) => {
					cxl_logical_device_command::set_qos_bandwidth_limit(&set_qos_bandwidth_limit,
											    &options);
				},
				Some((CXL_FM_LOGICAL_DEVICE_ERASE_COMMAND, erase)) => {
					cxl_logical_device_command::erase(&erase,
									  &options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_PPB_COMMAND, ppb)) => {
			match ppb.subcommand() {
				Some((CXL_FM_PPB_CONFIG_COMMAND, config)) => {
					cxl_ppb_command::config(&config,
								&options);
				},
				Some((CXL_FM_PPB_BIND_COMMAND, bind)) => {
					cxl_ppb_command::bind(&bind,
							      &options);
				},
				Some((CXL_FM_PPB_UNBIND_COMMAND, unbind)) => {
					cxl_ppb_command::unbind(&unbind,
								&options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_PHYSICAL_PORT_COMMAND, physical_port)) => {
			match physical_port.subcommand() {
				Some((CXL_FM_PHYSICAL_PORT_GET_INFO_COMMAND, get_info)) => {
					cxl_physical_port_command::get_info(&get_info,
									    &options);
				},
				Some((CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND, control)) => {
					cxl_physical_port_command::control(&control,
									   &options);
				},
				Some((CXL_FM_PHYSICAL_PORT_BIND_COMMAND, bind)) => {
					cxl_physical_port_command::bind(&bind,
									&options);
				},
				Some((CXL_FM_PHYSICAL_PORT_UNBIND_COMMAND, unbind)) => {
					cxl_physical_port_command::unbind(&unbind,
									  &options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_MLD_PORT_COMMAND, mld_port)) => {
			match mld_port.subcommand() {
				Some((CXL_FM_MLD_PORT_TUNNEL_COMMAND, tunnel)) => {
					cxl_mld_port_command::tunnel(&tunnel,
								     &options);
				},
				Some((CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND, send_config)) => {
					cxl_mld_port_command::send_config(&send_config,
									  &options);
				},
				Some((CXL_FM_MLD_PORT_SEND_MEM_REQUEST_COMMAND, send_memory_request)) => {
					cxl_mld_port_command::send_memory_request(&send_memory_request,
										  &options);
				},
				_ => unreachable!(),
			}
		},
		Some((CXL_FM_DCD_COMMAND, dcd)) => {
			match dcd.subcommand() {
				Some((CXL_FM_DCD_GET_INFO_COMMAND, get_info)) => {
					cxl_dcd_command::get_info(&get_info,
								  &options);
				},
				Some((CXL_FM_DCD_GET_CONFIG_COMMAND, get_capacity_config)) => {
					cxl_dcd_command::get_capacity_config(&get_capacity_config,
									     &options);
				},
				Some((CXL_FM_DCD_SET_CONFIG_COMMAND, set_capacity_config)) => {
					cxl_dcd_command::set_capacity_config(&set_capacity_config,
									     &options);
				},
				Some((CXL_FM_DCD_GET_EXTENT_COMMAND, get_extent_list)) => {
					cxl_dcd_command::get_extent_list(&get_extent_list,
									 &options);
				},
				Some((CXL_FM_DCD_ADD_CAPACITY_COMMAND, add_capacity)) => {
					cxl_dcd_command::add_capacity(&add_capacity,
								      &options);
				},
				Some((CXL_FM_DCD_RELEASE_CAPACITY_COMMAND, release_capacity)) => {
					cxl_dcd_command::release_capacity(&release_capacity,
									  &options);
				},
				_ => unreachable!(),
			}
		},
		_ => unreachable!(),
	}
}
