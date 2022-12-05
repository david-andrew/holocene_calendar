use yew::{function_component, use_state, html, TargetCast, Callback, Properties};
use yew_hooks::{use_list, use_drag_with_options, UseListHandle};
use gloo_file::{Blob, callbacks::FileReader};
use web_sys::{Event, HtmlInputElement, DragEvent};
use log::info;

// [Tasks]
// 1. draggable cards

// > 1...
// move onchange/ondrop/ondragover into the Input component
// replace list of files with a list of structs representing each card
//  - picture data string
//  - cropping bounds/other transforms
// separate list for storing any text alterations per each month
//  - holiday names
//  - include lunar phases
//  - poem (TODO: needs to be clear that this stays with the month when you drag the picture... perhaps a second draggable list that matches the pictures?)
// draggable/orderable cards in the list
// ability to type in the text for each card
// render to PDF process -> opens in a new window!
//   possibly save editor state to local storage?

// convert file loading example to functional style, and probably package as a hook
// card component to hold picture for each month + optional spot for text
// maybe find some sort of image editor type thing? mainly for cropping images


//[Design Notes]
// picking which holidays are included and their names should be handled in a modal
// ---> basically all global settings will be from a settings modal



#[function_component(App)]
fn app() -> Html {

    let months: Vec<String> = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ].iter().map(|s| s.to_string()).collect();

    //TODO: how to have readers be dropped when they finish adding the image to the list?
    //perhaps have a set of readers, and then remove from the set?
    let readers: UseListHandle<FileReader> = use_list(vec![]);
    let add_reader = {
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
    let onchange = Callback::from(
        {
            let add_reader = add_reader.clone();
            let append_image = append_image.clone();
            move |e: Event| {
                
                // grab new files from the event
                let input: HtmlInputElement = e.target_unchecked_into();            
                {
                    if let Some(files) = input.files() {
                            // collect all the input files into a vector of blobs
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
                                }
                            });
                            add_reader.emit(reader);
                        }
                    }
                }
            }
        }
    );

    let ondragover = Callback::from(|e:DragEvent|{e.prevent_default();});
    let ondrop = Callback::from(
        {
            let add_reader = add_reader.clone();
            let append_image = append_image.clone();
            move |e:DragEvent| {
                e.prevent_default();
                if let Some(files) = e.data_transfer().unwrap().files() {
                    // collect all the input files into a vector of blobs
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
                            }
                        });
                        add_reader.emit(reader);
                    }
                }
            }
        }
    );

    

    html! {
        <div>
            <div class="bg-blue-500 h-20 flex items-center justify-center text-5xl text-white">{"Holocene Calendar Maker"}</div>
            // <input type="file" multiple=true onchange={onchange} accept="image/*" />
            <FileInput onchange={onchange} ondragover={ondragover} ondrop={ondrop} />
            <div class="flex flex-wrap">
                { for images.current().iter().zip(months).map(|(data, month)| 
                    html! { 
                        <Card src={data.clone()} title={month} />
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
    title: String,
    poem: Option<String>,
}

#[function_component(Card)]
fn card(props: &CardProps) -> Html {

    let CardProps {src, title, poem} = props;

    //simple card component with a picture and a body for text
    html! {
        <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700">
            <a href="#">
                <img class="rounded-t-lg" src={src.clone()} alt="" />
            </a>
            <div class="p-5">
                <a href="#">
                    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{title.clone()}</h5>
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


#[derive(Properties, PartialEq)]
struct InputProps {
    onchange: Callback<Event>,
    ondrop: Callback<DragEvent>,
    ondragover: Callback<DragEvent>,
}
#[function_component(FileInput)]
fn file_input(props: &InputProps) -> Html {
    let InputProps {onchange, ondrop, ondragover} = props;
    html! {
        <div class="flex items-center justify-center w-full" 
            ondragover={ondragover} 
            ondrop={ondrop}>
            <label for="dropzone-file" class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    <svg aria-hidden="true" class="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path></svg>
                    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">{"Click to upload"}</span>{" or drag and drop"}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{"SVG, PNG, JPG or GIF (MAX. 800x400px)"}</p>
                </div>
                <input id="dropzone-file" type="file" class="hidden" onchange={onchange} multiple=true accept="image/*" />
            </label>
        </div>
    } 
}

