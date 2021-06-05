use diesel::types::*;
use chrono::NaiveDateTime;

#[derive(QueryableByName,Serialize)]
pub(crate) struct TypedSalary {
    #[sql_type = "Text"]
    pub type_: String,
    #[sql_type = "Int8"]
    pub avg_salary: i64,

}
#[derive(QueryableByName,Serialize)]
pub(crate) struct AvgSalary{
    #[sql_type = "Int8"]
    pub avg_salary: i64,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct Ids{
    #[sql_type = "Int4"]
    pub id: i32,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct ContractsEf{
    #[sql_type = "Int4"]
    pub id: i32,
    #[sql_type = "Int4"]
    pub ef: i32,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct ExtendedDepHeads{
    #[sql_type = "Text"]
    pub department_name: String,
    #[sql_type = "Int4"]
    pub id: i32,
    #[sql_type = "Text"]
    pub firstname: String,
    #[sql_type = "Text"]
    pub secondname: String,
    #[sql_type = "Text"]
    pub familyname: String,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct CostSum{
    #[sql_type = "Int4"]
    pub cost_sum: i32,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct DepartmentHeads{
    #[sql_type = "Text"]
    pub department_name: String,
    #[sql_type = "Int4"]
    pub id: i32,
    #[sql_type = "Text"]
    pub firstname: String,
    #[sql_type = "Nullable<Text>"]
    pub secondname: Option<String>,
    #[sql_type = "Text"]
    pub familyname: String,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct Stuff{
    #[sql_type = "Int4"]
    pub id: i32,
    #[sql_type = "Text"]
    pub firstname: String,
    #[sql_type = "Nullable<Text>"]
    pub secondname: Option<String>,
    #[sql_type = "Text"]
    pub familyname: String,
    #[sql_type = "Int4"]
    pub age: i32,
    #[sql_type = "Text"]
    pub worker_type: String,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct EquipmentByTime{
    #[sql_type = "Int4"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Int4"]
    pub contract_id: i32,
    #[sql_type = "Int4"]
    pub project_id: i32,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct FullContractData{
    #[sql_type = "Int4"]
    pub contract_id: i32,
    #[sql_type = "Int8"]
    pub cost: i64,
    #[sql_type = "Timestamp"]
    pub contract_start: NaiveDateTime,
    #[sql_type = "Timestamp"]
    pub contract_end: NaiveDateTime,
    #[sql_type = "Int4"]
    pub project_id: i32,
    #[sql_type = "Int4"]
    pub group_id: i32,
    #[sql_type = "Int4"]
    pub head_id: i32,
    #[sql_type = "Timestamp"]
    pub project_start: NaiveDateTime,
    #[sql_type = "Timestamp"]
    pub project_end: NaiveDateTime,
    #[sql_type = "Int4"]
    pub eq_list_id: i32,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct ContractProject {
    #[sql_type = "Int4"]
    pub contract_id: i32,
    #[sql_type = "Int4"]
    pub project_id: i32,
}
#[derive(QueryableByName,Serialize)]
pub(crate) struct WorkDone {
    #[sql_type = "Text"]
    pub company_name: String,
    #[sql_type = "Int4"]
    pub contract_id: i32,
    #[sql_type = "Int4"]
    pub project_id: i32,
    #[sql_type = "Int4"]
    pub cost: i32,
}