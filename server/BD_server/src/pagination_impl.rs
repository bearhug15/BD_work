use crate::models::{Company, WorkerTypes, Department, GroupTypes, Contract, EqGroupList, EquipmentType, Project, Worker, ConstrAttr, EngAttr, LabAttr, MechAttr, Equipment, GroupsBind, WorkingCompany, DepartmentHead, EqGroup, Groups, PcBind};
use diesel::{PgConnection, QueryDsl, ExpressionMethods, AppearsOnTable, RunQueryDsl};
use diesel::query_builder::QueryFragment;
use diesel::pg::Pg;
use crate::pagination::Paginate;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl Company{
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::company;

        let column = match column_name.as_str(){
            "company_name"=>{company::company_name}
            _=>{company::company_name}
        };
        let mut query = company::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let res = match order.as_str(){
                    "ASC"=>{
                        match query.order_by(column.asc()).paginate(page).load::<(Company, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                    "DESC" | _=>{
                        match query.order_by(column.desc()).paginate(page).load::<(Company, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                };

                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl WorkerTypes{
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::worker_types;

        let column = match column_name.as_str(){
            "worker_type"=>{worker_types::worker_type}
            _=>{worker_types::worker_type}
        };
        let mut query = worker_types::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let res = match order.as_str(){
                    "ASC"=>{
                        match query.order_by(column.asc()).paginate(page).load::<(WorkerTypes, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                    "DESC" | _=>{
                        match query.order_by(column.desc()).paginate(page).load::<(WorkerTypes, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                };

                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl Department{
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::department;

        let column = match column_name.as_str(){
            "department_name"=>{department::department_name}
            _=>{department::department_name}
        };
        let mut query = department::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let res = match order.as_str(){
                    "ASC"=>{
                        match query.order_by(column.asc()).paginate(page).load::<(Department, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                    "DESC" | _=>{
                        match query.order_by(column.desc()).paginate(page).load::<(Department, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                };

                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl GroupTypes{
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::group_types;

        let column = match column_name.as_str(){
            "group_type"=>{group_types::group_type}
            _=>{group_types::group_type}
        };
        let mut query = group_types::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let res = match order.as_str(){
                    "ASC"=>{
                        match query.order_by(column.asc()).paginate(page).load::<(GroupTypes, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                    "DESC" | _=>{
                        match query.order_by(column.desc()).paginate(page).load::<(GroupTypes, i64)>(connection) {
                            Ok(sub_res)=>{sub_res},
                            Err(_)=>{return Err("Error at loading page")},
                        }
                    }
                };

                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl Contract{
    fn sort_by_column<U: 'static>(mut query: crate::schema::contract::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::contract::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::contract::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::contract;

        let mut query = contract::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "contract_id" => Contract::sort_by_column(query, contract::contract_id, order),
                    "contract_start" => Contract::sort_by_column(query, contract::contract_start, order),
                    "contract_end" => Contract::sort_by_column(query, contract::contract_end, order),
                    "cost" => Contract::sort_by_column(query, contract::cost, order),
                    _ => Contract::sort_by_column(query, contract::contract_id, order)
                };
                let res =match query_res.paginate(page).load::<(Contract, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl EqGroupList{
    fn sort_by_column<U: 'static>(mut query: crate::schema::eq_group_list::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::eq_group_list::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::eq_group_list::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::eq_group_list;

        let mut query = eq_group_list::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "eq_list_id" => EqGroupList::sort_by_column(query, eq_group_list::eq_list_id, order),
                    _ => EqGroupList::sort_by_column(query, eq_group_list::eq_list_id, order)
                };
                let res =match query_res.paginate(page).load::<(EqGroupList, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl EquipmentType{
    fn sort_by_column<U: 'static>(mut query: crate::schema::equipment_type::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::equipment_type::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::equipment_type::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::equipment_type;

        let mut query = equipment_type::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "type" => EquipmentType::sort_by_column(query, equipment_type::type_, order),
                    _ => EquipmentType::sort_by_column(query, equipment_type::type_, order)
                };
                let res =match query_res.paginate(page).load::<(EquipmentType, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl Project{
    fn sort_by_column<U: 'static>(mut query: crate::schema::project::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::project::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::project::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::project;

        let mut query = project::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "project_id" => Project::sort_by_column(query, project::project_id, order),
                    "cost" => Project::sort_by_column(query, project::cost, order),
                    "data" => Project::sort_by_column(query, project::data, order),
                    _ => Project::sort_by_column(query, project::project_id, order)
                };
                let res =match query_res.paginate(page).load::<(Project, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl Worker{
    fn sort_by_column<U: 'static>(mut query: crate::schema::worker::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::worker::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::worker::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::worker;

        let mut query = worker::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "worker_id" => Worker::sort_by_column(query, worker::worker_id, order),
                    "firstname" => Worker::sort_by_column(query, worker::firstname, order),
                    "secondname" => Worker::sort_by_column(query, worker::secondname, order),
                    "familyname" => Worker::sort_by_column(query, worker::familyname, order),
                    "worker_type" => Worker::sort_by_column(query, worker::worker_type, order),
                    "department_name" => Worker::sort_by_column(query, worker::department_name, order),
                    "age" => Worker::sort_by_column(query, worker::age, order),
                    "salary" => Worker::sort_by_column(query, worker::salary, order),
                    _ => Worker::sort_by_column(query, worker::worker_id, order)
                };
                let res =match query_res.paginate(page).load::<(Worker, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl ConstrAttr{
    fn sort_by_column<U: 'static>(mut query: crate::schema::constr_attr::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::constr_attr::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::constr_attr::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::constr_attr;

        let mut query = constr_attr::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "worker_id" => ConstrAttr::sort_by_column(query, constr_attr::worker_id, order),
                    "cert_number" => ConstrAttr::sort_by_column(query, constr_attr::cert_number, order),
                    _ => ConstrAttr::sort_by_column(query, constr_attr::worker_id, order)
                };
                let res =match query_res.paginate(page).load::<(ConstrAttr, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl EngAttr{
    fn sort_by_column<U: 'static>(mut query: crate::schema::eng_attr::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::eng_attr::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::eng_attr::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::eng_attr;

        let mut query = eng_attr::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "worker_id" => EngAttr::sort_by_column(query, eng_attr::worker_id, order),
                    "category" => EngAttr::sort_by_column(query, eng_attr::category, order),
                    _ => EngAttr::sort_by_column(query, eng_attr::worker_id, order)
                };
                let res =match query_res.paginate(page).load::<(EngAttr, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl LabAttr{
    fn sort_by_column<U: 'static>(mut query: crate::schema::lab_attr::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::lab_attr::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::lab_attr::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::lab_attr;

        let mut query = lab_attr::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "worker_id" => LabAttr::sort_by_column(query, lab_attr::worker_id, order),
                    "lab_number" => LabAttr::sort_by_column(query, lab_attr::lab_number, order),
                    _ => LabAttr::sort_by_column(query, lab_attr::worker_id, order)
                };
                let res =match query_res.paginate(page).load::<(LabAttr, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl MechAttr{
    fn sort_by_column<U: 'static>(mut query: crate::schema::mech_attr::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::mech_attr::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::mech_attr::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::mech_attr;

        let mut query = mech_attr::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "worker_id" => MechAttr::sort_by_column(query, mech_attr::worker_id, order),
                    "repair_type" => MechAttr::sort_by_column(query, mech_attr::repair_type, order),
                    _ => MechAttr::sort_by_column(query, mech_attr::worker_id, order)
                };
                let res =match query_res.paginate(page).load::<(MechAttr, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl Equipment{
    fn sort_by_column<U: 'static>(mut query: crate::schema::equipment::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::equipment::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::equipment::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::equipment;

        let mut query = equipment::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "eq_id" => Equipment::sort_by_column(query, equipment::eq_id, order),
                    "name" => Equipment::sort_by_column(query, equipment::name, order),
                    "type" => Equipment::sort_by_column(query, equipment::type_, order),
                    "department_name" => Equipment::sort_by_column(query, equipment::department_name, order),
                    _ => Equipment::sort_by_column(query, equipment::eq_id, order)
                };
                let res =match query_res.paginate(page).load::<(Equipment, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl GroupsBind{
    fn sort_by_column<U: 'static>(mut query: crate::schema::groups_bind::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::groups_bind::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::groups_bind::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::groups_bind;

        let mut query = groups_bind::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "group_id" => GroupsBind::sort_by_column(query, groups_bind::group_id, order),
                    "cost" => GroupsBind::sort_by_column(query, groups_bind::cost, order),
                    "group_type" => GroupsBind::sort_by_column(query, groups_bind::group_type, order),
                    _ => GroupsBind::sort_by_column(query, groups_bind::group_id, order)
                };
                let res =match query_res.paginate(page).load::<(GroupsBind, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl WorkingCompany{
    fn sort_by_column<U: 'static>(mut query: crate::schema::working_company::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::working_company::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::working_company::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::working_company;

        let mut query = working_company::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "group_id" => WorkingCompany::sort_by_column(query, working_company::group_id, order),
                    "company_name" => WorkingCompany::sort_by_column(query, working_company::company_name, order),
                    _ => WorkingCompany::sort_by_column(query, working_company::group_id, order)
                };
                let res =match query_res.paginate(page).load::<(WorkingCompany, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl DepartmentHead{
    fn sort_by_column<U: 'static>(mut query: crate::schema::department_head::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::department_head::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::department_head::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::department_head;

        let mut query = department_head::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "department_name" => DepartmentHead::sort_by_column(query, department_head::department_name, order),
                    "worker_id" => DepartmentHead::sort_by_column(query, department_head::worker_id, order),
                    _ => DepartmentHead::sort_by_column(query, department_head::department_name, order)
                };
                let res =match query_res.paginate(page).load::<(DepartmentHead, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl EqGroup{
    fn sort_by_column<U: 'static>(mut query: crate::schema::eq_group::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::eq_group::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::eq_group::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::eq_group;

        let mut query = eq_group::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "id" => EqGroup::sort_by_column(query, eq_group::id, order),
                    "eq_list_id" => EqGroup::sort_by_column(query, eq_group::eq_list_id, order),
                    "eq_id" => EqGroup::sort_by_column(query, eq_group::eq_id, order),
                    _ => EqGroup::sort_by_column(query, eq_group::eq_list_id, order)
                };
                let res =match query_res.paginate(page).load::<(EqGroup, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl Groups{
    fn sort_by_column<U: 'static>(mut query: crate::schema::groups::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::groups::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::groups::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::groups;

        let mut query = groups::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "id" => Groups::sort_by_column(query, groups::id, order),
                    "group_id" => Groups::sort_by_column(query, groups::group_id, order),
                    "worker_id" => Groups::sort_by_column(query, groups::worker_id, order),
                    _ => Groups::sort_by_column(query, groups::group_id, order)
                };
                let res =match query_res.paginate(page).load::<(Groups, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}
impl PcBind{
    fn sort_by_column<U: 'static>(mut query: crate::schema::pc_bind::BoxedQuery<'static, Pg>,
                                  column: U,
                                  sort_dir: String) -> crate::schema::pc_bind::BoxedQuery<'static, Pg>
        where U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::pc_bind::table>{
        match sort_dir.as_str() {
            "ASC" => query.order_by(column.asc()),
            "DESC" => query.order_by(column.desc()),
            _ => query.order_by(column.desc())
        }
    }
    pub fn get_page(params: Params, connection: &PgConnection,column_name:String,order:String)->Result<(Vec<Self>, i64),&'static str>{
        use crate::schema::pc_bind;

        let mut query = pc_bind::table.into_boxed();
        let (values,total_pages) = match params.page{
            None => { (
                match query.load(connection) {
                    Ok(res)=>{res}
                    Err(_)=>{return Err("Error at loading table")}
                },
                1)}
            Some(page) => {
                let query_res = match column_name.as_str()  {
                    "contract_id" => PcBind::sort_by_column(query, pc_bind::contract_id, order),
                    "project_id" => PcBind::sort_by_column(query, pc_bind::project_id, order),
                    "head_id" => PcBind::sort_by_column(query, pc_bind::group_id, order),
                    "group_id" => PcBind::sort_by_column(query, pc_bind::head_id, order),
                    "project_start" => PcBind::sort_by_column(query, pc_bind::project_start, order),
                    "project_end" => PcBind::sort_by_column(query, pc_bind::project_end, order),
                    "eq_list_id" => PcBind::sort_by_column(query, pc_bind::eq_list_id, order),
                    _ => PcBind::sort_by_column(query, pc_bind::contract_id, order)
                };
                let res =match query_res.paginate(page).load::<(PcBind, i64)>(connection) {
                    Ok(sub_res)=>{sub_res},
                    Err(_)=>{return Err("Error at loading page")},
                };
                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let values = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;

                (values, total_pages)
            }
        };
        Ok((values,total_pages))
    }
}