const TAMANHO_LISTA: usize = 9; 

// Option<u8> - Cria um array de numeros u8 (Sem sinal e de até 8 bits) 
// ou None (Equivalente ao Null)
// usize - Um numero sem sinal que representa o indice de uma lista
struct Cartas {
    nums: [Option<u8>; TAMANHO_LISTA], // Lista
    size: usize, // Total de numero adicionados a lista
}

impl Cartas {
    pub fn new() -> Self {
        Cartas {
            nums: [None; TAMANHO_LISTA],
            size: 0
        }
    }

    pub fn recebe_carta(&mut self, num: u8) {
        // Cria um array que vai de 0 a self.size + 1 e percorre
        for index in 0..(self.size + 1) {
            // Verifica o valor de self.nums[index]
            // Se for None(null), inclui o valor na ultima posição
            // Se não for None, verifica se o valor é >= ao numero da posição
            // se for, inicia a logica de inclusão do valor na posição correta
            match self.nums[index] {
                None => {
                    self.incluir_ultima_posicao(num);
                    break;
                },
                Some(num_carta) => {
                    if num_carta < num { continue };
                    
                    self.incluir_posicao(num, index);
                    break;
                }
            }
        }
    }
    
   fn incluir_posicao(&mut self, num: u8, posicao: usize) {
        // Cria um array da posição a self.size, reverte e percorre.
        // Ex: posicao = 2, self.size = 6
        // (posicao..self.size) = [2, 3, 4, 5]
        // (posicao..self.size).rev() = [5, 4, 3, 2]
        for index in (posicao..self.size).rev() {
            let number: Option<u8> = self.nums[index];
            
            // Se o valor não é None(null), passa o valor pra proxima posição
            // do array e transforma o atual em None.
            if number.is_some() {
                self.nums[index + 1] = number;
                self.nums[index] = None; 
            }
        }
        
        self.nums[posicao] = Some(num);
        self.size += 1;
    }
    
    fn incluir_ultima_posicao(&mut self, num: u8) {
        self.nums[self.size] = Some(num);
        self.size += 1;
    }

    pub fn ver_lista_cartas(&self) {
        print!("Lista de cartas: ");
        
        // Crio um array de 0 a self.size
        for index in 0..self.size {
            let number: Option<u8> = self.nums[index];
            if number.is_some() { print!("{} ", number.unwrap()); };
        }
        
        // Quebra a linha
        println!("");
    }
    
    pub fn ver_lista_cartas_invertida(&self) {
        print!("Lista de cartas invertida: ");
        // Como não consigo em rust fazer -> for (i = self.size - 1, i > 0, i--)
        // Utilizarei uma alternativa semelhante: Crio uma lista de 0 a self.size
        // E a reverto. Ex: (0..3) = [0, 1, 2], (0..3).rev() = [2, 1, 0]
        for index in (0..self.size).rev() {
            let number: Option<u8> = self.nums[index];
            if number.is_some() { print!("{} ", number.unwrap()); };
        }
        
        // Quebra a linha
        println!("");
    }
}

fn main() {
    let mut cartas: Cartas = Cartas::new();

    cartas.recebe_carta(1);
    cartas.recebe_carta(9);
    cartas.recebe_carta(4);
    cartas.recebe_carta(2);
    cartas.recebe_carta(3);
    cartas.recebe_carta(5);
    cartas.recebe_carta(7);
    cartas.recebe_carta(6);
    cartas.recebe_carta(8);
    
    cartas.ver_lista_cartas();
    
    cartas.ver_lista_cartas_invertida();
}
