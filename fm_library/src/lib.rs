/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM library implementation.
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

pub mod cxl_fm_lib {
	use std::net::{TcpStream};
	use std::io::Write;
	use std::io::BufReader;
	use std::io::BufRead;

	/*
	 * Available commands
	 */
	pub const CXL_FM_DISCOVER_FM_COMMAND: &str		= "DISCOVER_FM";
	pub const CXL_FM_DISCOVER_CXL_DEVICE_COMMAND: &str	= "DISCOVER_CXL_DEVICE";
	pub const CXL_FM_DISCOVER_CXL_SWITCH_COMMAND: &str	= "DISCOVER_CXL_SWITCH";
	pub const CXL_FM_DISCOVER_LD_COMMAND: &str		= "DISCOVER_LOGICAL_DEVICE";

	pub const CXL_FM_GET_FM_INFO_COMMAND: &str		= "GET_FM_INFO";
	pub const CXL_FM_START_FM_COMMAND: &str			= "START_FM";
	pub const CXL_FM_RESTART_FM_COMMAND: &str		= "RESTART_FM";
	pub const CXL_FM_STOP_FM_COMMAND: &str			= "STOP_FM";
	pub const CXL_FM_GET_FM_CONFIG_COMMAND: &str		= "GET_FM_CONFIG";
	pub const CXL_FM_SET_FM_CONFIG_COMMAND: &str		= "SET_FM_CONFIG";
	pub const CXL_FM_GET_FM_EVENTS_COMMAND: &str		= "GET_FM_EVENTS";

	pub const CXL_FM_GET_SWITCH_INFO_COMMAND: &str		= "GET_SWITCH_INFO";
	pub const CXL_FM_GET_SWITCH_CONFIG_COMMAND: &str	= "GET_SWITCH_CONFIG";
	pub const CXL_FM_SET_SWITCH_CONFIG_COMMAND: &str	= "SET_SWITCH_CONFIG";

	pub const CXL_FM_GET_MHD_INFO_COMMAND: &str		= "GET_MHD_INFO";

	pub const CXL_FM_BIND_LD_COMMAND: &str			= "BIND_LOGICAL_DEVICE";
	pub const CXL_FM_UNBIND_LD_COMMAND: &str		= "UNBIND_LOGICAL_DEVICE";
	pub const CXL_FM_CONNECT_MLD_COMMAND: &str		= "CONNECT_MLD";
	pub const CXL_FM_DISCONNECT_MLD_COMMAND: &str		= "DISCONNECT_MLD";
	pub const CXL_FM_GET_LD_ALLOCATION_COMMAND: &str	= "GET_LD_ALLOCATION";
	pub const CXL_FM_SET_LD_ALLOCATION_COMMAND: &str	= "SET_LD_ALLOCATION";
	pub const CXL_FM_GET_QOS_CONTROL_COMMAND: &str		= "GET_LD_QOS_CONTROL";
	pub const CXL_FM_SET_QOS_CONTROL_COMMAND: &str		= "SET_LD_QOS_CONTROL";
	pub const CXL_FM_GET_QOS_STATUS_COMMAND: &str		= "GET_LD_QOS_STATUS";
	pub const CXL_FM_GET_QOS_BANDWIDTH_COMMAND: &str	= "GET_LD_QOS_BANDWIDTH";
	pub const CXL_FM_SET_QOS_BANDWIDTH_COMMAND: &str	= "SET_LD_QOS_BANDWIDTH";
	pub const CXL_FM_GET_QOS_BANDWIDTH_LIMIT_COMMAND: &str	= "GET_LD_QOS_BANDWIDTH_LIMIT";
	pub const CXL_FM_SET_QOS_BANDWIDTH_LIMIT_COMMAND: &str	= "SET_LD_QOS_BANDWIDTH_LIMIT";
	pub const CXL_FM_LD_ERASE: &str				= "LD_ERASE";

	pub const CXL_FM_GET_PPB_CONFIG_COMMAND: &str		= "GET_PPB_CONFIG";
	pub const CXL_FM_PPB_BIND_COMMAND: &str			= "PPB_BIND";
	pub const CXL_FM_PPB_UNBIND_COMMAND: &str		= "PPB_UNBIND";

	pub const CXL_FM_GET_PHYSICAL_PORT_INFO_COMMAND: &str	= "GET_PHYSICAL_PORT_INFO";
	pub const CXL_FM_PHYSICAL_PORT_CONTROL_COMMAND: &str	= "PHYSICAL_PORT_CONTROL";
	pub const CXL_FM_BIND_PHYSICAL_PORT_COMMAND: &str	= "BIND_PHYSICAL_PORT";
	pub const CXL_FM_UNBIND_PHYSICAL_PORT_COMMAND: &str	= "UNBIND_PHYSICAL_PORT";

	pub const CXL_FM_MLD_PORT_TUNNEL_COMMAND: &str		= "MLD_PORT_TUNNEL";
	pub const CXL_FM_MLD_PORT_SEND_CONFIG_COMMAND: &str	= "MLD_PORT_SEND_CONFIG";
	pub const CXL_FM_MLD_PORT_SEND_MEM_REQ_COMMAND: &str	= "MLD_PORT_SEND_MEM_REQ";

	pub const CXL_FM_DCD_GET_INFO_COMMAND: &str		= "DCD_GET_INFO";
	pub const CXL_FM_DCD_GET_CONFIG_COMMAND: &str		= "DCD_GET_CONFIG";
	pub const CXL_FM_DCD_SET_CONFIG_COMMAND: &str		= "DCD_SET_CONFIG";
	pub const CXL_FM_DCD_GET_EXTENT_COMMAND: &str		= "DCD_GET_EXTENT";
	pub const CXL_FM_DCD_ADD_CAPACITY_COMMAND: &str		= "DCD_ADD_CAPACITY";
	pub const CXL_FM_DCD_RELEASE_CAPACITY_COMMAND: &str	= "DCD_RELEASE_CAPACITY";

	/*
	 * Service responces
	 */
	pub const CXL_FM_UNKNOWN_COMMAND: &str			= "UNKNOWN_COMMAND";
	pub const CXL_FM_NO_DATA: &str				= "NO_DATA";

	/*
	 * struct CxlFmOptions - configuration options
	 * @ip_port: IP address + port
	 * @is_debug: does it need to show debug output?
	 */
	pub struct CxlFmOptions {
		pub ip_port: String,
		pub is_debug: bool,
	}

	/*
	 * Send command to FM
	 */
	pub fn send_command(mut stream: &TcpStream, command: &str, env: &CxlFmOptions) {
		let full_command = format!("{command}\n");

		if env.is_debug {
			println!("COMMAND: {:#?}", full_command);
		}

		stream.write_all(full_command.as_bytes()).unwrap();

		let buf_reader = BufReader::new(&mut stream);
		let mut response_line = buf_reader.lines();

		if env.is_debug {
			println!("RESPONCE: {:#?}", response_line.next().unwrap().unwrap());
		}
	}

	/*
	 * Send responce from FM
	 */
	pub fn send_responce(mut stream: &TcpStream, responce: &str, env: &CxlFmOptions) {
		let full_responce = format!("{responce}\n");

		if env.is_debug {
			println!("RESPONCE: {:#?}", full_responce);
		}

		stream.write_all(full_responce.as_bytes()).unwrap();
	}
}
