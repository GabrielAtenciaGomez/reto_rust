extern crate reqwest;

use anyhow::Result;
use isahc::{ResponseExt};
use serde_json::Value;

pub fn configuracion(json: &Value, configuracion: &str) -> String {
    let mut json_copia: Value = json.clone();
    let mut operacion = String::new();
    let mut resultado = String::new();
    if !json.is_null() {
        // esta linea prepara la forma en como va a ir navegando dentro del json
        let keys = configuracion.split("/");

        //este for navega hasta llegar al nivel que se desea segun la configuracion
        for elem in keys.into_iter() {
            let elem_limpio = elem.trim_matches(' ');

            //esta linea permite saber si el nivel al que se quiere acceder es un array
            if json_copia.clone()[elem_limpio].is_array() {
                //println!(" array");
                //let index = elem.parse::<usize>().unwrap_or(1);
                //let vector: Vec<Value> = serde_json::from_value(jsonCopia.clone()).expect("holas");

                *&mut json_copia = json_copia[elem_limpio].clone();

            //esta linea permite saber si el nivel al que se quiere acceder es un json para que sea tratado
            } else if json_copia.clone()[elem_limpio].is_object() {
                *&mut json_copia = json_copia[elem_limpio].clone();
            //  println!(" -------------------------objeto--------------------");

            //esta linea permite saber si el nivel en el que estamos es un array o un json
            } else if elem_limpio.parse::<isize>().unwrap_or(-1) != -1 {
                let index = elem_limpio.parse::<usize>().unwrap_or(0);

                //println!("dentro array  ");
                *&mut json_copia = json_copia[index].clone();

            // esta parte es para cuando ya se va a seleccionar solo un numero final, es decir, ya se va a guardar el valor final del campo de json
            } else {
                //esta variable guarda una referencia
                let aux = elem_limpio;

                //aqui se verifica si ya es la peticion solicitada
                if &aux[..2] == "('" && &aux[aux.len() - 1..] == ")" {
                    //println!("caracter: {}", &elem_limpio[..1]);

                    println!("1");
                    //se separa la peticion de el nombre de acceso a ultimo nivel del json ej: parte1:('state'      parte2: trad ->espa)
                    let peticion: Vec<&str> = aux.split(",").collect();

                    //se procede a almacenar el tipo de peticion solicitada para su tratamiento mas adelante
                    operacion = peticion[1].to_string().replace(")", "");

                    // se limpia y se procede a obtener el valor del dato ya ubicado en el ultimo nivel de busqueda
                    let index_sucio = peticion[0].to_string().replace("'", "").replace("(", "");
                    let index = index_sucio.trim_matches(' ');

                    // se procede a verificar si es un array el que almacena el valor de la ultima consulta
                    if json_copia.is_array() {
                        //se valida que el ultimo nivel si corresponda a un indice numerico
                        if index.parse::<isize>().unwrap_or(-1) != -1 {
                            *&mut json_copia =
                                json_copia[index.parse::<usize>().unwrap_or(0)].clone();
                        } else {
                            println!("error");
                        }

                    //si no es un array se le da el tratamiento comoi un json
                    } else {
                        *&mut json_copia = json_copia
                            [peticion[0].to_string().replace("'", "").replace("(", "")]
                        .clone();
                    }
                //si no hay coincidencia con algun elemento del json se procede a para el algotimo
                } else if json_copia[elem_limpio].clone().is_null() {
                    println!("2");
                    *&mut json_copia = json_copia[elem_limpio].clone();
                    break;
                } else {
                    println!("3");
                    *&mut json_copia = Value::Null;
                }
            }

            //println!("vector ={}", json_copia.clone());
        }
    } else {
        println!("else");
    }

    //se verifica que el dato obtenido del json sea diferente de null
    if !json_copia.clone().is_null() {
        let dato = json_copia
            .to_string()
            .trim_matches(' ')
            .replace("\"", "")
            .replace("[", "")
            .replace("]", "");
        //println!("vector ={} operacion: {}", &dato, operacion);
        let operacion_partida: Vec<&str> = operacion.split("->").collect();

        if operacion_partida[0].trim().eq("suma")
            || operacion_partida[0].trim().eq("resta")
            || operacion_partida[0].trim().eq("mult")
            || operacion_partida[0].trim().eq("div")
        {
            resultado = realizar_conversion(
                &dato,
                &operacion_partida[0].trim(),
                &operacion_partida[1].to_string(),
            );
        } else if operacion_partida[0].trim().eq("traduce"){
            resultado = realizar_conversion(
                &dato,
                &operacion_partida[0].trim(),
                &operacion_partida[1].to_string(),
            );

        }else {
            println!("entra: {}", &operacion.replace(" ", ""));
            resultado = realizar_conversion(
                &dato,
                &operacion.replace(" ", ""),
                &operacion_partida[1].to_string(),
            );
        }
    } else {
        println!("es null");
        resultado = "null".to_string();
    }

    resultado.to_string()
}

fn realizar_conversion(num1: &String, operacion: &str, num2: &String) -> String {
    let mut resultado = String::new();

    match operacion {
        "t" => println!("0"),
        "suma" => resultado = suma(num1, num2).clone(),
        "resta" => resultado = resta(num1, num2),
        "mult" => resultado = multiplicacion(num1, num2),
        "div" => resultado = division(num1, num2),
        "cm->mts" => resultado = cm_mts(num1),
        "mts->cm" => resultado = mts_cm(num1),
        "km->m" => resultado = km_m(num1),
        "m->km" => resultado = m_km(num1),
        "traduce" => {
           

            resultado = match traducir(num1, num2) {
                Ok(frase_traducida) =>{ frase_traducida},
                Err(error) => {panic!("Problem: {:?}", error);}
            }
        }

        _ => println!("something else!"),
    }
    //println!("{}", resultado);

    resultado
}

pub fn traducir(frase: &str, idioma: &str) -> Result<String> {  

    let frase_completa = frase.replace(" ", "%20");
    let mut busqueda = String::new();
    busqueda.push_str("https://translation.googleapis.com/language/translate/v2?key=AIzaSyBAttp8bKHyd5SBkZT_KOytb5Oakc6zmdU&target=");
    busqueda.push_str(idioma.trim());
    busqueda.push_str("&q=");
    busqueda.push_str(&frase_completa);
    //busqueda.push_str("hola");
    //println!("url busqueda: {}", busqueda);

    let mut response = isahc::get(busqueda)?;

    // Print some basic info about the response to standard output.
    //println!("Status: {}", response.status());
    //println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
    //print!("{}", response.text()?);

    let x = match response.text() {
        Ok(string) => string,
        Err(error) => {
            panic!("Problem : {:?}", error);
        }
    };

    let json: serde_json::Value = serde_json::from_str(&x).expect(" error");


  let frase_traducida=json["data"]["translations"][0]["translatedText"].to_string();
  let  resul_limpio= frase_traducida[1..frase_traducida.len()-1].to_string();
  
    return Ok(resul_limpio.to_string());
}

pub fn suma(num1: &String, num2: &String) -> String {
    let mut resultado = String::new();

    if num1.matches(".").count() == 1 && num2.matches(".").count() == 1 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        resultado = (a + b).to_string();
    } else if num1.matches(".").count() == 1 && num2.matches(".").count() == 0 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<isize>().unwrap_or(0);
        resultado = (a + (b as f64)).to_string();
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 1 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        resultado = ((a as f64) + b).to_string();
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 0 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<isize>().unwrap_or(0);
        resultado = (a + b).to_string();
    }

    resultado
}

pub fn resta(num1: &String, num2: &String) -> String {
    let mut resultado = String::new();

    if num1.matches(".").count() == 1 && num2.matches(".").count() == 1 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        resultado = (a - b).to_string();
    } else if num1.matches(".").count() == 1 && num2.matches(".").count() == 0 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<isize>().unwrap_or(0);
        resultado = (a - (b as f64)).to_string();
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 1 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        resultado = ((a as f64) - b).to_string();
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 0 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<isize>().unwrap_or(0);
        resultado = (a - b).to_string();
    }

    resultado
}

pub fn multiplicacion(num1: &String, num2: &String) -> String {
    let mut resultado = String::new();

    if num1.matches(".").count() == 1 && num2.matches(".").count() == 1 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        resultado = (a * b).to_string();
    } else if num1.matches(".").count() == 1 && num2.matches(".").count() == 0 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<isize>().unwrap_or(0);
        resultado = (a * (b as f64)).to_string();
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 1 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        resultado = ((a as f64) * b).to_string();
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 0 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<isize>().unwrap_or(0);
        resultado = (a * b).to_string();
    }

    resultado
}

pub fn division(num1: &String, num2: &String) -> String {
    let mut resultado = String::new();

    if num1.matches(".").count() == 1 && num2.matches(".").count() == 1 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        if b != 0.0 {
            resultado = (a / b).to_string();
        } else {
            resultado = "error".to_string();
        }
    } else if num1.matches(".").count() == 1 && num2.matches(".").count() == 0 {
        let a = num1.parse::<f64>().unwrap_or(0.0);
        let b = num2.parse::<isize>().unwrap_or(0);

        if b != 0 {
            resultado = (a / (b as f64)).to_string();
        } else {
            resultado = "error".to_string();
        }
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 1 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<f64>().unwrap_or(0.0);

        if b != 0.0 {
            resultado = ((a as f64) / b).to_string();
        } else {
            resultado = "error".to_string();
        }
    } else if num1.matches(".").count() == 0 && num2.matches(".").count() == 0 {
        let a = num1.parse::<isize>().unwrap_or(0);
        let b = num2.parse::<isize>().unwrap_or(0);
        if b != 0 {
            resultado = (a / b).to_string();
        } else {
            resultado = "error".to_string();
        }
    }

    resultado
}

fn cm_mts(num: &String) -> String {
    division(num, &"100.0".to_string())
}
fn mts_cm(num: &String) -> String {
    multiplicacion(num, &"100".to_string())
}

fn km_m(num1: &String) -> String {
    multiplicacion(num1, &"1000".to_string())
}

fn m_km(num1: &String) -> String {
    division(num1, &"1000.0".to_string())
}
