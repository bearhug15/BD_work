table!{
    company (company_name){
        company_name -> Text,
    }
}
table! {
    constr_attr (worker_id) {
        worker_id -> Int4,
        cert_number -> Int4,
    }
}

table! {
    contract (contract_id) {
        contract_id -> Int4,
        cost -> Nullable<Int8>,
        contract_start -> Timestamp,
        contract_end -> Timestamp,
    }
}

table! {
    department (department_name) {
        department_name -> Text,
    }
}

table! {
    department_head (department_name) {
        department_name -> Text,
        worker_id -> Int4,
    }
}

table! {
    eng_attr (worker_id) {
        worker_id -> Int4,
        category -> Int4,
    }
}

table! {
    eq_group_list (eq_list_id) {
        eq_list_id -> Int4,
    }
}

table! {
    equipment (eq_id) {
        eq_id -> Int4,
        name -> Text,
        department_name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

table! {
    equipment_type (type_) {
        #[sql_name = "type"]
        type_ -> Text,
    }
}

table! {
    group_types (group_type) {
        group_type -> Text,
    }
}

table! {
    groups_bind (group_id) {
        group_id -> Int4,
        cost -> Int8,
        group_type -> Text,
    }
}

table! {
    lab_attr (worker_id) {
        worker_id -> Int4,
        lab_number -> Int8,
    }
}

table! {
    mech_attr (worker_id) {
        worker_id -> Int4,
        repair_type -> Text,
    }
}

table! {
    project (project_id) {
        project_id -> Int4,
        cost -> Nullable<Int8>,
    }
}

table! {
    worker (worker_id) {
        worker_id -> Int4,
        firstname -> Text,
        secondname -> Nullable<Text>,
        familyname -> Text,
        age -> Int4,
        salary -> Int8,
        department_name -> Text,
        worker_type -> Text,
    }
}

table! {
    worker_types (worker_type) {
        worker_type -> Text,
    }
}

table! {
    working_company (group_id) {
        group_id -> Int4,
        company_name -> Text,
    }
}

table! {
    eq_group (id ){
        id -> Int4,
        eq_list_id -> Int4,
        eq_id -> Int4,
    }
}

table! {
    groups (id) {
        id -> Int4,
        group_id -> Int4,
        worker_id -> Int4,
    }
}

table! {
    pc_bind (contract_id,project_id) {
        contract_id -> Int4,
        project_id -> Int4,
        group_id -> Int4,
        head_id -> Int4,
        project_start -> Timestamp,
        project_end -> Timestamp,
        eq_list_id -> Int4,
    }
}
joinable!( worker -> department (department_name));
joinable!( worker -> worker_types (worker_type));

allow_tables_to_appear_in_same_query!(
    constr_attr,
    contract,
    department,
    department_head,
    eng_attr,
    eq_group_list,
    equipment,
    equipment_type,
    group_types,
    groups_bind,
    lab_attr,
    mech_attr,
    project,
    worker,
    worker_types,
    working_company,
    eq_group,
    groups,
    pc_bind,
);
