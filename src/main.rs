//use serde_json::Value;
use std::fs;
mod logica;
mod tests;


use logica::operaciones_json;

fn main() {
    
    #[cfg(test)]
   tests::test::test_traduccion::espaniol_a_ingles();


    let filename = "dato2.json";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let json: serde_json::Value = serde_json::from_str(&contents).expect("erro1");
    
    

    //('modified',traduce ->es)
    //('state',mult -> 5)
    //('state',suma -> 5)
    //('state',cm -> mts)
    //mts->cm
    
    println!("resultado: {}", operaciones_json::configuracion(&json,"meta/view/('attribution', traduce -> es)"));




}

