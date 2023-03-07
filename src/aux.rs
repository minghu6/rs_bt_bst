

macro_rules! def_coll_init {
    (seq | $name:ident, $new:expr, $push:ident) => {
        macro_rules! $name {
            ( $$($value:expr),* ) => {{
                #[allow(unused_mut)]
                let mut _coll = $new;

                $$(
                    _coll.$push($value);
                )*

                _coll
            }};
        }
        #[allow(unused)]
        pub(crate) use $name;
    };
    (map | $name:ident, $new:expr) => {
        #[allow(unused)]
        macro_rules! $name {
            ( $$($k:expr => $v:expr),* $$(,)? ) => {{
                let mut _coll = $new;

                $$(
                    _coll.insert($k, $v);
                )*

                _coll
            }};
        }
        #[allow(unused)]
        pub(crate) use $name;
    };
}


def_coll_init!(seq | vecdeq, std::collections::VecDeque::new(), push_back);
def_coll_init!(map | hashmap, std::collections::HashMap::new());


pub trait Reverse {
    fn reverse(&self) -> Self;
}


impl Reverse for either::Either<(), ()> {
    fn reverse(&self) -> Self {
        match self {
            either::Either::Left(_) => either::Either::Right(()),
            either::Either::Right(_) => either::Either::Left(()),
        }
    }
}


#[cfg(test)]
pub fn gen_unique() -> impl FnMut() -> usize {
    let mut set = std::collections::HashSet::new();

    move || {
        let rand = || rand::random::<usize>();
        let mut v = rand();

        while set.contains(&v) {
            v = rand();
        }

        set.insert(v);

        v
    }
}
