//! Typed Collections
//!
//! Compile-time, polymorphic containers that do not rely on trait objects.

#[deny(dead_code, missing_docs)]

#[cfg(test)] extern crate void;
#[cfg(test)] extern crate ferrous;

#[macro_export]
macro_rules! typed_stack {
    ($tr:ident) => {
        use void::Void;

        pub trait Link {
            type Next: Link + $tr;

            fn add(&mut self, next: Self::Next);
            fn next(&self) -> Option<&Self::Next>;
            fn next_mut(&mut self) -> Option<&mut Self::Next>;
        }

        impl Link for Void {
            type Next = Void;

            fn add(&mut self, _next: Self::Next) {
            }

            fn next(&self) -> Option<&Self::Next> {
                None
            }

            fn next_mut(&mut self) -> Option<&mut Self::Next> {
                None
            }
        }

        impl<S1, S2: Link + $tr> Link for LinkImpl<S1, S2> {
            type Next = S2;

            fn add(&mut self, next: Self::Next) {
                self.next = Some(next);
            }

            fn next(&self) -> Option<&Self::Next> {
                self.next.as_ref()
            }

            fn next_mut(&mut self) -> Option<&mut Self::Next> {
                self.next.as_mut()
            }
        }

        #[derive(Debug)]
        pub struct LinkImpl<S1, S2> {
            obj: S1,
            next: Option<S2>,
        }

        impl<S1, S2> LinkImpl<S1, S2> {
            pub fn new(obj: S1) -> LinkImpl<S1, S2> {
                LinkImpl {
                    obj: obj,
                    next: None,
                }
            }
        }

        pub struct TypedStack<T> {
            links: Option<T>,
        }

        impl<T: Link + $tr> TypedStack<T> {
            pub fn new () -> TypedStack<T> {
                TypedStack{
                    links: None,
                }
            }

            pub fn front(&mut self) -> Option<&T> {
                self.links.as_ref()
            }

            pub fn front_mut(&mut self) -> Option<&mut T> {
                self.links.as_mut()
            }

            pub fn add<N: $tr>(self, next: N) -> TypedStack<LinkImpl<N, T>> {
                let TypedStack { links: l } = self;
                let mut next = LinkImpl::new(next);

                if let Some(list) = l {
                    next.add(list);
                }

                TypedStack {
                    links: Some(next),
                }
            }
        }
    };
}



#[cfg(test)]
mod tests {
    use ferrous::dsl::*;

    pub trait Test {
        fn test_inc(&mut self);
        fn test_usize(&mut self) -> usize;
    }

    typed_stack!(Test);

    impl Test for Void {
        fn test_inc(&mut self) {
            unreachable!()
        }

        fn test_usize(&mut self) -> usize {
            unreachable!()
        }
    }

    impl<S1: Test, S2> Test for LinkImpl<S1, S2> {
        fn test_inc(&mut self) {
            self.obj.test_inc();
        }

        fn test_usize(&mut self) -> usize {
            self.obj.test_usize()
        }
    }

    #[derive(Debug)]
    struct Test1(usize);
    #[derive(Debug)]
    struct Test2(usize);
    #[derive(Debug)]
    struct Test3(usize);

    macro_rules! impl_test {
        ($t:ty) => {
            impl Test for $t {
                fn test_inc(&mut self) {
                    self.0 += 1;
                }
                fn test_usize(&mut self) -> usize {
                    self.0
                }
            }
        };
    }

    impl_test!(Test1);
    impl_test!(Test2);
    impl_test!(Test3);

    #[test]
    fn test_next() {
        let obj1 = Test1(0);
        let mut link1 = LinkImpl::new(obj1);
        let obj2 = Test2(0);
        let mut link2 = LinkImpl::new(obj2);
        let obj3 = Test3(0);
        let link3: LinkImpl<_, Void> = LinkImpl::new(obj3);

        link2.add(link3);
        link1.add(link2);

        let next = link1.next_mut();
        expect(&next).to(be_some());

        let next = next.unwrap().next();
        expect(&next).to(be_some());

        let last = next.unwrap().next();
        expect(&last).to(be_none());
    }

    fn recursive_inc_and_expect<T>(test: &mut T)
        where T: Link + Test {
            test.test_inc();
            expect(&test.test_usize()).to(equal(&1));

            if let Some(t) = test.next_mut() {
                recursive_inc_and_expect(t);
            }
        }

    #[test]
    fn test_impl() {
        let obj1 = Test1(0);
        let mut link1 = LinkImpl::new(obj1);
        let obj2 = Test2(0);
        let mut link2 = LinkImpl::new(obj2);
        let obj3 = Test3(0);
        let link3: LinkImpl<_, Void> = LinkImpl::new(obj3);

        link2.add(link3);
        link1.add(link2);

        recursive_inc_and_expect(&mut link1);
    }

    #[test]
    fn test_typed_typed() {
        let obj1 = Test1(0);
        let obj2 = Test2(0);
        let obj3 = Test3(0);

        let queue = TypedStack::<Void>::new();
        let queue = queue.add(obj1);
        let queue = queue.add(obj2);
        let mut queue = queue.add(obj3);

        expect(&queue.front()).to(be_some());

        recursive_inc_and_expect(queue.front_mut().unwrap());
    }
}
