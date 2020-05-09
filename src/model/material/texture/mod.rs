pub struct Texture {
    pub img_data: Option<Vec<u8>>,
    pub size: (u32, u32),
}
impl Texture {
    pub fn new() -> Self {
        return Texture {
            img_data: None,
            size: (0, 0),
        };
    }
    pub fn set_for_png(&mut self, path: &'static [u8]) {
        let png = std::io::Cursor::new(path);
        let decoder = png::Decoder::new(png);
        let (info, mut reader) = decoder.read_info().expect("can read info");

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).expect("can read png frame");
        self.img_data = Some(buf);
        self.size = (info.width, info.height)
    }
    pub fn new_for_png(path: &'static [u8]) -> Self {
        let mut t = Texture::new();
        t.set_for_png(path);
        t
    }
}
