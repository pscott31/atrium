mod add_button {
    use yew::{html::Buildable, prelude::*};
    use yew_icons::{Icon, IconId};
    #[allow(unused_parens)]
    pub struct AddButton<T, P, B, TOKEN, How>
    where
        T: Component<Properties = P>,
        P: Properties<Builder = B>,
        B: Buildable<TOKEN, Output = P>,
        B::WrappedToken: yew::html::HasAllProps<P, How>,
    {
        _marker: ::std::marker::PhantomData<(T, P, B, TOKEN, How)>,
        function_component: ::yew::functional::FunctionComponent<Self>,
    }
    impl<T, P, B, TOKEN, How> ::yew::functional::FunctionProvider
    for AddButton<T, P, B, TOKEN, How>
    where
        T: Component<Properties = P>,
        P: Properties<Builder = B>,
        B: Buildable<TOKEN, Output = P>,
        B::WrappedToken: yew::html::HasAllProps<P, How>,
    {
        type Properties = ();
        fn run(
            ctx: &mut ::yew::functional::HookContext,
            props: &Self::Properties,
        ) -> ::yew::html::HtmlResult {
            fn add_button<T, P, B, TOKEN, How>(
                _ctx: &mut ::yew::functional::HookContext,
                _: &(),
            ) -> Html
            where
                T: Component<Properties = P>,
                P: Properties<Builder = B>,
                B: Buildable<TOKEN, Output = P>,
                B::WrappedToken: yew::html::HasAllProps<P, How>,
            {
                {
                    let modal_visible = ::yew::functional::Hook::run(
                        use_state(|| false),
                        _ctx,
                    );
                    let onclick = {
                        let modal_visible = modal_visible.clone();
                        move |_| modal_visible.set(true)
                    };
                    {
                        #[allow(clippy::useless_conversion)]
                        <::yew::virtual_dom::VNode as ::std::convert::From<
                            _,
                        >>::from(
                            ::yew::virtual_dom::VNode::VList(
                                ::yew::virtual_dom::VList::with_children(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            ::std::convert::Into::into({
                                                #[allow(clippy::redundant_clone, unused_braces)]
                                                let node = ::std::convert::Into::<
                                                    ::yew::virtual_dom::VNode,
                                                >::into(
                                                    ::yew::virtual_dom::VTag::__new_other(
                                                        ::std::borrow::Cow::<
                                                            'static,
                                                            ::std::primitive::str,
                                                        >::Borrowed("button"),
                                                        ::std::default::Default::default(),
                                                        ::std::option::Option::None,
                                                        ::yew::virtual_dom::Attributes::Static(
                                                            &[
                                                                (
                                                                    "class",
                                                                    "button is-link",
                                                                    ::yew::virtual_dom::ApplyAttributeAs::Attribute,
                                                                ),
                                                            ],
                                                        ),
                                                        ::yew::virtual_dom::listeners::Listeners::Pending(
                                                            ::std::boxed::Box::new([
                                                                ::yew::html::onclick::Wrapper::__macro_new(onclick),
                                                            ]),
                                                        ),
                                                        ::yew::virtual_dom::VList::with_children(
                                                            <[_]>::into_vec(
                                                                #[rustc_box]
                                                                ::alloc::boxed::Box::new([
                                                                    ::std::convert::Into::into({
                                                                        #[allow(clippy::redundant_clone, unused_braces)]
                                                                        let node = ::std::convert::Into::<
                                                                            ::yew::virtual_dom::VNode,
                                                                        >::into(
                                                                            ::yew::virtual_dom::VTag::__new_other(
                                                                                ::std::borrow::Cow::<
                                                                                    'static,
                                                                                    ::std::primitive::str,
                                                                                >::Borrowed("span"),
                                                                                ::std::default::Default::default(),
                                                                                ::std::option::Option::None,
                                                                                ::yew::virtual_dom::Attributes::Static(
                                                                                    &[
                                                                                        (
                                                                                            "class",
                                                                                            "icon",
                                                                                            ::yew::virtual_dom::ApplyAttributeAs::Attribute,
                                                                                        ),
                                                                                    ],
                                                                                ),
                                                                                ::yew::virtual_dom::listeners::Listeners::None,
                                                                                ::yew::virtual_dom::VList::with_children(
                                                                                    <[_]>::into_vec(
                                                                                        #[rustc_box]
                                                                                        ::alloc::boxed::Box::new([
                                                                                            ::std::convert::Into::into({
                                                                                                let __yew_props = {
                                                                                                    #[allow(clippy::no_effect)]
                                                                                                    if false {
                                                                                                        let _ = |
                                                                                                            __yew_props: <Icon as ::yew::html::BaseComponent>::Properties|
                                                                                                        {
                                                                                                            let _ = &__yew_props.icon_id;
                                                                                                        };
                                                                                                    }
                                                                                                    let mut __yew_props = <<Icon as ::yew::html::BaseComponent>::Properties as ::yew::html::Properties>::builder();
                                                                                                    let __yew_required_props_token = ::yew::html::AssertAllProps;
                                                                                                    let __yew_required_props_token = __yew_props
                                                                                                        .icon_id(
                                                                                                            __yew_required_props_token,
                                                                                                            IconId::FontAwesomeRegularSquarePlus,
                                                                                                        );
                                                                                                    ::yew::html::Buildable::prepare_build(
                                                                                                            __yew_props,
                                                                                                            &__yew_required_props_token,
                                                                                                        )
                                                                                                        .build()
                                                                                                };
                                                                                                ::yew::virtual_dom::VChild::<
                                                                                                    Icon,
                                                                                                >::new(__yew_props, ::std::option::Option::None)
                                                                                            }),
                                                                                        ]),
                                                                                    ),
                                                                                    ::std::option::Option::None,
                                                                                ),
                                                                            ),
                                                                        );
                                                                        node
                                                                    }),
                                                                    ::std::convert::Into::into({
                                                                        #[allow(clippy::redundant_clone, unused_braces)]
                                                                        let node = ::std::convert::Into::<
                                                                            ::yew::virtual_dom::VNode,
                                                                        >::into(
                                                                            ::yew::virtual_dom::VTag::__new_other(
                                                                                ::std::borrow::Cow::<
                                                                                    'static,
                                                                                    ::std::primitive::str,
                                                                                >::Borrowed("span"),
                                                                                ::std::default::Default::default(),
                                                                                ::std::option::Option::None,
                                                                                ::yew::virtual_dom::Attributes::Static(&[]),
                                                                                ::yew::virtual_dom::listeners::Listeners::None,
                                                                                ::yew::virtual_dom::VList::with_children(
                                                                                    <[_]>::into_vec(
                                                                                        #[rustc_box]
                                                                                        ::alloc::boxed::Box::new([
                                                                                            ::std::convert::Into::into(
                                                                                                ::yew::virtual_dom::VText::new(
                                                                                                    ::yew::virtual_dom::AttrValue::Static("Add User"),
                                                                                                ),
                                                                                            ),
                                                                                        ]),
                                                                                    ),
                                                                                    ::std::option::Option::None,
                                                                                ),
                                                                            ),
                                                                        );
                                                                        node
                                                                    }),
                                                                ]),
                                                            ),
                                                            ::std::option::Option::None,
                                                        ),
                                                    ),
                                                );
                                                node
                                            }),
                                            ::std::convert::Into::into(
                                                if *modal_visible {
                                                    ::yew::virtual_dom::VNode::VList(
                                                        ::yew::virtual_dom::VList::with_children(
                                                            <[_]>::into_vec(
                                                                #[rustc_box]
                                                                ::alloc::boxed::Box::new([
                                                                    ::std::convert::Into::into({
                                                                        let __yew_props = {
                                                                            #[allow(clippy::no_effect)]
                                                                            if false {
                                                                                let _ = |
                                                                                    __yew_props: <T as ::yew::html::BaseComponent>::Properties|
                                                                                {
                                                                                    let _ = &__yew_props.foo;
                                                                                };
                                                                            }
                                                                            let mut __yew_props = <<T as ::yew::html::BaseComponent>::Properties as ::yew::html::Properties>::builder();
                                                                            let __yew_required_props_token = ::yew::html::AssertAllProps;
                                                                            let __yew_required_props_token = __yew_props
                                                                                .foo(__yew_required_props_token, "bar");
                                                                            ::yew::html::Buildable::prepare_build(
                                                                                    __yew_props,
                                                                                    &__yew_required_props_token,
                                                                                )
                                                                                .build()
                                                                        };
                                                                        ::yew::virtual_dom::VChild::<
                                                                            T,
                                                                        >::new(__yew_props, ::std::option::Option::None)
                                                                    }),
                                                                ]),
                                                            ),
                                                            ::std::option::Option::None,
                                                        ),
                                                    )
                                                } else {
                                                    ::yew::virtual_dom::VNode::VList(
                                                        ::yew::virtual_dom::VList::with_children(
                                                            ::alloc::vec::Vec::new(),
                                                            ::std::option::Option::None,
                                                        ),
                                                    )
                                                },
                                            ),
                                        ]),
                                    ),
                                    ::std::option::Option::None,
                                ),
                            ),
                        )
                    }
                }
            }
            ::yew::html::IntoHtmlResult::into_html_result(
                add_button::<T, P, B, TOKEN, How>(ctx, props),
            )
        }
    }
    #[automatically_derived]
    impl<T, P, B, TOKEN, How> ::std::fmt::Debug for AddButton<T, P, B, TOKEN, How>
    where
        T: Component<Properties = P>,
        P: Properties<Builder = B>,
        B: Buildable<TOKEN, Output = P>,
        B::WrappedToken: yew::html::HasAllProps<P, How>,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(&["AddButton<_>"], &[]))
        }
    }
    #[automatically_derived]
    impl<T, P, B, TOKEN, How> ::yew::html::BaseComponent
    for AddButton<T, P, B, TOKEN, How>
    where
        T: Component<Properties = P>,
        P: Properties<Builder = B>,
        B: Buildable<TOKEN, Output = P>,
        B::WrappedToken: yew::html::HasAllProps<P, How>,
        T: 'static,
        P: 'static,
        B: 'static,
        TOKEN: 'static,
        How: 'static,
        Self: 'static,
    {
        type Message = ();
        type Properties = ();
        #[inline]
        fn create(ctx: &::yew::html::Context<Self>) -> Self {
            Self {
                _marker: ::std::marker::PhantomData,
                function_component: ::yew::functional::FunctionComponent::<
                    Self,
                >::new(ctx),
            }
        }
        #[inline]
        fn update(
            &mut self,
            _ctx: &::yew::html::Context<Self>,
            _msg: Self::Message,
        ) -> ::std::primitive::bool {
            true
        }
        #[inline]
        fn changed(
            &mut self,
            _ctx: &::yew::html::Context<Self>,
            _old_props: &Self::Properties,
        ) -> ::std::primitive::bool {
            true
        }
        #[inline]
        fn view(&self, ctx: &::yew::html::Context<Self>) -> ::yew::html::HtmlResult {
            ::yew::functional::FunctionComponent::<
                Self,
            >::render(&self.function_component, ::yew::html::Context::<Self>::props(ctx))
        }
        #[inline]
        fn rendered(
            &mut self,
            _ctx: &::yew::html::Context<Self>,
            _first_render: ::std::primitive::bool,
        ) {
            ::yew::functional::FunctionComponent::<
                Self,
            >::rendered(&self.function_component)
        }
        #[inline]
        fn destroy(&mut self, _ctx: &::yew::html::Context<Self>) {
            ::yew::functional::FunctionComponent::<
                Self,
            >::destroy(&self.function_component)
        }
        #[inline]
        fn prepare_state(&self) -> ::std::option::Option<::std::string::String> {
            ::yew::functional::FunctionComponent::<
                Self,
            >::prepare_state(&self.function_component)
        }
    }
}
