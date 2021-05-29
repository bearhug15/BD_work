use crate::models::*;
use crate::schema::company::dsl::company;
use crate::schema::worker_types::dsl::worker_types;
use crate::schema::group_types::dsl::group_types;
use crate::schema::department::dsl::department;
use crate::schema::equipment_type::dsl::equipment_type;
use serde_json::Value;
use serde_json::value::Value::Null;

const part_size:usize=100;
pub enum Tables{
    Company(Vec<Company>),
    WorkerTypes(Vec<WorkerTypes>),
    Department(Vec<Department>),
    GroupTypes(Vec<GroupTypes>),
    EquipmentType(Vec<EquipmentType>),
    Contract(Vec<Contract>),
    EqGroupList(Vec<EqGroupList>),
    Project(Vec<Project>),
    Worker(Vec<Worker>),
    ConstrAttr(Vec<ConstrAttr>),
    EngAttr(Vec<EngAttr>),
    LabAttr(Vec<LabAttr>),
    MechAttr(Vec<MechAttr>),
    Equipment(Vec<Equipment>),
    GroupsBind(Vec<GroupsBind>),
    WorkingCompany(Vec<WorkingCompany>),
    DepartmentHead(Vec<DepartmentHead>),
    EqGroup(Vec<EqGroup>),
    Groups(Vec<Groups>),
    PcBind(Vec<PcBind>),
}

impl Tables {
    pub fn get_part(&self, part:usize)->Value{
        match &self {
            Tables::Company (value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() 
            }
            Tables::WorkerTypes(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap()}
            Tables::Department(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::GroupTypes(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap()}
            Tables::EquipmentType(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::Contract(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::EqGroupList(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::Project(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::Worker(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::ConstrAttr(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::EngAttr(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::LabAttr(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::MechAttr(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::Equipment(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::GroupsBind(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::WorkingCompany(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::DepartmentHead(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::EqGroup(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::Groups(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
            Tables::PcBind(value)=>{
                if  value.len()-1<part.clone()*part_size {
                    return Value::Null;
                }
                serde_json::from_str( serde_json::to_string(&value[part*part_size..]).unwrap().as_str()).unwrap() }
        }
        return Value::Null
    }
}

pub enum Forms{
    ContractsAtTime(Vec<i32>),
    ContractsByProject(),
    ContractsCostByTime(),
    ContractsEf(),
    DepartmentHead(),
    DepartmentStuffByAge(),
    DepartmentStuffByType(),
    EquipmentByContracts(),
    EquipmentByProjects(),
    EquipmentPlaceByTime(),
    FullStuffByAge(),
    FullStuffByType(),
    PCByEq(),
    ProjectsAtTime(),
    ProjectsByContract(),
    ProjectsCostByTime(),
    ProjectsWorkersByTime(),
    WorkDoneByCompanies(),
    WorkersContractsByTime(),
    WorkersInContract(),
}