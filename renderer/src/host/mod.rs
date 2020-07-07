pub trait Host {
    type Instance;

    fn root_instance(&self) -> Self::Instance;

    fn create_instance(&self, name: &str) -> Self::Instance;

    fn create_text_instance(&self, content: &str) -> Self::Instance;

    fn append_child(parent: &Self::Instance, child: Self::Instance);
}
