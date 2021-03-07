// import the flatbuffers runtime library
extern crate flatbuffers;
// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./my_table_generated.rs"]
mod my_table;

use crate::my_table::my_example::root_as_my_table;
pub use my_table::my_example::{
    MyTable, MyTableArgs, Payload, Request, RequestArgs, Response, ResponseArgs,
};
use std::fs;

fn write() {
    let mut fbb = flatbuffers::FlatBufferBuilder::new();
    let request1 = Request::create(
        &mut fbb,
        &RequestArgs {
            request_id: 1333u32,
        },
    );
    let response1 = Response::create(
        &mut fbb,
        &ResponseArgs {
            response_id: 1555u32,
        },
    );
    let request2 = Request::create(
        &mut fbb,
        &RequestArgs {
            request_id: 2333u32,
        },
    );
    let response2 = Response::create(
        &mut fbb,
        &ResponseArgs {
            response_id: 2555u32,
        },
    );
    let request3 = Request::create(
        &mut fbb,
        &RequestArgs {
            request_id: 3333u32,
        },
    );
    let messages = fbb.create_vector_of_unions(&[
        Payload::tag_as_request(request1),
        Payload::tag_as_response(response1),
        Payload::tag_as_request(request2),
        Payload::tag_as_response(response2),
        Payload::tag_as_aliased(request3),
    ]);

    let request4 = Payload::tag_as_request(Request::create(
        &mut fbb,
        &RequestArgs {
            request_id: 4333u32,
        },
    ));
    let msg = MyTable::create(
        &mut fbb,
        &MyTableArgs {
            union_vector_type: Some(messages.tags()),
            union_vector: Some(messages.values_offset()),
            union_single_type: request4.tag(),
            union_single: Some(request4.value_offset()),
            table_vector: None,
            table_single: None,
            struct_vector: None,
            struct_single: None,
        },
    );
    fbb.finish(msg, None);
    let data = fbb.finished_data();
    fs::write("/tmp/simple.bin", data).expect("Unable to write file");
}

fn read() {
    let data = fs::read("/tmp/simple.bin").unwrap();
    let simple = root_as_my_table(&data[..]).unwrap();
    let msg_types = simple.union_vector_type().unwrap();
    let msg_values = simple.union_vector().unwrap();
    msg_types.iter().zip(msg_values.iter()).for_each(|x| {
        let msg_type = x.0;
        let table = x.1;
        match msg_type {
            Payload::Request => {
                let req = Request::init_from_table(table);
                println!("Request id: {}", req.request_id())
            }
            Payload::Response => {
                let resp = Response::init_from_table(table);
                println!("Response id: {}", resp.response_id())
            }
            Payload::Aliased => {
                let req = Request::init_from_table(table);
                println!("Aliased: Request id: {}", req.request_id())
            }
            _ => println!("Invalid"),
        }
    });
    let req3 = Request::init_from_table(simple.union_single().unwrap());
    println!("Request id: {}", req3.request_id())
}

fn main() {
    write();
    read();
}
