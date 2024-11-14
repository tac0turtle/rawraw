pub trait TableRow {}

pub struct Table<Row> {
    _phantom: core::marker::PhantomData<Row>,
}
