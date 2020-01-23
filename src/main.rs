use builder::Builder;

#[derive(Debug)]
struct X {}

#[derive(Debug, Builder)]
struct Item<T, U>
where
    T: Default,
{
    a: u32,
    b: Option<&'static str>,
    c: String,
    #[builder(required)]
    d: X,
    e: T,
    #[builder(required)]
    f: U,
}
