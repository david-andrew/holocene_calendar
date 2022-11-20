use yew::{function_component, use_state, html, TargetCast, Callback, Properties};
use yew_hooks::{use_list, use_drag_with_options, UseListHandle};
use gloo_file::{Blob, callbacks::FileReader};
use web_sys::{Event, HtmlInputElement};
use log::info;


#[function_component(App)]
fn app() -> Html {

    //TODO: how to have readers be dropped when they finish adding the image to the list?
    //perhaps have a set of readers, and then remove from the set?
    let readers: UseListHandle<FileReader> = use_list(vec![]);
    let append_reader = {
        let readers = readers.clone();
        Callback::from(move |reader: FileReader| {
            readers.push(reader);
        })
    };


    let images: UseListHandle<String> = use_list(vec![]);
    let append_image = {
        let images = images.clone();
        Callback::from(move |image: String| {
            images.push(image);
        })
    };


    // function for receiving files from input, and adding their data to the list of images
    let onchange = {
        move |e: Event| {

            // grab new files from the event
            let input: HtmlInputElement = e.target_unchecked_into();            
            {
                if let Some(files) = input.files() {
                    let blobs = js_sys::try_iter(&files)
                        .unwrap()
                        .unwrap()
                        .map(|v| web_sys::File::from(v.unwrap()))
                        .map(Blob::from)
                        .collect::<Vec<_>>();
                    
                    //create a reader for each file. When the reader is done, add the image data to the list
                    for blob in blobs.iter() {
                        let append_image = append_image.clone();
                        let reader = gloo_file::callbacks::read_as_data_url(&blob.clone(), move |b| {                            
                            if let Ok(data) = b {
                                append_image.emit(data);
                                //TODO: remove the reader from the list of readers
                            }
                        });
                        append_reader.emit(reader);
                    }
                }
            }
        }
    };

    

    html! {
        <div>
            <div class="bg-blue-500 h-20 flex items-center justify-center text-5xl text-white">{"Holocene Calendar Maker"}</div>
            <input type="file" multiple=true onchange={onchange} accept="image/*" />
            <FileInput />
            <div class="flex">
                { for images.current().iter().map(|data| 
                    html! { 
                        <Card src={data.clone()} />
                    }
                ) }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

#[derive(Properties, PartialEq)]
struct CardProps {
    src: String,
}

#[function_component(Card)]
fn card(props: &CardProps) -> Html {
    //simple card component with a picture and a body for text
    html! {
        <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700">
            <a href="#">
                <img class="rounded-t-lg" src={props.src.clone()} alt="" />
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


#[function_component(FileInput)]
fn file_input() -> Html {
    html! {
        <div class="flex items-center justify-center w-full">
            <label for="dropzone-file" class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    <svg aria-hidden="true" class="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path></svg>
                    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">{"Click to upload"}</span>{" or drag and drop"}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{"SVG, PNG, JPG or GIF (MAX. 800x400px)"}</p>
                </div>
                <input id="dropzone-file" type="file" class="hidden" />
            </label>
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
