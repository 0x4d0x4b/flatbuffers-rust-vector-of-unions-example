// import the flatbuffers runtime library
extern crate flatbuffers;
// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./my_table_generated.rs"]
mod my_table;

use flatbuffers::union_value_offsets;
use flatbuffers::TagUnionValueOffset;

use crate::my_table::my_example::{root_as_my_table, PayloadUnionTableOffset};
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
    let messages = fbb.create_vector_of_unions(&union_value_offsets!(
        PayloadUnionTableOffset,
        request1,
        response1,
        request2,
        response2
    ));
    // the above is equivalent of
    // let messages = fbb.create_vector_of_unions(&[
    //     PayloadUnionTableOffset::from_value_offset(request1),
    //     PayloadUnionTableOffset::from_value_offset(response1),
    //     PayloadUnionTableOffset::from_value_offset(request2),
    //     PayloadUnionTableOffset::from_value_offset(response2),
    // ]);

    let request3 = PayloadUnionTableOffset::from_value_offset(Request::create(
        &mut fbb,
        &RequestArgs {
            request_id: 3333u32,
        },
    ));
    let msg = MyTable::create(
        &mut fbb,
        &MyTableArgs {
            union_vector_type: Some(messages.tags),
            union_vector: Some(messages.values),
            union_single_type: request3.tag,
            union_single: Some(request3.value),
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
