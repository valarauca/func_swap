//Copyright 2016 William Cody Laeder
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file execpt in compliance with the License.
//
//  http://www.apache.org/license/LICENSE-2.0
//
//Uneless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or 
//implied. See the License for the specific language governing permissions
//and limitations under the License.


#[macro_use]
extern crate lazy_static;

#[macro_use]
macro_rules! modfunc {
    ($STORE:ident,$FP:ident,$NAME:ident ($($name:ident:$args:ty),* ) => $ret:ty$body:block) => {
        lazy_static! {
            static ref $FP: ::std::sync::atomic::AtomicPtr<extern "Rust" fn($($args),*)->$ret> =
                ::std::sync::atomic::AtomicPtr::new(unsafe{::std::mem::transmute(0usize)});
        }
        pub fn $STORE(ptr: extern "Rust" fn($($args),*)->$ret) {
            use std::sync::atomic::Ordering;
            use std::mem::transmute;
            $FP.store(unsafe{transmute(ptr)},Ordering::SeqCst);
        }
        pub fn $NAME($($name:$args),*) -> $ret {
            {
                use std::sync::atomic::Ordering;
                use std::mem::transmute;
                let ptr = $FP.load(Ordering::Acquire);
                let ptr: usize = unsafe{transmute(ptr)};
                if ptr != 0 {
                    let func: extern "Rust" fn($($args),*)->$ret =
                        unsafe{transmute(ptr)};
                    return func($($name),*);
                } 
            }
            $body
        }
    };
    ($STORE:ident,$FP:ident,$NAME:ident($($name:ident:$arg:ty),*) => ($($ret:ty),*) $body:block) => {
        lazy_static! {
            static ref $FP: ::std::sync::atomic::AtomicPtr<extern "Rust" fn($($args),*)->($($ret),*)> =
                ::std::sync::atomic::AtomicPtr::new(unsafe{::std::mem::transmute(0usize)});
        }
        pub fn $STORE(ptr: extern "Rust" fn($($args),*)->($($ret),*)) {
            use std::sync::atomic::Ordering;
            use std::mem::transmute;
            $FP.store(unsafe{transmute(ptr)},Ordering::SeqCst);
        }
        pub fn $NAME($($name:$args),*) -> ($($ret),*) {
            {
                use std::sync::atomic::Ordering;
                use std::mem::transmute;
                let ptr = $FP.load(Ordering::Acquire);
                let ptr: usize = unsafe{transmute(ptr)};
                if ptr != 0 {
                    let func: extern "Rust" fn($($args),*)->$ret =
                        unsafe{transmute(ptr)};
                    return func($($name),*);
                } 
            }
            $body
        }
    };
    (ABI:$ABI:expr,$STORE:ident,$FP:ident,$NAME:ident ($($name:ident:$args:ty),* ) => $ret:ty$body:block) => {
        lazy_static! {
            static ref $FP: ::std::sync::atomic::AtomicPtr<extern $ABI fn($($args),*)->$ret> =
                ::std::sync::atomic::AtomicPtr::new(unsafe{::std::mem::transmute(0usize)});
        }
        pub fn $STORE(ptr: extern $ABI fn($($args),*)->$ret) {
            use std::sync::atomic::Ordering;
            use std::mem::transmute;
            $FP.store(unsafe{transmute(ptr)},Ordering::SeqCst);
        }
        pub fn $NAME($($name:$args),*) -> $ret {
            {
                use std::sync::atomic::Ordering;
                use std::mem::transmute;
                let ptr = $FP.load(Ordering::Acquire);
                let ptr: usize = unsafe{transmute(ptr)};
                if ptr != 0 {
                    let func: extern $ABI fn($($args),*)->$ret =
                        unsafe{transmute(ptr)};
                    return func($($name),*);
                } 
            }
            $body
        }
    };
    (ABI:$ABI:expr,$STORE:ident,$FP:ident,$NAME:ident($($name:ident:$arg:ty),*) => ($($ret:ty),*) $body:block) => {
        lazy_static! {
            static ref $FP: ::std::sync::atomic::AtomicPtr<extern $ABI fn($($args),*)->($($ret),*)> =
                ::std::sync::atomic::AtomicPtr::new(unsafe{::std::mem::transmute(0usize)});
        }
        pub fn $STORE(ptr: extern $ABI fn($($args),*)->($($ret),*)) {
            use std::sync::atomic::Ordering;
            use std::mem::transmute;
            $FP.store(unsafe{transmute(ptr)},Ordering::SeqCst);
        }
        pub fn $NAME($($name:$args),*) -> ($($ret),*) {
            {
                use std::sync::atomic::Ordering;
                use std::mem::transmute;
                let ptr = $FP.load(Ordering::Acquire);
                let ptr: usize = unsafe{transmute(ptr)};
                if ptr != 0 {
                    let func: extern $ABI fn($($args),*)->$ret =
                        unsafe{transmute(ptr)};
                    return func($($name),*);
                } 
            }
            $body
        }
    };
}



#[test]
fn test_the_other_thing() {

    //Define the function "add"
    //Takes are x and y
    modfunc!(replace_add,IDC, add(x: u64, y: u64) => u64 {
        x + y
    });
    //verify add works correctly
    assert_eq!(add(5,2),7);

    //define a function that is not add
    fn totally_not_add(x: u64, y: u64) -> u64 {
        return x*x+y;
    }
    replace_add(totally_not_add);

    //add no longer works as expected
    assert_eq!(add(5,2),27);
}
