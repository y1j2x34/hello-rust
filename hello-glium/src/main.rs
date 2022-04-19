// #[macro_use]
// extern crate glium;
// use rand;

// use glium::Surface;

// fn main() {
//     use glium::glutin;

//     let event_loop = glutin::event_loop::EventLoop::new();
//     let wb = glutin::window::WindowBuilder::new();
//     let cb = glutin::ContextBuilder::new();
//     let display = glium::Display::new(wb, cb, &event_loop).unwrap();
//     let random = || {
//         let val = rand::random::<u8>();
//         (val as f32) / 256.0
//     };
//     event_loop.run(move |ev, _, control_flow| {

//         let next_frame_time = std::time::Instant::now() +
//             std::time::Duration::from_nanos(16_666_667);

//         *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
//         match ev {
//             glutin::event::Event::WindowEvent { event, .. } => match event {
//                 glutin::event::WindowEvent::CloseRequested => {
//                     *control_flow = glutin::event_loop::ControlFlow::Exit;
//                     return;
//                 },
//                 _ => return,
//             },
//             _ => (),
//         }

//         let r = random();
//         let g = random();
//         let b = random();
//         let mut target = display.draw();
//         target.clear_color(r, g, b, 1.0);
//         target.finish().unwrap();

//     });
// }

use std::rc::Rc;
use std::cell::RefCell;

type NodeLeaf<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    left: NodeLeaf<T>,
    right: NodeLeaf<T>
}

impl<T> Node<T> {
    fn set_left(&mut self, left: Node::<T>) {
        self.left = self.wrap(left);
    }
    fn set_right(&mut self, right: Node::<T>) {
        self.right = self.wrap(right);
    }
    fn wrap(&self, node: Node::<T>) -> NodeLeaf<T> {
        return Some(Rc::new(RefCell::new(node)))
    }
}

fn main(){
    let mut root = Node::<u32> {
        value: 0, left: None, right: None
    };
    let left = Node::<u32> {
        value: 1, left: None, right: None
    };
    let right = Node::<u32> {
        value: 2, left: None, right: None
    };
    root.set_left(left);
    root.set_right(right);

    println!("root = {:?}", root.value);
    // println!("root.left = {:?}", root.left.unwrap().borrow().value);

    // root.left.unwrap().value = 4;

    if let Some(ref mut x) = root.left {
        x.borrow_mut().value = 4;
    }
    println!("root.left = {:?}", root.left.unwrap().borrow().value);
}