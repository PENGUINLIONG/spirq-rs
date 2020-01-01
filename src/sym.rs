//! Sym used to identify shader module resources.
use std::borrow::Borrow;
use std::ops::Deref;
use std::fmt;
use std::str::FromStr;

pub struct Sym(str);
impl Sym {
    pub fn new<S: AsRef<str> + ?Sized>(literal: &S) -> &Sym {
        unsafe { &*(literal.as_ref() as *const str as *const Sym) }
    }
    pub fn segs<'a>(&'a self) -> Segs<'a> { Segs(self.0.as_ref(), false) }
}
impl ToString for Sym {
    fn to_string(&self) -> String { self.0.to_string() }
}
impl ToOwned for Sym {
    type Owned = Symbol;
    fn to_owned(&self) -> Symbol { Symbol::from(self.0.to_owned()) }
}
impl AsRef<str> for Sym {
    fn as_ref(&self) -> &str { &self.0 }
}
impl AsRef<Sym> for str {
    fn as_ref(&self) -> &Sym { Sym::new(self) }
}
impl AsRef<Sym> for Sym {
    fn as_ref(&self) -> &Sym { self }
}

pub struct Symbol(Box<Sym>);
impl Symbol {
    pub fn segs<'a>(&'a self) -> Segs<'a> { Segs(&(*self.0).0, false) }
    pub fn push<'a>(&mut self, seg: &Seg<'a>) {
        let inner = format!("{}.{}", &(self.0).0, seg).into_boxed_str();
        self.0 = Symbol::from(inner.as_ref()).0;
    }
    pub fn pop(&mut self) -> Option<Symbol> {
        fn split_sym(literal: &str) -> Option<(Symbol, Symbol)> {
            if literal.len() == 0 { return None; }
            let end = literal.bytes().rposition(|c| c == '.' as u8).unwrap_or(0);
            let rv = literal[end + 1..].to_owned().into();
            let new_inner = literal[..end].to_owned().into();
            Some((rv, new_inner))
        }
        let (rv, new_inner) = split_sym(&(self.0).0)?;
        self.0 = new_inner.0;
        Some(rv)
    }
}
impl From<Box<Sym>> for Symbol {
    fn from(boxed: Box<Sym>) -> Symbol {
        Symbol(boxed)
    }
}
impl From<String> for Symbol {
    fn from(literal: String) -> Symbol {
        unsafe {
            let ptr = std::boxed::Box::leak(literal.into_boxed_str()) as *mut str;
            let ptr = ptr as *mut Sym;
            let inner = Box::from_raw(ptr);
            inner.into()
        }
    }
}
impl From<&'_ str> for Symbol {
    fn from(literal: &'_ str) -> Symbol {
        literal.to_owned().into()
    }
}
impl ToString for Symbol {
    fn to_string(&self) -> String { self.0.to_string() }
}
impl Borrow<Sym> for Symbol {
    fn borrow(&self) -> &Sym { self }
}
impl Deref for Symbol {
    type Target = Sym;
    fn deref(&self) -> &Sym { &self.0 }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Seg<'a> {
    Index(usize),
    Name(&'a str),
    Empty,
}
impl<'a> Seg<'a> {
    pub fn index(idx: usize) -> Self { Seg::Index(idx) }
    pub fn name(name: &'a str) -> Self { Seg::Name(name) }
    pub fn empty() -> Self { Seg::Empty }

    pub fn is_index(&self) -> bool {
        if let Seg::Index(_) = self { true } else { false }
    }
    pub fn is_name(&self) -> bool {
        if let Seg::Name(_) = self { true } else { false }
    }
    pub fn is_empty(&self) -> bool {
        if let Seg::Empty = self { true } else { false }
    }
}
impl<'a> fmt::Display for Seg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Seg::*;
        match self {
            Index(idx) => f.write_fmt(format_args!("{}", idx)),
            Name(name) => f.write_str(name),
            Empty => Ok(()),
        }
    }
}

pub struct Segs<'a>(&'a str, bool); // True means that we have reached the end.
impl<'a> Segs<'a> {
    pub fn remaining(&self) -> &Sym { Sym::new(self.0) }
}
impl<'a> Iterator for Segs<'a> {
    type Item = Seg<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return if self.1 { None } else { Some(Seg::Empty) };
        }
        let txt = if let Some(pos) = self.0.bytes().position(|c| c == '.' as u8) {
            let txt = &self.0[..pos];
            self.0 = &self.0[(pos + 1)..];
            txt
        } else {
            let txt = &self.0[..];
            self.0 = &"";
            self.1 = true;
            txt
        };
        if txt.is_empty() { return Some(Seg::Empty); }
        let seg = if let Ok(idx) = usize::from_str(txt) {
            Seg::Index(idx)
        } else {
            Seg::Name(txt)
        };
        return Some(seg);
    }
}
