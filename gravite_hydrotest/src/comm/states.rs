pub enum ToMcu {
    TogglePeripheral(u8),
    RunProcedure(u8),
    FlashFirmware,
}

pub enum FromMcu {
    Echo(u8),
    Disconnected,
}