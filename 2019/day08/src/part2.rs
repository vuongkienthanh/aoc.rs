use crate::parsing::parse_input;

pub fn process(_input: &str) -> usize {
    let input = parse_input(_input);

    let mut image = vec![Color::Transparent; 150];

    for chunk in input.chunks(150) {
        for (i, c) in chunk.into_iter().enumerate() {
            let color: Color = (*c).into();
            image[i] = image[i].combine(&color);
        }
    }

    let mut img = image.into_iter();
    for row in 0..6 {
        for col in 0..25 {
            let pixel = img.next().unwrap();
            match pixel {
                Color::Transparent | Color::Black => print!(" "),
                Color::White => print!("#"),
            }
        }
        println!();
    }

    0
}

#[derive(Clone, Debug)]
enum Color {
    Transparent,
    Black,
    White,
}

impl Color {
    fn combine(&self, other: &Color) -> Color {
        match (self, other) {
            (Color::Transparent, x) => x.clone(),
            (Color::White, _) => Color::White,
            (Color::Black, _) => Color::Black,
        }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            _ => panic!("should be 0 1 2"),
        }
    }
}
