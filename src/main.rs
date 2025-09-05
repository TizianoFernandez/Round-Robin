use std::io::Read;

use crate::proceso::Proceso;

mod proceso;
mod round_robin;

fn main() {
    let mut procesos_lista: Vec<proceso::Proceso> = Vec::new();

    println!("Simulador de Planificacion de Procesos - Round Robin");
    println!("Introduzca los datos de los procesos:");
    println!("Ingrese el Quantum (ms): ");
    let mut quantum_input = String::new();
    std::io::stdin().read_line(&mut quantum_input).unwrap();
    let quantum = quantum_input.trim().parse::<u32>().unwrap();

    println!("Ingrese el Tiempo de Intercambio con respecto al Quantum (q/n ms): ");
    let mut tiempo_intercambio_input = String::new();
    std::io::stdin().read_line(&mut tiempo_intercambio_input).unwrap();
    let tiempo_intercambio = tiempo_intercambio_input.trim().parse::<u32>().unwrap();

    println!("Ingrese cuantos procesos quiere ingresar: ");
    let mut procesos_input = String::new();
    std::io::stdin().read_line(&mut procesos_input).unwrap();
    let procesos = procesos_input.trim().parse::<u32>().unwrap();

    println!("Ingrese los datos de los procesos");
    for i in 0..procesos{
        println!("Ingrese el PID del proceso {}: ", i+1);
        let mut pid_input = String::new();
        std::io::stdin().read_line(&mut pid_input).unwrap();
        let pid = pid_input.trim().parse::<u32>().unwrap();

        println!("Ingrese el tiempo de servicio del proceso {} (ms): ", i+1);
        let mut tiempo_servicio_input = String::new();
        std::io::stdin().read_line(&mut tiempo_servicio_input).unwrap();
        let tiempo_servicio = tiempo_servicio_input.trim().parse::<u32>().unwrap();

        println!("Ingrese el tiempo de llegada del proceso {} (ms): ", i+1);
        let mut tiempo_llegada_input = String::new();
        std::io::stdin().read_line(&mut tiempo_llegada_input).unwrap();
        let tiempo_llegada = tiempo_llegada_input.trim().parse::<u32>().unwrap();

        procesos_lista.push(Proceso::new(pid, tiempo_servicio, tiempo_llegada));

        println!("Proceso {} - PID: {}, Tiempo de Servicio: {} ms", i+1, pid, tiempo_servicio);
    }
    println!("Procesos ingresados");
    for p in &procesos_lista {
        println!("Proceso - PID: {}, Tiempo de Servicio: {} ms, Tiempo de Llegada: {} ms", p.process_id, p.tiempo_servicio, p.llegada);
    }

    println!("Iniciando Planificacion...");
    let mut planificador = round_robin::RoundRobin::new(quantum, procesos_lista, tiempo_intercambio);
    planificador.planificar();

}
