extern crate aoc;
extern crate png;

fn main() {
    let input = aoc::input!();
    let image = Image::new(input.trim(), 25, 6);
    let layer_min_by_0 = image.layer_min_by(count_pixel_in_layer('0'.into()));
    let count_1s = count_pixel_in_layer('1'.into())(layer_min_by_0);
    let count_2s = count_pixel_in_layer('2'.into())(layer_min_by_0);

    println!("{} x {} = {}", count_1s, count_2s, count_1s * count_2s);

    let image_data: Vec<u8> = (0..6)
        .flat_map(|y| (0..25).map(move |x| (x, y)))
        .flat_map(|(x, y)| image.get_pixel_at(x, y).to_rgb())
        .collect();

    let path = std::path::Path::new("day8.png");
    let file = std::fs::File::create(path).unwrap();
    let w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 25, 6);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image_data).unwrap();
}

fn count_pixel_in_layer(needle: Pixel) -> impl Fn(&[Vec<Pixel>]) -> usize {
    move |l| l.iter().flatten().filter(|c| &&needle == c).count()
}

#[derive(Debug, PartialEq, Eq)]
enum Pixel {
    Black,
    White,
    Transparent,
}

impl Pixel {
    pub fn is_transparent(&self) -> bool {
        match self {
            Pixel::Transparent => true,
            _ => false,
        }
    }

    pub fn to_rgb(&self) -> Vec<u8> {
        match self {
            Pixel::Black => vec![0, 0, 0],
            Pixel::White => vec![255, 255, 255],
            Pixel::Transparent => panic!("Transparent pixel does not have a RGB value"),
        }
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '0' => Pixel::Black,
            '1' => Pixel::White,
            '2' => Pixel::Transparent,
            _ => panic!("Unmapped pixel color: {:?}", c),
        }
    }
}

impl Into<char> for Pixel {
    fn into(self) -> char {
        match self {
            Pixel::Black => '0',
            Pixel::White => '1',
            Pixel::Transparent => '2',
        }
    }
}

#[derive(Debug)]
struct Image {
    layers: Vec<Vec<Vec<Pixel>>>,
}

impl Image {
    pub fn new(input: &str, width: usize, height: usize) -> Self {
        let mut chars = input.chars().peekable();
        let mut image = Image { layers: Vec::new() };
        while chars.peek().is_some() {
            let layer = (0..height)
                .map(|_y| {
                    (0..width)
                        .map(|_x| chars.next().unwrap().into())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            image.layers.push(layer);
        }
        image
    }

    pub fn layer_min_by<F>(&self, cb: F) -> &Vec<Vec<Pixel>>
    where
        F: Fn(&[Vec<Pixel>]) -> usize,
    {
        let mut min = (std::usize::MAX, &self.layers[0]);
        for layer in &self.layers {
            let val = cb(layer);
            if val < min.0 {
                min = (val, layer);
            }
        }

        min.1
    }

    pub fn get_pixel_at(&self, x: usize, y: usize) -> &Pixel {
        for layer in &self.layers {
            let pixel = &layer[y][x];
            if !pixel.is_transparent() {
                return pixel;
            }
        }
        unimplemented!();
    }
}
