trait Baz {}

trait Bar<T>
where
    T: Baz,
{
}
