/*
Uso de pub en metodo de libreria:
debe usarse para que la funcion/metodo sea accesible desde fuera del archivo.
*/
pub fn say_hello() {
    println!("Hello World!")
}

pub fn print_num(limit: u8) {
    let numbers = generate_numbers(limit);

    show_numbers(&numbers);
}

//Si deseamos que la funcion retorne un valor, debemos expresarlo mediante la sintaxis -> DataType antes de los brackets
fn generate_numbers(number: u8) -> Vec<u8> {
    /*
    let mut sequence = Vec::new();

    El asignador ..= declara un rango de numeros incluyendo ambos limites del rango. Sin el =, el rango solo incluiria el limite izquierdo del rango.

    for n in 1..=number {
        sequence.push(n);
    }

    La siguiente expresion es lo que devuelve la funcion, asi que no hay necesidad de usar la palabra clave return. La expresion a devolver debe escribirse SIN punto y coma, de lo contrario se consideraria un statement.
    
    sequence
    */

    //Uso del metodo collect, el cual genera una coleccion de datos. Normalmente, se especifica el tipo a devolver con la sintaxis collect::<DataType>, pero aqui lo infiere del return -> DataType.
    (1..=number).collect()
}

//Uso de slices ([DataType o Rango]) para ejecutar una funcion iteradora con arreglos y vectores simultaneamente
//Si un parametro de funcion se expresa como referencia (&), el arg de invocacion debe expresarse igual
fn show_numbers(numbers: &[u8]) {
    for num in numbers {
        println!("{:?}", num)
    }
}

#[test]
fn test_generate(){
    let result = generate_numbers(3);
    assert_eq!(result, &[1,2,3])
}
