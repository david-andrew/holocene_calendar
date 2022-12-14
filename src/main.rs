use yew::{function_component, use_state, html, Html, TargetCast, MouseEvent, Callback, Properties, AttrValue, Children, use_node_ref};
use yew_hooks::{use_list, use_drag_with_options, UseListHandle};
use gloo_file::{Blob, callbacks::FileReader};
use web_sys::{Event, HtmlInputElement, HtmlElement, DragEvent};
use log::info;

// [Tasks]
// 3. specify aspect ratio of pictures/calendar -> use for shape of images in cards
// 4. draggable/sortable cards
// 5. editable poem text (owned by main component), appears in textbox on card

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




//[Resources]
// - https://developer.mozilla.org/en-US/docs/Web/CSS/break-after / https://www.w3schools.com/csSref/pr_print_pageba.php
//   --> for displaying output pdf for printing
//


#[function_component]
fn App() -> Html {

    //TODO: allow aspect ratio to be set-able
    let img_aspect_ratio = 13.0/19.0;

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
            if images.current().len() < 12 {
                images.push(image);
            }
        })
    };

    //callback for when a user drags a card to a new position
    let onreposition = Callback::from(
        {
            let images = images.clone();
            move |(old_index, new_index): (usize, usize)| {
                let image = images.remove(old_index);
                images.insert(new_index, image);
            }
        }
    );


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
            { // show the file input only if there are less than 12 images
                if images.current().len() < 12 {
                    html! { <FileInput onchange={onchange} ondragover={ondragover} ondrop={ondrop} />} 
                } else {
                    html! {<></>}
                }
            }
            // <div class="flex flex-wrap justify-center">
            //     { for images.current().iter().zip(months).enumerate().map(|(i, (data, month))| 
            //         html! { 
            //             <Card src={data.clone()} title={month} poem={"this is my poem"} //setpoem={None} img_aspect={img_aspect_ratio} 
            //                 onclose={
            //                     let images = images.clone();
            //                     Callback::from(move |_| {
            //                         images.remove(i);
            //                     })
            //                 }
            //             />
            //         }
            //     ) }
            // </div>
            <DraggableList onreposition={onreposition}>
                { for images.current().iter().zip(months).enumerate().map(|(i, (data, month))| 
                    html! { 
                        <Card src={data.clone()} title={month} poem={"this is my poem"} //setpoem={None} img_aspect={img_aspect_ratio} 
                            onclose={
                                let images = images.clone();
                                Callback::from(move |_| {
                                    images.remove(i);
                                })
                            }
                        />
                    }
                ) }
            </DraggableList>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}


#[derive(Properties, PartialEq, Clone)]
struct CardProps {
    src: AttrValue,
    #[prop_or(1.0)]
    img_aspect: f64,
    title: AttrValue,
    #[prop_or(None)]
    poem: Option<AttrValue>,
    #[prop_or(None)]
    setpoem: Option<Callback<AttrValue>>,
    #[prop_or(None)]
    onclose: Option<Callback<()>>,
}

#[function_component]
fn Card(props: &CardProps) -> Html {

    let CardProps {src, img_aspect, title, poem, setpoem, onclose} = props.clone();

    //simple card component with a picture and a body for text
    html! {
        <div class="relative">
            { // if there is a close button, show it
                if let Some(onclose) = onclose {
                    // let onclose = onclose.clone();
                    let onclick = Callback::from(move |_| {
                        onclose.emit(());
                    });
                    html! {
                        <button class="absolute -right-4 -top-4 z-10 text-white bg-red-500 hover:bg-red-700 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2" onclick={onclick}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                            <span class="sr-only">{"Remove Card"}</span>

                        </button>
                    }
                } else {
                    html! {<></>}
                }
            }
            <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700" draggable="false">
                // <a href="#">
                <img class="rounded-t-lg" src={src.clone()} alt="" draggable="false"/>
                // </a>
                <div class="p-5">
                    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{title.clone()}</h5>
                    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
                        {poem.clone().unwrap_or_else(|| AttrValue::from("No poem"))}
                    </p>
                </div>
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
#[function_component]
fn FileInput(props: &InputProps) -> Html {
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

#[derive(Properties, PartialEq, Clone)]
struct DraggableListProps {
    children: Children,
    onreposition: Callback<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]
enum DragSide {
    Left,
    Right,
}
#[derive(Clone)]
struct DragState {
    holding_index: usize,
    over_index: usize,
    side: DragSide,
}

#[function_component]
fn DraggableList(props: &DraggableListProps) -> Html {
    
    let DraggableListProps {children, onreposition} = props.clone();
    
    let dragging = use_state(|| false);
    let drag_state = use_state(|| DragState {
        holding_index: 0,
        over_index: 0,
        side: DragSide::Left,
    });
    // let num_children = props.children.len();
    // let container_ref = use_node_ref();

    html! {
        <div class="relative flex flex-wrap justify-center">// ref={container_ref.clone()}>
            <div class="absolute left-0 top-0 w-full h-full -z-10 bg-white"
                
            />
            { for children.iter().enumerate().map(|(i, child)| html! {
                <>
                    //left placeholder area
                    {{
                        let dragging = *(dragging.clone());
                        let DragState {holding_index:_, over_index, side} = *(drag_state.clone());
                        if dragging && over_index == i && side == DragSide::Left {
                            html! {
                                <div class="flex justify-center w-96 bg-white"
                                    ondragenter={Callback::from(move |e:DragEvent| {
                                        e.prevent_default();
                                    })}
                                    ondragover={Callback::from(move |e:DragEvent| {
                                        e.prevent_default();
                                    })}
                                />
                            }
                        } else {
                            html! {
                                <></>
                            }
                        }
                    }}
                    // div wrapping around the child
                    <div>
                        //left and right areas to detect hovering
                        <div class="relative w-full h-full" draggable="true"
                            ondragstart={
                                let dragging = dragging.clone();
                                let drag_state = drag_state.clone();
                                // let container_ref = container_ref.clone();
                                Callback::from(move |e:DragEvent| {
                                    dragging.set(true);
                                    drag_state.set(DragState {
                                        holding_index: i,
                                        over_index: i,
                                        side: DragSide::Left//if i < num_children - 1 {DragSide::Left} else {DragSide::Right},
                                    });
                                    //get the dimensions of the div container
                                    // let container = container_ref.cast::<HtmlElement>().unwrap();
                                    // container

                                })
                            }
                            ondragend={
                                let dragging = dragging.clone();
                                let drag_state = drag_state.clone();
                                let onreposition = onreposition.clone();
                                Callback::from(move |e:DragEvent| {
                                    dragging.set(false);
                                    let DragState {holding_index, over_index, side} = *drag_state;
                                    let mut final_index = over_index;
                                    if side == DragSide::Right {
                                        if final_index < holding_index {
                                            final_index += 1;
                                        }
                                    } else {
                                        if final_index > holding_index {
                                            final_index -= 1;
                                        }
                                    }
                                    if holding_index != final_index {
                                        onreposition.emit((holding_index, final_index));
                                    }
                                })
                            }
                        >
                            <div class="absolute left-0 top-0 w-full h-full z-10">
                                <div class="absolute left-0 top-0 w-1/2 h-full" 
                                    //on drag for left side
                                    ondragover={
                                        let dragging = dragging.clone();
                                        let drag_state = drag_state.clone();
                                        Callback::from(move |e:DragEvent| {
                                            if *dragging {
                                                let DragState {holding_index, over_index, side} = *drag_state;
                                                if over_index != i || side != DragSide::Left {
                                                    drag_state.set(DragState {
                                                        holding_index,
                                                        over_index: i,
                                                        side: DragSide::Left,
                                                    });
                                                }
                                            }
                                        })
                                    }
                                    
                                ></div>
                                <div class="absolute right-0 top-0 w-1/2 h-full" 
                                    //on drag for right side
                                    ondragover={
                                        let dragging = dragging.clone();
                                        let drag_state = drag_state.clone();
                                        Callback::from(move |e:DragEvent| {
                                            if *dragging {
                                                let DragState {holding_index, over_index, side} = *drag_state;
                                                if over_index != i || side != DragSide::Right {
                                                    drag_state.set(DragState {
                                                        holding_index,
                                                        over_index: i,
                                                        side: DragSide::Right,
                                                    });
                                                }
                                            }
                                        })
                                    }
                                ></div>
                            </div>
                            {{
                                let dragging = dragging.clone();
                                let drag_state = drag_state.clone();
                                let hide = *dragging && (*drag_state).holding_index == i;
                                html! {
                                    <div class={if hide {"absolute -left-96 w-96 -z-20"} else {"relative"}}>                                        
                                        {child}
                                    </div>
                                }
                            }}
                        </div>
                    </div>

                    //right placeholder area
                    {{
                        let dragging = *(dragging.clone());
                        let DragState {holding_index:_, over_index, side} = *(drag_state.clone());
                        if dragging && over_index == i && side == DragSide::Right {
                            html! {
                                <div class="flex justify-center w-96 bg-white"
                                    ondragenter={Callback::from(move |e:DragEvent| {
                                        e.prevent_default();
                                    })}
                                    ondragover={Callback::from(move |e:DragEvent| {
                                        e.prevent_default();
                                    })}
                                />
                            }
                        } else {
                            html! {
                                <></>
                            }
                        }
                    }}
                </>
            }) }
        </div>
    }
}
