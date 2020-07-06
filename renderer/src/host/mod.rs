enum Union<I, TI> {
    Instance(I),
    TextInstance(TI),
}

pub trait Host {
    type Instance;
    type TextInstance;

    fn create_instance() -> Self::Instance;

    fn create_text_instance() -> Self::TextInstance;

    fn create_root_instance() -> Self::Instance;

    fn append_child(parent: &Self::Instance, child: Union<&Self::Instance, &Self::TextInstance>);
}
