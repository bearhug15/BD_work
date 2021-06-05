package generators;

import org.json.simple.JSONArray;
import org.json.simple.JSONObject;
import org.json.simple.JSONValue;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.LinkedList;

public class TableRowGenerator {
    Method[] methods;
    public TableRowGenerator(){
        Class cl = TableRowGenerator.class;
        methods = cl.getDeclaredMethods();
    }
    public LinkedList<LinkedList<Object>> processJSON(String JSONString, String tableName){
        if(JSONString==null || tableName ==null){
            return null;
        }
        JSONObject JSONData;
        try {
            JSONData= (JSONObject) (new JSONParser()).parse(JSONString);
        } catch (ParseException e) {
            return null;
        }
        if(JSONData.containsKey("Correct")){
            if(!(boolean)JSONData.get("Correct")){
                return new LinkedList<LinkedList<Object>>();
            }
        }
        if(!JSONData.containsKey(tableName.substring(0, 1).toUpperCase() + tableName.substring(1))){
            return null;
        }
        String methodName = tableName+"DataProcessor";
        for(Method method: methods){
            if(method.getName().equals(methodName)){
                try {
                   return (LinkedList<LinkedList<Object>>)method.invoke(null,JSONData);
                } catch (IllegalAccessException e) {
                    e.printStackTrace();
                    return null;
                } catch (InvocationTargetException e) {
                    e.printStackTrace();
                    return null;
                }
            }
        }
        return null;
    }

    private static LinkedList<LinkedList<Object>> companyDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Company");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("company_name")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            String name =(String)valRow.get("company_name");
            row.add(name);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> workerTypesDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("WorkerTypes");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("worker_type")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            String name =(String)valRow.get("worker_type");
            row.add(name);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> departmentDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Department");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("department_name")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            String name =(String)valRow.get("department_name");
            row.add(name);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> groupTypesDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("GroupTypes");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("group_type")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            String name =(String)valRow.get("group_type");
            row.add(name);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> equipmentTypeDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("EquipmentType");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("type_")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            String name =(String)valRow.get("type_");
            row.add(name);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> contractDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Contract");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("contract_id")
                    ||!valRow.containsKey("cost")
                    ||!valRow.containsKey("contract_start")
                    ||!valRow.containsKey("contract_end")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("contract_id"));
            Long cost;
            if(valRow.get("cost")==null){
                cost=0L;
            }else {
                cost = Long.parseLong((String) valRow.get("cost"));
            }
            DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MMM-ddTHH:mm:ss");
            LocalDateTime contract_start = LocalDateTime.parse((String)valRow.get("contract_start"), dtf);
            LocalDateTime contract_end =LocalDateTime.parse((String)valRow.get("contract_end"), dtf);
            row.add(id);
            row.add(cost);
            row.add(contract_start);
            row.add(contract_end);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> eqGroupListDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("EqGroupList");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("eq_list_id")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("eq_list_id"));
            row.add(id);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> projectDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Project");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("project_id")
                    ||!valRow.containsKey("cost")
                    ||!valRow.containsKey("data")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("project_id"));
            Long cost =Long.parseLong((String)valRow.get("cost"));
            String jsonData = (String)valRow.get("data");
            row.add(id);
            row.add(cost);
            row.add(jsonData);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> workerDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Worker");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("firstname")
                    ||!valRow.containsKey("secondname")
                    ||!valRow.containsKey("familyname")
                    ||!valRow.containsKey("age")
                    ||!valRow.containsKey("salary")
                    ||!valRow.containsKey("department_name")
                    ||!valRow.containsKey("worker_type")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("contract_id"));
            String firstname= (String)valRow.get("firstname");
            String secondname;
            if(valRow.get("secondname")==null){
                secondname="";
            }else{
                secondname=(String)valRow.get("secondname");
            }
            String familyname= (String)valRow.get("familyname");
            Long salary =Long.parseLong((String)valRow.get("salary"));
            Integer age =Integer.parseInt((String)valRow.get("age"));
            String department_name= (String)valRow.get("department_name");
            String worker_type= (String)valRow.get("worker_type");
            row.add(id);
            row.add(firstname);
            row.add(secondname);
            row.add(familyname);
            row.add(salary);
            row.add(age);
            row.add(department_name);
            row.add(worker_type);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> constrAttrDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("ConstrAttr");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("cert_number")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("worker_id"));
            Integer number =Integer.parseInt((String)valRow.get("cert_number"));
            row.add(id);
            row.add(number);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> mechAttrDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("MechAttr");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("repair_type")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("worker_id"));
            String number =(String)valRow.get("repair_type");
            row.add(id);
            row.add(number);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> labAttrDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("LabAttr");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("lab_number")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("worker_id"));
            Long number =Long.parseLong((String)valRow.get("lab_number"));
            row.add(id);
            row.add(number);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> engAttrDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("EngAttr");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("category")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("worker_id"));
            Integer number =Integer.parseInt((String)valRow.get("category"));
            row.add(id);
            row.add(number);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> equipmentDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Equipment");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("eq_id")
                    ||!valRow.containsKey("name")
                    ||!valRow.containsKey("department_name")
                    ||!valRow.containsKey("type_")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("eq_id"));
            String name =(String)valRow.get("name");
            String department_name;
            if(valRow.get("department_name")==null){
                department_name ="";
            }else {
                department_name = (String) valRow.get("department_name");
            }
            String type_ =(String)valRow.get("type_");
            row.add(id);
            row.add(name);
            row.add(department_name);
            row.add(type_);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> groupsBindDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("GroupsBind");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("group_id")
                    ||!valRow.containsKey("cost")
                    ||!valRow.containsKey("group_type")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("group_id"));
            Long cost = Long.parseLong((String) valRow.get("cost"));
            String group_type =(String)valRow.get("group_type");
            row.add(id);
            row.add(cost);
            row.add(group_type);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> workingCompanyDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("WorkingCompany");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("group_id")
                    ||!valRow.containsKey("company_name")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("group_id"));
            String company_name =(String)valRow.get("company_name");
            row.add(id);
            row.add(company_name);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> departmentHeadDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("DepartmentHead");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("department_name")
                    ||!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("group_type")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("worker_id"));
            String group_type =(String)valRow.get("department_name");
            row.add(group_type);
            row.add(id);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> eqGroupDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("EqGroup");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("eq_list_id")
                    ||!valRow.containsKey("id")
                    ||!valRow.containsKey("eq_id")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("id"));
            Integer eqId =Integer.parseInt((String)valRow.get("eq_id"));
            Integer eqListId =Integer.parseInt((String)valRow.get("eq_list_id"));
            row.add(id);
            row.add(eqId);
            row.add(eqListId);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> groupsDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("Groups");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("id")
                    ||!valRow.containsKey("worker_id")
                    ||!valRow.containsKey("group_id")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer id =Integer.parseInt((String)valRow.get("id"));
            Integer group_id =Integer.parseInt((String)valRow.get("group_id"));
            Integer worker_id =Integer.parseInt((String)valRow.get("worker_id"));
            row.add(id);
            row.add(group_id);
            row.add(worker_id);
            rows.add(row);
        });
        return rows;
    }
    private static LinkedList<LinkedList<Object>> pcBindDataProcessor(JSONObject data){
        LinkedList<LinkedList<Object>> rows = new LinkedList<>();
        JSONArray values = (JSONArray)data.get("PcBind");
        values.forEach(valRowBuff->{
            JSONObject valRow = (JSONObject)valRowBuff;
            if(!valRow.containsKey("contract_id")
                    ||!valRow.containsKey("project_id")
                    ||!valRow.containsKey("group_id")
                    ||!valRow.containsKey("head_id")
                    ||!valRow.containsKey("project_start")
                    ||!valRow.containsKey("project_end")
                    ||!valRow.containsKey("eq_list_id")){
                return;
            }
            LinkedList<Object> row = new LinkedList<>();
            Integer contract_id =Integer.parseInt((String)valRow.get("contract_id"));
            Integer project_id =Integer.parseInt((String)valRow.get("project_id"));
            Integer group_id =Integer.parseInt((String)valRow.get("group_id"));
            Integer head_id =Integer.parseInt((String)valRow.get("head_id"));
            DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MMM-ddTHH:mm:ss");
            LocalDateTime contract_start = LocalDateTime.parse((String)valRow.get("project_start"), dtf);
            LocalDateTime contract_end =LocalDateTime.parse((String)valRow.get("project_end"), dtf);
            Integer eq_list_id =Integer.parseInt((String)valRow.get("eq_list_id"));
            row.add(contract_id);
            row.add(project_id);
            row.add(group_id);
            row.add(head_id);
            row.add(contract_start);
            row.add(contract_end);
            row.add(eq_list_id);
            rows.add(row);
        });
        return rows;
    }
}
