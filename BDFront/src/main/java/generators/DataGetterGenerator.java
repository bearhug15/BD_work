package generators;

import Entities.Entity;
import com.github.lgooddatepicker.components.DatePicker;
import com.github.lgooddatepicker.components.DateTimePicker;
import main.JSONConnection;
import main.JSONResponseProcessor;
import main.ListableFields;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;
import utils.MyEntry;

import javax.swing.*;
import java.awt.*;
import java.awt.event.KeyAdapter;
import java.awt.event.KeyEvent;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.util.Collection;
import java.util.Vector;

import static java.lang.Math.PI;
import static java.lang.Math.max;


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
    static boolean isGettingGroupType=false;
    public static boolean createEquipmentTypeDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingEquipmentType = true;
        boolean done=true;
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
            newArgs.add(new MyEntry<>("type", textField.getText()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("equipment_type", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("equipment_type", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if (result!=null) {
                if(JSONResponseProcessor.isCorrect(result)) {
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Equipment type change set");
                        ListableFields.removeEquipmentType(lastArg);
                    } else {
                        JOptionPane.showMessageDialog(component, "New equipment type added");
                    }
                    ListableFields.addEquipmentType(textField.getText());
                }else{
                    done=false;
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing type", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating type", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done = false;
            }
        } else {
            done = false;
        }
        isGettingEquipmentType = false;
        return done;
    }

    public static boolean createEquipmentDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingEquipment = true;
        boolean done=true;
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
                boolean subEntity = createEquipmentTypeDataGetter(component, connection, null);
                if (subEntity) {
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
                boolean subEntity = createDepartmentDataGetter(component, connection, null);
                if (subEntity ) {
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
            newArgs.add(new MyEntry<>("name", nameField.getText()));
            newArgs.add(new MyEntry<>("type", types.toString()));
            newArgs.add(new MyEntry<>("department_name", departments.toString()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("equipment", String.valueOf(id), newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("equipment", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if(!JSONResponseProcessor.isCorrect(result)){
                JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
            JSONObject data;
            try {
                data =(JSONObject)(new JSONParser()).parse(result);
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
            int ID = (int) data.get("eq_id");
            if (true) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Equipment change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New equipment added. Id: " + String.valueOf(ID));
                }

            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing equipment", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating equipment", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done = false;
            }
        } else {
            done = false;
        }
        isGettingEquipment = false;
        return done;
    }

    public static boolean createEquipmentGroupDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingEquipmentGroup = true;
        boolean done=true;
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
            eqIds.forEach(id->{
                listModel.addElement(id);
            });//*/
            //listModel.addAll((Collection<String>) eqIds);
            //listModel.addAll(eqIds);
        }

        JScrollPane scrollList = new JScrollPane(eqList);
        JTextField number = new JTextField();
        number.addKeyListener(new KeyAdapter() {
            public void keyPressed(KeyEvent ke) {
                String value = number.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    number.setText(value.substring(0, max(l - 1,0)));
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
                if(!JSONResponseProcessor.isCorrect(newIdStringBuffer)){
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    return false;
                }
                JSONObject data;
                try {
                    data =(JSONObject)(new JSONParser()).parse(JSONString);
                } catch (ParseException e) {
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    return false;
                }
                int id = (int) data.get("id");
                newId=id;
                Vector<String> buffer = new Vector<>(valuesBuffer.size());
                String newIdString = String.valueOf(newId);
                valuesBuffer.forEach(a -> {
                    buffer.add(newIdString);
                });
                newArgs.add(new MyEntry<>("eq_list_id", buffer));
                JSONString = StringJSONMessageGenerator.addSerialTableEntryMsg("eq_group", newArgs);
            }
            String successful = connection.makeRequest(JSONString);
            if (successful!=null) {
                if(JSONResponseProcessor.isCorrect(successful)) {
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Equipment group change set");
                    } else {
                        JOptionPane.showMessageDialog(component, "New equipment group added. Id: " + String.valueOf(newId));
                        eqListId = newId;
                    }

                }else{
                    done=false;
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing equipment group", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating equipment group", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        } else {
            done=false;;
        }

        isGettingEquipmentGroup = false;
        return done;
    }

    public static boolean createDepartmentDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingDepartment = true;
        boolean done=true;
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
            newArgs.add(new MyEntry<>("department_name", textField.getText()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("department", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("department", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if (result!=null) {
                if(JSONResponseProcessor.isCorrect(result)) {
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Department name change set");
                        ListableFields.removeDepartmentName(lastArg);
                    } else {
                        JOptionPane.showMessageDialog(component, "New department added");
                    }

                    ListableFields.addDepartmentName(textField.getText());
                }else{
                    done=false;
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing department name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating department", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;
            }
        } else {
            done=false;
        }

        isGettingDepartment = false;
        return done;
    }

    public static boolean createDepartmentHeadDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingDepartmentHead=true;
        boolean done=true;
        String department_name ="";
        int workerId =-1;
        Vector<Object> messages = new Vector<>();
        if(lastArgs!=null){
            department_name = (String) lastArgs.elementAt(0);
            workerId = (int) lastArgs.elementAt(1);
        }
        JLabel workerIdLabel = new JLabel("Worker id");
        messages.add(workerIdLabel);
        JTextField workerIdField = new JTextField(String.valueOf(workerId));
        workerIdField.addKeyListener(new KeyAdapter() {
            public void keyPressed(KeyEvent ke) {
                String value = workerIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    workerIdField.setText(value.substring(0, max(l - 1,0)));
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
                boolean subEntity = createDepartmentDataGetter(component, connection, null);
                if (subEntity ) {
                    departments.removeAllItems();
                    ListableFields.getDepartmentNames().forEach(type -> {
                        departments.addItem(type);
                    });
                    departments.repaint();
                }
            });
            messages.add(departmentsButton);
        }

        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Set department head", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            try {
                if (Integer.parseInt(workerIdField.getText()) < 1) {
                    JOptionPane.showMessageDialog(component, "Worker id should be greater then 0", "Input error", JOptionPane.ERROR_MESSAGE);
                    return false;
                }
            }catch (NumberFormatException e){
                JOptionPane.showMessageDialog(component, "Wrong worker id", "Input error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("worker_id", workerIdField.getText()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("department_head", departments.toString(), newArgs);
            } else {
                newArgs.add(new MyEntry<>("department_name", departments.toString()));
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("department_head", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if(!JSONResponseProcessor.isCorrect(result)){
                JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
            JSONObject data;
            try {
                data =(JSONObject)(new JSONParser()).parse(result);
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
            int ID = (int) data.get("worker_id");
            if (true) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Department head change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New department head added.");
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing department head", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating department head", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        } else {
            done=false;;
        }

        isGettingDepartmentHead=false;
        return done;

    }

    public static boolean createWorkerDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingWorker =true;
        boolean done=true;
        int worker_id=-1;
        String firstname="";
        String secondname="";
        String familyname="";
        int age=0;
        long salary=0;
        String department_name="";
        String worker_type="";
        if(lastArgs!=null){
            worker_id = (Integer)lastArgs.elementAt(0);
            firstname =(String)lastArgs.elementAt(1);
            secondname=(String)lastArgs.elementAt(2);
            familyname=(String)lastArgs.elementAt(3);
            age = (Integer)lastArgs.elementAt(4);
            salary = (Long)lastArgs.elementAt(5);
            department_name = (String)lastArgs.elementAt(6);
            worker_type = (String)lastArgs.elementAt(7);
        }
        Vector<Object> messages = new Vector<>();
        JLabel firstnameLabel = new JLabel("firstname");
        JTextField firstnameField = new JTextField(firstname);
        JLabel secondnameLabel = new JLabel("secondname");
        JTextField secondnameField = new JTextField(secondname);
        JLabel familynameLabel = new JLabel("familyname");
        JTextField familynameField = new JTextField(familyname);
        messages.add(firstnameLabel);
        messages.add(firstnameField);
        messages.add(secondnameLabel);
        messages.add(secondnameField);
        messages.add(familynameLabel);
        messages.add(familynameField);
        JTextField ageField = new JTextField(String.valueOf(age));
        ageField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = ageField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    ageField.setText(value.substring(0, max(l - 1,0)));
                }
            }
        });
        JLabel ageLabel = new JLabel("age");
        messages.add(ageLabel);
        messages.add(ageField);
        JTextField salaryField = new JTextField(String.valueOf(salary));
        salaryField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = salaryField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    salaryField.setText(value.substring(0, max(l - 1,0)));
                }
            }
        });
        JLabel salaryLabel = new JLabel("salary");
        messages.add(salaryLabel);
        messages.add(salaryField);
        JComboBox departments = new JComboBox(ListableFields.getDepartmentNames().toArray());
        departments.setSelectedItem(department_name);
        messages.add("Department");
        messages.add(departments);
        if (!isGettingDepartment) {
            JButton departmentsButton = new JButton("add department");
            departmentsButton.addActionListener(action -> {
                boolean subEntity = createDepartmentDataGetter(component, connection, null);
                if (subEntity ) {
                    departments.removeAllItems();
                    ListableFields.getDepartmentNames().forEach(type -> {
                        departments.addItem(type);
                    });
                    departments.repaint();
                }
            });
            messages.add(departmentsButton);
        }
        JComboBox workerTypes = new JComboBox(ListableFields.getWorkerTypes().toArray());
        departments.setSelectedItem(worker_type);
        messages.add("Worker type");
        messages.add(workerTypes);
        if (!isGettingWorkerType) {
            JButton workerTypesButton = new JButton("add worker type");
            workerTypesButton.addActionListener(action -> {
                boolean subEntity = createWorkerTypeDataGetter(component, connection, null);
                if (subEntity ) {
                    workerTypes.removeAllItems();
                    ListableFields.getWorkerTypes().forEach(type -> {
                        workerTypes.addItem(type);
                    });
                    workerTypes.repaint();
                }
            });
            messages.add(workerTypesButton);
        }
        int res =JOptionPane.showOptionDialog(component,messages.toArray(),"Worker",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if(res==JOptionPane.OK_OPTION){
            String newFirstname = firstnameField.getText();
            String newSecondname = secondnameField.getText();
            String newFamilyname = familynameField.getText();
            String newAge = ageField.getText();
            String newSalary = salaryField.getText();
            String newDepartmentName=(String)departments.getSelectedItem();
            String newWorkerType=(String)workerTypes.getSelectedItem();
            if(Integer.parseInt(newAge)<18){
                JOptionPane.showMessageDialog(component,"age should be > 17","Error input",JOptionPane.ERROR_MESSAGE);
                isGettingWorker =false;
                return false;
            }
            if(Integer.parseInt(newSalary)<1){
                JOptionPane.showMessageDialog(component,"salary should be > 0","Error input",JOptionPane.ERROR_MESSAGE);
                isGettingWorker =false;
                return false;
            }
            if(newDepartmentName=="" || newWorkerType==""){
                JOptionPane.showMessageDialog(component,"Fields couldn't be empty","Error input",JOptionPane.ERROR_MESSAGE);
                isGettingWorker =false;
                return false;
            }
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("firstname",newFirstname));
            newArgs.add(new MyEntry<>("secondname",newSecondname));
            newArgs.add(new MyEntry<>("familyname",newFamilyname));
            newArgs.add(new MyEntry<>("age",newAge));
            newArgs.add(new MyEntry<>("salary",newSalary));
            newArgs.add(new MyEntry<>("department_name",newDepartmentName));
            newArgs.add(new MyEntry<>("worker_type",newWorkerType));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.updateTableMsg("worker",String.valueOf(worker_id),newArgs);
            }else{
                JSONString =StringJSONMessageGenerator.addTableEntryMsg("worker",newArgs);
            }
            String response = connection.makeRequest(JSONString);
            if(response!=null){
                if(JSONResponseProcessor.isCorrect(response)) {
                    JSONObject data;
                    try {
                        data =(JSONObject)(new JSONParser()).parse(response);
                    } catch (ParseException e) {
                        JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                        isGettingWorker =false;
                        return false;
                    }
                    int id = (int) data.get("id");
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Worker change set. Id: " +String.valueOf(id));
                    } else {
                        JOptionPane.showMessageDialog(component, "New worker added.");
                    }
                }
            }else{
                JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                done=false;
            }
        }
        isGettingWorker =false;
        return done;
    }

    public static boolean createContractDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingContract=true;
        boolean done=true;
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
                    costField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });

        DateTimePicker startDatePicker = new DateTimePicker();
        DateTimePicker endDatePicker = new DateTimePicker();
        if(lastArgs!=null){
            DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MMM-dd HH:mm:ss");
            try {
                startDatePicker.setDateTimePermissive(LocalDateTime.parse(contract_start, dtf));
                endDatePicker.setDateTimePermissive(LocalDateTime.parse(contract_end, dtf));
            }catch (DateTimeParseException e){
                startDatePicker.setDateTimePermissive(LocalDateTime.now());
                endDatePicker.setDateTimePermissive(LocalDateTime.now());
            }
        }
        else {
            startDatePicker.setDateTimePermissive(LocalDateTime.now());
            endDatePicker.setDateTimePermissive(LocalDateTime.now());
        }
        JLabel warningLabel = new JLabel("");
        startDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        endDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        Vector<Object> messages = new Vector<>();
        messages.add("Set starting contract cost");
        messages.add(costField);
        messages.add("Set contract start date");
        messages.add(startDatePicker);
        messages.add("Set contract end date");
        messages.add(endDatePicker);
        messages.add(warningLabel);
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Set contract",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if (res==JOptionPane.OK_OPTION){
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(!startDate.isBefore(endDate)){
                JOptionPane.showMessageDialog(component,"Start date should be before end date","Input error",JOptionPane.ERROR_MESSAGE);
                return false;
            }
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("cost",costField.getText()));
            newArgs.add(new MyEntry<>("contract_start",startDate.toString()));
            newArgs.add(new MyEntry<>("contract_end",endDate.toString()));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.updateTableMsg("contract",String.valueOf(lastContractId),newArgs);
            }else{
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("contract",newArgs);
            }
            String result = connection.makeRequest(JSONString);
            MyEntry<Boolean, Integer> idContainer;
            try {
                idContainer = JSONResponseProcessor.processId(result,"contract_id");
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                isGettingContract = false;
                return false;
            }
            if (idContainer.getKey()) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Contract change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New contract added.");
                }

            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing contract", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating contract", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        }
        else{
            done=false;
        }
        isGettingContract=false;
        return done;
    }

    public static boolean createProjectDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingProject=true;
        boolean done=true;
        int lastProjectId=-1;
        int cost =0;
        if(lastArgs!=null){
            lastProjectId =(int)lastArgs.elementAt(0);
            cost = (int)lastArgs.elementAt(1);
        }
        JTextField costField = new JTextField(String.valueOf(cost));
        costField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = costField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    costField.setText(value.substring(0, max(l - 1,0)));
                }
            }
        });


        Vector<Object> messages = new Vector<>();
        if(lastArgs!=null) {
            messages.add("Set project cost");
        }else{
            messages.add("Set new project cost");
        }
        messages.add(costField);
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Set project",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if (res==JOptionPane.OK_OPTION){
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("cost",costField.getText()));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.updateTableMsg("project",String.valueOf(lastProjectId),newArgs);
            }else{
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("project",newArgs);
            }
            String result = connection.makeRequest(JSONString);
            MyEntry<Boolean, Integer> idContainer;
            try {
                idContainer = JSONResponseProcessor.processId(result,"project_id");
            } catch (ParseException e) {
                JOptionPane.showMessageDialog(component, "Error result format", "Server error", JOptionPane.ERROR_MESSAGE);
                isGettingProject = false;
                return false;
            }
            if (idContainer.getKey()) {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Project change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New project added");
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing contract", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating contract", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        }
        else{
            done=false;
        }
        isGettingProject=false;
        return done;
    }

    public static boolean createWorkerTypeDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingWorkerType = true;
        boolean done=true;
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
            newArgs.add(new MyEntry<>("worker_type", textField.getText()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("worker_types", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("worker_types", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if (result!=null) {
                if(JSONResponseProcessor.isCorrect(result)) {
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Worker type name change set");
                        ListableFields.removeWorkerType(lastArg);
                    } else {
                        JOptionPane.showMessageDialog(component, "New worker type added");
                    }

                    ListableFields.addWorkerType(textField.getText());
                }else{
                    done=false;
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing worker type name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating worker type", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        } else {
            done=false;;
        }
        isGettingWorkerType = false;
        return done;
    }

    public static boolean createGroupsDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        int id=-1;
        int group_id=0;
        int worker_id=0;
        boolean done=true;
        if(lastArgs!=null){
            id=(Integer) lastArgs.elementAt(0);
            group_id=(Integer) lastArgs.elementAt(1);
            worker_id=(Integer) lastArgs.elementAt(2);
        }
        Vector<Object> message = new Vector<>();
        JTextField groupIdField = new JTextField(String.valueOf(group_id));
        groupIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = groupIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    groupIdField.setText(value.substring(0, max(l - 1,0)));
                }
            }
        });
        JTextField workerIdField = new JTextField(String.valueOf(worker_id));
        workerIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = workerIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    workerIdField.setText(value.substring(0, max(l - 1,0)));
                }
            }
        });
        message.add(new JLabel("Group id:"));
        message.add(groupIdField);
        message.add(new JLabel("Worker id:"));
        message.add(workerIdField);
        int res = JOptionPane.showOptionDialog(component,message.toArray(),"Groups",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if(res == JOptionPane.OK_OPTION){
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            if(Integer.parseInt(groupIdField.getText())<1||Integer.parseInt(workerIdField.getText())<1){
                JOptionPane.showMessageDialog(component,"Ids should be >0","Error input",JOptionPane.ERROR_MESSAGE);
                return false;
            }
            newArgs.add(new MyEntry<>("group_id",groupIdField.getText()));
            newArgs.add(new MyEntry<>("worker_id",workerIdField.getText()));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.updateTableMsg("groups",String.valueOf(id),newArgs);
            }else{
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("groups",newArgs);
            }
            String response = connection.makeRequest(JSONString);
            if(JSONResponseProcessor.isCorrect(response)){
                JSONObject data;
                try {
                    data = (JSONObject) (new JSONParser()).parse(response);
                } catch (ParseException e) {
                    JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                    return false;
                }
                int ID = (int)data.get("id");
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Groups change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New groups added. Id: " + String.valueOf(ID));
                }

            }else {
                JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
        }else{
            done=false;
        }

        return done;
    }

    public static boolean createGroupsBindDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs){
        boolean done=true;
        int group_id=-1;
        long cost=0;
        String group_type="";
        Vector<Object> messages = new Vector<>();
        if(lastArgs!=null){
            group_id = (Integer)lastArgs.elementAt(0);
            cost = (Long)lastArgs.elementAt(1);
            group_type = (String)lastArgs.elementAt(2);
        }
        JTextField costField = new JTextField(String.valueOf(cost));
        costField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = costField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    costField.setText(value.substring(0, max(l-1 ,0)));
                }
            }
        });
        messages.add(new JLabel("Cost:" ));
        messages.add(costField);
        JComboBox groupType = new JComboBox(ListableFields.getGroupTypes().toArray());
        groupType.setSelectedItem(group_type);
        messages.add("Group type");
        messages.add(groupType);
        if (!isGettingGroupType) {
            JButton groupTypesButton = new JButton("add group type");
            groupTypesButton.addActionListener(action -> {
                boolean subEntity = createGroupTypeDataGetter(component, connection, null);
                if (subEntity) {
                    groupType.removeAllItems();
                    ListableFields.getGroupTypes().forEach(type -> {
                        groupType.addItem(type);
                    });
                    groupType.repaint();
                }
            });
            messages.add(groupTypesButton);
        }
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Groups bind",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if(res == JOptionPane.OK_OPTION){
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            if(Integer.parseInt(costField.getText())<1){
                JOptionPane.showMessageDialog(component,"Cost should be >0","Error input",JOptionPane.ERROR_MESSAGE);
                return false;
            }
            newArgs.add(new MyEntry<>("cost",costField.getText()));
            newArgs.add(new MyEntry<>("group_type",(String)groupType.getSelectedItem()));
            String JSONString;
            if(lastArgs!=null){
                JSONString = StringJSONMessageGenerator.updateTableMsg("groups_bind",String.valueOf(group_id),newArgs);
            }else{
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("groups_bind",newArgs);
            }
            String response = connection.makeRequest(JSONString);
            if(JSONResponseProcessor.isCorrect(response)){
                JSONObject data;
                try {
                    data = (JSONObject) (new JSONParser()).parse(response);
                } catch (ParseException e) {
                    JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                    return false;
                }
                long ID = (long)data.get("id");
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Groups bind change set");
                } else {
                    JOptionPane.showMessageDialog(component, "New groups bind added. Id: " + String.valueOf(ID));
                }

            }else {
                JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                return false;
            }
        }else{
            done=false;
        }
        return done;
    }

    public static boolean createGroupTypeDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs){
        isGettingGroupType = true;
        boolean done=true;
        String lastArg;
        if (lastArgs != null) {
            lastArg = (String) lastArgs.elementAt(0);
        } else {
            lastArg = "";
        }
        JTextField textField = new JTextField(lastArg);
        Object[] message = {
                "Group type name: ", textField
        };
        int res = JOptionPane.showOptionDialog(component, message, "Change group type name", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if (res == JOptionPane.OK_OPTION) {
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("group_type", textField.getText()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("group_type", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("group_type", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if (result!=null) {
                if(JSONResponseProcessor.isCorrect(result)) {
                    JOptionPane.showMessageDialog(component, "Error at server", "Error", JOptionPane.ERROR_MESSAGE);
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Group type name change set");
                        ListableFields.removeGroupType(lastArg);
                    } else {
                        JOptionPane.showMessageDialog(component, "New group type added");
                    }
                    ListableFields.addGroupType(textField.getText());
                }else{
                    done=false;
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing group type name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating group type", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        } else {
            done=false;;
        }
        isGettingGroupType = false;
        return done;
    }

    public static boolean createProjectContractBinderDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs){
        boolean done=true;
        int contract_id=-1;
        int project_id=-1;
        int group_id=-1;
        int head_id=-1;
        int eq_list_id=-1;
        String contract_start="";
        String contract_end="";
        if(lastArgs!=null){
            JOptionPane.showMessageDialog(component,"This table couldn't be edited","Error",JOptionPane.ERROR_MESSAGE);
            return false;
        }
        JTextField contractIdField = new JTextField(String.valueOf(contract_id));
        contractIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = contractIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    contractIdField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });
        JTextField projectIdField = new JTextField(String.valueOf(project_id));
        projectIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = projectIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    projectIdField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });
        JTextField groupIdField = new JTextField(String.valueOf(group_id));
        groupIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = groupIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    groupIdField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });

        JTextField headIdField = new JTextField(String.valueOf(head_id));
        headIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = headIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    headIdField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });
        JTextField eqListIdField = new JTextField(String.valueOf(eq_list_id));
        eqListIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = eqListIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    eqListIdField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });

        DateTimePicker startDatePicker = new DateTimePicker();
        DateTimePicker endDatePicker = new DateTimePicker();
        if(lastArgs!=null){
            DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MMM-dd HH:mm:ss");
            try {
                startDatePicker.setDateTimePermissive(LocalDateTime.parse(contract_start, dtf));
                endDatePicker.setDateTimePermissive(LocalDateTime.parse(contract_end, dtf));
            }catch (DateTimeParseException e){
                startDatePicker.setDateTimePermissive(LocalDateTime.now());
                endDatePicker.setDateTimePermissive(LocalDateTime.now());
            }
        }
        else {
            startDatePicker.setDateTimePermissive(LocalDateTime.now());
            endDatePicker.setDateTimePermissive(LocalDateTime.now());
        }
        JLabel warningLabel = new JLabel("");
        startDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        endDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        Vector<Object> messages = new Vector<>();
        messages.add("Set contract id");
        messages.add(contractIdField);
        messages.add("Set project id");
        messages.add(projectIdField);
        messages.add("Set group id");
        messages.add(groupIdField);
        messages.add("Set head id");
        messages.add(headIdField);
        messages.add("Set contract start date");
        messages.add(startDatePicker);
        messages.add("Set contract end date");
        messages.add(endDatePicker);
        messages.add(warningLabel);
        messages.add("Set equipment list id");
        messages.add(eqListIdField);

        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Project Contract bind", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if(res==JOptionPane.OK_OPTION){
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(!startDate.isBefore(endDate)){
                JOptionPane.showMessageDialog(component,"Start date should be before end date","Input error",JOptionPane.ERROR_MESSAGE);
                return false;
            }
            try {
                if (Integer.parseInt(contractIdField.getText()) < 1
                        || Integer.parseInt(projectIdField.getText()) < 1
                        || Integer.parseInt(groupIdField.getText()) < 1
                        || Integer.parseInt(headIdField.getText()) < 1
                        || Integer.parseInt(eqListIdField.getText()) < 1) {
                    JOptionPane.showMessageDialog(component,"Ids should be >0","Error",JOptionPane.ERROR_MESSAGE);
                    return false;
                }
            }catch(NumberFormatException e){
                JOptionPane.showMessageDialog(component,"Number format error","Error",JOptionPane.ERROR_MESSAGE);
                return false;
            }
            Vector<MyEntry<String, String>> newArgs = new Vector<>();
            newArgs.add(new MyEntry<>("contract_id",contractIdField.getText()));
            newArgs.add(new MyEntry<>("project_id",projectIdField.getText()));
            newArgs.add(new MyEntry<>("group_id",groupIdField.getText()));
            newArgs.add(new MyEntry<>("head_id",headIdField.getText()));
            newArgs.add(new MyEntry<>("eq_list_id",eqListIdField.getText()));
            newArgs.add(new MyEntry<>("contract_start",startDate.toString()));
            newArgs.add(new MyEntry<>("contract_end",endDate.toString()));
            String JSONString = StringJSONMessageGenerator.addTableEntryMsg("pc_bind",newArgs);
            String response = connection.makeRequest(JSONString);
            if(JSONResponseProcessor.isCorrect(response)){
                JSONObject data;
                try {
                    data = (JSONObject) (new JSONParser()).parse(response);
                } catch (ParseException e) {
                    JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                    return false;
                }
                long ID = (long)data.get("id");
                JOptionPane.showMessageDialog(component, "New contract project bind added");

            }else{
                JOptionPane.showMessageDialog(component,"Error","Error",JOptionPane.ERROR_MESSAGE);
                done=false;
            }
        }else{
            done=false;
        }
        return done;
    }

    public static boolean createCompanyDataGetter(Component component, JSONConnection connection, Vector<Object> lastArgs) {
        isGettingCompany = true;
        boolean done=true;
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
            newArgs.add(new MyEntry<>("company_name", textField.getText()));
            String JSONString;
            if (lastArgs != null) {
                JSONString = StringJSONMessageGenerator.updateTableMsg("company", lastArg, newArgs);
            } else {
                JSONString = StringJSONMessageGenerator.addTableEntryMsg("company", newArgs);
            }
            String result = connection.makeRequest(JSONString);
            if (result!=null) {
                if(JSONResponseProcessor.isCorrect(result)) {
                    if (lastArgs != null) {
                        JOptionPane.showMessageDialog(component, "Company name change set");
                        ListableFields.removeCompany(lastArg);
                    } else {
                        JOptionPane.showMessageDialog(component, "New company added");
                    }

                    ListableFields.addCompany(textField.getText());
                }else{
                    JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                    done=false;
                }
            } else {
                if (lastArgs != null) {
                    JOptionPane.showMessageDialog(component, "Error at changing company name", "Error", JOptionPane.ERROR_MESSAGE);
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating company", "Error", JOptionPane.ERROR_MESSAGE);
                }
                done=false;;
            }
        } else {
            done=false;;
        }
        isGettingCompany = false;
        return done;
    }
    //todo forms
    public static String AVGSalaryByWorkerTypeForm(Component component, JSONConnection connection){
        String JSONString = StringJSONMessageGenerator.getAVGSalaryByWorkerTypeForm();
        String response = connection.makeRequest(JSONString);
        if(JSONResponseProcessor.isCorrect(response)){
            return response;
        }else{
            JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
            return null;
        }
    }

    public static String ContractsAtTimeForm(Component component, JSONConnection connection){
        DateTimePicker startDatePicker = new DateTimePicker();
        DateTimePicker endDatePicker = new DateTimePicker();
        startDatePicker.setDateTimePermissive(LocalDateTime.now());
        endDatePicker.setDateTimePermissive(LocalDateTime.now());
        JLabel warningLabel = new JLabel("");
        startDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimeStrict();
            LocalDateTime endDate = endDatePicker.getDateTimeStrict();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        endDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimeStrict();
            LocalDateTime endDate = endDatePicker.getDateTimeStrict();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        Vector<Object> messages = new Vector<>();
        messages.add("Set contract start date");
        messages.add(startDatePicker);
        messages.add("Set contract end date");
        messages.add(endDatePicker);
        messages.add(warningLabel);
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Get contracts",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if(res == JOptionPane.OK_OPTION){
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(!startDate.isBefore(endDate)){
                JOptionPane.showMessageDialog(component,"Start date should be before end date","Input error",JOptionPane.ERROR_MESSAGE);
                return null;
            }
            String JSONString = StringJSONMessageGenerator.getContractsAtTimeForm(startDate.toString(),endDate.toString());
            String response = connection.makeRequest(JSONString);
            if(JSONResponseProcessor.isCorrect(response)){
                return response;
            }else{
                JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                return null;
            }
        }
        return null;
    }

    public static String ContractsByProjectFrom(Component component, JSONConnection connection){
        JTextField projectIdField = new JTextField(String.valueOf(0));
        projectIdField.addKeyListener(new KeyAdapter() {
            @Override
            public void keyPressed(KeyEvent ke) {
                String value = projectIdField.getText();
                int l = value.length();
                if (ke.getKeyChar() >= '0' && ke.getKeyChar() <= '9') {
                } else {
                    projectIdField.setText(value.substring(0,max(l - 1,0)));
                }
            }
        });
        Vector<Object> messages = new Vector<>();
        messages.add(new Label("Enter project id"));
        messages.add(projectIdField);
        int res = JOptionPane.showOptionDialog(component, messages.toArray(), "Get contracts", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if(res==JOptionPane.OK_OPTION){
            try {
                if (Integer.parseInt(projectIdField.getText()) < 1) {
                    JOptionPane.showMessageDialog(component, "ID should be > 0", "Input error", JOptionPane.ERROR_MESSAGE);
                    return null;
                }
            }catch(NumberFormatException e){
                JOptionPane.showMessageDialog(component, "Wrong number format", "Input error", JOptionPane.ERROR_MESSAGE);
                return null;
            }
            String JSONString = StringJSONMessageGenerator.getContractsByProjectForm(projectIdField.getText());
            String response = connection.makeRequest(JSONString);
            if(JSONResponseProcessor.isCorrect(response)){
                return response;
            }else{
                JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                return null;
            }
        }else{
            return null;
        }
    }

    public static String ContractsCostByTimeForm(Component component, JSONConnection connection){
        DateTimePicker startDatePicker = new DateTimePicker();
        DateTimePicker endDatePicker = new DateTimePicker();
        startDatePicker.setDateTimePermissive(LocalDateTime.now());
        endDatePicker.setDateTimePermissive(LocalDateTime.now());
        JLabel warningLabel = new JLabel("");
        startDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimeStrict();
            LocalDateTime endDate = endDatePicker.getDateTimeStrict();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        endDatePicker.addDateTimeChangeListener(date->{
            LocalDateTime startDate = startDatePicker.getDateTimeStrict();
            LocalDateTime endDate = endDatePicker.getDateTimeStrict();
            if(startDate.isBefore(endDate)){
                warningLabel.setText("");
            }else{
                warningLabel.setText("Start date should be before end date");
            }
        });
        Vector<Object> messages = new Vector<>();
        messages.add("Set contract start date");
        messages.add(startDatePicker);
        messages.add("Set contract end date");
        messages.add(endDatePicker);
        messages.add(warningLabel);
        int res = JOptionPane.showOptionDialog(component,messages.toArray(),"Get contract cost",JOptionPane.OK_CANCEL_OPTION,JOptionPane.PLAIN_MESSAGE,null,null,null);
        if(res == JOptionPane.OK_OPTION){
            LocalDateTime startDate = startDatePicker.getDateTimePermissive();
            LocalDateTime endDate = endDatePicker.getDateTimePermissive();
            if(!startDate.isBefore(endDate)){
                JOptionPane.showMessageDialog(component,"Start date should be before end date","Input error",JOptionPane.ERROR_MESSAGE);
                return null;
            }
            String JSONString = StringJSONMessageGenerator.getContractsAtTimeForm(startDate.toString(),endDate.toString());
            String response = connection.makeRequest(JSONString);
            if(JSONResponseProcessor.isCorrect(response)){
                return response;
            }else{
                JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
                return null;
            }
        }
        return null;
    }
    public static String ContractsEfForm(Component component, JSONConnection connection){
        String JSONString = StringJSONMessageGenerator.getContractsEfForm();
        String response = connection.makeRequest(JSONString);
        if(JSONResponseProcessor.isCorrect(response)){
            return response;
        }else{
            JOptionPane.showMessageDialog(component, "Error", "Error", JOptionPane.ERROR_MESSAGE);
            return null;
        }
    }
    
}