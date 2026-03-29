use crate::comm::serial::{Connected, SerialHandle};

pub enum ConnectionState {
    Disconnected {
        ports: Vec<String>,
        selected: usize,
        baud_rate: u32,
        last_error: Option<String>,
    },
    Connected {
        handle: SerialHandle<Connected>,
    },
}

impl ConnectionState {
    pub fn new() -> Self {
        Self::Disconnected {
            ports: Self::list_ports(),
            selected: 0,
            baud_rate: 115200,
            last_error: None,
        }
    }
    pub fn list_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.port_name)
            .collect()
    }

    pub fn try_connect(&mut self, port: &str, baud: u32) -> Result<(), String> {
        let handle = SerialHandle::new(port, baud);
        match handle.connect() {
            Ok(connected) => {
                *self = ConnectionState::Connected { handle: connected };
                Ok(())
            }
            Err(e) => {
                let err_msg = e.to_string();
                if let ConnectionState::Disconnected { last_error, .. } = self {
                    *last_error = Some(err_msg.clone());
                }
                Err(err_msg)
            }
        }
    }

    pub fn try_disconnect(&mut self) {
        let old_state = std::mem::replace(self, Self::new());
        if let ConnectionState::Connected { handle } = old_state {
            let disconnected_name = handle.port_name.clone();
            let _ = handle.disconnect();
            
            if let ConnectionState::Disconnected { ports, selected, .. } = self {
                if let Some(pos) = ports.iter().position(|p| p == &disconnected_name) {
                    *selected = pos;
                }
            }
        }
    }
}