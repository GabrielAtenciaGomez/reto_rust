

pub mod test_traduccion {


    #[test]
    pub fn espaniol_a_ingles() {

        let mut resultado = String::new();
         resultado= match crate::logica::operaciones_json::traducir("blanco","en") {

            Ok(string)=>{string}
            , Err(error)=>{
                panic!("error al intentar traducir");
            }

        };
       

        assert_eq!(resultado.to_lowercase() , "white");
    }
    #[test]
    pub fn ingles_a_espaniol() {

        let mut resultado = String::new();
         resultado= match crate::logica::operaciones_json::traducir("white","es") {

            Ok(string)=>{string}
            , Err(error)=>{
                panic!("error al intentar traducir");
            }

        };
       

        assert_eq!(resultado.to_lowercase() , "blanco");
    }

    #[test]
    pub fn suma() {

        let mut resultado = crate::logica::operaciones_json::suma(&"1".to_string(),&"2".to_string());
       

        assert_eq!(resultado.to_lowercase() , "3");
    }

    #[test]
    pub fn resta() {

        let mut resultado = crate::logica::operaciones_json::resta(&"2".to_string(),&"1".to_string());     

        assert_eq!(resultado.to_lowercase() , "1");
    }

    #[test]
    pub fn resta_decimales() {

        let mut resultado = crate::logica::operaciones_json::resta(&"2.1".to_string(),&"1".to_string());     

        assert_eq!(resultado.to_lowercase() , "1.1");
    }


    #[test]
    pub fn multiplicacion() {

        let mut resultado = crate::logica::operaciones_json::multiplicacion(&"2".to_string(),&"2".to_string());     

        assert_eq!(resultado.to_lowercase() , "4");
    }

}