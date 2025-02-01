pub trait TypedValue<T>: Send + Sized + Sync {
    fn value(&self) -> T;
}

pub trait DimensionUnit<T>: TypedValue<T> {
    type DefaultUnit: DimensionUnit<T>;

    fn to_default(&self) -> Self::DefaultUnit;
    fn from_default(val: Self::DefaultUnit) -> Self;
}
