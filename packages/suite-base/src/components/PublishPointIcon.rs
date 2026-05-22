```rust
use std::rc::Rc;

use mui::{
    components::{svgicons::SvgIcon, SvgIconProps},
};

#[derive(Clone, Debug)]
pub struct PublishPointIcon {
    props: SvgIconProps,
}

impl PublishPointIcon {
    pub fn new(props: SvgIconProps) -> Self {
        Self { props }
    }

    #[allow(unused)]
    pub fn as_svg_icon(&self) -> Rc<dyn SvgIcon<SvgIconProps>> {
        Rc::new(Self { props })
    }
}
```