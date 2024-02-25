pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// with this logic everyone can now add their implementations of their components
// and we mustn't worry about errors and check if the draw method implemented for the component
pub struct Button {
    pub width: u32,
    pub heigth: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!("not implemented yet");
    }
}

// we can also create this with generic types
// pub struct Screen<T: Draw>{
//      pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//      T: Draw,
// {
//      pub fn run(&self){
//          for component in self.components.iter(){
//              component.draw();
//          }
//      }
// }
