#[derive(Clone)]
pub struct Proceso {
    pub process_id: u32,
    pub tiempo_servicio: u32,
    pub restante: u32,
    pub retorno: u32,
    pub llegada: u32, 
    pub espera: u32,
}

impl Proceso {
    pub fn new(pid: u32, tiempo: u32, llegada:u32) -> Proceso {
        return Proceso { process_id: pid, tiempo_servicio: tiempo, restante: tiempo, retorno: 0, espera: 0, llegada }
    }

    pub fn restar_tiempo(&mut self, tiempo: u32) {
        self.restante -= tiempo;
    }
}