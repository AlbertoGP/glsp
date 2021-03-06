#![cfg_attr(not(feature = "unsafe-internals"), forbid(unsafe_code))]

#![doc(html_root_url = "https://docs.rs/glsp/0.1")]

#![feature(optin_builtin_traits)]
#![feature(negative_impls)]
#![feature(drain_filter)]

#[macro_use]
mod error;

#[macro_use]
mod val;

#[macro_use]
mod wrap;

#[macro_use]
mod collections;

#[macro_use]
mod engine;

mod ast;
mod code;
mod compile;
mod class;
mod encoder;
mod eval;
mod gc;
mod iter;
mod lex;
mod parse;
mod print;
mod serde;
mod transform;
mod vm;

pub use self::{
	code::{Coro, CoroState, GFn},
	collections::{
		Arr, Deque, DequeAccess, DequeAccessRange, DequeIndex, DequeOps, DequeRange, IntoElement, 
		FromElement, IterDeque, IterDequeTo, IterTab, IterTabKeys, IterTabKeysTo, IterTabTo, 
		IterTabValues, IterTabValuesTo, Splay, Str, Tab, TabEntries
	},
	class::{Class, Obj},
	engine::{
		GSend, GStore, PrWriter, EprWriter, Lib, LibRef, LibRefMut, RClass, RData, RFn,
		RRef, RRefMut, RRoot, RStore, Sym, ToSym, with_lazy_val
	},
	error::{GError, GResult},
	eval::{EnvMode, Expander, Expansion},
	gc::{Allocate, GC_DEFAULT_RATIO, GC_MIN_RATIO, Root},
	iter::{GIter, GIterLen, Iterable, IterableOps},
	val::{Hashable, Num, Val},
	wrap::{
		ArgType, Callable, CallableOps, forwarder, FromVal, IntoResult, MakeArg, MakeTemp,
		make_temps, OrNil, ToCallArgs, ToVal, WrappedFn, wrapped_arg_limits
	}
};

pub use self::engine::glsp::*;

//undocumented apis required by the glsp-stdlib crate or by macros
#[doc(hidden)]
pub use self::{
	engine::{Engine, EngineBuilder, Span, stock_syms, SymKind},
	gc::{Slot},
	parse::{Parser},
	print::{dump_fn, dump_form, dump_macro}
};
