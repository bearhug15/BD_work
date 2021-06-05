package main;

import org.json.simple.JSONArray;
import utils.MyEntry;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;

public class JSONResponseProcessor {
    public static MyEntry<Boolean,Integer>processId(String JSONString, String fieldName)throws ParseException {
        JSONObject responseObject =(JSONObject)(new JSONParser()).parse(JSONString);
        boolean successful = (boolean)responseObject.get("Correct");
        if (successful){
            int id = (int)responseObject.get(fieldName);
            return new MyEntry<>(true,id);
        }else{
            return  new MyEntry<>(false,null);
        }
    }

    public static boolean processInitData(String JSONString){
        JSONObject data;
        try {
            data = (JSONObject)(new JSONParser()).parse(JSONString);
        } catch (ParseException e) {
            return false;
        }
        if(!data.containsKey("company")
                ||!data.containsKey("worker_types")
                ||!data.containsKey("group_types")
                ||!data.containsKey("department")
                ||!data.containsKey("equipment_type")){
            return false;
        }
        JSONArray companies = (JSONArray)data.get("company");
        companies.forEach(company_buff->{
            JSONObject company = (JSONObject)company_buff;
            String name = (String) company.get("company_name");
            ListableFields.getCompanies().add(name);
        });
        JSONArray worker_types = (JSONArray)data.get("worker_types");
        worker_types.forEach(worker_type_buff->{
            JSONObject worker_type = (JSONObject)worker_type_buff;
            String name = (String) worker_type.get("worker_type");
            ListableFields.getWorkerTypes().add(name);
        });
        JSONArray group_types = (JSONArray)data.get("group_types");
        group_types.forEach(group_type_buff->{
            JSONObject group_type = (JSONObject)group_type_buff;
            String name = (String) group_type.get("group_type");
            ListableFields.getGroupTypes().add(name);
        });
        JSONArray department = (JSONArray)data.get("department");
        department.forEach(department_buff->{
            JSONObject dep = (JSONObject)department_buff;
            String name = (String) dep.get("department_name");
            ListableFields.getDepartmentNames().add(name);
        });
        JSONArray equipment_type = (JSONArray)data.get("equipment_type");
        equipment_type.forEach(type_buff->{
            JSONObject type_ = (JSONObject)type_buff;
            String name = (String) type_.get("type_");
            ListableFields.getEquipmentTypes().add(name);
        });

        return true;
    }

    public static boolean isCorrect(String JSONString){
        JSONObject responseObject;
        try {
            responseObject =(JSONObject)(new JSONParser()).parse(JSONString);
        } catch (ParseException e) {
            return false;
        }
        if(!responseObject.containsKey("Correct")){
            return true;
        }else{
            return (boolean)responseObject.get("Correct");
        }
    }
}
