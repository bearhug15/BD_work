#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;
use self::models::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::{env, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read, Write};
use std::cmp::min;
use string_builder::Builder;
use diesel::pg::types::sql_types::Json;
use serde_json::{Value, Map};
use std::collections::HashMap;
use BD_work::establish_connection;
use crate::schema::company::dsl::*;
use crate::schema::worker_types::dsl::*;
use crate::schema::group_types::dsl::*;
use crate::schema::department::dsl::*;
use crate::schema::equipment_type::dsl::*;
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc, RwLock};
use crate::tables_interaction::{process_init_ask, read_from_table, update_table, replace_entries_in_table, add_to_table, serial_add_to_table, delete_entry_in_table, process_form};
use crate::containers::{Forms, Tables};

pub mod schema;
pub mod models;
pub mod containers;
mod tables_interaction;
mod pagination;
mod pagination_impl;
mod form_structs;


fn get_data(mut stream: &TcpStream)->Result<String, &'static str>{

    let mut prelude_read: usize =0;
    let mut prelude_buffer: [u8;8]=[0;8];
    while prelude_read<8 {
        match stream.read(&mut prelude_buffer[prelude_read..]){
            Ok(amount) => {prelude_read+=amount;}
            Err(_) => {return Err("at prelude")}
        }
    }
    let mut length: usize = usize::from_be_bytes(prelude_buffer);
    let mut data_builder = Builder::default();
    let part_size:usize =1024;
    let mut data_buffer:[u8;1024]=[0;1024];
    while length>0 {
        match stream.read(&mut data_buffer[..min(&length,&part_size).clone()] ){
            Ok(amount) => {
                length-=amount;
                data_builder.append(&data_buffer[..amount]);
            }
            Err(_) => {return Err("at getting string")}
        }
    }
    return Ok(data_builder.string().unwrap())
}

fn send_data(mut stream: &TcpStream, data: String){
    let size = data.as_bytes().len();
    match stream.write(&mut size.to_be_bytes()){
        Ok(_) => {
            match stream.write(&mut data.as_bytes()){
                Ok(_) => {}
                Err(_) => {panic!("couldn't send data")}
            }
        }
        Err(_) => {panic!("couldn't send prelude")}
    }
}

fn process_message(connection: &PgConnection,mes_type:&String, message: &Map<String,Value>, )-> Value{

    match mes_type.as_str() {
        "initAsk"=>{
            let mut res  = process_init_ask(&connection);
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            res
        }
        "fullDataAsk"=>{
            let mut res = read_from_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("fieldName").unwrap().as_str().unwrap().to_string(),
                message.get("order").unwrap().as_str().unwrap().to_string(),
                "0".to_string());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            res
        }
        "partDataAsk"=>{
            let mut res = read_from_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("fieldName").unwrap().as_str().unwrap().to_string(),
                message.get("order").unwrap().as_str().unwrap().to_string(),
                "0".to_string());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            res
        }
        "updateTable"=>{
            let mut res = update_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("key").unwrap().as_str().unwrap().to_string(),
                message.get("values").unwrap());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            res
        }
        "replaceEntriesTable"=>{
            let mut res = replace_entries_in_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("keyName").unwrap().as_str().unwrap().to_string(),
                message.get("key").unwrap().as_str().unwrap().to_string(),
                message.get("values").unwrap());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            if(res.is_boolean()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),res);
                res = Value::Object(val_map);
            }
            res
        }
        "addEntry"=>{
            let mut res = add_to_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("values").unwrap());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            res
        }
        "serialAddTable"=>{
            let mut res = serial_add_to_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("values").unwrap());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            if(res.is_boolean()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),res);
                res = Value::Object(val_map);
            }
            res
        }
        "deleteEntry"=>{
            let mut res = delete_entry_in_table(
                connection,
                message.get("tableName").unwrap().as_str().unwrap().to_string(),
                message.get("keyName").unwrap().as_str().unwrap().to_string(),
                message.get("key").unwrap().to_string());
            if(res.is_null()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            if(res.is_boolean()){
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),res);
                res = Value::Object(val_map);
            }
            res
        }
        "form"=>{
            let mut res =process_form(
                connection,
                message.get("type").unwrap().as_str().unwrap().to_string(),
                 message.get("value").unwrap()
            );
            if res.is_null() || (res.is_boolean() && !res.as_bool().unwrap()) {
                let mut val_map:Map<String,Value> = Map::new();
                val_map.insert("Correct".to_string(),Value::Bool(false));
                res = Value::Object(val_map);
            }
            res
        }
        _ => {Value::Null }
    }
}
fn process_input(connection: &PgConnection,input_json: Value)-> Value{
    match input_json{
        Value::Object(entries) => {
            if !entries.contains_key("messageType"){
                return Value::Null;
            }
            match entries.get("messageType"){
                Some(mes_type_option) => {
                    match mes_type_option{
                        Value::String(mes_type) => {
                            return process_message(connection,mes_type, &entries)
                        }
                        _=>{
                            return Value::Null
                        }
                    }
                }
                None => {}
            }
        }
        _=>{
            return Value::Null
        }
    }
    //todo remove
    return  Value::Null
}


fn handle_client(mut stream: &TcpStream){
    let connection= establish_connection();
    loop{
        match get_data(&mut stream){
            Ok(data) => {
                let req:Value = serde_json::from_str(&data).unwrap();
                let res =process_input(&connection,req);
                send_data(&mut stream,serde_json::to_string(&res).unwrap())
            }
            Err(_) => {println!("Connection error");break;}
        }
    }
}

fn server(){
    let server = TcpListener::bind("127.0.0.1:20000").unwrap();

    for stream in server.incoming(){
        match stream{
            Ok(stream) => {
                thread::spawn(move ||{
                    handle_client(&stream);
                });
            }
            Err(e) => {
                println!("Unable to connect {}", e);
            }
        }
    }
}


fn main() {
    let t = vec![0,1,2,3];
    server()


}
