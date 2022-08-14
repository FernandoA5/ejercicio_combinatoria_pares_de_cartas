#[derive(Debug)]
struct Cartas{
    carta: String,
    tipo: String,
}

fn main() {
    //Corazon, Trebol, Diamantes, Picas, comodin: 🥳
    //inicializar cartas
    let mazo = inicializar_cartas();
    
    let mut combinacion=0;
    let mut dos_pares=0; 
    
    for i in 0..mazo.len(){
        let c1 = String::from(mazo[i].carta.to_string());
        for j in (i+1)..mazo.len(){
            let c2 = String::from(mazo[j].carta.to_string());
            for k in (j+1)..mazo.len(){
                let c3 = String::from(mazo[k].carta.to_string());
                for l in (k+1)..mazo.len(){
                    let c4 = String::from(mazo[l].carta.to_string());
                    for m in (l+1)..mazo.len()
                    {                                                   
                        let c5 = String::from(mazo[m].carta.to_string());
                        if detectar_pares(c1.clone(),c2.clone(),c3.clone(),c4.clone(),c5.clone())
                        {
                            dos_pares+=1;
                        }
                        
                        combinacion+=1;          
                    }
                }
            }
        }
    }
    println!("combinaciones {}: ", combinacion);
    println!("Manos posibles con dos pares: {} ", dos_pares);

    //Combinatoria sin repetición:
    //n!/r!(n-r)!
    //53/5!(53-5)! = 2869685
}

fn detectar_pares(c1: String, c2: String, c3: String, c4: String, c5: String) -> bool{
    let cartas = [c1,c2,c3,c4,c5];
    let mut pares = 0;
    for i in 0..cartas.len(){
        for j in (i+1)..cartas.len(){
            if cartas[i] == cartas[j] || cartas[j] == "🥳".to_string(){
                pares+=1;
            }
        }
    }  
    if pares >= 4{
        return true;
    } else {
        return false;
    }
}
fn inicializar_cartas() -> Vec<Cartas>
{
    let mut cartas = Vec::new();
    let valores = ["A","2","3","4","5","6","7","8","9","10","J","Q","K"];
    let tipos = ["Corazon","Trebol","Diamantes","Picas"];
    for i in 0..valores.len()
    {
        for j in 0..tipos.len()
        {
            cartas.push(Cartas{carta: valores[i].to_string(), tipo: tipos[j].to_string()});
        }
    }
    cartas.push(Cartas{carta: "🥳".to_string(), tipo: "🥳".to_string()});
    cartas
}

