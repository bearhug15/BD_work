package main;

import Entities.Entity;
import generators.DataGetterGenerator;
import generators.StringJSONMessageGenerator;
import main.JSONConnection;
import utils.Table;

import javax.swing.*;
import javax.swing.table.DefaultTableModel;
import java.awt.*;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.Vector;

public class TablePanel extends JPanel {
    JTable table;
    String tableName;
    main.JSONConnection JSONConnection;
    int tablePart =0;
    Vector<String> columnNames;
    String sortField;
    String order;
    HashMap<Long,LinkedList<LinkedList<Object>>> rows = new HashMap<>();
    public TablePanel(String tableName, JSONConnection JSONConnection, Vector<String> columnNames){
        this.tableName =tableName;
        this.JSONConnection = JSONConnection;
        this.columnNames =columnNames;
        DefaultTableModel tableModel = new DefaultTableModel(columnNames,10);
        table = new JTable(tableModel);
        sortField = columnNames.elementAt(0);
        order="ASC";
    }
    public void getNewTableData(){
        tablePart=0;
        String JSONString = StringJSONMessageGenerator.getTablePartMsg(tableName,sortField,order,0);
        String result = JSONConnection.makeRequest(JSONString);

        //todo
    }
    public void getTablePartData(long part){
        String JSONString = StringJSONMessageGenerator.getTablePartMsg(tableName,sortField,order,0);
        String result = JSONConnection.makeRequest(JSONString);
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
