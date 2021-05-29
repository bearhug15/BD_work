use diesel::{PgConnection, RunQueryDsl, QuerySource, BoolExpressionMethods};
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
use chrono::NaiveDateTime;
use crate::schema::worker::columns::worker_id;
use crate::schema::equipment::columns::eq_id;
use crate::schema::groups_bind::columns::group_id;
use crate::schema::department_head::dsl::department_head;
use crate::schema::pc_bind::columns::{head_id, eq_list_id};
use crate::schema::mech_attr::columns::repair_type;
use chrono::format::Numeric::Timestamp;
//use crate::containers::Tables::{equipment_type, department, group_types, worker_types, company};

const BUCKET_SIZE: i32 = 100;
macro_rules! load_value{
    ($name:ident,$table_struct:ident,$loc_table:ident,$dest:literal,$connection:ident) =>{
        * $loc_table.get($dest).unwrap().lock().unwrap() = Box::new(Some(Tables::$table_struct(match $name.load::<$table_struct>($connection){
                    Ok(res) => {res}
                    Err(_) => {return Value::Null}
                })));
    };
}

pub fn process_init_ask(connection: &PgConnection, tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>) ->Box<Value>{
    let mut loc_tables =tables.read().unwrap();

    let mut loc_company_table =  loc_tables.get("company").unwrap().lock().unwrap();
    let mut company_value:Value;
    match &mut **loc_company_table{
        None => {
            let company_table = match company.load::<Company>(connection){
                Ok(res) => {res}
                Err(_) => {return Box::new(Value::Null)}
            };
            company_value =serde_json::from_str(serde_json::to_string(&company_table).unwrap().as_str()).unwrap();
            *loc_company_table = Box::new(Some(Tables::Company(company_table)));
        }
        Some(sub_company) => {
            match sub_company {
                Tables::Company(buff) => {
                    company_value =serde_json::from_str(serde_json::to_string(buff).unwrap().as_str()).unwrap();
                }
                _ => {panic!("Wrong table")}
            }
        }
    }

    let mut loc_worker_types_table = loc_tables.get("worker_types").unwrap().lock().unwrap();
    let mut worker_types_value: Value;
    match &mut **loc_worker_types_table{
        None => {
            let worker_types_table = match worker_types.load::<WorkerTypes>(connection){
                Ok(res) => {res}
                Err(_) => {return Box::new(Value::Null)}
            };
            worker_types_value = serde_json::from_str(serde_json::to_string(&worker_types_table).unwrap().as_str()).unwrap();
            *loc_worker_types_table = Box::new(Some(Tables::WorkerTypes(worker_types_table)));
        }
        Some(sub_worker_types) => {
            match sub_worker_types {
                Tables::WorkerTypes(buff) => {
                    worker_types_value =serde_json::from_str(serde_json::to_string(buff).unwrap().as_str()).unwrap();
                }
                _ => {panic!("Wrong table")}
            }
        }
    }

    let mut loc_group_types_table= loc_tables.get("group_types").unwrap().lock().unwrap();
    let mut group_types_value: Value;
    match &mut **loc_group_types_table{
        None => {
            let group_types_table = match group_types.load::<GroupTypes>(connection){
                Ok(res) => {res}
                Err(_) => {return Box::new(Value::Null)}
            };
            group_types_value = serde_json::from_str(serde_json::to_string(&group_types_table).unwrap().as_str()).unwrap();
            *loc_group_types_table = Box::new(Some(Tables::GroupTypes(group_types_table)));
        }
        Some(sub_group_types) => {
            match sub_group_types {
                Tables::GroupTypes(buff) => {
                    group_types_value =serde_json::from_str(serde_json::to_string(buff).unwrap().as_str()).unwrap();
                }
                _ => {panic!("Wrong table")}
            }
        }
    }

    let mut loc_department_table= loc_tables.get("department").unwrap().lock().unwrap();
    let mut department_value: Value;
    match &mut **loc_department_table{
        None => {
            let department_table = match department.load::<Department>(connection){
                Ok(res) => {res}
                Err(_) => {return Box::new(Value::Null)}
            };
            department_value = serde_json::from_str(serde_json::to_string(&department_table).unwrap().as_str()).unwrap();
            *loc_department_table = Box::new(Some(Tables::Department(department_table)));
        }
        Some(sub_department_types) => {
            match sub_department_types {
                Tables::Department(buff) => {
                    department_value =serde_json::from_str(serde_json::to_string(buff).unwrap().as_str()).unwrap();
                }
                _ => {panic!("Wrong table")}
            }
        }
    }

    let mut loc_equipment_type_table= loc_tables.get("department").unwrap().lock().unwrap();
    let mut equipment_type_value: Value;
    match &mut **loc_equipment_type_table{
        None => {
            let equipment_type_table = match equipment_type.load::<EquipmentType>(connection){
                Ok(res) => {res}
                Err(_) => {return Box::new(Value::Null)}
            };
            equipment_type_value = serde_json::from_str(serde_json::to_string(&equipment_type_table).unwrap().as_str()).unwrap();
            *loc_equipment_type_table = Box::new(Some(Tables::EquipmentType(equipment_type_table)));
        }
        Some(sub_equipment_type_types) => {
            match sub_equipment_type_types {
                Tables::EquipmentType(buff) => {
                    equipment_type_value =serde_json::from_str(serde_json::to_string(buff).unwrap().as_str()).unwrap();
                }
                _ => {panic!("Wrong table")}
            }
        }
    }

    let mut valMap:Map<String,Value> = Map::new();
    valMap.insert("company".to_string(), company_value);
    valMap.insert("worker_types".to_string(),worker_types_value);
    valMap.insert("group_types".to_string(),group_types_value);
    valMap.insert("department".to_string(),department_value);
    valMap.insert("equipment".to_string(),equipment_type_value);
    let val = Value::Object(valMap);

    return Box::new(val);
}
pub fn read_from_table(connection: &PgConnection, table_name:String, part: usize,tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>)->Value{
    let loc_table = tables.read().unwrap();
    let mut loc_named_table = loc_table.get(table_name.as_str()).unwrap().lock().unwrap();
    if(**loc_named_table).is_none(){
        match table_name.as_str(){
            "company"=>{ load_value!(company,Company,loc_table,"company",connection); }
            "worker_types"=>{ load_value!(worker_types,WorkerTypes,loc_table,"worker_types",connection); }
            "department"=>{ load_value!(department,Department,loc_table,"department",connection); }
            "group_types"=>{ load_value!(group_types,GroupTypes,loc_table,"group_types",connection); }
            "equipment_type"=>{ load_value!(equipment_type,EquipmentType,loc_table,"equipment_type",connection);}
            "contract"=>{ load_value!(contract,Contract,loc_table,"contract",connection); }
            "eq_group_list"=>{ load_value!(eq_group_list,EqGroupList,loc_table,"eq_group_list",connection); }
            "project"=>{ load_value!(project,Project,loc_table,"project",connection); }
            "worker"=>{ load_value!(worker,Worker,loc_table,"worker",connection); }
            "constr_attr"=>{ load_value!(constr_attr,ConstrAttr,loc_table,"constr_attr",connection); }
            "eng_attr"=>{ load_value!(eng_attr,EngAttr,loc_table,"eng_attr",connection); }
            "lab_attr"=>{ load_value!(lab_attr,LabAttr,loc_table,"lab_attr",connection); }
            "mech_attr"=>{ load_value!(mech_attr,MechAttr,loc_table,"mech_attr",connection);}
            "equipment"=>{ load_value!(equipment,Equipment,loc_table,"equipment",connection); }
            "groups_bind"=>{ load_value!(groups_bind,GroupsBind,loc_table,"groups_bind",connection); }
            "working_company"=>{ load_value!(working_company,WorkingCompany,loc_table,"working_company",connection); }
            "department_head"=>{ load_value!(department_head,DepartmentHead,loc_table,"department_head",connection); }
            "eq_group"=>{load_value!(eq_group,EqGroup,loc_table,"eq_group",connection); }
            "groups"=>{load_value!(groups,Groups,loc_table,"groups",connection); }
            "pc_bind"=>{load_value!(pc_bind,PcBind,loc_table,"pc_bind",connection); }
            _ => {return Value::Null }
        };
    }
    (loc_named_table.as_ref().as_ref()).unwrap().get_part(part)
    //return Value::Null
}
macro_rules! string_key_update{
    ($table_name:ident,$field_name:ident,$key:ident,$values:ident)=>{
        let val =  match $values.get(stringify!($field_name)).unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = $table_name.filter($field_name.eq($key));
            diesel::update(target).set($field_name.eq(val));
    };
}
pub fn update_table(connection: &PgConnection, table_name:String, key:String,values:&Value,tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>)->Value{
    let loc_table = tables.read().unwrap();
    let mut loc_named_table = loc_table.get(table_name.as_str()).unwrap().lock().unwrap();
    match table_name.as_str() {
        "company" => {
            let val =  match values.get("company_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = company.filter(company_name.eq(key));
            let q =diesel::update(target).set(company_name.eq(val));
            let res =q.get_result::<Company>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.company_name).unwrap().as_str()).unwrap();
        }
        "worker_types" =>{
            let val =  match values.get("worker_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = worker_types.filter(worker_type.eq(key));
            let q =diesel::update(target).set(worker_type.eq(val));
            let res =q.get_result::<WorkerTypes>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.worker_type).unwrap().as_str()).unwrap();
        }
        "department" => {
            let val =  match values.get("department_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = department.filter(department_name.eq(key));
            let q =diesel::update(target).set(department_name.eq(val));
            let res =q.get_result::<Department>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap();
        }
        "group_types" =>{
            let val =  match values.get("group_type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = group_types.filter(group_type.eq(key));
            let q= diesel::update(target).set(group_type.eq(val));
            let res =q.get_result::<GroupTypes>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.group_type).unwrap().as_str()).unwrap();
        }
        "equipment_type" => {
            let val =  match values.get("type").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let target = equipment_type.filter(type_.eq(key));
            let q=diesel::update(target).set(type_.eq(val));
            let res =q.get_result::<EquipmentType>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.type_).unwrap().as_str()).unwrap();
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
            let target = contract.filter(contract_id.eq(old_id));
            let new_val =Contract{
                contract_id: old_id,
                cost: Some(new_cost),
                contract_start: new_start,
                contract_end: new_end
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Contract>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap();
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
            let target = project.filter(project_id.eq(&old_id));
            let new_val =Project{
                project_id: old_id,
                cost: Some(new_cost)
            };
            let q=diesel::update(target).set(&new_val);
            let res = q.get_result::<Project>(connection).expect("Error updating");
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.project_id).unwrap().as_str()).unwrap();
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
            let target = worker.filter(worker_id.eq(old_id));
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.eq_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.id).unwrap().as_str()).unwrap();
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
            *loc_named_table =Box::new(None);
            return serde_json::from_str( serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap();
        }
        _ => {}
    }
    return Value::Null;
}

pub fn add_to_table(connection: &PgConnection, table_name:String,values:&Value,tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>)->Value{
    let loc_table = tables.read().unwrap();
    let mut loc_named_table = loc_table.get(table_name.as_str()).unwrap().lock().unwrap();

    match table_name.as_str(){
        "company"=>{
            let val =  match values.get("company_name").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            return match diesel::insert_into(company).values(company_name.eq(val)).get_result::<Company>(connection) {
                Ok(res) => {
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.company_name).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_type).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.department_name).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.group_type).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.type_).unwrap().as_str()).unwrap()
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
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            let new_end = match values.get("contract_end").unwrap(){
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::contract::dsl::*;
            return match diesel::insert_into(contract).values((
                cost.eq(new_cost),
                contract_start.eq(new_start),
                contract_end.eq(new_end))
            ).get_result::<Contract>(connection) {
                Ok(res) => {
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap()
                }
                Err(_) => { Value::Null }
            }
        }
        "eq_group_list"=>{
            return match diesel::insert_into(eq_group_list).default_values().get_result::<EqGroupList>(connection) {
                Ok(res) => {
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.eq_list_id).unwrap().as_str()).unwrap()
                }
                Err(_) => { Value::Null }
            }
        }
        "project"=>{
            let new_cost =  match values.get("cost").unwrap(){
                Value::String(s)=>{s.parse::<i64>().unwrap()}
                _=>{panic!("")}
            };
            use crate::schema::project::dsl::*;
            return match diesel::insert_into(project).values(cost.eq(new_cost)).get_result::<Project>(connection) {
                Ok(res) => {
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.project_id).unwrap().as_str()).unwrap()
                }
                Err(_) => { Value::Null }
            }
        }
        "worker"=>{
            let new_firstname = match values.get("firstname").unwrap(){
                Value::String(s)=>{s}
                _=>{panic!("")}
            };
            let new_secondname = match values.get("secondname").unwrap(){
                Value::String(s)=>{Some(s)}
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.eq_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.group_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.worker_id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.id).unwrap().as_str()).unwrap()
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.id).unwrap().as_str()).unwrap()
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
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
                _=>{panic!("")}
            };
            let new_project_end = match values.get("project_end").unwrap(){
                Value::String(s)=>{s.parse::<NaiveDateTime>().unwrap()}
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
                    *loc_named_table = Box::new(None);
                    serde_json::from_str(serde_json::to_string(&res.contract_id).unwrap().as_str()).unwrap()
                }
                Err(_) => { Value::Null }
            }
        }
        _ => {return Value::Null }
    }

    return Value::Null;
}

pub fn replace_entries_in_table(connection: &PgConnection, table_name:String, key_name:String, key:String, values:&Value, tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>) -> Value {
    let loc_table = tables.read().unwrap();
    let mut loc_named_table = loc_table.get(table_name.as_str()).unwrap().lock().unwrap();
    let val_arr = values.as_object().unwrap();
    match table_name.as_str(){
        "company"=>{
            diesel::delete(company.filter(company_name.eq(key))).execute(connection);
            let company_names = val_arr.get("company_name").unwrap().as_array().unwrap();
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
            while i<ids.len(){
                let val1 = ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = cert_numbers.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
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
            *loc_named_table =Box::new(None);
            while i<ids.len(){
                let val1 = ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = categories.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
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
            *loc_named_table =Box::new(None);
            while i<ids.len(){
                let val1 = ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = lab_numbers.get(i).unwrap().as_str().unwrap().parse::<i64>().unwrap();
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
            *loc_named_table =Box::new(None);
            while i<ids.len(){
                let val1 = ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = repair_types.get(i).unwrap().as_str().unwrap();
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
            *loc_named_table =Box::new(None);
            while i<eq_list_ids.len(){
                let val1 = eq_list_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = eq_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
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
            *loc_named_table =Box::new(None);
            while i<group_ids.len(){
                let val1 = group_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let val2 = worker_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
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

pub fn serial_add_to_table(connection: &PgConnection, table_name:String,values:&Value,tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>)->Value{
    let loc_table = tables.read().unwrap();
    let mut loc_named_table = loc_table.get(table_name.as_str()).unwrap().lock().unwrap();
    let val_arr = values.as_object().unwrap();
    match table_name.as_str(){
        "company"=>{
            let company_names = val_arr.get("company_name").unwrap().as_array().unwrap();
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
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
            *loc_named_table =Box::new(None);
            for sub_val in types.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(group_types).values(group_type.eq(val)).get_result::<GroupTypes>(connection) {
                    Ok(res) => {
                        *loc_named_table = Box::new(None);
                        serde_json::from_str(serde_json::to_string(&res.group_type).unwrap().as_str()).unwrap()
                    }
                    Err(_) => { return Value::Null }
                }
            }
            return Value::Bool(true)
        }
        "equipment_type"=>{
            let types = val_arr.get("type").unwrap().as_array().unwrap();
            *loc_named_table =Box::new(None);
            for sub_val in types.iter() {
                let val =sub_val.as_str().unwrap();
                match diesel::insert_into(equipment_type).values(type_.eq(val)).get_result::<EquipmentType>(connection) {
                    Ok(res) => {
                        *loc_named_table = Box::new(None);
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
            *loc_named_table =Box::new(None);
            while i<eq_list_ids.len() {
                let new_eq_id =eq_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
                let new_eq_list_id = eq_list_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
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
            *loc_named_table =Box::new(None);
            while i<group_ids.len() {
            let new_group_id =  group_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
            let new_worker_id =  worker_ids.get(i).unwrap().as_str().unwrap().parse::<i32>().unwrap();
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

pub fn delete_entry_in_table(connection: &PgConnection, table_name:String, key_name:String, key:String, tables: &Arc<RwLock<HashMap<String,Mutex<Box<Option<Tables>>>>>>) -> Value {
    let loc_table = tables.read().unwrap();
    let mut loc_named_table = loc_table.get(table_name.as_str()).unwrap().lock().unwrap();
    match table_name.as_str(){
        "company"=>{
            diesel::delete(company.filter(company_name.eq(key))).execute(connection);
            return Value::Bool(true);
        }
        "worker_types"=>{
            diesel::delete(worker_types.filter(worker_type.eq(key))).execute(connection);
            return Value::Bool(true);
        }
        "department"=>{
            diesel::delete(department.filter(department_name.eq(key))).execute(connection);
            return Value::Bool(true);
        }
        "group_types"=>{
            diesel::delete(group_types.filter(group_type.eq(key))).execute(connection);

            return Value::Bool(true);
        }
        "equipment_type"=>{
            diesel::delete(equipment_type.filter(type_.eq(key))).execute(connection);
            return Value::Bool(true);
        }
        "contract"=>{
            match key_name.as_str(){
                "contract_id"=>{
                    diesel::delete(contract.filter(contract_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "eq_group_list"=>{
            use crate::schema::eq_group_list::dsl::*;
            match key_name.as_str(){
                "eq_list_id"=>{
                    diesel::delete(eq_group_list.filter(eq_list_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "project"=>{
            match key_name.as_str(){
                "contract_id"=>{
                    diesel::delete(project.filter(project_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
            _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "worker"=>{
            match key_name.as_str(){
                "worker_id"=>{
                    diesel::delete(worker.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "constr_attr"=>{
            use crate::schema::constr_attr::dsl::*;
            diesel::delete(constr_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            return Value::Bool(true);
        }
        "eng_attr"=>{
            use crate::schema::eng_attr::dsl::*;
            diesel::delete(eng_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            return Value::Bool(true);
        }
        "lab_attr"=>{
            use crate::schema::lab_attr::dsl::*;
            diesel::delete(lab_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            return Value::Bool(true);
        }
        "mech_attr"=>{
            use crate::schema::mech_attr::dsl::*;
            diesel::delete(mech_attr.filter(worker_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
            return Value::Bool(true);
        }
        "equipment"=>{
            match key_name.as_str(){
                "eq_id"=>{
                    diesel::delete(equipment.filter(eq_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "groups_bind"=>{
            match key_name.as_str(){
                "group_id"=>{
                    diesel::delete(groups_bind.filter(group_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "working_company"=>{
            use crate::schema::working_company::dsl::*;
            match key_name.as_str(){
                "group_id"=>{
                    diesel::delete(working_company.filter(group_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        "department_head"=>{
            use crate::schema::department_head::dsl::*;
            match key_name.as_str(){
                "department_name"=>{
                    diesel::delete(department_head.filter(department_name.eq(key))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
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
            return Value::Bool(true);
        }
        "pc_bind"=>{
            use crate::schema::pc_bind::dsl::*;
            match key_name.as_str(){
                "contract_id"=>{
                    diesel::delete(pc_bind.filter(contract_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                "project_id"=>{
                    diesel::delete(pc_bind.filter(project_id.eq(key.parse::<i32>().unwrap()))).execute(connection);
                }
                _ => {return Value::Null;}
            }
            return Value::Bool(true);
        }
        _ => { return Value::Null; }
    }
    return Value::Null;
}

pub fn get_form_data(connection: &PgConnection, form_type:String,sub_values:Value)->Value{
    let values = sub_values.as_object().unwrap();
    match form_type.as_str(){
        "contractsAtTime"=>{

            //use crate::schema::contract::dsl::*;
            //diesel::select(contract).filter(start_time.between(contract_start,contract_end));
            return Value::Null;
        }
        "contractsByProject"=>{
            return Value::Null;
        }
        "contractsCostByTime"=>{
            return Value::Null;
        }
        "contractsEf"=>{
            return Value::Null;
        }
        "departmentHeads"=>{
            return Value::Null;
        }
        "departmentStuffByAge"=>{
            return Value::Null;
        }
        "departmentStuffByType"=>{
            return Value::Null;
        }
        "equipmentByContract"=>{
            return Value::Null;
        }
        "equipmentByProject"=>{
            return Value::Null;
        }
        "equipmentPlaceByTime"=>{
            return Value::Null;
        }
        "fullStuffByAge"=>{
            return Value::Null;
        }
        "fullStuffByType"=>{
            return Value::Null;
        }
        "PCByEq"=>{
            return Value::Null;
        }
        "projectAtTime"=>{
            return Value::Null;
        }
        "projectsByContract"=>{
            return Value::Null;
        }
        "projectsCostByTime"=>{
            return Value::Null;
        }
        "projectsWorkersByTime"=>{
            return Value::Null;
        }
        "workDoneByCompanies"=>{
            return Value::Null;
        }
        "workersContractsByTime"=>{
            return Value::Null;
        }
        "workersInContract"=>{
            return Value::Null;
        }
        _ => {return Value::Null;}
    }
    return Value::Null;
}