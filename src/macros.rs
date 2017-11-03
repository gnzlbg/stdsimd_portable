pub use super::*;

macro_rules! impl_from {
    ($TO:ty, $FROM:ty) => {
        impl From<$FROM> for $TO {
            #[inline(always)]
            fn from(v: $FROM) -> Self {
                unsafe { ::std::mem::transmute(v) }
            }
        }
    }
}

macro_rules! impl_new {
    ($name:ident, $elty:ident, $($elid:ident),+) => {
        impl $name {
            #[inline(always)]
            pub const fn new($($elid: $elty),*) -> $name {
                $name(st::$name::new($($elid),*))
            }
        }
    }
}

macro_rules! impl_zeros {
    ($name:ident, $($zero:expr),+) => {
        impl $name {
            #[inline(always)]
            pub const fn zeros() -> $name {
                $name(st::$name::new($($zero),*))
            }
        }
    }
}

macro_rules! impl_ones {
    ($name:ident, $($one:expr),+) => {
        impl $name {
            #[inline(always)]
            pub const fn ones() -> $name {
                $name(st::$name::new($($one),*))
            }
        }
    }
}

macro_rules! impl_get {
    ($name:ident, $elty:ident) => {
        impl $name {
            #[inline(always)]
            pub fn get(&self, idx: usize) -> $elty {
                self.0.extract(idx as u32)
            }
        }
    }
}

macro_rules! impl_set {
    ($name:ident, $elty:ident) => {
        impl $name {
            #[inline(always)]
            pub fn set(&mut self, idx: usize, v: $elty) {
                *self = $name::from(self.0.replace(idx as u32, v));
            }
        }
    }
}

macro_rules! impl_add {
    ($name:ident, $feature:expr, $intr:ident) => {
        impl Add<$name> for $name {
            type Output = $name;
            fn add(self, other: $name) -> Self::Output {
                if cfg!(target_feature = $feature) {
                    let x: st::$name = unsafe {
                        std::mem::transmute(
                            vendor::$intr(
                                std::mem::transmute(self.0),
                                std::mem::transmute(other.0)
                            ))
                    };
                    $name::from(x)
                } else {
                    let mut v = $name::zeros();
                    for i in 0..4 {
                        v.set(i, self.0.extract(i as u32) + other.0.extract(i as u32));
                    }
                    v
                }
            }
        }
    }
}

macro_rules! impl_sub {
    ($name:ident, $feature:expr, $intr:ident) => {
        impl Sub<$name> for $name {
            type Output = $name;
            fn sub(self, other: $name) -> Self::Output {
                if cfg!(target_feature = $feature) {
                    let x: st::$name = unsafe {
                        std::mem::transmute(
                            vendor::$intr(
                                std::mem::transmute(self.0),
                                std::mem::transmute(other.0)
                            ))
                    };
                    $name::from(x)
                } else {
                    let mut v = $name::zeros();
                    for i in 0..4 {
                        v.set(i, self.0.extract(i as u32) - other.0.extract(i as u32));
                    }
                    v
                }
            }
        }
    }
}

macro_rules! impl_mul {
    ($name:ident, $feature:expr, $intr:ident) => {
        impl Mul<$name> for $name {
            type Output = $name;
            fn mul(self, other: $name) -> Self::Output {
                if cfg!(target_feature = $feature) {
                    let x: st::$name = unsafe {
                        std::mem::transmute(
                            vendor::$intr(
                                std::mem::transmute(self.0),
                                std::mem::transmute(other.0)
                            ))
                    };
                    $name::from(x)
                } else {
                    let mut v = $name::zeros();
                    for i in 0..4 {
                        v.set(i, self.0.extract(i as u32) * other.0.extract(i as u32));
                    }
                    v
                }
            }
        }
    }
}

