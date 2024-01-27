use telers::extractors::FromEventAndContext;
use telers_macros::FromContext;

#[derive(Clone)]
enum NoGeneric {
    _Variant1,
}

#[derive(FromContext)]
#[context(key = "no_generic", from = NoGeneric)]
enum NoGenericWrapper {
    _Variant1(NoGeneric),
}

impl From<NoGeneric> for NoGenericWrapper {
    fn from(data: NoGeneric) -> Self {
        Self::_Variant1(data)
    }
}

#[derive(Clone)]
enum SingleGeneric<T> {
    _Variant1(T),
}

#[derive(FromContext)]
#[context(key = "single_generic", from = SingleGeneric)]
enum SingleGenericWrapper<T> {
    _Variant1(SingleGeneric<T>),
}

impl<T> From<SingleGeneric<T>> for SingleGenericWrapper<T> {
    fn from(data: SingleGeneric<T>) -> Self {
        Self::_Variant1(data)
    }
}

#[derive(Clone)]
enum MultiGeneric<T, E> {
    _Variant1(T),
    _Variant2(E),
}

#[derive(FromContext)]
#[context(key = "multi_generic", from = MultiGeneric)]
enum MultiGenericWrapper<T, E> {
    _Variant1(MultiGeneric<T, E>),
    _Variant2(MultiGeneric<T, E>),
}

impl<T, E> From<MultiGeneric<T, E>> for MultiGenericWrapper<T, E> {
    fn from(data: MultiGeneric<T, E>) -> Self {
        match data {
            MultiGeneric::_Variant1(_) => Self::_Variant1(data),
            MultiGeneric::_Variant2(_) => Self::_Variant2(data),
        }
    }
}

#[derive(Clone)]
enum NoGenericWithLifetime<'a> {
    _Variant1(&'a str),
}

#[derive(FromContext)]
#[context(key = "no_generic_with_lifetime", from = NoGenericWithLifetime)]
enum NoGenericWithLifetimeWrapper<'a> {
    _Variant1(NoGenericWithLifetime<'a>),
}

impl<'a> From<NoGenericWithLifetime<'a>> for NoGenericWithLifetimeWrapper<'a> {
    fn from(data: NoGenericWithLifetime<'a>) -> Self {
        Self::_Variant1(data)
    }
}

#[derive(Clone)]
enum SingleGenericWithLifetime<'a, T> {
    _Variant1(&'a T),
}

#[derive(FromContext)]
#[context(key = "single_generic_with_lifetime", from = SingleGenericWithLifetime)]
enum SingleGenericWithLifetimeWrapper<'a, T> {
    _Variant1(SingleGenericWithLifetime<'a, T>),
}

impl<'a, T> From<SingleGenericWithLifetime<'a, T>> for SingleGenericWithLifetimeWrapper<'a, T> {
    fn from(data: SingleGenericWithLifetime<'a, T>) -> Self {
        Self::_Variant1(data)
    }
}

#[derive(Clone)]
enum MultiGenericWithLifetime<'a, T, E> {
    _Variant1(&'a T),
    _Variant2(&'a E),
}

#[derive(FromContext)]
#[context(key = "multi_generic_with_lifetime", from = MultiGenericWithLifetime)]
enum MultiGenericWithLifetimeWrapper<'a, T, E> {
    _Variant1(MultiGenericWithLifetime<'a, T, E>),
    _Variant2(MultiGenericWithLifetime<'a, T, E>),
}

impl<'a, T, E> From<MultiGenericWithLifetime<'a, T, E>>
    for MultiGenericWithLifetimeWrapper<'a, T, E>
{
    fn from(data: MultiGenericWithLifetime<'a, T, E>) -> Self {
        match data {
            MultiGenericWithLifetime::_Variant1(_) => Self::_Variant1(data),
            MultiGenericWithLifetime::_Variant2(_) => Self::_Variant2(data),
        }
    }
}

#[derive(Clone)]
enum SingleGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>> {
    _Variant1(&'a T),
}

#[derive(FromContext)]
#[context(key = "single_generic_with_lifetime_and_trait_bound", from = SingleGenericWithLifetimeAndTraitBound)]
enum SingleGenericWithLifetimeAndTraitBoundWrapper<'a, T: AsRef<str>> {
    _Variant1(SingleGenericWithLifetimeAndTraitBound<'a, T>),
}

impl<'a, T: AsRef<str>> From<SingleGenericWithLifetimeAndTraitBound<'a, T>>
    for SingleGenericWithLifetimeAndTraitBoundWrapper<'a, T>
{
    fn from(data: SingleGenericWithLifetimeAndTraitBound<'a, T>) -> Self {
        Self::_Variant1(data)
    }
}

#[derive(Clone)]
enum MultiGenericWithLifetimeAndTraitBound<'a, T: AsRef<str>, E: AsRef<str>> {
    _Variant1(&'a T),
    _Variant2(&'a E),
}

#[derive(FromContext)]
#[context(key = "multi_generic_with_lifetime_and_trait_bound", from = MultiGenericWithLifetimeAndTraitBound)]
enum MultiGenericWithLifetimeAndTraitBoundWrapper<'a, T: AsRef<str>, E: AsRef<str>> {
    _Variant1(MultiGenericWithLifetimeAndTraitBound<'a, T, E>),
    _Variant2(MultiGenericWithLifetimeAndTraitBound<'a, T, E>),
}

impl<'a, T: AsRef<str>, E: AsRef<str>> From<MultiGenericWithLifetimeAndTraitBound<'a, T, E>>
    for MultiGenericWithLifetimeAndTraitBoundWrapper<'a, T, E>
{
    fn from(data: MultiGenericWithLifetimeAndTraitBound<'a, T, E>) -> Self {
        match data {
            MultiGenericWithLifetimeAndTraitBound::_Variant1(_) => Self::_Variant1(data),
            MultiGenericWithLifetimeAndTraitBound::_Variant2(_) => Self::_Variant2(data),
        }
    }
}

#[derive(Clone)]
enum SingleGenericWithLifetimeAndMultiTraitBound<'a, T: AsRef<str> + Clone> {
    _Variant1(&'a T),
}

#[derive(FromContext)]
#[context(key = "single_generic_with_lifetime_and_multi_trait_bound", from = SingleGenericWithLifetimeAndMultiTraitBound)]
enum SingleGenericWithLifetimeAndMultiTraitBoundWrapper<'a, T: AsRef<str> + Clone> {
    _Variant1(SingleGenericWithLifetimeAndMultiTraitBound<'a, T>),
}

impl<'a, T: AsRef<str> + Clone> From<SingleGenericWithLifetimeAndMultiTraitBound<'a, T>>
    for SingleGenericWithLifetimeAndMultiTraitBoundWrapper<'a, T>
{
    fn from(data: SingleGenericWithLifetimeAndMultiTraitBound<'a, T>) -> Self {
        Self::_Variant1(data)
    }
}

#[derive(Clone)]
enum MultiGenericWithLifetimeAndMultiTraitBound<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
> {
    _Variant1(&'a T),
    _Variant2(&'b E),
}

#[derive(FromContext)]
#[context(key = "multi_generic_with_lifetime_and_multi_trait_bound", from = MultiGenericWithLifetimeAndMultiTraitBound)]
enum MultiGenericWithLifetimeAndMultiTraitBoundWrapper<
    'a,
    'b,
    T: AsRef<str> + Clone,
    E: AsRef<str> + Clone,
> {
    _Variant1(MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>),
    _Variant2(MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>),
}

impl<'a, 'b, T: AsRef<str> + Clone, E: AsRef<str> + Clone>
    From<MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>>
    for MultiGenericWithLifetimeAndMultiTraitBoundWrapper<'a, 'b, T, E>
{
    fn from(data: MultiGenericWithLifetimeAndMultiTraitBound<'a, 'b, T, E>) -> Self {
        match data {
            MultiGenericWithLifetimeAndMultiTraitBound::_Variant1(_) => Self::_Variant1(data),
            MultiGenericWithLifetimeAndMultiTraitBound::_Variant2(_) => Self::_Variant2(data),
        }
    }
}

#[allow(unreachable_code)]
fn _check_bounds<Client, T: FromEventAndContext<Client>>() {
    unimplemented!("This function is only used for checking bounds");

    _check_bounds::<(), NoGenericWrapper>();
    _check_bounds::<(), SingleGenericWrapper<i32>>();
    _check_bounds::<(), MultiGenericWrapper<i32, i32>>();
    _check_bounds::<(), NoGenericWithLifetimeWrapper<'_>>();
    _check_bounds::<(), SingleGenericWithLifetimeWrapper<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeWrapper<'_, String, String>>();
    _check_bounds::<(), SingleGenericWithLifetimeAndTraitBoundWrapper<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeAndTraitBoundWrapper<'_, String, String>>();
    _check_bounds::<(), SingleGenericWithLifetimeAndMultiTraitBoundWrapper<'_, String>>();
    _check_bounds::<(), MultiGenericWithLifetimeAndMultiTraitBoundWrapper<'_, '_, String, String>>(
    );
}
