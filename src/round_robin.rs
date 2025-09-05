use crate::proceso::Proceso;

struct Registro {
    proceso: Proceso,
    tiempo_reloj: u32,
}

pub struct RoundRobin {
    quantum: u32,
    procesos: Vec<Proceso>,
    tiempo_intercambio: u32,
    reloj: u32,
    proceso_anterior: Option<Proceso>,
    registros: Vec<Registro>,
}

impl RoundRobin {
    pub fn new(quantum: u32, procesos: Vec<Proceso>, tiempo_intercambio: u32) -> RoundRobin {
        return RoundRobin {
            quantum,
            procesos,
            tiempo_intercambio: quantum / tiempo_intercambio,
            reloj: 0,
            proceso_anterior: None,
            registros: Vec::new(),
        };
    }

    pub fn planificar(&mut self) {
        print!(
            "Planificando con Round Robin (quantum = {}, tiempo de intercambio = {})\n",
            self.quantum, self.tiempo_intercambio
        );

        while self.tiempo_servicio_total() > 0 {
            for proceso in self.procesos.iter_mut() {

                if proceso.restante == 0 {
                    continue;
                }

                let pid_actual = proceso.process_id;
                let cambio = match &self.proceso_anterior {
                    Some(p) => p.process_id != pid_actual,
                    None => false,
                };

                if proceso.restante > 0 {
                    if cambio {
                        //Call al Scheduler de Corto plazo para quitar el proceso
                        self.reloj += self.tiempo_intercambio / 2;
                    }
                    if proceso.restante > self.quantum {
                        if cambio || self.proceso_anterior.is_none() {
                            //Call al Scheduler de Corto plazo para insertar el proceso
                            self.reloj += self.tiempo_intercambio / 2;
                        }
                        //{Ejecutar Proceso L(i).PID por "Q" tiempo}
                        proceso.restar_tiempo(self.quantum);
                        self.reloj += self.quantum;

                    } else {
                        if cambio || self.proceso_anterior.is_none(){
                            //Call al Scheduler de Corto plazo para insertar el proceso
                            self.reloj += self.tiempo_intercambio / 2;
                        }
                        //{Ejecutar Proceso L(i).PID por "L(i).TS" tiempo}
                        self.reloj += proceso.restante;
                        proceso.restante = 0;
                        proceso.retorno = self.reloj - proceso.llegada;
                    }
                }

                //{ Mostrar por pantalla L(i) y Q }
                println!(
                    "Proceso: {} | Tiempo Restante: {} | Tiempo Reloj: {}",
                    proceso.process_id, proceso.restante, self.reloj
                );

                self.registros.push(Registro {
                    proceso: proceso.clone(),
                    tiempo_reloj: self.reloj,
                });
                self.proceso_anterior = Some(proceso.clone());
            }
        }
        self.finalizar();
    }

    pub fn finalizar(&self) {
        let mut espera_promedio: f32 = 0.0;
        let mut retorno_promedio: f32 = 0.0;
        println!("Resultados de la Planificacion: ");
        println!(
            "Resueltos {} procesos en {} ms.",
            self.procesos.len(),
            self.reloj
        );

        for proceso in &self.procesos {
            println!("Proceso: {}", proceso.process_id);
            println!("Tiempo de Servicio: {} ms", proceso.tiempo_servicio);
            println!("Tiempo de Retorno: {} ms", proceso.retorno);
            println!(
                "Tiempo de Espera: {} ms",
                proceso.retorno - proceso.tiempo_servicio
            );
            espera_promedio += (proceso.retorno - proceso.tiempo_servicio) as f32;
            retorno_promedio += proceso.retorno as f32;
        }

        println!(
            "Tiempo de Espera Promedio: {} ms",
            espera_promedio / self.procesos.len() as f32
        );

        println!(
            "Tiempo de Retorno Promedio: {} ms",
            retorno_promedio / self.procesos.len() as f32
        );
    }

    fn tiempo_servicio_total(&self) -> u32 {
        let mut total = 0;
        for p in &self.procesos {
            total += p.restante;
        }
        return total;
    }

    fn verificar_anterior(&self, proceso_actual: u32) -> bool {
        match &self.proceso_anterior {
            Some(prev) => prev.process_id == proceso_actual,
            None => false,
        }
    }
}
