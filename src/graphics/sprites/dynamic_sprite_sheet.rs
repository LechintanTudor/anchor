use crate::graphics::Image;
use image::{GenericImage, RgbaImage};
use std::cmp::{self, Ordering};
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
struct SpriteData {
    index: usize,
    image: Image,
}

impl PartialEq for SpriteData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for SpriteData {}

impl PartialOrd for SpriteData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpriteData {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_dimension_1 = cmp::max(self.image.width(), self.image.height());
        let max_dimension_2 = cmp::max(other.image.width(), other.image.height());
        max_dimension_1.cmp(&max_dimension_2)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Default, Debug)]
enum AtlasNodeState {
    #[default]
    Unused,
    Used,
    UsedLeaf(SpriteData),
}

impl AtlasNodeState {
    fn is_used(&self) -> bool {
        !matches!(self, Self::Unused)
    }
}

#[derive(Clone, Debug)]
struct AtlasNode {
    bounds: Bounds,
    state: AtlasNodeState,
    children: Option<Box<(AtlasNode, AtlasNode)>>,
}

impl AtlasNode {
    fn root(sprite: SpriteData) -> Self {
        Self {
            bounds: Bounds { x: 0, y: 0, w: sprite.image.width(), h: sprite.image.height() },
            state: AtlasNodeState::UsedLeaf(sprite),
            children: None,
        }
    }

    fn from_bounds(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { bounds: Bounds { x, y, w, h }, state: AtlasNodeState::Unused, children: None }
    }

    fn insert(&mut self, sprite: SpriteData) {
        let width = sprite.image.width();
        let height = sprite.image.height();

        match self.find(width, height) {
            Some(node) => node.set_sprite(sprite),
            None => {
                let node = self.grow(width, height).unwrap();
                node.set_sprite(sprite);
            }
        }
    }

    fn find(&mut self, width: u32, height: u32) -> Option<&mut AtlasNode> {
        if self.state.is_used() {
            let (right, down) = self.children.as_mut().map(Box::as_mut)?;

            match right.find(width, height) {
                Some(node) => Some(node),
                None => down.find(width, height),
            }
        } else {
            if self.bounds.w >= width && self.bounds.h >= height {
                Some(self)
            } else {
                None
            }
        }
    }

    fn set_sprite(&mut self, sprite: SpriteData) {
        let bounds = self.bounds;
        let width = sprite.image.width();
        let height = sprite.image.height();

        self.state = AtlasNodeState::UsedLeaf(sprite);
        self.children = Some(Box::new((
            Self::from_bounds(bounds.x, bounds.y + height, bounds.w, bounds.h - height),
            Self::from_bounds(bounds.x + width, bounds.y, bounds.w - width, bounds.h),
        )));
    }

    fn grow(&mut self, width: u32, height: u32) -> Option<&mut AtlasNode> {
        let can_grow_right = self.bounds.w >= width;
        let can_grow_down = self.bounds.h >= height;

        let should_grow_right = can_grow_right && (self.bounds.h >= self.bounds.w + width);
        let should_grow_down = can_grow_down && (self.bounds.w >= self.bounds.h + height);

        if should_grow_right {
            self.grow_right(width, height)
        } else if should_grow_down {
            self.grow_down(width, height)
        } else if can_grow_right {
            self.grow_right(width, height)
        } else if can_grow_down {
            self.grow_down(width, height)
        } else {
            None
        }
    }

    fn grow_right(&mut self, width: u32, height: u32) -> Option<&mut AtlasNode> {
        let bounds = self.bounds;

        *self = AtlasNode {
            bounds: Bounds { x: 0, y: 0, w: bounds.w + width, h: bounds.h },
            state: AtlasNodeState::Used,
            children: Some(Box::new((
                self.clone(),
                AtlasNode::from_bounds(bounds.w, 0, width, bounds.h),
            ))),
        };

        self.find(width, height)
    }

    fn grow_down(&mut self, width: u32, height: u32) -> Option<&mut AtlasNode> {
        let bounds = self.bounds;

        *self = AtlasNode {
            bounds: Bounds { x: 0, y: 0, w: bounds.w, h: bounds.h + height },
            state: AtlasNodeState::Used,
            children: Some(Box::new((
                AtlasNode::from_bounds(0, bounds.h, bounds.w, height),
                self.clone(),
            ))),
        };

        self.find(width, height)
    }

    fn for_each_leaf(&self, mut f: impl FnMut(usize, u32, u32, &Image)) {
        let mut nodes = vec![self];

        while let Some(node) = nodes.pop() {
            if let AtlasNodeState::UsedLeaf(sprite) = &node.state {
                f(sprite.index, node.bounds.x, node.bounds.y, &sprite.image);
            }

            if node.state.is_used() {
                if let Some((right, down)) = node.children.as_ref().map(Box::as_ref) {
                    nodes.extend([right, down]);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct DynamicSpriteSheetBuilder {
    sprites: BinaryHeap<SpriteData>,
}

impl DynamicSpriteSheetBuilder {
    pub fn add_sprite(&mut self, image: Image) -> usize {
        let index = self.sprites.len() + 1;
        self.sprites.push(SpriteData { index, image });
        index
    }

    pub fn build(&mut self) -> DynamicSpriteSheet {
        let sprite_count = self.sprites.len();

        let sprite_tree = {
            let mut root = AtlasNode::root(self.sprites.pop().unwrap());

            for sprite in self.sprites.drain() {
                root.insert(sprite);
            }

            root
        };

        let width = sprite_tree.bounds.w;
        let height = sprite_tree.bounds.h;

        let mut atlas = RgbaImage::new(width, height);
        let mut bounds = vec![Bounds::default(); sprite_count + 1];
        bounds[0] = Bounds { x: 0, y: 0, w: width, h: height };

        sprite_tree.for_each_leaf(|index, x, y, image| {
            atlas.copy_from(&image.0, x, y).unwrap();
            bounds[index] = Bounds { x, y, w: image.width(), h: image.height() };
        });

        DynamicSpriteSheet { image: Image(atlas), bounds }
    }
}

#[derive(Clone, Debug)]
pub struct DynamicSpriteSheet {
    image: Image,
    bounds: Vec<Bounds>,
}

impl DynamicSpriteSheet {
    #[inline]
    pub fn builder() -> DynamicSpriteSheetBuilder {
        DynamicSpriteSheetBuilder::default()
    }

    #[inline]
    pub fn get_bounds(&self, index: usize) -> Option<&Bounds> {
        self.bounds.get(index)
    }

    #[inline]
    pub fn image(&self) -> &Image {
        &self.image
    }
}
