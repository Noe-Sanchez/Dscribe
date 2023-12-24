use std::thread;
use mavlink::common;
use mavlink::MavFrame;
use mavlink::common::MavType;
use mavlink::MavHeader;
use mavlink::common::MavMessage;
use std::net::UdpSocket;

fn main() {
    // Create UDP socket client 
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't find any address to bind to");
    println!("Created UDP socket on port {}", socket.local_addr().unwrap().port());

    // Create heartbeat thread 
    let heartbeat_thread: thread::JoinHandle<_> = thread::spawn(|| {
        // Handle socket borrows
        let socket: UdpSocket = socket;
        println!("Starting heartbeat thread");
        let mut sequence: u16 = 0;
        loop {
            let heartbeat_msg: MavMessage = MavMessage::HEARTBEAT(common::HEARTBEAT_DATA {
                mavtype: MavType::MAV_TYPE_GCS,
                autopilot: common::MavAutopilot::MAV_AUTOPILOT_INVALID,
                base_mode: common::MavModeFlag::empty(),
                custom_mode: 0,
                system_status: common::MavState::MAV_STATE_UNINIT,
                mavlink_version: 2,
            });
            let heartbeat_header: MavHeader = MavHeader {
                system_id: 45,
                component_id: 0,
                sequence: 0,
            };
            let heartbeat_frame: MavFrame<MavMessage> = MavFrame {
                msg : heartbeat_msg,
                header : heartbeat_header,
                protocol_version : mavlink::MavlinkVersion::V2,
            };

            sequence += 1;
            // Print heartbeat sequence and time in human readable format
            println!("Sending {} heartbeat at time {:?}", sequence, std::time::SystemTime::now());

            // Send heartbeat using frame ser fn
            let packet_bytes: Vec<u8> = heartbeat_frame.ser();
            socket.send_to(&packet_bytes, "172.20.247.116:14552").expect("Couldn't send data");


            // Sleep for 0.5 second
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // Wait for 10 seconds
    thread::sleep(std::time::Duration::from_secs(10));

    // Join heartbeat thread
    println!("Joining heartbeat thread");
    drop(heartbeat_thread);
    //heartbeat_thread.join().unwrap();

}
