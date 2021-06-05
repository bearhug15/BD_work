package main;

import javax.swing.*;
import java.awt.*;
import java.io.IOException;
import java.lang.reflect.Method;
import java.util.Collections;
import java.util.HashMap;
import java.util.Vector;

import generators.DataGetterGenerator;
import generators.StringJSONMessageGenerator;
import org.json.simple.parser.ParseException;
import utils.FormReq;
import utils.InitData;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.JSONArray;
import utils.Table;

public class Main extends JFrame {
    String address="127.0.0.1";
    int port=20000;
    InitData initData;
    JSONConnection connection;
    public static void main(String[] args){
        new Main(args);
    }
    public Main(String[] args){
        super("Project company database");

        setPreferredSize(new Dimension(1000,800));
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        try {
            connection = new JSONConnection(main.Main.super.rootPane, address, port);
            String JSONInitString = connection.getInitData();
            if(!JSONResponseProcessor.processInitData(JSONInitString)){
                System.out.println("Error in init");
            }
            //initData = createInitData(JSONInitString);//*/
            //initData = createInitData(initCheck());
        }catch (CancelledConnectionCreation e){
            System.exit(1);
        }catch (IOException e) {
            System.out.println("Exception at getting init");
        }
        ListableFields fields = new ListableFields();
        JMenuBar forms = new JMenuBar();
        JMenu formMenu =new JMenu("Forms");
        setForms(formMenu);
        forms.add(formMenu);

        JTabbedPane tables = new JTabbedPane(JTabbedPane.NORTH);
        fillTabbedPane(tables,initData, connection);

        add(forms,BorderLayout.NORTH);
        add(tables,BorderLayout.CENTER);

        pack();
        setVisible(true);
        //DataGetterGenerator.createContractDataGetter(Main.super.rootPane,connection,null);
        //DataGetterGenerator.createEquipmentGroupDataGetter(Main.super.rootPane,connection,null);
        //DataGetterGenerator.createGroupDataGetter(Main.super.rootPane,connection,null);
        //DataGetterGenerator.createEquipmentDataGetter(Main.super.rootPane,connection,null);
        //DataGetterGenerator.createGroupDataGetter(Main.super.rootPane,connection,null);
        //*/
    }


    private void fillTabbedPane(JTabbedPane tabbedPane, InitData initData, JSONConnection JSONConnection){
        tabbedPane.add("Companies",createTablePanel("Company",JSONConnection));
        tabbedPane.add("Worker types",createTablePanel("WorkerType",JSONConnection));
        tabbedPane.add("Equipment types",createTablePanel("EquipmentType",JSONConnection));
        tabbedPane.add("Group types",createTablePanel("GroupType",JSONConnection));
        tabbedPane.add("Departments",createTablePanel("Department",JSONConnection));
        tabbedPane.add("Workers",createTablePanel("Worker",JSONConnection));
        tabbedPane.add("Equipment",createTablePanel("Equipment",JSONConnection));
        tabbedPane.add("Equipment groups",createTablePanel("EquipmentGroup",JSONConnection));
        tabbedPane.add("Department heads",createTablePanel("DepartmentHead",JSONConnection));
        tabbedPane.add("Contracts",createTablePanel("Contract",JSONConnection));
        tabbedPane.add("Projects",createTablePanel("Project",JSONConnection));
        //tabbedPane.add("Groups",createTablePanel("Group",JSONConnection));
        tabbedPane.add("Contracts&Projects",createTablePanel("ProjectContractBinder",JSONConnection));
        tabbedPane.add("Groups",createTablePanel("Groups",JSONConnection));
        tabbedPane.add("Groups bind",createTablePanel("GroupsBind",JSONConnection));
        //tabbedPane.add("Groups bind",createTablePanel("GroupsBind",JSONConnection));
    }
    private JPanel createTablePanel(String tableName, JSONConnection JSONConnection){
        JPanel mainPanel = new JPanel();

        TablePanel tablePanel = new TablePanel(tableName,JSONConnection,new Vector(Collections.singleton("first")));
        JMenuBar tableBar = new JMenuBar();
        JMenuItem refreshButton = new JMenuItem("Refresh table");

        refreshButton.addActionListener(action->{
            this.setCursor(Cursor.getPredefinedCursor(Cursor.WAIT_CURSOR));
            tablePanel.getNewTableData();
            this.setCursor(Cursor.getPredefinedCursor(Cursor.DEFAULT_CURSOR));
        });

        JMenuItem createButton = new JMenuItem("Create entry");
        createButton.addActionListener(action->{
            tablePanel.createTableEntry();
        });

        tableBar.add(refreshButton);
        tableBar.add(createButton);
        mainPanel.add(tableBar,BorderLayout.NORTH);
        mainPanel.add(tablePanel,BorderLayout.CENTER);

        return mainPanel;
    }

    private String initCheck(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append(
                "{ \"tables\" : " +
                    "[{\"name\" : \"one\"," +
                        "\"fields\": " +
                        "[{\"name\":\"field1\",\"type\":\"int\"}," +
                        "{\"name\":\"field2\",\"type\":\"String\"}]}," +
                    "{\"name\" : \"two\"," +
                        "\"fields\": " +
                        "[{\"name\":\"field1\",\"type\":\"int\"}," +
                        "{\"name\":\"field2\",\"type\":\"String\"}]}," +
                    "{\"name\" : \"three\"," +
                        "\"fields\": " +
                        "[{\"name\":\"field1\",\"type\":\"int\"}," +
                        "{\"name\":\"field2\",\"type\":\"String\"}]}]," +
                "\"forms\" : " +
                    "[\"getContractsEf\"," +
                    "\"getEquipmentByContract\"," +
                    "\"getContractsEfGf\"" +
                    "]" +
                " }");
        return stringJSONBuild.toString();
    }
    private void setForms(JMenu menu){
        JMenuItem AVGSalaryByWorkerTypeItem = new JMenuItem("Get average salaries divided by type");
        AVGSalaryByWorkerTypeItem.addActionListener(action->{
            DataGetterGenerator.AVGSalaryByWorkerTypeForm(Main.super.rootPane,connection);
        });
        JMenuItem contractsAtTimeItem = new JMenuItem("Get contracts at time");
        contractsAtTimeItem.addActionListener(action->{
            DataGetterGenerator.ContractsAtTimeForm(Main.super.rootPane,connection);
        });

        JMenuItem contractsByProjectItem = new JMenuItem("Get contracts by project id");
        contractsByProjectItem.addActionListener(action->{
            DataGetterGenerator.ContractsByProjectFrom(Main.super.rootPane,connection);
        });
        JMenuItem contractsEfItem = new JMenuItem("Get contracts efficiency");
        contractsEfItem.addActionListener(action->{
            DataGetterGenerator.ContractsEfForm(Main.super.rootPane,connection);
        });
        JMenuItem contractsCostByTimeItem = new JMenuItem("Get contracts cost by time");
        contractsCostByTimeItem.addActionListener(action->{
            DataGetterGenerator.ContractsCostByTimeForm(Main.super.rootPane,connection);
        });
        JMenuItem departmentHeadsItem = new JMenuItem("Get department heads");
        contractsCostByTimeItem.addActionListener(action->{
            DataGetterGenerator.DepartmentHeadsForm(Main.super.rootPane,connection);
        });
        JMenuItem departmentStuffByAgeItem = new JMenuItem("Get department stuff by age");
        contractsCostByTimeItem.addActionListener(action->{
            DataGetterGenerator.DepartmentStuffByAgeForm(Main.super.rootPane,connection);
        });
        JMenuItem departmentStuffByTypeItem = new JMenuItem("Get department stuff by type");
        contractsCostByTimeItem.addActionListener(action->{
            DataGetterGenerator.DepartmentStuffByTypeForm(Main.super.rootPane,connection);
        });
        JMenuItem EquipmentByContractItem = new JMenuItem("Get equipment by contract");
        contractsCostByTimeItem.addActionListener(action->{
            DataGetterGenerator.EquipmentByContractForm(Main.super.rootPane,connection);
        });
        JMenuItem EquipmentByProjectItem = new JMenuItem("Get equipment by project");
        contractsCostByTimeItem.addActionListener(action->{
            DataGetterGenerator.EquipmentByProjectForm(Main.super.rootPane,connection);
        });
        menu.add(AVGSalaryByWorkerTypeItem);
        menu.add(contractsAtTimeItem);
        menu.add(contractsByProjectItem);
        menu.add(contractsEfItem);
        menu.add(contractsCostByTimeItem);
        menu.add(departmentHeadsItem);
        menu.add(departmentStuffByAgeItem);
        menu.add(departmentStuffByTypeItem);
        menu.add(EquipmentByContractItem);
        menu.add(EquipmentByProjectItem);
    }
}
