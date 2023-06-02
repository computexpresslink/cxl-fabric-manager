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

pub mod cxl_logical_device_command {
	use std::net::{TcpStream};
	use fm_library::cxl_fm_lib::CxlFmOptions;
	use fm_library::cxl_fm_lib::send_responce;
	use fm_library::cxl_fm_lib::CXL_FM_BIND_LD_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_UNBIND_LD_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_CONNECT_MLD_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_DISCONNECT_MLD_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_GET_LD_ALLOCATION_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_SET_LD_ALLOCATION_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_GET_QOS_CONTROL_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_SET_QOS_CONTROL_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_GET_QOS_STATUS_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_GET_QOS_BANDWIDTH_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_SET_QOS_BANDWIDTH_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_GET_QOS_BANDWIDTH_LIMIT_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_SET_QOS_BANDWIDTH_LIMIT_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_LD_ERASE;
	use fm_library::cxl_fm_lib::CXL_FM_NO_DATA;

	/*
	 * Bind Logical Device (LD)
	 */
	pub fn bind(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_BIND_LD_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Unbind Logical Device (LD)
	 */
	pub fn unbind(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_UNBIND_LD_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Connect Multi Logical Device (MLD) to CXL switch
	 */
	pub fn connect(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_CONNECT_MLD_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Disconnect Multi Logical Device (MLD) from CXL switch
	 */
	pub fn disconnect(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_DISCONNECT_MLD_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Get Logical Device (LD) allocations
	 */
	pub fn get_allocation(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_LD_ALLOCATION_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Set Logical Device (LD) allocations
	 */
	pub fn set_allocation(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_SET_LD_ALLOCATION_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Get QoS control
	 */
	pub fn get_qos_control(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_QOS_CONTROL_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Set QoS control
	 */
	pub fn set_qos_control(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_SET_QOS_CONTROL_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Get QoS status
	 */
	pub fn get_qos_status(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_QOS_STATUS_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Get QoS allocated bandwidth
	 */
	pub fn get_qos_bandwidth(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_QOS_BANDWIDTH_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Set QoS allocated bandwidth
	 */
	pub fn set_qos_bandwidth(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_SET_QOS_BANDWIDTH_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Get QoS bandwidth limit
	 */
	pub fn get_qos_bandwidth_limit(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_GET_QOS_BANDWIDTH_LIMIT_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Set QoS bandwidth limit
	 */
	pub fn set_qos_bandwidth_limit(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_SET_QOS_BANDWIDTH_LIMIT_COMMAND);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}

	/*
	 * Secure erase after unbinding
	 */
	pub fn erase(stream: &TcpStream, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", CXL_FM_LD_ERASE);
		}

		send_responce(stream, CXL_FM_NO_DATA, env);
	}
}
