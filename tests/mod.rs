#[macro_use]
extern crate typed;
extern crate ferrous;
extern crate void;

use ferrous::dsl::*;

pub trait Increment {
    fn increment(&mut self, amt: usize) -> usize;
}

typed_stack!(Increment);

// TODO: Figure out an easy way to not have to manually impl trait here.
// This exposes a lot of the inner workings of the crate.
impl Increment for Void {
    fn increment(&mut self, _amt: usize) -> usize {
        unreachable!()
    }
}

impl<S1: Increment, S2> Increment for LinkImpl<S1, S2> {
    fn increment(&mut self, amt: usize) -> usize {
        self.obj.increment(amt)
    }
}

macro_rules! impl_trait {
    ($t:ty) => {
        impl Increment for $t {
            fn increment(&mut self, amt: usize) -> usize {
                self.0 += amt;
                self.0
            }
        }
    };
}

#[derive(Debug)]
struct Test1(usize);
#[derive(Debug)]
struct Test2(usize);
#[derive(Debug)]
struct Test3(usize);

impl_trait!(Test1);
impl_trait!(Test2);
impl_trait!(Test3);

#[test]
fn test_example() {
    let obj1 = Test1(0);
    let obj2 = Test2(0);
    let obj3 = Test3(0);

    let queue = TypedStack::<Void>::new();
    let queue = queue.add(obj1);
    let queue = queue.add(obj2);
    let mut queue = queue.add(obj3);

    expect(&queue.front()).to(be_some());

    iter(queue.front_mut().unwrap());
}

fn iter<T>(inc: &mut T)
where T: Link + Increment {
    expect(&inc.increment(10)).to(equal(&10));

    if let Some(t) = inc.next_mut() {
        iter(t);
    }
}
