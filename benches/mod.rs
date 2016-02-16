#![feature(test)]
extern crate test;

#[macro_use]
extern crate typed;
extern crate void;

use test::Bencher;

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
    } }

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
#[derive(Debug)]
struct Test4(usize);
#[derive(Debug)]
struct Test5(usize);

impl_trait!(Test1);
impl_trait!(Test2);
impl_trait!(Test3);
impl_trait!(Test4);
impl_trait!(Test5);

#[bench]
fn bench_dynamic_dispath_64_elements(b: &mut Bencher) {
    let mut vec = Vec::new();
    for i in 0..64 {
        match i % 5 {
            0 => {
                vec.push(Box::new(Test1(0)) as Box<Increment>);
            },
            1 => {
                vec.push(Box::new(Test2(0)) as Box<Increment>);
            },
            2 => {
                vec.push(Box::new(Test3(0)) as Box<Increment>);
            },
            3 => {
                vec.push(Box::new(Test4(0)) as Box<Increment>);
            },
            _ => {
                vec.push(Box::new(Test5(0)) as Box<Increment>);
            }
        }
    }

    b.iter(|| {
        for i in vec.iter_mut() {
            i.increment(10);
        }
    });
}

#[bench]
fn bench_typed_stack_64_elements(b: &mut Bencher) {
    let s = TypedStack::<Void>::new();
    // let stack = construct(s, 100);
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let s = s.add(Test5(0));
    let s = s.add(Test1(0));
    let s = s.add(Test2(0));
    let s = s.add(Test3(0));
    let s = s.add(Test4(0));
    let mut s = s.add(Test5(0));

    b.iter(|| { iter(s.front_mut().unwrap()) });
}

// fn construct<T>(stack: TypedStack<T>, count: usize)
// -> TypedStack<LinkImpl<Test5, LinkImpl<Test4, LinkImpl<Test3, LinkImpl<Test2, LinkImpl<Test1, T>>>>>>
// where T: Increment + Link {
    // let s = stack.add(Test1(0));
    // let s = s.add(Test2(0));
    // let s = s.add(Test3(0));
    // let s = s.add(Test4(0));
    // let s = s.add(Test5(0));

//     let count = count - 5;

//     if count != 0 {
//         construct(s, count)
//     } else {
//         s
//     }
// }

#[inline]
fn iter<T>(inc: &mut T)
where T: Link + Increment {
    inc.increment(10);

    if let Some(t) = inc.next_mut() {
        iter(t);
    }
}
