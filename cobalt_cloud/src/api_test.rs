use dioxus::prelude::*;

#[component]
pub fn Test() -> Element {
    let ondrop = move |evt: DragEvent| {
        spawn(async move {
            if let Some(engine) = evt.data().file_engine() {
                 for name in engine.files() {
                     let _bytes = engine.read_file(&name).await;
                 }
            }
        });
    };
    rsx! { div { ondrop } }
}
