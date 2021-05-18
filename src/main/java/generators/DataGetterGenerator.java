package generators;

import Entities.Entity;
import com.github.lgooddatepicker.components.DatePicker;
import main.JSONConnection;
import main.JSONResponseProcessor;
import main.ListableFields;
import org.json.simple.parser.ParseException;
import utils.MyEntry;

import javax.swing.*;
import javax.swing.table.DefaultTableModel;
import java.awt.*;
import java.awt.event.KeyAdapter;
import java.awt.event.KeyEvent;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.util.Vector;


public class DataGetterGenerator {
    static boolean isGettingEquipmentType = false;
    static boolean isGettingEquipment = false;
    static boolean isGettingEquipmentGroup = false;
    static boolean isGettingDepartment = false;
    static boolean isGettingDepartmentHead = false;
    static boolean isGettingWorker = false;
    static boolean isGettingContract = false;
    static boolean isGettingProject = false;
    static boolean isGettingWorkerType = false;
    static boolean isGettingGroup = false;
    static boolean isGettingCompany = false;

    public static Entity createEquipmentTypeDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingEquipmentType = true;
        Entity entity = new Entity();
        String lastArg;
        if (lastArgs != null) {
            lastArg = (String) lastArgs.elementAt(0);
        } else {
            lastArg = "";
        }
        JTextField textField = new JTextField(lastArg);
        Object[] message = {
                "Equipment type: ", textField
        };
        int res = JOptionPane.showOptionDialog(component, message, "Change equipment type", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("type", textField.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.changeTableMsg("equipment_type", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("equipment_type", newArgs);
            }
            boolean result = connection.makeStatement(JSONString);
            if (result) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Equipment type change set");
                    ListableFields.removeEquipmentType(lastArg);
                } else {
                    JOptionPane.showMessageDialog(component, "New equipment type added");
                }
                entity.addField("type", textField.toString(), "TEXT");
                ListableFields.addEquipmentType(textField.toString());
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing type", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating type", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }
        isGettingEquipmentType = false;
        return entity;
    }

    public static Entity createEquipmentDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingEquipment = true;
        Entity entity = new Entity();
        int id;
        String lastName;
        String department_name;
        String lastType;
        if (lastArgs != null) {
            id = (int) lastArgs.elementAt(0);
            lastName = (String) lastArgs.elementAt(1);
            department_name = (String) lastArgs.elementAt(2);
            lastType = (String) lastArgs.elementAt(3);
        } else {
            id = 0;
            lastName = "";
            department_name = "";
            lastType = "";
        }
        Vector<Object> messages = new Vector<>();
        JTextField nameField = new JTextField(lastName);
        messages.add("Name:");
        messages.add(nameField);

        Object[] buffArray = ListableFields.getEquipmentTypes().toArray();
        String[] sBuffArray = new String[buffArray.length];
        for(int i = 0 ; i < buffArray.length ; i ++){
            try {
                sBuffArray[i] = buffArray[i].toString();
            } catch (NullPointerException ex) {
                // do some default initialization
            }
        }
        JComboBox types = new JComboBox(sBuffArray);
        if(lastArgs!=null) {
            types.setSelectedItem(lastType);
        }
        messages.add("Type:");
        messages.add(types);
        if (!isGettingEquipmentType) {
            JButton typesButton = new JButton("add equipment type");
            typesButton.addActionListener(action -> {
                Entity subEntity = createEquipmentTypeDataGetter(component, connection, null);
                if (subEntity != null) {
                    types.removeAllItems();
                    ListableFields.getEquipmentTypes().forEach(type -> {
                        types.addItem(type);
                    });
                    types.repaint();
                }
            });
            messages.add(typesButton);
        }

        buffArray = ListableFields.getDepartmentNames().toArray();
        sBuffArray = new String[buffArray.length];
        for(int i = 0 ; i < buffArray.length ; i ++){
            try {
                sBuffArray[i] = buffArray[i].toString();
            } catch (NullPointerException ex) {
                // do some default initialization
            }
        }
        JComboBox departments = new JComboBox(ListableFields.getDepartmentNames().toArray());
        departments.setSelectedItem(department_name);
        messages.add("Department");
        messages.add(departments);
        if (!isGettingDepartment) {
            JButton departmentsButton = new JButton("add department");
            departmentsButton.addActionListener(action -> {
                Entity subEntity = createDepartmentDataGetter(component, connection, null);
                if (subEntity != null) {
                    departments.removeAllItems();
                    ListableFields.getEquipmentTypes().forEach(type -> {
                        departments.addItem(type);
                    });
                    departments.repaint();
                }
            });
            messages.add(departmentsButton);
        }
        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Set equipment", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("name", nameField.toString()));
            newArgs.add(new MyEntry<>("type", types.toString()));
            newArgs.add(new MyEntry<>("department_name", departments.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.changeTableMsg("equipment", String.valueOf(id), newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("equipment", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            MyEntry<Boolean, Integer> idContainer;
            try {
                idContainer = JSONResponseProcessor.processId(result);
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                isGettingEquipment = false;
                return null;
            }
            if (idContainer.getKey()) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Equipment change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New equipment added. Id: " + String.valueOf(idContainer.getValue()));
                }
                entity.addField("id", idContainer.getValue(), "INTEGER");
                entity.addField("type", types.toString(), "TEXT");
                entity.addField("department_name", departments.toString(), "TEXT");
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing equipment", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating equipment", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }
        isGettingEquipment = false;
        return entity;
    }

    public static Entity createEquipmentGroupDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingEquipmentGroup = true;
        Entity entity = new Entity();
        int eqListId = -1;
        Vector<String> eqIds = null;
        Vector<Object> messages = new Vector<>();
        JList<String> eqList;
        DefaultListModel<String> listModel = new DefaultListModel<String>();
        eqList = new JList<String>(listModel);
        if (lastArgs != null) {
            eqListId = (int) lastArgs.elementAt(0);
            eqIds = (Vector<String>) lastArgs.elementAt(1);
            JTextField eqListField = new JTextField(String.valueOf(eqListId));
            eqListField.setEditable(false);
            messages.add(eqListField);
            listModel.addAll(eqIds);
        }

        JScrollPane scrollList = new JScrollPane(eqList);
        JTextField number = new JTextField();
        number.addKeyListener(new KeyAdapter() {
            public void keyPressed(KeyEvent ke) {
                String value = number.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    number.setText(value.substring(0, l - 1));
                }
            }
        });

        JButton addButton = new JButton("add");
        addButton.addActionListener(action -> {
            String numberText = number.getText();
            number.setText("");
            int value;
            try {
                value = Integer.parseInt(numberText);
            } catch (NumberFormatException e) {
                return;
            }
            listModel.addElement(String.valueOf(value));
        });
        JButton deleteButton = new JButton("delete");
        deleteButton.addActionListener(action -> {
            if (!eqList.isSelectionEmpty()) {
                String value = eqList.getSelectedValue();
                listModel.removeElement(value);
            }
        });
        messages.add(scrollList);
        messages.add(number);
        messages.add(addButton);
        messages.add(deleteButton);

        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Set equipment group", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, Vector<String>>> newArgs = new Vector<>();
            Vector<String> valuesBuffer = new Vector<>();
            for (int i = 0; i < eqList.getModel().getSize(); i++) {
                valuesBuffer.add(String.valueOf(eqList.getModel().getElementAt(i)));
            }
            newArgs.add(new MyEntry<>("eq_id", valuesBuffer));
            String JSONString;
            int newId = -1;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.replaceEntriesTableMsg("eq_group", "eq_list_id", String.valueOf(eqListId), newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("eq_group_list", null);
                String newIdStringBuffer = connection.makeRequest(JSONString);
                MyEntry<Boolean, Integer> idResult;
                try {
                    idResult = JSONResponseProcessor.processId(newIdStringBuffer);
                } catch (ParseException e) {
                    JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                    isGettingEquipment = false;
                    return null;
                }
                if (idResult.getKey()) {
                    newId = idResult.getValue();
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating new group", "Server error", JOptionPane.ERROR_MESSAGE);
                    isGettingEquipment = false;
                    return null;
                }
                Vector<String> buffer = new Vector<>(valuesBuffer.size());
                String newIdString = String.valueOf(newId);
                valuesBuffer.forEach(a -> {
                    buffer.add(newIdString);
                });
                newArgs.add(new MyEntry<>("eq_list_id", buffer));
                JSONString = StringJSONMessageGenerator.addSerialTableEntryMsg("eq_group", newArgs);
            }
            boolean successful = connection.makeStatement(JSONString);
            if (successful) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Equipment group change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New equipment group added. Id: " + String.valueOf(newId));
                    eqListId = newId;
                }
                entity.addField("id", eqListId, "INTEGER");
                entity.addField("eq_ids", valuesBuffer, "INTEGER");
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing equipment group", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating equipment group", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }

        isGettingEquipmentGroup = false;
        return entity;
    }

    public static Entity createDepartmentDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingDepartment = true;
        Entity entity = new Entity();
        String lastArg;
        if (lastArgs != null) {
            lastArg = (String) lastArgs.elementAt(0);
        } else {
            lastArg = "";
        }
        JTextField textField = new JTextField(lastArg);
        Object[] message = {
                "Department name: ", textField
        };
        int res = JOptionPane.showOptionDialog(component, message, "Change department name", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("type", textField.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.changeTableMsg("department", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("department", newArgs);
            }
            boolean result = connection.makeStatement(JSONString);
            if (result) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Department name change set");
                    ListableFields.removeDepartmentName(lastArg);
                } else {
                    JOptionPane.showMessageDialog(component, "New department added");
                }
                entity.addField("department_name", textField.toString(), "TEXT");
                ListableFields.addDepartmentName(textField.toString());
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing department name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating department", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }

        isGettingDepartment = false;
        return entity;
    }

    public static Entity createDepartmentHead(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingDepartmentHead=true;
        Entity entity = new Entity();
        String department_name ="";
        int workerId =-1;
        Vector<Object> messages = new Vector<>();
        if(lastArgs!=null){
            department_name = (String) lastArgs.elementAt(0);
            workerId = (int) lastArgs.elementAt(1);
        }
        JTextField workerIdField = new JTextField(String.valueOf(workerId));
        workerIdField.addKeyListener(new KeyAdapter() {
            public void keyPressed(KeyEvent ke) {
                String value = workerIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    workerIdField.setText(value.substring(0, l - 1));
                }
            }
        });
        messages.add(workerIdField);
        JComboBox departments = new JComboBox(ListableFields.getDepartmentNames().toArray());
        departments.setSelectedItem(department_name);
        messages.add("Department");
        messages.add(departments);
        if (!isGettingDepartment) {
            JButton departmentsButton = new JButton("add department");
            departmentsButton.addActionListener(action -> {
                Entity subEntity = createDepartmentDataGetter(component, connection, null);
                if (subEntity != null) {
                    departments.removeAllItems();
                    ListableFields.getEquipmentTypes().forEach(type -> {
                        departments.addItem(type);
                    });
                    departments.repaint();
                }
            });
            messages.add(departmentsButton);
        }

        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Set department head", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("worker_id", workerIdField.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.changeTableMsg("department_head", departments.toString(), newArgs);
            } else {
                newArgs.add(new MyEntry<>("department_name", departments.toString()));
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("department_head", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            MyEntry<Boolean, Integer> idContainer;
            try {
                idContainer = JSONResponseProcessor.processId(result);
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                isGettingDepartmentHead = false;
                return null;
            }
            if (idContainer.getKey()) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Department head change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New department head added.");
                }
                entity.addField("department_name", departments.toString(), "TEXT");
                entity.addField("worker_id", idContainer.getValue(), "INTEGER");
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing department head", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating department head", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }

        isGettingDepartmentHead=false;
        return null;

    }
    //todo
    public static Entity createWorkerDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {

        return null;
    }

    public static Entity createContractDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingContract=true;
        Entity entity = new Entity();
        int lastContractId=-1;
        int cost =0;
        String contract_start="";
        String contract_end="";
        if(lastArgs!=null){
            lastContractId =(int)lastArgs.elementAt(0);
            cost = (int)lastArgs.elementAt(1);
            contract_start=(String)lastArgs.elementAt(2);
            contract_end=(String)lastArgs.elementAt(3);
        }
        JTextField costField = new JTextField(String.valueOf(cost));
        costField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = costField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    costField.setText(value.substring(0, l - 1));
                }
            }
        });

        DatePicker startDatePicker = new DatePicker();
        DatePicker endDatePicker = new DatePicker();
        if(lastArgs!=null){
            DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MMM-dd");
            try {
                startDatePicker.setDate(LocalDate.parse(contract_start, dtf));
                endDatePicker.setDate(LocalDate.parse(contract_end, dtf));
            }catch (DateTimeParseException e){
                startDatePicker.setDate(LocalDate.now());
                endDatePicker.setDate(LocalDate.now());
            }
        }
        else {
            startDatePicker.setDate(LocalDate.now());
            endDatePicker.setDate(LocalDate.now());
        }
        JLabel warningLabel = new JLabel("");
        startDatePicker.addDateChangeListener(date->{
            LocalDate startDate = startDatePicker.getDate();
            LocalDate endDate = endDatePicker.getDate();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        endDatePicker.addDateChangeListener(date->{
            LocalDate startDate = startDatePicker.getDate();
            LocalDate endDate = endDatePicker.getDate();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        Vector<Object> messages = new Vector<>();
        if(lastArgs!=null) {
            messages.add("This is final cost including all other costs.");
            messages.add(" Change at your own risk!");
        }else{
            messages.add("Set starting contract cost");
        }
        messages.add(costField);
        if(lastArgs!=null) {
            messages.add("Contract start changes automatically, depending on associated projects.");
            messages.add("Change at your own risk!");
        }else{
            messages.add("Set contract start date");
        }
        messages.add(startDatePicker);
        if(lastArgs!=null) {
            messages.add("Contract end changes automatically, depending on associated projects.");
            messages.add(" Change at your own risk!");
        }else{
            messages.add("Set contract end date");
        }
        messages.add(endDatePicker);
        messages.add(warningLabel);
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Set contract",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if (res==JOptionPane.OK_OPTION){
            LocalDate startDate = startDatePicker.getDate();
            LocalDate endDate = endDatePicker.getDate();
            if(!startDate.isBefore(endDate)){
                JOptionPane.showMessageDialog(component,"Start date should be before end date","Input error",JOptionPane.ERROR_MESSAGE);
                return null;
            }
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("cost",costField.getText()));
            newArgs.add(new MyEntry<>("contract_start",startDate.toString()));
            newArgs.add(new MyEntry<>("contract_end",endDate.toString()));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.changeTableMsg("contract",String.valueOf(lastContractId),newArgs);
            }else{
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("contract",newArgs);
            }
            String result = connection.makeRequest(JSONString);
            MyEntry<Boolean, Integer> idContainer;
            try {
                idContainer = JSONResponseProcessor.processId(result);
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                isGettingContract = false;
                return null;
            }
            if (idContainer.getKey()) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Contract change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New contract added.");
                }
                entity.addField("contract_id",idContainer.getValue(),"INTEGER");
                entity.addField("cost",costField.getText(),"INTEGER");
                entity.addField("contract_start",startDate.toString(),"TIMESTAMP");
                entity.addField("contract_end",endDate.toString(),"TIMESTAMP");
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing contract", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating contract", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        }
        else{
            entity=null;
        }
        isGettingContract=false;
        return entity;
    }

    public static Entity createProjectDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingProject=true;
        Entity entity = new Entity();
        int lastProjectId=-1;
        int cost =0;
        String project_start="";
        String project_end="";
        if(lastArgs!=null){
            lastProjectId =(int)lastArgs.elementAt(0);
            cost = (int)lastArgs.elementAt(1);
            project_start=(String)lastArgs.elementAt(2);
            project_end=(String)lastArgs.elementAt(3);
        }
        JTextField costField = new JTextField(String.valueOf(cost));
        costField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = costField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    costField.setText(value.substring(0, l - 1));
                }
            }
        });

        DatePicker startDatePicker = new DatePicker();
        DatePicker endDatePicker = new DatePicker();
        if(lastArgs!=null){
            DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MMM-dd");
            try {
                startDatePicker.setDate(LocalDate.parse(project_start, dtf));
                endDatePicker.setDate(LocalDate.parse(project_end, dtf));
            }catch (DateTimeParseException e){
                startDatePicker.setDate(LocalDate.now());
                endDatePicker.setDate(LocalDate.now());
            }
        }
        else {
            startDatePicker.setDate(LocalDate.now());
            endDatePicker.setDate(LocalDate.now());
        }
        JLabel warningLabel = new JLabel("");
        startDatePicker.addDateChangeListener(date->{
            LocalDate startDate = startDatePicker.getDate();
            LocalDate endDate = endDatePicker.getDate();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        endDatePicker.addDateChangeListener(date->{
            LocalDate startDate = startDatePicker.getDate();
            LocalDate endDate = endDatePicker.getDate();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        Vector<Object> messages = new Vector<>();
        if(lastArgs!=null) {
            messages.add("Set project cost");
        }else{
            messages.add("Set new project cost");
        }
        messages.add(costField);
        if(lastArgs!=null) {
            messages.add("Set project start date");
        }else{
            messages.add("Set new project start date");
        }
        messages.add(startDatePicker);
        if(lastArgs!=null) {
            messages.add("Set project end date");
        }else{
            messages.add("Set new project end date");
        }
        messages.add(endDatePicker);
        messages.add(warningLabel);
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Set project",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if (res==JOptionPane.OK_OPTION){
            LocalDate startDate = startDatePicker.getDate();
            LocalDate endDate = endDatePicker.getDate();
            if(!startDate.isBefore(endDate)){
                JOptionPane.showMessageDialog(component,"Start date should be before end date","Input error",JOptionPane.ERROR_MESSAGE);
                return null;
            }
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("cost",costField.getText()));
            newArgs.add(new MyEntry<>("project_start",startDate.toString()));
            newArgs.add(new MyEntry<>("project_end",endDate.toString()));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.changeTableMsg("project",String.valueOf(lastProjectId),newArgs);
            }else{
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("project",newArgs);
            }
            String result = connection.makeRequest(JSONString);
            MyEntry<Boolean, Integer> idContainer;
            try {
                idContainer = JSONResponseProcessor.processId(result);
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                isGettingProject = false;
                return null;
            }
            if (idContainer.getKey()) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Project change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New project added");
                }
                entity.addField("project_id",idContainer.getValue(),"INTEGER");
                entity.addField("cost",costField.getText(),"INTEGER");
                entity.addField("project_start",startDate.toString(),"TIMESTAMP");
                entity.addField("project_end",endDate.toString(),"TIMESTAMP");
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing contract", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating contract", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        }
        else{
            entity=null;
        }
        isGettingProject=false;
        return entity;
    }

    public static Entity createWorkerTypeDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingWorkerType = true;
        Entity entity = new Entity();
        String lastArg;
        if (lastArgs != null) {
            lastArg = (String) lastArgs.elementAt(0);
        } else {
            lastArg = "";
        }
        JTextField textField = new JTextField(lastArg);
        Object[] message = {
                "Worker type: ", textField
        };
        int res = JOptionPane.showOptionDialog(component, message, "Change worker type name", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("worker_type", textField.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.changeTableMsg("worker_types", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("worker_types", newArgs);
            }
            boolean result = connection.makeStatement(JSONString);
            if (result) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Worker type name change set");
                    ListableFields.removeWorkerType(lastArg);
                } else {
                    JOptionPane.showMessageDialog(component, "New worker type added");
                }
                entity.addField("company_name", textField.toString(), "TEXT");
                ListableFields.addWorkerType(textField.toString());
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing worker type name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating worker type", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }
        isGettingWorkerType = false;
        return entity;
    }

    public static Entity createGroupDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingGroup = true;
        Entity entity = new Entity();
        int groupId = -1;
        Vector<String> workerIds = null;
        int cost=0;
        String group_type;
        Vector<Object> messages = new Vector<>();
        JList<String> eqList;
        DefaultListModel<String> listModel = new DefaultListModel<String>();
        eqList = new JList<String>(listModel);
        if (lastArgs != null) {
            groupId = (int) lastArgs.elementAt(0);
            cost = (int)lastArgs.elementAt(1);
            group_type = (String)lastArgs.elementAt(2);
            workerIds = (Vector<String>) lastArgs.elementAt(2);
            JTextField eqListField = new JTextField(String.valueOf(groupId));
            eqListField.setEditable(false);
            messages.add(eqListField);
            listModel.addAll(workerIds);
        }else{
            group_type="";
        }

        JScrollPane scrollList = new JScrollPane(eqList);
        JTextField number = new JTextField();
        number.addKeyListener(new KeyAdapter() {
            public void keyPressed(KeyEvent ke) {
                String value = number.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    number.setText(value.substring(0, l - 1));
                }
            }
        });

        JButton addButton = new JButton("add");
        addButton.addActionListener(action -> {
            String numberText = number.getText();
            number.setText("");
            int value;
            try {
                value = Integer.parseInt(numberText);
            } catch (NumberFormatException e) {
                return;
            }
            listModel.addElement(String.valueOf(value));
        });
        JButton deleteButton = new JButton("delete");
        deleteButton.addActionListener(action -> {
            if (!eqList.isSelectionEmpty()) {
                String value = eqList.getSelectedValue();
                listModel.removeElement(value);
            }
        });
        JTextField costField = new JTextField(String.valueOf(cost));
        costField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = costField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    costField.setText(value.substring(0, l - 1));
                }
            }
        });
        if(lastArgs!=null) {
            messages.add("Set worker group cost");
        }
        else{
            messages.add("Set new worker group cost");
        }
        messages.add(costField);
        messages.add("Type:");
        Object[] buffArray = ListableFields.getGroupTypes().toArray();
        String[] sBuffArray = new String[buffArray.length];
        for(int i = 0 ; i < buffArray.length ; i ++){
            try {
                sBuffArray[i] = buffArray[i].toString();
            } catch (NullPointerException ex) {
                // do some default initialization
            }
        }
        JComboBox<String> group_types = new JComboBox<String>(sBuffArray);
        if(lastArgs!=null) {
            group_types.setSelectedItem(group_type);
        }
        messages.add(group_types);
        if (!isGettingEquipmentType) {
            JButton typesButton = new JButton("add equipment type");
            typesButton.addActionListener(action -> {
                Entity subEntity = createGroupTypeDataGetter(component, connection, null);
                if (subEntity != null) {
                    group_types.removeAllItems();
                    ListableFields.getGroupTypes().forEach(type -> {
                        group_types.addItem(type);
                    });
                    group_types.repaint();
                }
            });
            messages.add(typesButton);
        }


        messages.add(scrollList);
        messages.add(number);
        messages.add(addButton);
        messages.add(deleteButton);
//todo change
        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Set worker group", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, Vector<String>>> newArgs = new Vector<>();
            Vector<String> valuesBuffer = new Vector<>();
            for (int i = 0; i < eqList.getModel().getSize(); i++) {
                valuesBuffer.add(String.valueOf(eqList.getModel().getElementAt(i)));
            }
            newArgs.add(new MyEntry<>("group_id", valuesBuffer));
            String JSONString;
            int newId = -1;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.replaceEntriesTableMsg("groups", "group_id", String.valueOf(groupId), newArgs);
            } else {
                Vector<MyEntry<String,String>> subArgs =new Vector<>();
                subArgs.add(new MyEntry<>("cost",costField.getText()));
                subArgs.add(new MyEntry<>("group_type",(String)group_types.getSelectedItem()));
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("groups_bind", null);
                String newIdStringBuffer = connection.makeRequest(JSONString);
                MyEntry<Boolean, Integer> idResult;
                try {
                    idResult = JSONResponseProcessor.processId(newIdStringBuffer);
                } catch (ParseException e) {
                    JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                    isGettingEquipment = false;
                    return null;
                }
                if (idResult.getKey()) {
                    newId = idResult.getValue();
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating new group", "Server error", JOptionPane.ERROR_MESSAGE);
                    isGettingEquipment = false;
                    return null;
                }
                Vector<String> buffer = new Vector<>(valuesBuffer.size());
                String newIdString = String.valueOf(newId);
                valuesBuffer.forEach(a -> {
                    buffer.add(newIdString);
                });
                newArgs.add(new MyEntry<>("eq_list_id", buffer));
                JSONString = StringJSONMessageGenerator.addSerialTableEntryMsg("eq_group", newArgs);
            }
            boolean successful = connection.makeStatement(JSONString);
            if (successful) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Equipment group change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New equipment group added. Id: " + String.valueOf(newId));
                    groupId = newId;
                }
                entity.addField("id", groupId, "INTEGER");
                entity.addField("eq_ids", valuesBuffer, "INTEGER");
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing equipment group", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating equipment group", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }

        isGettingGroup = false;
        return entity;
    }
    public static Entity createGroupTypeDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs){
        return null;
    }
    public static Entity createProjectContractBinderDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs){

        return null;
    }
    public static Entity createCompany(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingCompany = true;
        Entity entity = new Entity();
        String lastArg;
        if (lastArgs != null) {
            lastArg = (String) lastArgs.elementAt(0);
        } else {
            lastArg = "";
        }
        JTextField textField = new JTextField(lastArg);
        Object[] message = {
                "Company name: ", textField
        };
        int res = JOptionPane.showOptionDialog(component, message, "Change company name", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("company_name", textField.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.changeTableMsg("company", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("company", newArgs);
            }
            boolean result = connection.makeStatement(JSONString);
            if (result) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Company name change set");
                    ListableFields.removeCompany(lastArg);
                } else {
                    JOptionPane.showMessageDialog(component, "New company added");
                }
                entity.addField("company_name", textField.toString(), "TEXT");
                ListableFields.addCompany(textField.toString());
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing company name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating company", "Error", JOptionPane.ERROR_MESSAGE);
                }
                entity = null;
            }
        } else {
            entity = null;
        }
        isGettingCompany = false;
        return entity;
    }
}