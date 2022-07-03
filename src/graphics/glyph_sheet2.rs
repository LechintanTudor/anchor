use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug)]
pub struct GlyphBounds {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl GlyphBounds {
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

pub struct GlyphSheet {
    glyphs: FxHashMap<char, GlyphBounds>,
    atlas: Box<[u8]>,
    atlas_width: u32,
    atlas_height: u32,
    cursor_x: u32,
    cursor_y: u32,
    next_cursor_y: u32,
}

impl Default for GlyphSheet {
    fn default() -> Self {
        Self::with_size(256, 256)
    }
}

impl GlyphSheet {
    pub fn with_size(width: u32, height: u32) -> Self {
        Self {
            glyphs: Default::default(),
            atlas: vec![0; (width * height) as usize].into_boxed_slice(),
            atlas_width: width,
            atlas_height: height,
            cursor_x: 0,
            cursor_y: 0,
            next_cursor_y: 0,
        }
    }

    pub fn insert(&mut self, c: char, bitmap: &[u8], bitmap_width: u32, bitmap_height: u32) {
        assert_eq!(bitmap.len(), (bitmap_width * bitmap_height) as usize);

        // Resize atlas until we have a free position available
        let (x, y) = loop {
            match self.get_next_free_position(bitmap_width, bitmap_height) {
                Some(position) => break position,
                None => self.resize(),
            }
        };

        // Copy bitmap to atlas, row by row
        for i in 0..bitmap_height {
            let bitmap_offset = (i * bitmap_width) as usize;
            let atlas_offset = ((y + i) * self.atlas_width + x) as usize;

            unsafe {
                let src = bitmap.as_ptr().add(bitmap_offset);
                let dst = self.atlas.as_mut_ptr().add(atlas_offset);
                std::ptr::copy_nonoverlapping(src, dst, bitmap.len());
            }
        }

        // Update glyph map and cursors
        self.glyphs.insert(c, GlyphBounds { x, y, w: bitmap_width, h: bitmap_height });
        self.cursor_x = x + bitmap_width;
        self.cursor_y = y;
        self.next_cursor_y = self.next_cursor_y.max(y + bitmap_height);
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.atlas_width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.atlas_height
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.atlas
    }

    fn get_next_free_position(&self, width: u32, height: u32) -> Option<(u32, u32)> {
        if self.cursor_x + width < self.atlas_width {
            if self.cursor_y + height < self.atlas_height {
                Some((self.cursor_x, self.cursor_y))
            } else {
                None
            }
        } else {
            if width < self.atlas_width && self.next_cursor_y + height < self.atlas_height {
                Some((self.cursor_x, self.next_cursor_y))
            } else {
                None
            }
        }
    }

    pub fn resize(&mut self) {
        let sheet = Self::with_size(self.atlas_width, self.atlas_height);
        todo!()
    }
}
