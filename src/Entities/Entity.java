package Entities;

import utils.MyEntry;

import javax.swing.*;
import java.util.Vector;

class Entity extends JButton {
    Vector<MyEntry<JTextField,String>> fields = new Vector<>();
    public void addField(String fieldName,Object value, String type){
        JTextField field = new JTextField(String.valueOf(value));
        field.setEditable(false);
        fields.add(new MyEntry<JTextField, String>(field,type));
        add(field);
    }

    public void deleteField(JTextField value){
        try{
            remove(value);
        }catch (Exception e){
        }
        fields.remove(new MyEntry<>(value,null));
    }

    public Vector<MyEntry<JTextField, String>> getFields() {
        return fields;
    }
}
