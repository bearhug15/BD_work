package main;

import utils.MyEntry;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;

public class JSONResponseProcessor {
    public static MyEntry<Boolean,Integer>processId(String JSONString)throws ParseException {
        JSONObject responseObject =(JSONObject)(new JSONParser()).parse(JSONString);
        boolean successful = (boolean)responseObject.get("successful");
        if (successful){
            int id = (int)responseObject.get("id");
            return new MyEntry<>(true,id);
        }else{
            return  new MyEntry<>(false,null);
        }
    }
}
