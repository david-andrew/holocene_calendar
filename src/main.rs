use yew::prelude::*;
// use yew_hooks::{use_list, use_drag_with_options};
use gloo_file::File;
use web_sys::{Event, HtmlInputElement};
use log::info;
use std::rc::Rc;


#[function_component]
fn App() -> Html {
    
    // hello world counter
    let counter_handle = use_state(|| 0);
    let counter = *counter_handle;
    let onclick = move |delta: i32| move |_| counter_handle.set(counter + delta);

    // list of files
    //TODO: instead of this being a list of files, it should be a list of whatever struct we use for the pictures
    let files_handle = use_state::<Rc<Vec<File>>,_>(|| Rc::new(vec![]));
    let onchange = {
        let files_handle = files_handle.clone();
        move |e: Event| {
            let mut result = Vec::new();

            // copy all the existing files into the result
            for file in (*files_handle).iter() {
                result.push(file.clone());
            }

            // grab new files from the event
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                result.extend(files);
            }
            info!("{result:?}");
            files_handle.set(Rc::new(result));
        }
    };
    

    html! {
        <div>
            <div class="bg-blue-500 h-20 flex items-center justify-center text-5xl text-white">{"Holocene Calendar Maker"}</div>
            <button onclick={onclick.clone()(-1)} class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">
                { "-1" }
            </button>
            <button onclick={onclick.clone()(1)} class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
                { "+1" }
            </button>
            <p>{ counter }</p>
            <input type="file" multiple=true onchange={onchange} />
            <Card/>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}


#[function_component]
fn Card() -> Html {
    //simple card component with a picture and a body for text
    html! {
        <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700">
        <a href="#">
            <img class="rounded-t-lg" src="https://flowbite.com/docs/images/blog/image-1.jpg" alt="" />
        </a>
        <div class="p-5">
            <a href="#">
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{"Noteworthy technology acquisitions 2021"}</h5>
            </a>
            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
                {"Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order."}
            </p>
            <a href="#" class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                {"Read more"}
                <svg aria-hidden="true" class="w-4 h-4 ml-2 -mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"/>
                </svg>
            </a>
        </div>
        </div>
    }
}



//hooks to grab
//use_drop -> for sorting the pictures by month
//

//[Tasks]
// display list of pictures loaded in from browse file
// browse file button should have no state (i.e. not display file name/number selected).
//    also the button should say something like "add pictures"
// replace list of files with a list of structs representing each card
// draggable/orderable cards in the list
// ability to type in the text for each card
// render to PDF process -> opens in a new window!
//   possibly save editor state to local storage?

// convert file loading example to functional style, and probably package as a hook
// card component to hold picture for each month + optional spot for text
// maybe find some sort of image editor type thing? mainly for cropping images





// use std::collections::HashMap;
// use yew::{html, html::TargetCast, Component, Context, Html};
// use gloo_file::callbacks::FileReader;

// type Chunks = bool;

// pub enum Msg {
//     Loaded(String, String),
//     LoadedBytes(String, Vec<u8>),
//     Files(Vec<File>, Chunks),
//     ToggleReadBytes,
// }

// pub struct Model {
//     readers: HashMap<String, FileReader>,
//     files: Vec<String>,
//     read_bytes: bool,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             readers: HashMap::default(),
//             files: vec![],
//             read_bytes: false,
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Loaded(file_name, data) => {
//                 let info = format!("file_name: {}, data: {:?}", file_name, data);
//                 self.files.push(info);
//                 self.readers.remove(&file_name);
//                 true
//             }
//             Msg::LoadedBytes(file_name, data) => {
//                 let info = format!("file_name: {}, data: {:?}", file_name, data);
//                 self.files.push(info);
//                 self.readers.remove(&file_name);
//                 true
//             }
//             Msg::Files(files, bytes) => {
//                 for file in files.into_iter() {
//                     let file_name = file.name();
//                     let task = {
//                         let file_name = file_name.clone();
//                         let link = ctx.link().clone();

//                         if bytes {
//                             gloo_file::callbacks::read_as_bytes(&file, move |res| {
//                                 link.send_message(Msg::LoadedBytes(
//                                     file_name,
//                                     res.expect("failed to read file"),
//                                 ))
//                             })
//                         } else {
//                             gloo_file::callbacks::read_as_text(&file, move |res| {
//                                 link.send_message(Msg::Loaded(
//                                     file_name,
//                                     res.unwrap_or_else(|e| e.to_string()),
//                                 ))
//                             })
//                         }
//                     };
//                     self.readers.insert(file_name, task);
//                 }
//                 true
//             }
//             Msg::ToggleReadBytes => {
//                 self.read_bytes = !self.read_bytes;
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let flag = self.read_bytes;
//         html! {
//             <div>
//                 <div>
//                     <p>{ "Choose a file to upload to see the uploaded bytes" }</p>
//                     <input type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
//                             let mut result = Vec::new();
//                             let input: HtmlInputElement = e.target_unchecked_into();

//                             if let Some(files) = input.files() {
//                                 let files = js_sys::try_iter(&files)
//                                     .unwrap()
//                                     .unwrap()
//                                     .map(|v| web_sys::File::from(v.unwrap()))
//                                     .map(File::from);
//                                 result.extend(files);
//                             }
//                             Msg::Files(result, flag)
//                         })}
//                     />
//                 </div>
//                 <div>
//                     <label>{ "Read bytes" }</label>
//                     <input type="checkbox" checked={flag} onclick={ctx.link().callback(|_| Msg::ToggleReadBytes)} />
//                 </div>
//                 <ul>
//                     { for self.files.iter().map(|f| Self::view_file(f)) }
//                 </ul>
//             </div>
//         }
//     }
// }

// impl Model {
//     fn view_file(data: &str) -> Html {
//         html! {
//             <li>{ data }</li>
//         }
//     }
// }

// // fn main() {
// //     yew::start_app::<Model>();
// // }
