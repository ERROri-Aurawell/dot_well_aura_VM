//Um dos maiores demônios que eu decidi dar vida.

mod code_file;
mod data_type;
use code_file::code::{AURA_FILE, AURA_LABELS, AURA_STRINGS, AURA_VALUES};
use data_type::types::{DataTypes, Memory};

fn find_label(what: &str) -> usize {
    let mut resp: Option<usize> = None;
    for label in AURA_LABELS.lines() {
        if let Some((n, _)) = label.split_once(what) {
            let n = n.get(..n.len() - 2).unwrap();
            let parsed: usize = n.parse().unwrap();
            resp = Some(parsed);
            break;
        }
    }
    if resp.is_none() {
        panic!("what? -> {}", what);
    }

    resp.unwrap()
}

fn find_string(onde: usize) -> &'static str {
    if let Some(r) = AURA_STRINGS.lines().nth(onde) {
        r
    } else {
        panic!("Não foi possivel achar a string na posição: {}", onde);
    }
}

fn get_data(tipo: &str, valor: &str, vm_registers: &[DataTypes; 6], memory: &Memory) -> DataTypes {
    match tipo {
        "0" => DataTypes::String(valor.to_string()),
        "1" => DataTypes::I8(valor.parse().unwrap()),
        "2" => DataTypes::I16(valor.parse().unwrap()),
        "3" => DataTypes::I32(valor.parse().unwrap()),
        "4" => DataTypes::Float(valor.parse().unwrap()),
        "5" => DataTypes::Doble(valor.parse().unwrap()),
        "6" => match valor {
            //Valores especiais
            v if v.starts_with("A") => {
                let reg: usize = valor.replace("A", "").parse().unwrap();
                if &reg >= &vm_registers.len() {
                    panic!("");
                };
                vm_registers[reg].clone()
            }
            v if v.starts_with("B") => {
                let res = memory.get_value(v);
                if let Some(resp) = res {
                    resp.clone()
                } else {
                    panic!("");
                }
            }
            v if v.starts_with("_C") => {
                let resp = find_label(v);
                DataTypes::Operational(resp)
            }
            v if v.starts_with("%") => {
                //Ponteiro para as strings estáticas imbutidas
                let onde: &str = &v[1..];
                let onde: usize = onde.parse().unwrap();
                let resp: &str = find_string(onde);
                DataTypes::String(resp.to_string())
            }
            _ => DataTypes::Operational(valor.parse().unwrap()),
        },
        _ => {
            panic!("");
        }
    }
}

fn main() {
    let mut vm_registers: [DataTypes; 6] = [
        DataTypes::None,
        DataTypes::None,
        DataTypes::None,
        DataTypes::None,
        DataTypes::None,
        DataTypes::None,
    ];

    let mut memory: Memory = Memory::new();

    let mut line: usize;
    let mut running: bool = true;

    line = find_label("_C_main");

    let content: Vec<&str> = AURA_FILE.lines().collect();

    while running {
        line += 1;
        //println!("\n\n{} - linha {}", content[line], line);

        match content[line] {
            l if l.starts_with("NEW") => {
                let l = l.replace(";", "").replace(" ", "");
                let var_name = &l[3..];
                memory.new_value(var_name);
                //println!("'{}' CREATED", var_name);
            }
            l if l.starts_with("RM") => {
                let l = l.replace(";", "").replace(" ", "");
                let var_name = &l[2..];
                if let Err(err) = memory.remove_value(var_name) {
                    panic!("{err}");
                }
                //println!("'{}' DELETED", var_name);
            }
            l if l.starts_with("MOVE") => {
                let l2 = l.replace(" ", "").replace(";", "");
                let data: Vec<&str> = l2[4..].split(",").collect();
                //println!("{:#?}", data);

                let (tipo, valor, onde): (&str, &str, &str);

                match data[..] {
                    [a, b, c] => {
                        tipo = a;
                        valor = b;
                        onde = c;
                    }
                    [b, c] => {
                        tipo = "6";
                        valor = b;
                        onde = c;
                    }
                    _ => {
                        panic!("");
                    }
                };

                //println!("Movendo {}, tipo {} para {}", &valor, &tipo, &onde);

                let value: DataTypes = get_data(tipo, valor, &vm_registers, &memory);
                match onde {
                    w if w.starts_with("A") => {
                        let reg: usize = w.replace("A", "").parse().unwrap();
                        if &reg > &vm_registers.len() {
                            panic!("Salvando dados em um registrador inexistente -> {}", l);
                        };
                        vm_registers[reg] = value;
                    }
                    w if w.starts_with("B") => {
                        let resp = memory.edit_value(w, value);

                        if let Err(r) = resp {
                            panic!("{}", &r);
                        }
                    }
                    _ => {
                        panic!("")
                    }
                };
            }
            l if l.starts_with("CALL") => {
                // 1. Extrair o tipo da chamada que está em A0 de forma segura
                let syscall_type = match &vm_registers[0] {
                    DataTypes::I8(v) => *v as i32,
                    DataTypes::I16(v) => *v as i32,
                    DataTypes::I32(v) => *v,
                    DataTypes::Operational(v) => *v as i32,
                    _ => panic!("Erro: A0 (Tipo da chamada) precisa ser um número inteiro válido."),
                };

                //println!("CHAMADA {}", &syscall_type);

                match syscall_type {
                    // -------------------------------------------------------------
                    // 1 - PRINT: Imprime o conteúdo de A1
                    // -------------------------------------------------------------
                    1 => match &vm_registers[1] {
                        DataTypes::String(s) => println!("{}", s),
                        DataTypes::I8(n) => println!("{}", n),
                        DataTypes::I32(n) => println!("{}", n),
                        DataTypes::Float(n) => println!("{}", n),
                        DataTypes::Doble(n) => println!("{}", n),
                        DataTypes::None => println!("None"),
                        DataTypes::Operational(n) => match n {
                            1 => println!("true"),
                            0 => println!("false"),
                            _ => println!("{}", n),
                        },
                        _ => println!("{:#?}", vm_registers[1]),
                    },
                    // -------------------------------------------------------------
                    // 2 - NOT, REVERSE
                    // -------------------------------------------------------------
                    2 => {
                        //Se A1 for 0, reverte booleano.
                        //Se A1 for 1, reverte sinal numérico

                        let type_of = match &vm_registers[1] {
                            DataTypes::Operational(v) => *v,
                            _ => {
                                panic!("TYPEOF EXPRESSION MUST BE AN Operational");
                            }
                        };

                        let res: DataTypes = match type_of {
                            0 => match &vm_registers[2] {
                                DataTypes::Operational(v) => {
                                    if *v == 1 {
                                        DataTypes::Operational(0)
                                    } else {
                                        DataTypes::Operational(1)
                                    }
                                }
                                _ => panic!("CANNOT REVERSE ANY OTHER TYPE EXCEPT Operational"),
                            },
                            1 => match &vm_registers[2] {
                                DataTypes::Doble(v) => DataTypes::Doble(-v),
                                DataTypes::Float(v) => DataTypes::Float(-v),
                                DataTypes::I32(v) => DataTypes::I32(-v),
                                DataTypes::I16(v) => DataTypes::I16(-v),
                                DataTypes::I8(v) => DataTypes::I8(-v),
                                _ => panic!(
                                    "CANNOT REVERSE SIGNAL OF UNSINED OR NOT NUMERICAL VALUES"
                                ),
                            },
                            _ => panic!("UNSUPORTED : {}", l),
                        };
                        vm_registers[4] = res;
                    }

                    // -------------------------------------------------------------
                    // 3 - IF TRUE JUMP: Se A1 for "true" (1), pula para o endereço em A2
                    // -------------------------------------------------------------
                    3 => {
                        let is_true = match &vm_registers[1] {
                            DataTypes::Operational(n) => *n != 0,
                            _ => panic!(
                                "Erro: A1 para IF TRUE JUMP precisa ser um Operational (usize)."
                            ),
                        };

                        if is_true {
                            let target_line = match &vm_registers[2] {
                                DataTypes::Operational(addr) => *addr,
                                _ => panic!(
                                    "Erro: A2 para IF TRUE JUMP precisa conter um endereço operacional (_C)."
                                ),
                            };

                            //println!("Pulando pra linha: {}", target_line + 1);
                            // Atualiza o ponteiro de linha.
                            line = target_line;
                        }
                    }

                    // -------------------------------------------------------------
                    // 4 - EXPRESSÃO BOOLEANA: Compara A1 e A3 usando o operador de A2, joga em A4
                    // -------------------------------------------------------------
                    4 => {
                        let op = match &vm_registers[2] {
                            DataTypes::Operational(v) => *v,
                            _ => panic!("Erro: Operador em A2 inválido."),
                        };

                        //println!("Booleano. Expressão: {}", &op);
                        let resultado = match op {
                            0 => {
                                // ==
                                if vm_registers[1] == vm_registers[3] {
                                    1
                                } else {
                                    0
                                }
                            }
                            1 => {
                                // !=
                                if vm_registers[1] != vm_registers[3] {
                                    1
                                } else {
                                    0
                                }
                            }
                            2 => {
                                // >
                                if vm_registers[1] > vm_registers[3] {
                                    1
                                } else {
                                    0
                                }
                            }
                            3 => {
                                // <
                                if vm_registers[1] < vm_registers[3] {
                                    1
                                } else {
                                    0
                                }
                            }
                            4 => {
                                // >=
                                if vm_registers[1] >= vm_registers[3] {
                                    1
                                } else {
                                    0
                                }
                            }
                            5 => {
                                // <=
                                /*
                                println!(
                                    "{:?} <= {:?} : {}",
                                    vm_registers[1],
                                    vm_registers[3],
                                    vm_registers[1] <= vm_registers[3]
                                );
                                */
                                if vm_registers[1] <= vm_registers[3] {
                                    1
                                } else {
                                    0
                                }
                            }
                            6 => {
                                // ||
                                if vm_registers[1] == DataTypes::Operational(1)
                                    || vm_registers[3] == DataTypes::Operational(1)
                                {
                                    1
                                } else {
                                    0
                                }
                            }
                            7 => {
                                // &&
                                if vm_registers[1] == DataTypes::Operational(1)
                                    && vm_registers[3] == DataTypes::Operational(1)
                                {
                                    1
                                } else {
                                    0
                                }
                            }
                            _ => panic!("Operador lógico {} inválido.", op),
                        };

                        // Guarda o resultado (1 para true, 0 para false) no registrador de resposta A4
                        vm_registers[4] = DataTypes::Operational(resultado);
                    }
                    // -------------------------------------------------------------
                    // 5 - EXPRESSÃO MATEMÁTICA: Manipula A1 (Double) e A3 (Double) usando o operador de A2, joga em A4 como Double
                    // -------------------------------------------------------------
                    5 => {
                        let op = match &vm_registers[2] {
                            DataTypes::Operational(v) => *v,
                            _ => panic!("Erro: Operador em A2 inválido."),
                        };
                        let num1 = match vm_registers[1] {
                            DataTypes::Doble(t) => t,
                            DataTypes::Float(t) => t as f64,
                            DataTypes::I32(t) => t as f64,
                            DataTypes::I16(t) => t as f64,
                            DataTypes::I8(t) => t as f64,
                            DataTypes::Operational(t) => t as f64,
                            _ => {
                                panic!("Valor precisa necessariamente ser um número");
                            }
                        };

                        let num2 = match vm_registers[3] {
                            DataTypes::Doble(t) => t,
                            DataTypes::Float(t) => t as f64,
                            DataTypes::I32(t) => t as f64,
                            DataTypes::I16(t) => t as f64,
                            DataTypes::I8(t) => t as f64,
                            DataTypes::Operational(t) => t as f64,
                            _ => {
                                panic!("Valor precisa necessariamente ser um número");
                            }
                        };
                        let resultado: f64 = match op {
                            0 => {
                                // +
                                num1 + num2
                            }
                            1 => {
                                // -
                                num1 - num2
                            }
                            2 => {
                                // *
                                num1 * num2
                            }
                            3 => {
                                // /
                                if num1 == 0.0 || num2 == 0.0 {
                                    panic!("CANNOT DIVIDE WITH ZERO");
                                };
                                num1 / num2
                            }
                            4 => {
                                // %
                                num1 % num2
                            }
                            5 => {
                                // **
                                num1.powf(num2)
                            }
                            _ => panic!("Operador matemático {} inválido.", op),
                        };

                        // Guarda o resultado no registrador de resposta A4
                        vm_registers[4] = DataTypes::Doble(resultado);
                    }
                    // -------------------------------------------------------------
                    // 6 - INCREMENTAR: Incrementa em 1 o valor em A1. Devolve o resultado em A4
                    // -------------------------------------------------------------
                    6 => {
                        let value = &vm_registers[1];
                        let new_v = match value {
                            DataTypes::Doble(v) => DataTypes::Doble(v + 1.0),
                            DataTypes::Float(v) => DataTypes::Float(v + 1.0),
                            DataTypes::I32(v) => DataTypes::I32(v + 1),
                            DataTypes::I16(v) => DataTypes::I16(v + 1),
                            DataTypes::I8(v) => DataTypes::I8(v + 1),
                            DataTypes::Operational(v) => DataTypes::Operational(v + 1),
                            _ => {
                                panic!("Impossível incrementar tipos não numéricos");
                            }
                        };

                        vm_registers[4] = new_v;
                    }

                    // -------------------------------------------------------------
                    // 7 - DECREMENTAR: Decrementa em 1 o valor em A1. Devolve o resultado em A4
                    // -------------------------------------------------------------
                    7 => {
                        let value = &vm_registers[1];
                        let new_v = match value {
                            DataTypes::Doble(v) => DataTypes::Doble(v - 1.0),
                            DataTypes::Float(v) => DataTypes::Float(v - 1.0),
                            DataTypes::I32(v) => DataTypes::I32(v - 1),
                            DataTypes::I16(v) => DataTypes::I16(v - 1),
                            DataTypes::I8(v) => DataTypes::I8(v - 1),
                            DataTypes::Operational(v) => DataTypes::Operational(v - 1),
                            _ => {
                                panic!("Impossível incrementar tipos não numéricos");
                            }
                        };

                        vm_registers[4] = new_v;
                    }
                    // -------------------------------------------------------------
                    // 8 - CONVERT:
                    // -------------------------------------------------------------
                    8 => {
                        // A1 = O dado original (que veio de uma variável ou registrador)
                        let dado_original = &vm_registers[1];

                        // A2 = O tipo de destino (0 a 6) em formato string/usize
                        let tipo_destino = match &vm_registers[2] {
                            DataTypes::Operational(v) => v.to_string(),
                            _ => panic!("Tipo de destino precisa ser Operational"),
                        };

                        // Executa a conversão e joga o resultado em A4
                        match dado_original.convert_to(&tipo_destino) {
                            Ok(resultado) => vm_registers[4] = resultado,
                            Err(_) => panic!("CONVERSION ERROR IN: {}", l),
                        }
                    }
                    // -------------------------------------------------------------
                    // 60 - PARAR A EXECUÇÃO: Termina a VM retornando o status de A1
                    // -------------------------------------------------------------
                    60 => {
                        let exit_code = match &vm_registers[1] {
                            DataTypes::Operational(v) => *v,
                            _ => 0,
                        };
                        println!(
                            "\n[VM] Execução finalizada pelo programa com código {}.",
                            exit_code
                        );
                        running = false;
                    }

                    _ => panic!("Syscall {} não reconhecida em A0.", syscall_type),
                }
            }
            _ => {}
        }
    }
}
