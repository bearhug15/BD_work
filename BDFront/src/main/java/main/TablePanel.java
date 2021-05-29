package main;

import generators.StringJSONMessageGenerator;
import main.JSONConnection;
import utils.Table;

import javax.swing.*;
import java.util.LinkedList;

public class TablePanel extends JPanel {
    Table table;
    main.JSONConnection JSONConnection;
    int tablePart =0;
    LinkedList<JPanel> entries = new LinkedList<>();
    public TablePanel(Table table, JSONConnection JSONConnection){
        this.table =table;
        this.JSONConnection = JSONConnection;
    }
    public void getNewTableData(){
        tablePart=0;
        String JSONString = StringJSONMessageGenerator.getTablePartMsg(table.getName(),0);
        String result = JSONConnection.makeRequest(JSONString);
    }
}
