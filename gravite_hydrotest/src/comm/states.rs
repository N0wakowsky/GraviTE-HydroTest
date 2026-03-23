use std::sync::mpsc::{Receiver, Sender};

pub enum ToMcu {
    TogglePeripheral(u8),
    RunProcedure(u8),
    FlashFirmware,
}

pub enum FromMcu {
    Echo(u8),
    ProcedureStatus(u8),
    Error(String),
    Disconnected,
}

pub struct CommHandle {
    pub tx: Sender<ToMcu>,
    pub rx: Receiver<FromMcu>,
}