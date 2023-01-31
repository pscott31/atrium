mod add_button {
    use yew::{html::Buildable, prelude::*};
    use yew_icons::{Icon, IconId};
    pub struct MyProps {
        proppy: String,
    }
    #[doc(hidden)]
    struct MyPropsWrapper {
        proppy_wrapper: ::std::option::Option<String>,
    }
    #[automatically_derived]
    impl ::std::default::Default for MyPropsWrapper {
        fn default() -> Self {
            MyPropsWrapper {
                proppy_wrapper: ::std::option::Option::None,
            }
        }
    }
    #[doc(hidden)]
    pub struct MyPropsBuilder {
        wrapped: ::std::boxed::Box<MyPropsWrapper>,
    }
    #[automatically_derived]
    impl MyPropsBuilder {
        #[doc(hidden)]
        pub fn proppy<__YewTokenTy>(
            &mut self,
            token: __YewTokenTy,
            value: impl ::yew::html::IntoPropValue<String>,
        ) -> HasMyPropsproppy<__YewTokenTy> {
            self
                .wrapped
                .proppy_wrapper = ::std::option::Option::Some(value.into_prop_value());
            HasMyPropsproppy(::std::marker::PhantomData)
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod _MyProps {
        #[allow(non_camel_case_types)]
        pub struct proppy;
    }
    #[doc(hidden)]
    pub struct HasMyPropsproppy<How>(::std::marker::PhantomData<How>);
    #[automatically_derived]
    impl<B> ::yew::html::HasProp<_MyProps::proppy, HasMyPropsproppy<B>>
    for HasMyPropsproppy<B> {}
    #[automatically_derived]
    impl<B, P, How> ::yew::html::HasProp<P, &dyn ::yew::html::HasProp<P, How>>
    for HasMyPropsproppy<B>
    where
        B: ::yew::html::HasProp<P, How>,
    {}
    #[doc(hidden)]
    pub struct CheckMyPropsAll<How>(::std::marker::PhantomData<How>);
    #[automatically_derived]
    impl<B, P, How> ::yew::html::HasProp<P, &dyn ::yew::html::HasProp<P, How>>
    for CheckMyPropsAll<B>
    where
        B: ::yew::html::HasProp<P, How>,
    {}
    #[automatically_derived]
    impl<
        __YewToken,
        HowMyPropsproppy,
    > ::yew::html::HasAllProps<MyProps, (HowMyPropsproppy,)>
    for CheckMyPropsAll<__YewToken>
    where
        __YewToken: ::yew::html::HasProp<_MyProps::proppy, HowMyPropsproppy>,
    {}
    #[automatically_derived]
    impl<__YewToken> ::yew::html::Buildable<__YewToken> for MyPropsBuilder {
        type Output = MyProps;
        type WrappedToken = CheckMyPropsAll<__YewToken>;
        fn build(this: Self) -> Self::Output {
            MyProps {
                proppy: ::std::option::Option::unwrap(this.wrapped.proppy_wrapper),
            }
        }
    }
    impl ::yew::html::Properties for MyProps {
        type Builder = MyPropsBuilder;
        fn builder() -> Self::Builder {
            MyPropsBuilder {
                wrapped: ::std::boxed::Box::new(::std::default::Default::default()),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyProps {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyProps {
        #[inline]
        fn eq(&self, other: &MyProps) -> bool {
            self.proppy == other.proppy
        }
    }
}
