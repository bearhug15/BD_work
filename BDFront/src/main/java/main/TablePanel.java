package main;

import Entities.Entity;
import generators.DataGetterGenerator;
import generators.StringJSONMessageGenerator;
import main.JSONConnection;
import utils.Table;

import javax.swing.*;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.LinkedList;

public class TablePanel extends JPanel {
    JTable table;
    String tableName;
    main.JSONConnection JSONConnection;
    int tablePart =0;
    LinkedList<JPanel> entries = new LinkedList<>();
    public TablePanel(String tableName,JSONConnection JSONConnection){
        this.tableName =tableName;
        this.JSONConnection = JSONConnection;
    }
    public void getNewTableData(){
        tablePart=0;
        String JSONString = StringJSONMessageGenerator.getTablePartMsg(tableName,0);
        String result = JSONConnection.makeRequest(JSONString);
        //todo
    }
    public void createTableEntry(){
        Class c= DataGetterGenerator.class;
        Method[] methods = c.getDeclaredMethods();
        String methodName = "create"+tableName+"DataGetter";
        for(int i=0;i<methods.length;++i){
            if(methods[i].getName().equals(methodName)){
                try {
                    Entity res =(Entity)methods[i].invoke(null,table,JSONConnection,null);
                    if(res!=null){
                        //todo table
                    }
                } catch (IllegalAccessException e) {
                    e.printStackTrace();
                } catch (InvocationTargetException e) {
                    e.printStackTrace();
                }
                return;
            }
        }
    }
}
