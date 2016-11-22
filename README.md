#func-swap
swap functions!

This crate is a macro that allows for you to define a function which can be
hotswapped at runtime with another function (of the same symbol). The swap
is memory safe even in concurrent systems. The over head is just a pointer
load, and an != 0 jump at the start of your function (on AMD64/ARMv8).
On older ARM there is a half memory barrier (load), this barrier is very
cheap.

Replacing a function does trigger a full `Ordering::SeqCst` memory barrier
to ensure that all future calls will see the new function.

To use this crate include:

```
[dependencies]
func_swap = "0.1.0"
lazy_static = "0.2.*"
```

Example Usage:

```rust
#[macro_use]
extern crate func_swap;
#[macro_use]
extern crate lazy_static;

//define the add function so it is replaceable
modfunc!(replace,FP, add(x: u64, y: u64) => u64{
	x + y
});
assert!( add(5u64,2u64) == 7u64);

//define a completely different function
fn not_addition(x: u64, y: u64) -> u64 {
	(x*x)+y
}

//swap the values
replace(not_addition);
assert!( add(5u64,2u64) == 27u64);
```

To demystify the macro:

```
modfunc!(
	SWAP_FUNCTION: ident,		//define a function to swap functions
	FUNCTION_POINTER: ident,	//define a constant, this name will really shouldn't be used
	SWAPPABLE: ident			//define the main function
	( var:type, var2:type)		//args
	type OR (type,type,...)		//return values
	{
		//body of your function
	});
```

##Question and Answer:

###Can I use generics?

No-ish. Function signatures need to be concrete and known at compile time.
You likely can get away with TraitObjects. But I haven't tested this. YYMV

###Can I capture values?

No. This is not a lambda expression this is defining a static function.

###Can I swap with functions that I load via dynamic linking?

Yes. But use this with care. When you unload the library some flags will
invalidate the existing function pointers. Which may cause the function
to call into un-allocated virtual memory... leading to a SEGFAULT. 

If you unload a function ensure you set the value to NULL. `unsafe{mem::transmute(0usize)`. This will restore default behavior.

###What ABI is used by default?

The `extern "Rust"` ABI is used. It should be noted this is non-stable. So
if you are dynamically linking use SO's/DLL's generated with the same
revision of the Rust-Compiler.

###Can I use non-Rust ABI's?

Yes. If you start the macro with `ABI:` The first argument is an `expr` of
the ABI. This looks like:

```rust
modfunc!(ABI: "C", swap_function, functionptrname, function(args: yada)
```

This feature is not tested heavily, YMMV. It also should be noted the 
default function will remain a native "Rust" function. But the function that _can_ be swapped will be the the alien ABI.
