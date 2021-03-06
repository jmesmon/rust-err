#[macro_export]
macro_rules! some_error {
    ($call:expr, $($arg:expr),*) => (
        match $call {
            Some(e) => panic!("{} => {:?} :: {}", stringify!($call), e, format!($($arg),*)),
            None => ()
        }
    );
    ($call:expr) => (
        match $call {
            Some(e) => panic!("{} => {:?}", stringify!($call), e),
            None => ()
        }
    )
}

#[macro_export]
macro_rules! or_panic {
    ($e:expr) => ({
        let v : Result<_,_> = $e;
        let s = stringify!($e);
        or_panic!(v, "failure in {:?} = {:?}", s, v.err())
    });
    ($e:expr, $($arg:expr),+) => (
        match $e {
            Ok(e) => e,
            Err(_) => {
                panic!($($arg),+)
            }
        }
    );
}

#[macro_export]
macro_rules! try_none {
    ($e:expr) => ({
        match $e {
            Ok(v) => v,
            Err(_) => None,
        }
    })
}

#[macro_export]
macro_rules! from  {
    ($enum_base:ident => $enum_elem:ident($elem_type:ty)) => (
        impl ::std::convert::From<$elem_type> for $enum_base {
            fn from(err: $elem_type) -> $enum_base {
                $enum_base::$enum_elem(err)
            }
        }
    )
}

#[macro_export]
macro_rules! auto_from_enum {
    (enum $enum_name:ident { $( $elem:ident ( $elem_type:ty ) ),* }) => (
        #[derive(Debug)]
        enum $enum_name { $($elem($elem_type)),* }

        $(
        impl ::std::convert::From<$elem_type> for $enum_name {
            fn from(err: $elem_type) -> $enum_name {
                $enum_name::$elem(err)
            }
        }
        )*
    )
}

#[macro_export]
macro_rules! from_enum {
    (auto $enum_name:ident $elem:ident ( $elem_type:ty ) ) => (
        impl ::std::convert::From<$elem_type> for $enum_name {
            fn from(err: $elem_type) -> $enum_name {
                $enum_name::$elem(err)
            }
        }
    );

    (bare $enum_name:ident $elem:ident ( $($elem_type:ty),*) ) => (
    );

    ($(pub enum $enum_name:ident { $($elem_kind:ident $elem:ident ( $($elem_type:ty),* ) ),* })* ) => (
        $(
        #[derive(Debug)]
        pub enum $enum_name { $($elem($($elem_type),*)),* }

        $(from_enum!{$elem_kind $enum_name $elem ( $($elem_type),* ) })*
        )*
    )
}

#[cfg(test)]
mod test {

    #[derive(Debug)]
    pub enum AnError {
        Foo(&'static str)
    }

    from! { AnError => Foo(&'static str) }

    fn f() -> Result<(),AnError> {
        try!(Err("hi"))
    }

    #[test]
    fn it_works() {
        or_panic!(Result::Ok::<(),&'static str>(()));
        or_panic!(Result::Ok::<(),&'static str>(()), "HI");
        or_panic!(Result::Ok::<(),&'static str>(()), "HI {:?}", 3);

        println!("Got: {:?}", f())
    }

    mod auto {
        auto_from_enum! {enum FooEnum {
            Foo(&'static str)
        }}

        fn y() -> Result<(),FooEnum> {
            try!(Err("hi"))
        }

        #[test]
        fn test() {
            println!("Got: {:?}", y());
        }
    }

    mod ctrl {
        #![allow(dead_code)]
        from_enum! {
            pub enum GenEnum {
                auto Foo(&'static str),
                bare Bar(usize)
            }
            pub enum NoopEnum {
                bare Noop(usize),
                auto Foo(&'static str)
            }
        }

        fn x() -> Result<(),GenEnum> {
            try!(Err("hi"))
        }

        #[test]
        fn test() {
            println!("Got: {:?} {:?}", x(), GenEnum::Bar(3));
        }

        /*
        #[should_fail_to_build]
        fn fail_conv() {
            fn y() -> Result<(),GenEnum> {
                try!(Err(3usize))
            }
            println!("Got: {:?}", y());
        }
        */
    }
}
