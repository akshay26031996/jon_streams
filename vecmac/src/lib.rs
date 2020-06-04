#[macro_export]
macro_rules! avec {
    // input to a macro needs to be valid syntactically
    // howevert it is not compiled and hence it need not
    // to be correct rust program. Output gets compiled
    // hence, output needs to a valid rust program

    // one or more comma seperated things
    ($($element: expr),*) => {
        {
            // check that count is constant
            const C: usize = $crate::count![@COUNT; $($element),*];
            #[allow(unused_mut)]
            let mut vs = Vec::with_capacity(C);
            $(vs.push($element);)*
            vs
        }
    };
    ($($element: expr,)*) => {
        $crate::avec![$($element),*]
    };
    ($element: expr; $count: expr) => {
        {
            // let count = $count;
            // let mut vs = Vec::with_capacity(count);
            // let x = $element;
            // for _ in 0..count {
            //     vs.push(x.clone());
            // }
            // :: represnts the root path
            // vs.extend(::std::iter::repeat($element).take(count));
            let mut vs = Vec::new();
            vs.resize($count, $element);
            vs
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element: expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element: expr) => {
        ()
    };
}


#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let _x: Vec<u32> = avec![1, 2, 3, 4, 5, 6, 7, 8, 9,];
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![42; 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42, "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
