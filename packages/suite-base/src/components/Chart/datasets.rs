```rust
use std::vec::Vec;

type Point = (usize, f32, f32, Option<String>);

/**
 * iterate_objects iterates over the dataset, yielding a `Point` for each entry.
 */
pub fn iterate_objects(dataset: Vec<TypedData>) -> impl Iterator<Item = Point> {
    let mut index = 0;
    for datum in dataset {
        if datum.is_none() {
            index += 1;
            continue;
        }

        let datum = datum.unwrap();
        let { x, y, label } = datum;

        yield (index, x, y, label);
        index += 1;
    }
}

/**
 * ExtractPoint maps an object type with array properties to one with the
 * arrays replaced by their element type. For example:
 * type Foo = {
 *   foo: Float32Array;
 *   bar: number[];
 *   baz: string[];
 * }
 * would be mapped to:
 * ExtractPoint<Foo> == {
 *   foo: number;
 *   bar: number;
 *   baz: string;
 * }
 * It is used to go from `TypedData`'s various incarnations to what a single
 * point would look like as a `Datum`.
 */
type ExtractPoint<T> = {
    [P in keyof T]: P extends "foo" | "bar" | "baz" { number } : NonNullable<T[P]>[0];
} & {
    index: usize;
    label: Option<String>;
};

/**
 *   Iterate over a typed dataset one point at a time. This abstraction is
 *   necessary because the Plot panel extends TypedData with more fields; we
 *   still want those to be available while iterating.
 */
pub fn iterate_typed<T>(dataset: Vec<T>) -> impl Iterator<Item = ExtractPoint<T>> {
    let point: ExtractPoint<T> = {
        index: 0,
        label: None,
    };

    let mut index = 0;
    for slice in dataset {
        // Find a property for which we can check the length
        let first = slice.iter().next();
        if first.is_none() {
            continue;
        }

        for (j, key) in slice.iter().enumerate() {
            point[key] = match key.to_string().as_str() {
                "foo" => slice[j].into_inner()[0],
                "bar" => slice[j].into_inner()[0],
                "baz" => slice[j].into_inner()[0],
                _ => unreachable!(),
            };

            point.index = index;
            index += 1;
            yield point;
        }
    }
}

pub fn find_indices(dataset: Vec<TypedData>, index: usize) -> Option<(usize, usize)> {
    let mut offset = index;
    for (i, slice) in dataset.iter().enumerate() {
        if slice.is_none() {
            continue;
        }

        let {
            x: { length: num_elements },
        } = slice;

        if offset == num_elements && i == dataset.len() - 1 {
            return Some((i, offset));
        }

        if offset >= num_elements {
            offset -= num_elements;
            continue;
        }

        return Some((i, offset));
    }

    None
}
```