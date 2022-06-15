/// Creates a static lifetime version of its implementor.
/// Can be used to access type IDs of non-static types.
pub unsafe trait StaticKeyed {
    type StaticKey: 'static;
}

/* Example implementation:

struct GenericType<'a> {
    a: &'a str
}

unsafe impl<'a> StaticKeyed for GenericType<'a> {
    type StaticKey = GenericType<'static>;
}

*/
