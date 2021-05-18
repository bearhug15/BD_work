package generators;

import Entities.Entity;
import utils.Table;

import javax.swing.*;
import java.awt.*;
import java.time.temporal.ChronoUnit;
import java.util.HashMap;
import java.util.LinkedList;

import org.json.simple.JSONObject;
import org.json.simple.JSONArray;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;

public class EntityGenerator {
    static HashMap<String,Class> StringToClass;
    static {
        StringToClass = new HashMap<>();
        StringToClass.put("INTEGER",Integer.class);
        StringToClass.put("TEXT",String.class);
        StringToClass.put("TIMESTAMP", ChronoUnit.class);
        StringToClass.put("SERIAL",Integer.class);
        StringToClass.put("BIGINT",Long.class);
    }
    public static LinkedList<Entity> createEntities(Component component, Table table, String JSONString) throws UnknownFieldException, ParseException {
        LinkedList<Entity> entities = new LinkedList<>();
        HashMap<String, String> tableFields  = table.getFields();
        JSONObject data = (JSONObject)(new JSONParser()).parse(JSONString);
        JSONArray entriesArray = (JSONArray)data.get("entries");
        entriesArray.forEach(e->{
            JSONObject entry = (JSONObject)e;
            JPanel entryPanel = new JPanel(new GridLayout(0,2));
            Entity entity = new Entity();
            entry.forEach((fieldName,fieldValue)->{
                String fieldType = tableFields.get(fieldName);
                if(fieldName==null){
                    throw new UnknownFieldException();
                }
                Class fieldClass = StringToClass.get(fieldType);
                if(fieldClass==null){
                    throw new UnknownFieldException();
                }
                entity.addField((String) fieldName,fieldValue,fieldType);
            });
            entities.add(entity);

            entity.addActionListener(action->{

            });
        });
        return entities;
    }
    private static void checkUpdater(){

    }

}

class UnknownFieldException extends RuntimeException{}