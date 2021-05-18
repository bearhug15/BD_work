package main;

import javax.swing.*;
import java.awt.*;
import java.lang.reflect.Method;
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
    int port=10000;
    InitData initData;
    JSONConnection connection;
    public static void main(String[] args){
        new Main(args);

    }
    public Main(String[] args){
        super("Project company database");

        setPreferredSize(new Dimension(1000,800));
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        JSONConnection JSONConnection =null;

        try {
            /*connection = new JSONConnection(main.Main.super.rootPane, address, port);
            String JSONInitString = connection.getInitData();
            initData = createInitData(JSONInitString);//*/
            initData = createInitData(initCheck());
        }catch (CancelledConnectionCreation e){
            //System.exit(1);
        }catch (ParseException e){
            System.out.println("Parse exception");
        } /*catch (IOException e) {
            System.out.println("Exception at getting init");
        }*/

        JTabbedPane tables = new JTabbedPane(JTabbedPane.NORTH);
        fillTabbedPane(tables,initData, JSONConnection);
        add(tables,BorderLayout.CENTER);

        pack();
        setVisible(true);
        //DataGetterGenerator.createContractDataGetter(Main.super.rootPane,connection,null);
        //DataGetterGenerator.createEquipmentGroupDataGetter(Main.super.rootPane,connection,null);
        //DataGetterGenerator.createGroupDataGetter(Main.super.rootPane,connection,null);
        DataGetterGenerator.createEquipmentDataGetter(Main.super.rootPane,connection,null);
        //*/
    }
    public void addTabsFromDB(){

    }

    private InitData createInitData(String initString) throws  ParseException{
        InitData initData=null;
        Vector<Table> tables = new Vector<>();
        JSONObject initJSONObject = (JSONObject) (new JSONParser()).parse(initString);
        //try {
            JSONArray tablesArray = (JSONArray) initJSONObject.get("tables");
            tablesArray.forEach(entry->{
                JSONObject table = (JSONObject) entry;
                String name = (String)table.get("name");
                HashMap<String,String> fields = new HashMap<String,String>();
                JSONArray fieldsArray = (JSONArray) table.get("fields");
                fieldsArray.forEach(field->{
                    String fieldName = (String)((JSONObject)field).get("name");
                    String fieldType = (String)((JSONObject)field).get("type");
                    fields.put(fieldName,fieldType);
                });
                tables.add(new Table(name,fields));
                //System.out.println(name);
            });//*/
        Vector<FormReq> forms = new Vector<>();
        Class generator = StringJSONMessageGenerator.class;
        Method[] methods = generator.getDeclaredMethods();
            JSONArray formsArray = (JSONArray) initJSONObject.get("forms");
            formsArray.forEach(formName->{
                String sFormName =(String)formName;
                for (Method method:methods){
                    if(method.getName().equals(sFormName+"Form")){
                        forms.add(new FormReq((String)formName));
                        break;
                    }
                }
            });

        return new InitData(tables,forms);
    }

    private void fillTabbedPane(JTabbedPane tabbedPane, InitData initData, JSONConnection JSONConnection){
        initData.getTables().forEach(table->{
            tabbedPane.addTab(table.getName(),createTablePanel(table, JSONConnection));
        });
    }
    private JPanel createTablePanel(Table table, JSONConnection JSONConnection){
        JPanel mainPanel = new JPanel();

        TablePanel tablePanel = new TablePanel(table, JSONConnection);
        JMenuBar tableBar = new JMenuBar();
        JMenuItem refreshButton = new JMenuItem("Refresh table");

        refreshButton.addActionListener(action->{
            this.setCursor(Cursor.getPredefinedCursor(Cursor.WAIT_CURSOR));
            tablePanel.getNewTableData();
            this.setCursor(Cursor.getPredefinedCursor(Cursor.DEFAULT_CURSOR));
        });

        tableBar.add(refreshButton);
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
}
