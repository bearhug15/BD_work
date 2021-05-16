package generators;

import Entities.Entity;
import main.JSONConnection;
import utils.MyEntry;

import javax.swing.*;
import java.awt.*;
import java.util.Vector;


public class DataGetterGenerator {
    public static Entity createEquipmentTypeDataGetter(Component component, JSONConnection connection, String name, Vector<Object> lastArgs) {
        Entity entity = new Entity();
        if (lastArgs != null) {
            String lastArg = (String) lastArgs.elementAt(1);
            JTextField textField = new JTextField(lastArg);
            Object[] message = {
                    "Equipment type", textField
            };
            int res = JOptionPane.showOptionDialog(component, message, "Change equipment type", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
            if (res == JOptionPane.OK_OPTION) {
                Vector<MyEntry<String, String>> newArgs = new Vector<>();
                newArgs.add(new MyEntry<>("type", textField.toString()));
                boolean result = connection.makeStatement(StringJSONMessageGenerator.changeTableMsg("equipment_type", lastArg, newArgs));
                if (result) {
                    JOptionPane.showMessageDialog(component, "Type name");
                    entity.addField("type", textField.toString(), "TEXT");
                    return entity;
                } else {
                    JOptionPane.showMessageDialog(component, "Error at changing type", "Error", JOptionPane.ERROR_MESSAGE);
                    return null;
                }
            } else {
                return null;
            }
        } else {
            JTextField textField = new JTextField();
            Object[] message = {
                    "Equipment type", textField
            };
            int res = JOptionPane.showOptionDialog(component, message, "Change equipment type", JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
            if (res == JOptionPane.OK_OPTION) {
                Vector<MyEntry<String, String>> newArgs = new Vector<>();
                newArgs.add(new MyEntry<>("type", textField.toString()));
                boolean result = connection.makeStatement(StringJSONMessageGenerator.addTableEntryMsg("equipment_type", newArgs));
                if (result) {
                    JOptionPane.showMessageDialog(component, "Type name");
                    entity.addField("type", textField.toString(), "TEXT");
                    return entity;
                } else {
                    JOptionPane.showMessageDialog(component, "Error at creating new type", "Error", JOptionPane.ERROR_MESSAGE);
                    return null;
                }
            } else {
                return null;
            }
        }
    }
    public static Entity createEquipmentDataGetter(Component component, JSONConnection connection, String name, Vector<Object> lastArgs,Vector<String> knownTypes){
        Entity entity = new Entity();
        if (lastArgs!=null){
            int id = (int) lastArgs.elementAt(0);
            String lastName  = (String) lastArgs.elementAt(1);
            int department_id = (int) lastArgs.elementAt(2);
            String lastType = (String) lastArgs.elementAt(3);
            JTextField nameField = new JTextField(lastName);
            JTextField departmentField = new JTextField(String.valueOf(department_id));
            JComboBox types = new JComboBox(knownTypes);
            types.setSelectedItem(lastType);

        }else{

        }
        return entity;
    }
    public static Entity createEquipmentGroupDataGetter(Component component, JSONConnection connection, String name, Vector<Object> lastArgs){ }
    public static Entity createDepartmentDataGetter(){}
    public static Entity createDepartmentHead(){}
    public static Entity createWorkerDataGetter(){}
    public static Entity createContractDataGetter(){}
    public static Entity createProjectDataGetter(){}
    public static Entity createWorkerTypeDataGetter(){}
    public static Entity createGroupDataGetter(){}
    public static Entity createCompany(){}
}