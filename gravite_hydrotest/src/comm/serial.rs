use std::any::Any;
use std::ffi::os_str::Display;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use serialport::SerialPort;

use super::states::{FromMcu, ToMcu};

pub struct Disconnected;
pub struct Connected;
pub struct Connecting;

pub enum SerialError {
    Open(serialport::Error),
    Disconnected,
}

pub struct SerialHandle<State> {
    pub port_name: String,
    pub baud_rate: u32,
    _state: std::marker::PhantomData<State>,
    pub tx: Option<Sender<ToMcu>>,
    pub rx: Option<Receiver<FromMcu>>,
}

fn spawn_comm_thread(port_name: String, baud_rate: u32, mut port: Box<dyn SerialPort>) -> SerialHandle<Connected> {
    let (tx_to_thread, rx_from_gui): (Sender<ToMcu>, Receiver<ToMcu>) = channel();
    let (tx_to_gui, rx_from_thread): (Sender<FromMcu>, Receiver<FromMcu>) = channel();


    thread::spawn(move || {
        let mut buf: [u8; 1] = [0u8; 1];

        loop {
            // sending data to MCU
            match rx_from_gui.try_recv() {
                Ok(msg) => {
                    let byte = match msg {
                        ToMcu::FlashFirmware => 0xFF,
                        ToMcu::TogglePeripheral(code) => code,
                        ToMcu::RunProcedure(code) => code,
                    };
                    if port.write_all(&[byte]).is_err() {
                        let _ = tx_to_gui.send(FromMcu::Disconnected);
                        return;
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    return;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }

            // receiving data from MCU
            match port.read(&mut buf) {
                Ok(1) => {
                    let code = buf[0];
                    let _ = tx_to_gui.send(FromMcu::Echo(code));
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(_) => {
                    let _ = tx_to_gui.send(FromMcu::Disconnected);
                    return;
                }
                _ => {}
            }

            thread::sleep(Duration::from_millis(10));
        }
    });

    SerialHandle { port_name, baud_rate, _state: std::marker::PhantomData, tx: Some(tx_to_thread), rx: Some(rx_from_thread) }
}

impl SerialHandle<Disconnected> {
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        Self { port_name: port_name.to_string(), baud_rate, _state: std::marker::PhantomData, tx: None, rx: None }
    }

    pub fn connect(self) -> Result<SerialHandle<Connected>, SerialError> {
        let port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(SerialError::Open)?;

        Ok(spawn_comm_thread(self.port_name, self.baud_rate, port))
    }
}

impl SerialHandle<Connected> {
    pub fn send(&self, msg: ToMcu) -> Result<(), SerialError> {
        self.tx.as_ref().unwrap().send(msg).map_err(|_| SerialError::Disconnected)
    }

    pub fn try_recv(&self) -> Option<FromMcu> {
        self.rx.as_ref().unwrap().try_recv().ok()
    }

    pub fn disconnect(self) -> SerialHandle<Disconnected> {
        drop(self.tx);
        SerialHandle::new(&self.port_name, self.baud_rate)
    }
}

impl std::fmt::Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SerialError::Open(e)  => write!(f, "Cannot open a port: {}", e),
            SerialError::Disconnected => write!(f, "Port disconnected"),
        }
    }
}