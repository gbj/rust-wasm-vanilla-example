use futures::StreamExt;
use leptos::{add_event_listener, body, create_element, document, log, spawn_local, window};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{console, Document, MouseEvent, Text, Window};

struct State {
    count: i32,
}

enum Msg {
    Increment,
    Decrement,
} // = JS type Msg = "inc" | "dec";

fn main() {
    // better error logging
    console_error_panic_hook::set_once();

    version_1_with_single_button();
}

fn version_1_with_single_button() {
    let mut state = State { count: 0 };

    let p = create_element("p");
    p.set_text_content(Some("Click the button to update this"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    let body = body().unwrap();
    body.append_child(&increment).unwrap();
    body.append_child(&p).unwrap();
    body.append_child(&decrement).unwrap();

    add_event_listener(&increment, "click", move |ev: MouseEvent| {
        log!("clicked +1");
        state.count += 1;
        p.set_text_content(Some(&state.count.to_string()));
    });

    /* add_event_listener(&decrement, "click", |ev: MouseEvent| {
        log!("clicked -1");
        state.clicks -= 1;
        p.set_text_content(Some(&clicks.to_string()));
    }); */
}

fn version_2_with_stale_closure() {
    let mut state = State { count: 0 };

    let p = create_element("p");
    p.set_text_content(Some("Click the button to update this"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    let body = body().unwrap();
    body.append_child(&increment).unwrap();
    body.append_child(&p).unwrap();
    body.append_child(&decrement).unwrap();

    add_event_listener(&increment, "click", {
        let p = p.clone();
        move |ev: MouseEvent| {
            log!("clicked +1");
            state.count += 1;
            p.set_text_content(Some(&state.count.to_string()));
        }
    });

    add_event_listener(&decrement, "click", move |ev: MouseEvent| {
        log!("clicked -1");
        state.count -= 1;
        p.set_text_content(Some(&state.count.to_string()));
    });
}

fn version_3_with_interior_mutability() {
    // this kind of wrapping is called "interior mutability" in Rust
    // in a sense, it moves borrow checking from the compile time to runtime
    let mut state = Rc::new(RefCell::new(State { count: 0 }));

    let p = create_element("p");
    p.set_text_content(Some("Click the button to update this"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    let body = body().unwrap();
    body.append_child(&increment);
    body.append_child(&p);
    body.append_child(&decrement);

    add_event_listener(&increment, "click", {
        let p = p.clone();
        let state = state.clone();
        move |ev: MouseEvent| {
            log!("clicked +1");
            state.borrow_mut().count += 1;
            p.set_text_content(Some(&state.borrow().count.to_string()));
        }
    });

    add_event_listener(&decrement, "click", move |ev: MouseEvent| {
        log!("clicked -1");
        state.borrow_mut().count -= 1;
        p.set_text_content(Some(&state.borrow().count.to_string()));
    });
}

fn version_4_with_async_channel_and_reducer_pattern() {
    let window = web_sys::window().expect("there to be a window");
    let document = window.document().expect("there to be a document");
    let body = document.body().expect("there to be a body");

    let p = document.create_element("p").expect("to create the element");
    p.set_text_content(Some("Hello, Ryan!"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    /* let state = Rc::new(
        RefCell::new(State { count: "0".to_string() })
    ); */

    let (mut message_sender, mut message_receiver) = futures::channel::mpsc::channel(4);

    body.append_child(&decrement);
    body.append_child(&p);
    body.append_child(&increment);

    spawn_local(async move {
        let mut count = 0;
        while let Some(msg) = message_receiver.next().await {
            match msg {
                Msg::Increment => count += 1,
                Msg::Decrement => count -= 1,
            }
            p.set_text_content(Some(&format!("count is {count}")));
        }
    });

    add_event_listener(&increment, "click", {
        let mut message_sender = message_sender.clone();
        move |_: web_sys::Event| {
            message_sender.try_send(Msg::Increment);
        }
    });

    add_event_listener(&decrement, "click", move |_: web_sys::Event| {
        message_sender.try_send(Msg::Decrement);
    });
}

// Version 1: with Leptos helpers

/*

// better error logging
    console_error_panic_hook::set_once();

    let mut state = State { count: 0 };

    let p = create_element("p");
    p.set_text_content(Some("Click the button to update this"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    let body = body().unwrap();
    body.append_child(&increment).unwrap();
    body.append_child(&p).unwrap();
    body.append_child(&decrement).unwrap();

    add_event_listener(&increment, "click", move |ev: MouseEvent| {
        log!("clicked +1");
        state.count += 1;
        p.set_text_content(Some(&state.count.to_string()));
    });

    /* add_event_listener(&decrement, "click", |ev: MouseEvent| {
        log!("clicked -1");
        state.clicks -= 1;
        p.set_text_content(Some(&clicks.to_string()));
    }); */

*/

// Version 2: with stale closures!

/*
   add_event_listener(&increment, "click", {
       let p = p.clone();
       move |ev: MouseEvent| {
           log!("clicked +1");
           clicks += 1;
           p.set_text_content(Some(&clicks.to_string()));
       }
   });

   add_event_listener(&decrement, "click", move |ev: MouseEvent| {
       log!("clicked -1");
       clicks -= 1;
       p.set_text_content(Some(&clicks.to_string()));
   });
*/

// Version 3: with RefCell
/*     let mut state = Rc::new(RefCell::new(State { count: 0 }));

    let p = create_element("p");
    p.set_text_content(Some("Click the button to update this"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    let body = body().unwrap();
    body.append_child(&increment);
    body.append_child(&p);
    body.append_child(&decrement);

    add_event_listener(&increment, "click", {
        let p = p.clone();
        let state = state.clone();
        move |ev: MouseEvent| {
            log!("clicked +1");
            state.borrow_mut().count += 1;
            p.set_text_content(Some(&state.borrow().count.to_string()));
        }
    });

    add_event_listener(&decrement, "click", move |ev: MouseEvent| {
        log!("clicked -1");
        state.borrow_mut().count -= 1;
        p.set_text_content(Some(&state.borrow().count.to_string()));
    });
*/

// Version 1: manual

/* let body = document().body().unwrap();

// create some DOM nodes
let p = document().create_element("p").unwrap();
p.set_text_content(Some("Hello, Wasm!"));
body.append_child(&p).unwrap();

let button = document().create_element("button").unwrap();
button.set_text_content(Some("Click me"));
button
    .add_event_listener_with_callback(
        "click",
        Closure::wrap(Box::new(|| {
            console::log_1(&JsValue::from("clicked!"));
        }) as Box<dyn Fn()>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
    )
    .unwrap();

body.append_child(&button).unwrap(); */

/* State machine version

enum Msg {
    Increment,
    Decrement
}


impl State {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }
}

    let mut state = State { count: 0 };

    let p = create_element("p");
    p.set_text_content(Some("Click the button to update this"));

    let increment = create_element("button");
    increment.set_text_content(Some("+1"));

    let decrement = create_element("button");
    decrement.set_text_content(Some("-1"));

    let body = body().unwrap();
    body.append_child(&increment).unwrap();
    body.append_child(&p).unwrap();
    body.append_child(&decrement).unwrap();

    let (mut msg_sender, mut msg_receiver) = futures::channel::mpsc::channel(4);
    spawn_local(async move {
        while let Some(msg) = msg_receiver.next().await {
            state.update(msg);
            p.set_text_content(Some(&format!("Value from stream is {}", state.count)));
        }
    });


    add_event_listener(&increment, "click", {
        let mut msg_sender = msg_sender.clone();
        move |ev: MouseEvent| {
            log!("clicked +1");
            //state.count += 1;
            //p.set_text_content(Some(&state.count.to_string()));
            msg_sender.try_send(Msg::Increment).unwrap();
        }
    });

    add_event_listener(&decrement, "click", move |ev: MouseEvent| {
        log!("clicked -1");
        msg_sender.try_send(Msg::Decrement).unwrap();
    });

*/
