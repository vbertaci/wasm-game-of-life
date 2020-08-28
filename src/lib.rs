mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, linha: u32, coluna: u32) -> usize {
        (linha * self.width + coluna) as usize
    }

    //retorna o numero de celulas vizinhas viva
    fn live_neighbor_count(&self, linha: u32, coluna: u32) -> u8 {
        let mut count = 0;
        for linha_iter in [self.height - 1, 0 , 1].iter().cloned() {
            for coluna_iter in [self.width - 1, 0, 1].iter().cloned() {
                if linha_iter == 0 && coluna_iter == 0 {
                    continue;
                }
                let linha_vizinha = (linha + linha_iter) % self.height;
                let coluna_vizinha = (coluna + coluna_iter) % self.width;
                let index = self.get_index(linha_vizinha, coluna_vizinha);
                count += self.cells[index] as u8;
            }
        }
        count
    } 

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        //Clona o objeto de parametro, no caso, nossa struct Universe
        let mut proxima = self.cells.clone();

        for linha in 0..self.height {
            for coluna in 0..self.width{
                let index = self.get_index(linha, coluna);
                let cell = self.cells[index];
                let vizinhos_vivos = self.live_neighbor_count(linha, coluna);

                let proxima_celula = match (cell, vizinhos_vivos) {
                    //Regra numero 1: Qualquer celula com menos de dois vizinhos morre
                    (Cell::Alive, x) if x < 2 => Cell::Dead,

                    //Regra numero 2: Qualquer vizinho com dois ou tes vizinhos vive para
                    //a proxima geracao
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,

                    //Regra numero 3: Qualquer celula com mais de 3 vizinhos morre
                    (Cell::Alive, x) if x > 3 => Cell::Dead,

                    //Regra numero 4: Qualquer celula morta com exatos 3 vizinhos, torna-se
                    //uma celula viva
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                proxima[index] = proxima_celula
            }
        }
        self.cells = proxima;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for linha in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in linha {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(()) 
    }
}