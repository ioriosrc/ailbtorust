```rust
use std::rc::Rc;

type TypedDataSet = charty_core::datasets::ScatterDataset;
type NormalDataSet = charty_core::datasets::ScatterDataset;

fn proxy_dataset(dataset: &TypedDataSet) -> Rc<NormalDataSet> {
    let mut data = dataset.data.clone();

    let length = data
        .iter()
        .map(|v| v.x.len())
        .sum::<usize>();

    Rc::new(NormalDataSet {
        ..dataset.clone(),
        data: data
            .into_iter()
            .map(|slice| slice.map(|point| point[x] as f64))
            .collect(),
        length,
    })
}

pub fn proxy_typed(data: &charty_core::datasets::ScatterDataset) -> charty_core::datasets::ScatterDataset {
    let mut datasets = data.datasets.clone();

    datasets.into_iter().map(proxy_dataset).collect()
}
```