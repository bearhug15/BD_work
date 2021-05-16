package utils;

import java.util.HashMap;
import java.util.Vector;

public class Table{
    String name;
    HashMap<String,String> fields;

    public Table(String name, HashMap<String,String> fields ){
        this.fields = fields;
        this.name = name;
    }
    public String getName() {
        return name;
    }

    public HashMap<String,String> getFields() {
        return fields;
    }

    public void setName(String name) {
        this.name = name;
    }

    public void setFieldsName(HashMap<String,String> fieldsName) {
        this.fields = fieldsName;
    }
}
