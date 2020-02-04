/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

#![allow(unused_results)]

use super::super::super::ConditionCode;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_CONDITION_CODE_HASH: HashMap<&'static str, ConditionCode> = {
		// GENERATOR-BEGIN: ConditionCodeHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(17);
		h.insert("None", ConditionCode::None);
		h.insert("o", ConditionCode::o);
		h.insert("no", ConditionCode::no);
		h.insert("b", ConditionCode::b);
		h.insert("ae", ConditionCode::ae);
		h.insert("e", ConditionCode::e);
		h.insert("ne", ConditionCode::ne);
		h.insert("be", ConditionCode::be);
		h.insert("a", ConditionCode::a);
		h.insert("s", ConditionCode::s);
		h.insert("ns", ConditionCode::ns);
		h.insert("p", ConditionCode::p);
		h.insert("np", ConditionCode::np);
		h.insert("l", ConditionCode::l);
		h.insert("ge", ConditionCode::ge);
		h.insert("le", ConditionCode::le);
		h.insert("g", ConditionCode::g);
		// GENERATOR-END: ConditionCodeHash
		h
	};
}