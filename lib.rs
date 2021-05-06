#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct LetterMover {
    letters: [char; BUFFER_WIDTH],
    num_letters: ModNumC<usize, BUFFER_WIDTH>,
    next_letter: ModNumC<usize, BUFFER_WIDTH>,
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
    dx: ModNumC<usize, BUFFER_WIDTH>,
    dy: ModNumC<usize, BUFFER_HEIGHT>,
    pub example: [usize; 6],
    pub bad:[usize;6],
    score:isize
}


impl LetterMover {
    pub fn new() -> Self {
        LetterMover {
            letters: ['*'; BUFFER_WIDTH],
            num_letters: ModNumC::new(1),
            next_letter: ModNumC::new(1),
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(BUFFER_HEIGHT / 2),
            dx: ModNumC::new(0),
            dy: ModNumC::new(0),
            example: [2,5,8,16,15,9],
            bad:[1,3,7,19,13,11],
            score:0
        }
    }

    fn letter_columns(&self) -> impl Iterator<Item=usize> {
        ModNumIterator::new(self.col)
            .take(self.num_letters.a())
            .map(|m| m.a())
    }

    pub fn tick(&mut self) {
        self.draw_food();
        self.draw_bad();
        self.draw_score();
        self.clear_current();
        self.update_location();
        self.draw_current();
        self.clear_food();
        self.check_reset();
    }

    fn clear_current(&self) {
        for x in self.letter_columns() {
            plot(' ', x, self.row.a(), ColorCode::new(Color::Black, Color::Black));
        }
    }

    fn clear_food(&mut self){
        for (i, x) in self.letter_columns().enumerate(){
            for y in self.example.iter(){
                if x == *y && self.row.a() == *y  {
                    self.letters[self.next_letter.a()] = '*';
                    self.next_letter += 1;
                    self.score += 1;
                    self.num_letters = self.num_letters.saturating_add(&ModNumC::new(1));
                }
            }
        }
    }


    fn update_location(&mut self) {
        self.col += self.dx;
        self.row += self.dy;
    }

    fn draw_current(&self) {
        for (i, x) in self.letter_columns().enumerate() {
            plot(self.letters[i],  x, self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
        }
    }

    fn draw_food(&self) {
        for x in self.example.iter() {
            plot('@', *x, *x, ColorCode::new(Color::Cyan, Color::Black));
        }
    }

    fn draw_bad(&self){
        for b in self.bad.iter(){
            plot('W',*b,*b,ColorCode::new(Color::Red, Color::Black));
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.dx -= 1;
            }
            KeyCode::ArrowRight => {
                self.dx += 1;
            }
            KeyCode::ArrowUp => {
                self.dy -= 1;
            }
            KeyCode::ArrowDown => {
                self.dy += 1;
            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            return;
        }
    }

    fn check_reset(&mut self){
        let mut reset = false;
        for (i, x) in self.letter_columns().enumerate() {
            for y in self.bad.iter() {
                if x == *y && self.row.a() == *y {
                    reset = true;
                }
            }
        }
        if reset {self.reset();}
    }

    fn reset(&mut self){
        self.clear_current();
        self.letters = ['*'; BUFFER_WIDTH];
        self.num_letters =  ModNumC::new(1);
        self.next_letter = ModNumC::new(1);
        self.col = ModNumC::new(BUFFER_WIDTH / 2);
        self.row = ModNumC::new(BUFFER_HEIGHT / 2);
        self.dx = ModNumC::new(0);
        self.dy = ModNumC::new(0);
    }

    fn draw_score(&mut self){
        plot_num(self.score,0,0,ColorCode::new(Color::Yellow,Color::Black));
    }

}
