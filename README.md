
# Rust Version Test

This projects shows that, in rust, the "same" object imported from two different versions of a same package is actually different.

- `mystr`, both in version 1.0.0 and 1.1.0 simply exports an empty object.
- `a` is a package that imports `base` 1.0.0 and simply exports it.
- `b` is a package that imports `base` 1.1.0 and simply exports it.
- `main` is the executable. It imports `a` and `b` two times each, then prints whether the two values imported from `a` (i.e. `base` 1.0.0) are the same, whether the two values imported from `b` (i.e. `base` 1.1.0) are the same and whether the object imported from `a` is the same as the one imported from `b`.

This is to show that:

- The object imported from `a` is always the same: it's `base` 1.0.0.
- The object imported from `b` is always the same: it's `base` 1.1.0.
- The object imported from `a` is different from the one imported from `b`, in spite of coming from a package with the same name.

To run this test, enter the `main` directory and run `node index.js`.
