use diesel::{PgConnection, RunQueryDsl, QuerySource, BoolExpressionMethods, Expression, AppearsOnTable, sql_query};
use serde_json::{Value, Map};
use std::sync::{Arc, RwLock, Mutex, MutexGuard};
use std::collections::HashMap;
use crate::containers::Tables;
use serde::Serialize;
use diesel::result::Error;
use crate::models::*;
use crate::schema::company::dsl::company;
use crate::schema::company::columns::company_name;
use crate::schema::worker_types::dsl::worker_types;
use crate::schema::worker_types::columns::worker_type;
use crate::schema::group_types::dsl::group_types;
use crate::schema::group_types::columns::group_type;
use crate::schema::department::dsl::department;
use crate::schema::department::columns::department_name;
use crate::schema::equipment_type::dsl::equipment_type;
use crate::schema::equipment_type::columns::type_;
use crate::schema::contract::dsl::contract;
use crate::schema::contract::columns::{contract_id, contract_start, contract_end};
use crate::schema::eq_group_list::dsl::eq_group_list;
use crate::schema::project::dsl::project;
use crate::schema::project::columns::project_id;
use crate::schema::worker::dsl::worker;
use crate::schema::constr_attr::dsl::constr_attr;
use crate::schema::eng_attr::dsl::eng_attr;
use crate::schema::lab_attr::dsl::lab_attr;
use crate::schema::mech_attr::dsl::mech_attr;
use crate::schema::equipment::dsl::equipment;
use crate::schema::groups_bind::dsl::groups_bind;
use crate::schema::working_company::dsl::working_company;
use crate::schema::eq_group::dsl::eq_group;
use crate::schema::groups::dsl::groups;
use crate::schema::pc_bind::dsl::pc_bind;
use crate::diesel::QueryDsl;
use diesel::expression::dsl::*;
use crate::diesel::ExpressionMethods;
use chrono::{NaiveDateTime, ParseError};
use crate::schema::worker::columns::{worker_id, salary};
use crate::schema::equipment::columns::eq_id;
use crate::schema::groups_bind::columns::group_id;
use crate::schema::department_head::dsl::department_head;
use crate::schema::pc_bind::columns::{head_id, eq_list_id};
use crate::pagination::*;
use diesel::pg::Pg;
use diesel::query_builder::{QueryFragment, SqlQuery};
use crate::pagination_impl::Params;
use diesel::sql_types::{Int8, Integer, Text, Int4, Timestamp, Decimal, Numeric, Double};
use std::num::ParseIntError;


pub fn process_init_ask(connection: &PgConnection) ->Value{

    let company_value = match company.load::<Company>(connection){
        Ok(res) => {serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap()}
        Err(_) => {Value::Null}
    };

    let worker_types_value = match worker_types.load::<WorkerTypes>(connection){
        Ok(res) => {serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap()}
        Err(_) => {Value::Null}
    };

    let group_types_value = match group_types.load::<GroupTypes>(connection){
        Ok(res) => {serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap()}
        Err(_) => {Value::Null}
    };

    let department_value = match department.load::<Department>(connection){
        Ok(res) => {serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap()}
        Err(_) => {Value::Null}
    };

    let equipment_type_value = match equipment_type.load::<EquipmentType>(connection){
        Ok(res) => {serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap()}
        Err(_) => {Value::Null}
    };

    let mut val_map:Map<String,Value> = Map::new();
    val_map.insert("company".to_string(), company_value);
    val_map.insert("worker_types".to_string(),worker_types_value);
    val_map.insert("group_types".to_string(),group_types_value);
    val_map.insert("department".to_string(),department_value);
    val_map.insert("equipment_type".to_string(),equipment_type_value);
    let val = Value::Object(val_map);

    return val;
}

macro_rules! load{
    ($table_name:ident,$params:ident,$connection:ident,$field_name:ident,$order:ident,$val_map:ident)=>{
        match $table_name::get_page($params,$connection,$field_name,$order){
            Ok((res,count)) => {
                $val_map.insert(stringify!($table_name).to_string(),serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap());
                return Value::Object($val_map);
            }
            Err(s) => {
                println!("{}",s);
                return Value::Null;
            }
        }
    }
}

pub fn read_from_table(connection: &PgConnection, table_name:String, field_name:String, order:String,part: String)->Value{
    let mut val_map:Map<String,Value> = Map::new();
    let page = match part.parse::<i64>(){
        Ok(res) =>{res}
        Err(_) => {return Value::Null}
    };
    let params = Params{
        page: Some(page),
        page_size: Some(10)
    };
    match table_name.as_str(){
        "company"=>{ load!(Company,params,connection,field_name,order,val_map) }
        "worker_types"=>{ load!(WorkerTypes,params,connection,field_name,order,val_map) }
        "department"=>{ load!(Department,params,connection,field_name,order,val_map) }
        "group_types"=>{ load!(GroupTypes,params,connection,field_name,order,val_map) }
        "equipment_type"=>{ load!(EquipmentType,params,connection,field_name,order,val_map) }
        "contract"=>{ load!(Contract,params,connection,field_name,order,val_map) }
        "eq_group_list"=>{ load!(EqGroupList,params,connection,field_name,order,val_map) }
        "project"=>{ load!(Project,params,connection,field_name,order,val_map) }
        "worker"=>{ load!(Worker,params,connection,field_name,order,val_map) }
        "constr_attr"=>{ load!(ConstrAttr,params,connection,field_name,order,val_map) }
        "eng_attr"=>{ load!(EngAttr,params,connection,field_name,order,val_map) }
        "lab_attr"=>{ load!(LabAttr,params,connection,field_name,order,val_map) }
        "mech_attr"=>{ load!(MechAttr,params,connection,field_name,order,val_map) }
        "equipment"=>{ load!(Equipment,params,connection,field_name,order,val_map) }
        "groups_bind"=>{ load!(GroupsBind,params,connection,field_name,order,val_map) }
        "working_company"=>{ load!(WorkingCompany,params,connection,field_name,order,val_map) }
        "department_head"=>{ load!(DepartmentHead,params,connection,field_name,order,val_map) }
        "eq_group"=>{ load!(EqGroup,params,connection,field_name,order,val_map) }
        "groups"=>{ load!(Groups,params,connection,field_name,order,val_map) }
        "pc_bind"=>{ load!(PcBind,params,connection,field_name,order,val_map) }
        _ => {Value::Null }
    }

}

pub fn update_table(connection: &PgConnection, table_name:String, key:String,values:&Value)->Value{
    let mut val_map:Map<String,Value> = Map::new();
    match table_name.as_str() {
        "company" => {
            let val =  match values.get("company_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = company.filter(company_name.eq(key));
            let q =diesel::update(target).set(company_name.eq(val));
            let res =q.get_result::<Company>(connection).expect("Error updating");
            val_map.insert("name".to_string(),serde_json::from_str( serde_json::to_string(&res.company_name).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "worker_types" =>{
            let val =  match values.get("worker_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = worker_types.filter(worker_type.eq(key));
            let q =diesel::update(target).set(worker_type.eq(val));
            let res =q.get_result::<WorkerTypes>(connection).expect("Error updating");
            val_map.insert("name".to_string(),serde_json::from_str( serde_json::to_string(&res.worker_type).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "department" => {
            let val =  match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = department.filter(department_name.eq(key));
            let q =diesel::update(target).set(department_name.eq(val));
            let res =q.get_result::<Department>(connection).expect("Error updating");
            val_map.insert("name".to_string(),serde_json::from_str( serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "group_types" =>{
            let val =  match values.get("group_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = group_types.filter(group_type.eq(key));
            let q= diesel::update(target).set(group_type.eq(val));
            let res =q.get_result::<GroupTypes>(connection).expect("Error updating");
            val_map.insert("name".to_string(),serde_json::from_str( serde_json::to_string(&res.group_type).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "equipment_type" => {
            let val =  match values.get("type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = equipment_type.filter(type_.eq(key));
            let q=diesel::update(target).set(type_.eq(val));
            let res =q.get_result::<EquipmentType>(connection).expect("Error updating");
            val_map.insert("name".to_string(),serde_json::from_str( serde_json::to_string(&res.type_).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "contract"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_cost = match values.get("cost").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let new_start = match values.get("contract_start").unwrap(){
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            let new_end = match values.get("contract_end").unwrap(){
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            let target = contract.filter(contract_id.eq(&old_id));
            let new_val =Contract{
                contract_id: old_id,
                cost: Some(new_cost),
                contract_start: new_start,
                contract_end: new_end
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Contract>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "eq_group_list"=>{
            return Value::Null;
        }
        "project"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_cost = match values.get("cost").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let mut new_data:Option<Value> = match values.get("data"){
                None=>{None}
                Some(res)=>{Some(res.clone())}
            };
            let target = project.filter(project_id.eq(&old_id));
            let new_val =Project{
                project_id: old_id,
                cost: Some(new_cost),
                data: new_data
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Project>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.project_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "worker"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_firstname = match values.get("firstname").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_secondname = match values.get("secondname").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_familyname = match values.get("familyname").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_age = match values.get("age").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_salary = match values.get("salary").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let new_worker_type = match values.get("worker_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_department_name = match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = worker.filter(worker_id.eq(&old_id));
            let new_val =Worker{
                worker_id: old_id,
                firstname: new_firstname.clone(),
                secondname: Some(new_secondname.clone()),
                familyname: new_familyname.clone(),
                age: new_age,
                salary: new_salary,
                department_name: new_department_name.clone(),
                worker_type: new_worker_type.clone()
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Worker>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "constr_attr"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_cert_number = match values.get("cert_number").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let target = constr_attr.filter(crate::schema::constr_attr::columns::worker_id.eq(&old_id));
            let new_val =ConstrAttr{
                worker_id: old_id,
                cert_number: new_cert_number
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<ConstrAttr>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "eng_attr"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_category = match values.get("category").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let target = eng_attr.filter(crate::schema::eng_attr::columns::worker_id.eq(&old_id));
            let new_val =EngAttr{
                worker_id: old_id,
                category: new_category
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<EngAttr>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "mech_attr"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_repair_type = match values.get("repair_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = mech_attr.filter(crate::schema::mech_attr::columns::worker_id.eq(&old_id));
            let new_val =MechAttr{
                worker_id: old_id,
                repair_type: new_repair_type.clone()
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<MechAttr>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "lab_attr"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_lab_number = match values.get("lab_number").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let target = lab_attr.filter(crate::schema::lab_attr::columns::worker_id.eq(&old_id));
            let new_val =LabAttr{
                worker_id: old_id,
                lab_number: new_lab_number
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<LabAttr>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "equipment"=> {
            let old_id = key.parse::<i32>().unwrap();
            let new_name = match values.get("name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_type = match values.get("type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_department_name:Option<String> = match values.get("department_name"){
                None => {None}
                Some(val) => {
                    match val{
                    Value::String(s)=>{Some(s.clone())}
                    _=>{panic!("")}
                    }
                }
            };
            let target = equipment.filter(eq_id.eq(&old_id));
            let new_val =Equipment{
                eq_id: 0,
                name: new_name.clone(),
                department_name: new_department_name,
                type_: new_type.clone()
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Equipment>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.eq_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "groups_bind"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_cost = match values.get("cost").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let new_group_type = match values.get("cost").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = groups_bind.filter(group_id.eq(&old_id));
            let new_val =GroupsBind{
                group_id: old_id,
                cost: new_cost,
                group_type: new_group_type.clone()
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<GroupsBind>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "working_company"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_company_name = match values.get("company_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = working_company.filter(crate::schema::working_company::columns::group_id.eq(&old_id));
            let new_val =WorkingCompany{
                group_id: old_id,
                company_name: new_company_name.clone()
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<WorkingCompany>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "department_head"=>{
            let val =  match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let target = department_head.filter(crate::schema::department_head::columns::department_name.eq(&key));
            let new_val =DepartmentHead{
                department_name: key.clone(),
                worker_id: val
            };
            let q = diesel::update(target).set(&new_val);
            let res = q.get_result::<DepartmentHead>(connection).expect("Error updating");
            val_map.insert("name".to_string(),serde_json::from_str( serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "eq_group"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_eq_list_id = match values.get("eq_list_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_eq_id = match values.get("eq_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let target = eq_group.filter(crate::schema::eq_group::columns::id.eq(&old_id));
            let new_val = EqGroup{
                id: old_id,
                eq_list_id: new_eq_list_id,
                eq_id: new_eq_id
            };
            let q = diesel::update(target).set(&new_val);
            let res = q.get_result::<EqGroup>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "groups"=>{
            let old_id = key.parse::<i32>().unwrap();
            let new_group_id = match values.get("group_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_worker_id = match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let target = groups.filter(crate::schema::groups::columns::id.eq(&old_id));
            let new_val = Groups{
                id: old_id,
                group_id: new_group_id,
                worker_id: new_worker_id
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Groups>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        "pc_bind"=>{
            let old_contract_id = match values.get("contract_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let old_project_id = match values.get("project_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_group_id = match values.get("group_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_head_id = match values.get("head_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_project_start = match values.get("project_start").unwrap(){
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            let new_project_end = match values.get("project_end").unwrap(){
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            let new_eq_list_id = match values.get("eq_list_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let target = pc_bind.filter(crate::schema::pc_bind::columns::contract_id.eq(&old_contract_id).and(crate::schema::pc_bind::columns::project_id.eq(&old_project_id)));
            let new_val = PcBind{
                contract_id: old_contract_id,
                project_id: old_project_id,
                group_id: new_group_id,
                head_id: new_head_id,
                project_start: new_project_start,
                project_end: new_project_end,
                eq_list_id: new_eq_list_id
            };
            let q =diesel::update(pc_bind).set(&new_val);
            let res = q.get_result::<PcBind>(connection).expect("Error updating");
            val_map.insert("id".to_string(),serde_json::from_str( serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap());
            return Value::Object(val_map);
        }
        _ => {}
    }
    return Value::Null;
}

pub fn add_to_table(connection: &PgConnection, table_name:String,values:&Value)->Value{

    let mut val_map:Map<String,Value> = Map::new();
    match table_name.as_str(){
        "company"=>{
            let val =  match values.get("company_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            return match diesel::insert_into(company).values(company_name.eq(val)).get_result::<Company>(connection) {
                Ok(res) => {
                    val_map.insert("name".to_string(),serde_json::from_str(serde_json::to_string(&res.company_name).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "worker_types"=>{
            let val =  match values.get("worker_type").unwrap(){
            Value::String(s)=>{s}
            _=>{panic!("")}
            };
            return match diesel::insert_into(worker_types).values(worker_type.eq(val)).get_result::<WorkerTypes>(connection) {
                Ok(res) => {
                    val_map.insert("name".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_type).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "department"=>{
            let val =  match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            return match diesel::insert_into(department).values(department_name.eq(val)).get_result::<Department>(connection) {
                Ok(res) => {
                    val_map.insert("name".to_string(),serde_json::from_str(serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "group_types"=>{
            let val =  match values.get("group_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            return match diesel::insert_into(group_types).values(group_type.eq(val)).get_result::<GroupTypes>(connection) {
                Ok(res) => {
                    val_map.insert("name".to_string(),serde_json::from_str(serde_json::to_string(&res.group_type).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "equipment_type"=>{
            let val =  match values.get("type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            return match diesel::insert_into(equipment_type).values(type_.eq(val)).get_result::<EquipmentType>(connection) {
                Ok(res) => {
                    val_map.insert("name".to_string(),serde_json::from_str(serde_json::to_string(&res.type_).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "contract"=>{
            let new_cost =  match values.get("cost").unwrap(){
                Value::String(s)=>{Some(s.parse::<i64>().unwrap())}
                _=>{panic!("")}
            };
            let new_start = match values.get("contract_start").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let new_end = match values.get("contract_end").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            use crate::schema::contract::dsl::*;
            return match diesel::insert_into(contract).values((
                cost.eq(new_cost),
                contract_start.eq(new_start),
                contract_end.eq(new_end))
            ).get_result::<Contract>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "eq_group_list"=>{
            return match diesel::insert_into(eq_group_list).default_values().get_result::<EqGroupList>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.eq_list_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "project"=>{
            let new_cost =  match values.get("cost").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let new_json = values.get("data").unwrap();
            use crate::schema::project::dsl::*;
            return match diesel::insert_into(project).values((cost.eq(new_cost),data.eq(new_json))).get_result::<Project>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.project_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "worker"=>{
            let new_firstname = match values.get("firstname").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_secondname = match values.get("secondname"){
                None => {None}
                Some(s) => {s.as_str()}
            };
            let new_familyname = match values.get("familyname").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_age = match values.get("age").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_salary = match values.get("salary").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let new_worker_type = match values.get("worker_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_department_name = match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            use crate::schema::worker::dsl::*;
            return match diesel::insert_into(worker).values((
                firstname.eq(new_firstname),
                secondname.eq(new_secondname),
                familyname.eq(new_familyname),
                age.eq(new_age),
                salary.eq(new_salary),
                worker_type.eq(new_worker_type),
                department_name.eq(new_department_name)
            )).get_result::<Worker>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "constr_attr"=>{
            let new_worker_id = match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_cert_number = match values.get("cert_number").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::constr_attr::dsl::*;
            return match diesel::insert_into(constr_attr).values((worker_id.eq(new_worker_id),cert_number.eq(new_cert_number))).get_result::<ConstrAttr>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "eng_attr"=>{
            let new_worker_id = match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_category = match values.get("category").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::eng_attr::dsl::*;
            return match diesel::insert_into(eng_attr).values((worker_id.eq(new_worker_id),category.eq(new_category))).get_result::<EngAttr>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "lab_attr"=>{
            let new_worker_id = match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_lab_number = match values.get("lab_number").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::lab_attr::dsl::*;
            return match diesel::insert_into(lab_attr).values((worker_id.eq(new_worker_id),lab_number.eq(new_lab_number))).get_result::<LabAttr>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "mech_attr"=>{
            let new_worker_id = match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_repair_type = match values.get("repair_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            use crate::schema::mech_attr::dsl::*;
            return match diesel::insert_into(mech_attr).values((worker_id.eq(new_worker_id),repair_type.eq(new_repair_type))).get_result::<MechAttr>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "equipment"=>{
            let new_name =  match values.get("name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_type_ =  match values.get("type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_department_name =  match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            use crate::schema::equipment::dsl::*;
            return match diesel::insert_into(equipment).values((
                name.eq(new_name),
                type_.eq(new_type_),
                department_name.eq(new_department_name)
            )).get_result::<Equipment>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.eq_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "groups_bind"=>{
            let new_cost =  match values.get("cost").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            let new_group_type =  match values.get("group_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            use crate::schema::groups_bind::dsl::*;
            return match diesel::insert_into(groups_bind).values((
                cost.eq(new_cost),
                group_type.eq(new_group_type)
            )).get_result::<GroupsBind>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "working_company"=>{
            let new_group_id =  match values.get("group_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_company_name =  match values.get("company_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            use crate::schema::working_company::dsl::*;
            return match diesel::insert_into(working_company).values((
                group_id.eq(new_group_id),
                company_name.eq(new_company_name)
            )).get_result::<WorkingCompany>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "department_head"=>{
            let new_worker_id =  match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_department_name =  match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            use crate::schema::department_head::dsl::*;
            return match diesel::insert_into(department_head).values((
                worker_id.eq(new_worker_id),
                department_name.eq(new_department_name)
            )).get_result::<DepartmentHead>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "eq_group"=>{
            let new_eq_id =  match values.get("eq_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_eq_list_id =  match values.get("eq_list_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::eq_group::dsl::*;
            return match diesel::insert_into(eq_group).values((
                eq_id.eq(new_eq_id),
                eq_list_id.eq(new_eq_list_id)
            )).get_result::<EqGroup>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "groups"=>{
            let new_group_id =  match values.get("group_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_worker_id =  match values.get("worker_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::groups::dsl::*;
            return match diesel::insert_into(groups).values((
                group_id.eq(new_group_id),
                worker_id.eq(new_worker_id)
            )).get_result::<Groups>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        "pc_bind"=>{
            let new_contract_id =  match values.get("contract_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_project_id =  match values.get("project_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_group_id =  match values.get("group_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_head_id =  match values.get("head_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            let new_project_start = match values.get("project_start").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let new_project_end = match values.get("project_end").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let new_eq_list_id =  match values.get("eq_list_id").unwrap(){
                Value::String(s)=>{s.parse::<i32>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::pc_bind::dsl::*;
            return match diesel::insert_into(pc_bind).values((
                contract_id.eq(new_contract_id),
                project_id.eq(new_project_id),
                group_id.eq(new_group_id),
                head_id.eq(new_head_id),
                project_start.eq(new_project_start),
                project_end.eq(new_project_end),
                eq_list_id.eq(new_eq_list_id)
            )).get_result::<PcBind>(connection) {
                Ok(res) => {
                    val_map.insert("id".to_string(),serde_json::from_str(serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap());
                    return Value::Object(val_map);
                }
                Err(_) => { Value::Null }
            }
        }
        _ => {return Value::Null }
    }

    return Value::Null;
}

pub fn replace_entries_in_table(connection: &PgConnection, table_name:String, key_name:String, key:String, values:&Value) -> Value {
    let val_arr = values.as_object().unwrap();
    match table_name.as_str(){
        "company"=>{
            diesel::delete(company.filter(company_name.eq(key))).execute(connection);
            let company_names = val_arr.get("company_name").unwrap().as_array().unwrap();
            for val in company_names.iter(){
                match diesel::insert_into(company).values(company_name.eq(val.as_str().unwrap())).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
            }
            return Value::Bool(true);
        }
        "worker_types"=>{
            diesel::delete(worker_types.filter(worker_type.eq(key))).execute(connection);
            let types = val_arr.get("worker_type").unwrap().as_array().unwrap();
            for val in types.iter(){
                match diesel::insert_into(worker_types).values(worker_type.eq(val.as_str().unwrap())).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
            }
            return Value::Bool(true);
        }
        "department"=>{
            diesel::delete(department.filter(department_name.eq(key))).execute(connection);
            let names = val_arr.get("department_name").unwrap().as_array().unwrap();
            for val in names.iter(){
                match diesel::insert_into(department).values(department_name.eq(val.as_str().unwrap())).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
            }
            return Value::Bool(true);
        }
        "group_types"=>{
            diesel::delete(group_types.filter(group_type.eq(key))).execute(connection);
            let types = val_arr.get("group_type").unwrap().as_array().unwrap();
            for val in types.iter(){
                match diesel::insert_into(group_types).values(group_type.eq(val.as_str().unwrap())).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
            }
            return Value::Bool(true);
        }
        "equipment_type"=>{
            diesel::delete(equipment_type.filter(type_.eq(key))).execute(connection);
            let types = val_arr.get("type").unwrap().as_array().unwrap();
            for val in types.iter(){
                match diesel::insert_into(equipment_type).values(type_.eq(val.as_str().unwrap())).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
            }
            return Value::Bool(true);
        }
        "contract"=>{ return Value::Null; }
        "eq_group_list"=>{ return Value::Null; }
        "project"=>{ return Value::Null; }
        "worker"=>{ return Value::Null; }
        "constr_attr"=>{
            use crate::schema::constr_attr::dsl::*;
            diesel::delete(constr_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            let ids = val_arr.get("worker_id").unwrap().as_array().unwrap();
            let cert_numbers = val_arr.get("cert_number").unwrap().as_array().unwrap();
            let mut i=0;
            while i<ids.len(){
                let val1 = ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = cert_numbers.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                match diesel::insert_into(constr_attr).values((worker_id.eq(val1),cert_number.eq(val2))).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "eng_attr"=>{
            use crate::schema::eng_attr::dsl::*;
            diesel::delete(eng_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            let ids = val_arr.get("worker_id").unwrap().as_array().unwrap();
            let categories = val_arr.get("category").unwrap().as_array().unwrap();
            let mut i=0;
            while i<ids.len(){
                let val1 = ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = categories.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                match diesel::insert_into(eng_attr).values((worker_id.eq(val1),category.eq(val2))).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "lab_attr"=>{
            use crate::schema::lab_attr::dsl::*;
            diesel::delete(lab_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            let ids = val_arr.get("worker_id").unwrap().as_array().unwrap();
            let lab_numbers = val_arr.get("lab_number").unwrap().as_array().unwrap();
            let mut i=0;
            while i<ids.len(){
                let val1 = ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = lab_numbers.get(i.clone()).unwrap().as_str().unwrap().parse::<i64>().unwrap();
                match diesel::insert_into(lab_attr).values((worker_id.eq(val1),lab_number.eq(val2))).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "mech_attr"=>{
            use crate::schema::mech_attr::dsl::*;
            diesel::delete(mech_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            let ids = val_arr.get("worker_id").unwrap().as_array().unwrap();
            let repair_types = val_arr.get("repair_type").unwrap().as_array().unwrap();
            let mut i=0;
            while i<ids.len(){
                let val1 = ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = repair_types.get(i.clone()).unwrap().as_str().unwrap();
                match diesel::insert_into(mech_attr).values((worker_id.eq(val1),repair_type.eq(val2))).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "equipment"=>{ return Value::Null; }
        "groups_bind"=>{ return Value::Null; }
        "working_company"=>{ return Value::Null; }
        "department_head"=>{ return Value::Null; }
        "eq_group"=>{
            use crate::schema::eq_group::dsl::*;
            match key_name.as_str(){
                "eq_list_id"=>{
                    diesel::delete(eq_group.filter(eq_list_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                "eq_id"=>{
                    diesel::delete(eq_group.filter(eq_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            let eq_list_ids = val_arr.get("eq_list_id").unwrap().as_array().unwrap();
            let eq_ids = val_arr.get("eq_id").unwrap().as_array().unwrap();
            let mut i=0;
            while i<eq_list_ids.len(){
                let val1 = eq_list_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = eq_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                match diesel::insert_into(eq_group).values((eq_list_id.eq(val1),eq_id.eq(val2))).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "groups"=>{
            use crate::schema::groups::dsl::*;
            match key_name.as_str(){
                "group_id"=>{
                    diesel::delete(groups.filter(group_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                "worker_id"=>{
                    diesel::delete(groups.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            let group_ids = val_arr.get("group_id").unwrap().as_array().unwrap();
            let worker_ids = val_arr.get("worker_id").unwrap().as_array().unwrap();
            let mut i=0;
            while i<group_ids.len(){
                let val1 = group_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = worker_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                match diesel::insert_into(groups).values((group_id.eq(val1),worker_id.eq(val2))).execute(connection){
                    Ok(_) => {}
                    Err(_) => {return Value::Null;}
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "pc_bind"=>{ return Value::Null; }
        _ => {
            return Value::Null; }
    }
    return Value::Null;
}

pub fn serial_add_to_table(connection: &PgConnection, table_name:String,values:&Value)->Value{
    let val_arr = values.as_object().unwrap();
    match table_name.as_str(){
        "company"=>{
            let company_names = val_arr.get("company_name").unwrap().as_array().unwrap();
            for sub_val in company_names.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(company).values(company_name.eq(val)).get_result::<Company>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.company_name).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "worker_types"=>{
            let types = val_arr.get("worker_type").unwrap().as_array().unwrap();
            for sub_val in types.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(worker_types).values(worker_type.eq(val)).get_result::<WorkerTypes>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.worker_type).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "department"=>{
            let names = val_arr.get("department_name").unwrap().as_array().unwrap();
            for sub_val in names.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(department).values(department_name.eq(val)).get_result::<Department>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "group_types"=>{
            let types = val_arr.get("group_type").unwrap().as_array().unwrap();
            for sub_val in types.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(group_types).values(group_type.eq(val)).get_result::<GroupTypes>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.group_type).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "equipment_type"=>{
            let types = val_arr.get("type").unwrap().as_array().unwrap();
            for sub_val in types.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(equipment_type).values(type_.eq(val)).get_result::<EquipmentType>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.type_).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "contract"=>{return Value::Null }
        "eq_group_list"=>{ return Value::Null }
        "project"=>{ return Value::Null }
        "worker"=>{ return Value::Null }
        "constr_attr"=>{return Value::Null }
        "eng_attr"=>{ return Value::Null }
        "lab_attr"=>{ return Value::Null }
        "mech_attr"=>{ return Value::Null }
        "equipment"=>{ return Value::Null }
        "groups_bind"=>{ return Value::Null }
        "working_company"=>{ return Value::Null }
        "department_head"=>{ return Value::Null }
        "eq_group"=>{
            use crate::schema::eq_group::dsl::*;
            let eq_list_ids = val_arr.get("eq_list_id").unwrap().as_array().unwrap();
            let eq_ids = val_arr.get("eq_id").unwrap().as_array().unwrap();
            let mut i=0;
            while i<eq_list_ids.len() {
                let new_eq_id =eq_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let new_eq_list_id = eq_list_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                match diesel::insert_into(eq_group).values((
                    eq_id.eq(new_eq_id),
                    eq_list_id.eq(new_eq_list_id)
                )).get_result::<EqGroup>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.id).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
                i+=1;
            }
            return Value::Bool(true);
        }
        "groups"=>{
            use crate::schema::groups::dsl::*;
            let group_ids = val_arr.get("group_id").unwrap().as_array().unwrap();
            let worker_ids = val_arr.get("worker_id").unwrap().as_array().unwrap();
            let mut i=0;
            while i<group_ids.len() {
            let new_group_id =  group_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
            let new_worker_id =  worker_ids.get(i.clone()).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                match diesel::insert_into(groups).values((
                    group_id.eq(new_group_id),
                    worker_id.eq(new_worker_id)
                )).get_result::<Groups>(connection) {
                    Ok(res) => {
                        serde_json::from_str(serde_json::to_string(&res.id).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "pc_bind"=>{ return Value::Null }
        _ => {return Value::Null }
    }
}

pub fn delete_entry_in_table(connection: &PgConnection, table_name:String, key_name:String, key:String) -> Value {
    return match table_name.as_str() {
        "company" => {
            match diesel::delete(company.filter(company_name.eq(key))).get_result::<Company>(connection) {
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "worker_types" => {
            match diesel::delete(worker_types.filter(worker_type.eq(key))).get_result::<WorkerTypes>(connection) {
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "department" => {
            match diesel::delete(department.filter(department_name.eq(key))).get_result::<Department>(connection) {
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "group_types" => {
            match diesel::delete(group_types.filter(group_type.eq(key))).get_result::<GroupTypes>(connection) {
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "equipment_type" => {
            match diesel::delete(equipment_type.filter(type_.eq(key))).get_result::<EquipmentType>(connection) {
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "contract" => {
            match key_name.as_str() {
                "contract_id" => {
                    match diesel::delete(contract.filter(contract_id.eq(key.parse::<i32>().unwrap()))).get_result::<Contract>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "eq_group_list" => {
            use crate::schema::eq_group_list::dsl::*;
            match key_name.as_str() {
                "eq_list_id" => {
                    match diesel::delete(eq_group_list.filter(eq_list_id.eq(key.parse::<i32>().unwrap()))).get_result::<EqGroupList>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "project" => {
            match key_name.as_str() {
                "contract_id" => {
                    match diesel::delete(project.filter(project_id.eq(key.parse::<i32>().unwrap()))).get_result::<Project>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "worker" => {
            match key_name.as_str() {
                "worker_id" => {
                    match diesel::delete(worker.filter(worker_id.eq(key.parse::<i32>().unwrap()))).get_result::<Worker>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "constr_attr" => {
            use crate::schema::constr_attr::dsl::*;
            match diesel::delete(constr_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).get_result::<ConstrAttr>(connection){
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "eng_attr" => {
            use crate::schema::eng_attr::dsl::*;
            match diesel::delete(eng_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).get_result::<EngAttr>(connection){
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "lab_attr" => {
            use crate::schema::lab_attr::dsl::*;
            match diesel::delete(lab_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).get_result::<LabAttr>(connection){
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "mech_attr" => {
            use crate::schema::mech_attr::dsl::*;
            match diesel::delete(mech_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).get_result::<MechAttr>(connection){
                Ok(_) => { Value::Bool(true) }
                Err(_) => { Value::Bool(false) }
            }
        }
        "equipment" => {
            match key_name.as_str() {
                "eq_id" => {
                    match diesel::delete(equipment.filter(eq_id.eq(key.parse::<i32>().unwrap()))).get_result::<Equipment>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "groups_bind" => {
            match key_name.as_str() {
                "group_id" => {
                    match diesel::delete(groups_bind.filter(group_id.eq(key.parse::<i32>().unwrap()))).get_result::<GroupsBind>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "working_company" => {
            use crate::schema::working_company::dsl::*;
            match key_name.as_str() {
                "group_id" => {
                    match diesel::delete(working_company.filter(group_id.eq(key.parse::<i32>().unwrap()))).get_result::<WorkingCompany>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "department_head" => {
            use crate::schema::department_head::dsl::*;
            match key_name.as_str() {
                "department_name" => {
                    match diesel::delete(department_head.filter(department_name.eq(key))).get_result::<DepartmentHead>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "eq_group" => {
            use crate::schema::eq_group::dsl::*;
            match key_name.as_str() {
                "eq_list_id" => {
                    match diesel::delete(eq_group.filter(eq_list_id.eq(key.parse::<i32>().unwrap()))).get_result::<EqGroup>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                "eq_id" => {
                    match diesel::delete(eq_group.filter(eq_id.eq(key.parse::<i32>().unwrap()))).get_result::<EqGroup>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "groups" => {
            use crate::schema::groups::dsl::*;
            match key_name.as_str() {
                "group_id" => {
                    match diesel::delete(groups.filter(group_id.eq(key.parse::<i32>().unwrap()))).get_result::<Groups>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                "worker_id" => {
                    match diesel::delete(groups.filter(worker_id.eq(key.parse::<i32>().unwrap()))).get_result::<Groups>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        "pc_bind" => {
            use crate::schema::pc_bind::dsl::*;
            match key_name.as_str() {
                "contract_id" => {
                    match diesel::delete(pc_bind.filter(contract_id.eq(key.parse::<i32>().unwrap()))).get_result::<PcBind>(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                "project_id" => {
                    match diesel::delete(pc_bind.filter(project_id.eq(key.parse::<i32>().unwrap()))).execute(connection){
                        Ok(_) => { Value::Bool(true) }
                        Err(_) => { Value::Bool(false) }
                    }
                }
                _ => { Value::Bool(false) }
            }
        }
        _ => { Value::Bool(false) }
    };
}

pub fn process_form(connection: &PgConnection, form_type:String, values:&Value) -> Value{
    let args = values.as_object().unwrap();
    use crate::form_structs::*;
    match form_type.as_str(){
        "AVGSalaryByWorkerType"=>{
            match diesel::sql_query(
                "SELECT worker_type AS type_, CAST(AVG(salary) AS Bigint) AS avg_salary \
                FROM worker \
                GROUP BY worker_type")
                .get_results::<TypedSalary>(connection) {
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(err) => { println!("{}",err.to_string());Value::Bool(false) }
            }
        }
        "contractsAtTime"=>{
            let start = match args.get("timeStart").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let end = match args.get("timeEnd").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            match diesel::sql_query(
                "SELECT contract_id \
                FROM contract \
                WHERE $1 > contract.contract_start AND $2 contract.contract_end \
                OR  $3 > contract.contract_start AND $4 < contract.contract_end ;")
                .bind::<Timestamp,_>(start.clone())
                .bind::<Timestamp,_>(start)
                .bind::<Timestamp,_>(end.clone())
                .bind::<Timestamp,_>(end)
                .get_results::<Ids>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(err) => {println!("{}",err.to_string()); Value::Bool(false) }
            }
        }
        "contractsByProject"=>{
            let pId = args.get("projectId").unwrap().as_str().unwrap();
            let id = match pId.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            match diesel::sql_query(
                "SELECT contract_id as id \
                FROM pc_bind \
                WHERE project_id = $1 ")
                .bind::<Int4,_>(id)
                .get_results::<Ids>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(err) => { println!("{}",err.to_string());Value::Bool(false) }
            }
        }
        "contractsCostByTime"=>{
            let start = match args.get("timeStart").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let end = match args.get("timeEnd").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            match diesel::sql_query(
                "SELECT SUM(cost) as cost_sum
                FROM contract
                WHERE contract_start>= $1
                AND contract_end<= $2 ;")
                .bind::<Timestamp,_>(start)
                .bind::<Timestamp,_>(end)
                .get_results::<CostSum>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(err) => { println!("{}",err.to_string());Value::Bool(false) }
            }
        }
        "contractsEf"=>{
            match diesel::sql_query(
                "SELECT contract.contract_id AS id, \
                CAST(contract.cost/(EXTRACT(DAY from contract.contract_end - contract.contract_start)) AS INTEGER) AS ef\
                FROM contract;")
                .get_results::<Ids>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "departmentHeads"=>{
            match diesel::sql_query(
                "SELECT department.name, worker.worker_id as id, firstname ,secondname,familyname
                FROM department
                JOIN department_head
                ON department.department_name = department_head.department_name
                JOIN worker
                ON department_head.worker_id = worker.worker_id;")
                .get_results::<ExtendedDepHeads>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "departmentStuffByAge"=>{
            let name = args.get("departmentName").unwrap().as_str().unwrap();
            let start_age = args.get("ageStart").unwrap().as_str().unwrap();
            let end_age = args.get("ageEnd").unwrap().as_str().unwrap();
            let start_age_value = match start_age.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            let end_age_value = match end_age.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            match diesel::sql_query(
                "SELECT worker.*
                FROM worker
                JOIN department
                ON worker.department_name = department.department_name
                WHERE department.name = $1
                AND worker.age >= $2
                AND worker.age<= $3 ")
                .bind::<Text,_>(name.clone())
                .bind::<Int4,_>(start_age_value)
                .bind::<Int4,_>(end_age_value)
                .get_results::<Worker>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "departmentStuffByType"=>{
            let name = args.get("departmentName").unwrap().as_str().unwrap();
            let type_name = args.get("workerType").unwrap().as_str().unwrap();
            match diesel::sql_query(
                "SELECT worker.*
                FROM worker
                JOIN department
                ON worker.department_name = department.department_name
                WHERE department.name = $1
                AND worker.worker_type = $2 ")
                .bind::<Text,_>(name.clone())
                .bind::<Text,_>(type_name.clone())
                .get_results::<Worker>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "equipmentByContract"=>{
            let p_id = args.get("contractId").unwrap().as_str().unwrap();
            let id = match p_id.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            match diesel::sql_query(
                "SELECT equipment.*
                FROM (SELECT eq_list_id
                    FROM pc_bind
                    WHERE contract_id = $1) AS sub_res
                JOIN eq_group
                ON eq_group.eq_list_id = sub_res.eq_list_id
                JOIN equipment
                ON equipment.eq_id = eq_group.eq_id;")
                .bind::<Int4,_>(id)
                .get_results::<Equipment>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "equipmentByProject"=>{
            let p_id = args.get("projectId").unwrap().as_str().unwrap();
            let id = match p_id.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            match diesel::sql_query(
                "SELECT equipment.*
                FROM (SELECT eq_list_id, project_start, project_id
                    FROM pc_bind
                    WHERE project_id = $1) AS sub_res
                JOIN eq_group
                ON eq_group.eq_list_id = sub_res.eq_list_id
                JOIN equipment
                ON equipment.eq_id = eq_group.eq_id;")
                .bind::<Int4,_>(id)
                .get_results::<Equipment>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "equipmentPlaceByTime"=>{
            let time = match args.get("time").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            match diesel::sql_query(
                "SELECT equipment.eq_id as id, equipment.name as name, sub_res.contract_id as contract_id, sub_res.project_id as project_id
                FROM (SELECT contract_id, project_id, eq_list_id
                    FROM pc_bind
                    WHERE $1 BETWEEN project_start AND project_end) AS sub_res
                JOIN eq_group
                ON eq_group.eq_list_id = sub_res.eq_list_id
                JOIN equipment
                ON eq_group.eq_id = equipment.eq_id;")
                .bind::<Timestamp,_>(time)
                .get_results::<EquipmentByTime>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "fullContractData"=>{
            match diesel::sql_query(
                "SELECT *
                FROM contract INNER JOIN pc_bind
                ON contract.contract_id = pc_bind.contract_id;")
                .get_results::<FullContractData>(connection) {
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "fullStuffByAge"=>{
            let age_value_start = match args.get("ageStart"){
                None => {return Value::Null}
                Some(res) => {res}
            };
            let age_start = match age_value_start.as_str().unwrap().parse::<i32>(){
                Err(_) => {return Value::Null}
                Ok(res) => {res}
            };
            let age_value_end = match args.get("ageEnd"){
                None => {return Value::Null}
                Some(res) => {res}
            };
            let age_end = match age_value_end.as_str().unwrap().parse::<i32>(){
                Err(_) => {return Value::Null}
                Ok(res) => {res}
            };
            match diesel::sql_query(
                "SELECT *
                FROM worker
                HAVING worker.age >= $1
                AND worker.age <= $2
                ORDER BY worker_id;")
                .bind::<Int4,_>(age_start)
                .bind::<Int4,_>(age_end)
                .get_results::<Worker>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "fullStuffByType"=>{
            let type_name = args.get("workerType").unwrap().as_str().unwrap();
            match diesel::sql_query(
                "SELECT worker.*
                FROM worker
                WHERE worker.worker_type = $1
                ORDER BY firstname;")
                .bind::<Text,_>(type_name.clone())
                .get_results::<Worker>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "PCByEq"=>{
            let id = args.get("eqId").unwrap().as_str().unwrap();
            let id_val = match id.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null}
            };
            match diesel::sql_query(
                "SELECT pc_bind.contract_id as contract_id, pc_bind.project_id as project_id
                FROM (SELECT eq_list_id
                FROM eq_group
                WHERE eq_id = $1 ) AS lists
                JOIN pc_bind
                ON lists.eq_list_id = pc_bind.eq_list_id;")
                .bind::<Int4,_>(id_val.clone())
                .get_results::<ContractProject>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "projectsAtTime"=>{
            let start = match args.get("timeStart").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let end = match args.get("timeEnd").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            match diesel::sql_query(
                "SELECT DISTINCT contract_id as id
                FROM pc_bind
                WHERE ( $1 BETWEEN project_start AND project_end)
                OR ( $2 BETWEEN project_start AND project_end);")
                .bind::<Timestamp,_>(start)
                .bind::<Timestamp,_>(end)
                .get_results::<Ids>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "projectsByContract"=>{
            let id = args.get("contractId").unwrap().as_str().unwrap();
            let id_val = match id.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null}
            };
            match diesel::sql_query(
                "SELECT DISTINCT project_id as id
                FROM pc_bind
                WHERE contract_id = $1 ")
                .bind::<Int4,_>(id_val.clone())
                .get_results::<Ids>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "projectsCostByTime"=>{
            let start = match args.get("timeStart").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let end = match args.get("timeEnd").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            match diesel::sql_query(
                "SELECT SUM(project.cost) as cost_sum
                FROM pc_bind
                JOIN project
                ON project.project_id = pc_bind.project_id
                WHERE project_start>= $1
                AND project_end<= $2 ")
                .bind::<Timestamp,_>(start)
                .bind::<Timestamp,_>(end)
                .get_results::<CostSum>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "projectsWorkersByTime"=>{
            let start = match args.get("timeStart").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let end = match args.get("timeEnd").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            match diesel::sql_query(
                "SELECT DISTINCT worker.*
                FROM (SELECT contract_id, project_id, group_id
                    FROM pc_bind
                    WHERE (project_start BETWEEN $1 AND $2) OR  (project_end BETWEEN $3 AND $4)) AS in_time
                JOIN groups
                ON groups.group_id = in_time.group_id
                JOIN worker
                ON groups.worker_id = worker.worker_id")
                .bind::<Timestamp,_>(&start)
                .bind::<Timestamp,_>(&end)
                .bind::<Timestamp,_>(start)
                .bind::<Timestamp,_>(end)
                .get_results::<Worker>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "workDoneByCompanies"=>{
            match diesel::sql_query(
                "SELECT working_company.company_name as company_name,
                pc_bind.contract_id as contract_id,
                pc_bind.project_id as project_id,
                (groups_bind.cost+project.cost) as cost
                FROM working_company
                JOIN groups_bind
                ON working_company.group_id = groups_bind.group_id
                JOIN pc_bind
                ON pc_bind.group_id = working_company.group_id
                JOIN project
                ON pc_bind.project_id = project.project_id;")
                .get_results::<WorkDone>(connection) {
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "workersContractsByTime"=>{
            let start = match args.get("timeStart").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let end = match args.get("timeEnd").unwrap(){
                Value::String(s)=>{match NaiveDateTime::parse_from_str(s,"%Y-%m-%dT%H:%M"){
                    Ok(res) => {res}
                    Err(e) => {panic!(e.to_string());}
                }}
                _=>{panic!("")}
            };
            let id = args.get("workerId").unwrap().as_str().unwrap();
            let id_val = match id.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            match diesel::sql_query(
                "SELECT sub_res.contract_id
                FROM (SELECT contract_id, group_id
                    FROM pc_bind
                    WHERE project_start<= $1
                    OR project_end >= $2) AS sub_res
                JOIN groups
                ON sub_res.group_id = groups.group_id
                JOIN worker
                ON worker.worker_id = groups.worker_id
                WHERE worker.worker_id = $3 ")
                .bind::<Timestamp,_>(start)
                .bind::<Timestamp,_>(end)
                .bind::<Int4,_>(id_val)
                .get_results::<Ids>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        "workersInContract"=>{
            let p_id = args.get("contractId").unwrap().as_str().unwrap();
            let id = match p_id.parse::<i32>(){
                Ok(res) => {res}
                Err(_) => {return Value::Null;}
            };
            match diesel::sql_query(
                "SELECT DISTINCT worker.*
                FROM (SELECT group_id
                    FROM pc_bind
                    WHERE contract_id = $1 ) AS gr
		        JOIN groups
		        ON groups.group_id = gr.group_id
		        JOIN worker
		        ON groups.worker_id = worker.worker_id;")
                .bind::<Int4,_>(id)
                .get_results::<Worker>(connection){
                Ok(res) => { serde_json::from_str(serde_json::to_string(&res).unwrap().as_str()).unwrap() }
                Err(_) => { Value::Bool(false) }
            }
        }
        _ => {Value::Null}
    }
}
