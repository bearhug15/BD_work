import javax.swing.*;
import javax.swing.colorchooser.AbstractColorChooserPanel;
import javax.swing.event.ChangeEvent;
import javax.swing.event.ChangeListener;
import java.awt.*;
import java.awt.event.KeyAdapter;
import java.awt.event.KeyEvent;
import java.util.Hashtable;
import java.util.Vector;

public class DataGetterCreator {
    static public Vector<Object> createDataGetter(Component component,String name, String oldParams, Vector<Object> lastArgs){
        String[] params = oldParams.split(",");
        if(params.length%5!=0){
            System.out.println("Wrong params - length");
            return null;
        }
        Object[] message = new Object[1];
        JPanel scrollable = new JPanel(new GridLayout(0,1));
        JScrollPane scrollPane = new JScrollPane(scrollable);
        scrollPane.setPreferredSize(new Dimension(100,300));
        for(int i=0;i<params.length/5;++i){
            String paramName = params[i*5];
            switch(params[i*5+1]){
                case "i":{
                    try {
                        int start = Integer.parseInt(params[i * 5 + 2]);
                        int end = Integer.parseInt(params[i * 5 + 3]);
                        int step = Integer.parseInt(params[i * 5 + 4]);
                        String messageText= "Enter "+paramName+ " from "+String.valueOf(start)+" to "+String.valueOf(end);
                        int startArgs=start;
                        if(lastArgs!=null){
                            startArgs = (Integer)lastArgs.elementAt(i);
                        }
                        JTextField field = new JTextField(String.valueOf(startArgs));
                        JSlider slider = new JSlider(JSlider.HORIZONTAL,start,end,startArgs);
                        slider.setMinorTickSpacing((end-start)/10>1?(end-start)/10:1);
                        slider.setPaintTicks(true);
                        slider.setPaintLabels(true);
                        Hashtable sliderTable = new Hashtable();
                        sliderTable.put(start,new JLabel(String.valueOf(start)));
                        sliderTable.put(end,new JLabel(String.valueOf(end)));
                        if(end-start>0){
                            sliderTable.put((end+start)/2,new JLabel(String.valueOf((end+start)/2)));
                        }
                        slider.setLabelTable(sliderTable);
                        slider.addChangeListener(new ChangeListener() {
                            @Override
                            public void stateChanged(ChangeEvent changeEvent) {
                                slider.setValue(((slider.getValue()-start)/step)*step+start);
                                field.setText(String.valueOf(slider.getValue()));
                            }
                        });
                        field.addKeyListener(new KeyAdapter(){
                            @Override
                            public void keyReleased(KeyEvent ke) {
                                String typed = field.getText();
                                slider.setValue(0);
                                /*if(!typed.matches("\\d+") || typed.length() > 3) {
                                    return;
                                }*/
                                if(typed=="-" ||typed.isBlank()){
                                    return;
                                }
                                int value;
                                try {
                                    value = Integer.parseInt(typed);
                                }catch(NumberFormatException e){
                                    slider.setValue(0);
                                    return;
                                }
                                value = ((value-start)/step)*step+start;
                                field.setText(String.valueOf(value));
                                slider.setValue(value);
                            }
                        });
                        scrollable.add(new JLabel(messageText));
                        scrollable.add(field);
                        scrollable.add(slider);
                    } catch(NumberFormatException e){
                        System.out.println("Wrong params - int");
                        e.printStackTrace();
                        return null;
                    }
                    break;
                }
                case "d":{
                    try {
                        double start = Double.parseDouble(params[i * 5 + 2]);
                        double end = Double.parseDouble(params[i * 5 + 3]);
                        double step = Double.parseDouble(params[i * 5 + 4]);
                        String messageText= "Enter "+paramName+ " from "+String.valueOf(start)+" to "+String.valueOf(end);
                        double startArgs=start;
                        if(lastArgs!=null){
                            startArgs = (Double)lastArgs.elementAt(i);
                        }
                        JTextField field = new JTextField(String.valueOf(startArgs));
                        JSlider slider = new JSlider(JSlider.HORIZONTAL,(int)(start/step),(int)(end/step)+1,(int)(startArgs/step));
                        slider.setMinorTickSpacing(Math.max(((int) (end - start)) / 10, 1));
                        slider.setPaintTicks(true);
                        slider.setPaintLabels(true);
                        Hashtable sliderTable = new Hashtable();
                        sliderTable.put((int)(start/step),new JLabel(String.valueOf(start)));
                        sliderTable.put((int)(end/step),new JLabel(String.valueOf(end)));
                        if(end-start>0){
                            sliderTable.put((int)((end+start)/2),new JLabel(String.valueOf((end+start)/2)));
                        }
                        slider.setLabelTable(sliderTable);
                        slider.addChangeListener(new ChangeListener() {
                            @Override
                            public void stateChanged(ChangeEvent changeEvent) {
                                double value = (((slider.getValue()*step-start)/step)*step+start);
                                if(value>end){
                                    value=(end/step);
                                }else{
                                    value/=step;
                                }
                                slider.setValue((int)value);
                                field.setText(String.valueOf(value*step));
                            }
                        });
                        field.addKeyListener(new KeyAdapter(){
                            @Override
                            public void keyReleased(KeyEvent ke) {
                                String typed = field.getText();
                                slider.setValue(0);
                                /*if(!typed.matches("\\d+") || typed.length() > 3) {
                                    return;
                                }*/
                                if(typed=="-" ||typed.isBlank()){
                                    return;
                                }
                                double value;
                                try {
                                    value = Double.parseDouble(typed);
                                }catch(NumberFormatException e){
                                    value=0;
                                    slider.setValue(0);
                                    return;
                                }
                                value = ((double)((((int)((value-start)/step))/step)*step))*step+start;
                                field.setText(String.valueOf(value));
                                slider.setValue((int)value);
                            }
                        });
                        scrollable.add(new JLabel(messageText));
                        scrollable.add(field);
                        scrollable.add(slider);
                    } catch(NumberFormatException e){
                        System.out.println("Wrong params - double");
                        e.printStackTrace();
                        return null;
                    }
                    break;
                }
                case "f":{
                    try {
                        float start = Float.parseFloat(params[i * 5 + 2]);
                        float end = Float.parseFloat(params[i * 5 + 3]);
                        float step = Float.parseFloat(params[i * 5 + 4]);
                        String messageText= "Enter "+paramName+ " from "+String.valueOf(start)+" to "+String.valueOf(end);
                        float startArgs=start;
                        if(lastArgs!=null){
                            startArgs = (Float)lastArgs.elementAt(i);
                        }
                        JTextField field = new JTextField(String.valueOf(startArgs));
                        JSlider slider = new JSlider(JSlider.HORIZONTAL,(int)(start/step),(int)(end/step)+1,(int)(startArgs/step));
                        slider.setMinorTickSpacing(Math.max(((int) (end - start)) / 10, 1));
                        slider.setPaintTicks(true);
                        slider.setPaintLabels(true);
                        Hashtable sliderTable = new Hashtable();
                        sliderTable.put((int)(start/step),new JLabel(String.valueOf(start)));
                        sliderTable.put((int)(end/step),new JLabel(String.valueOf(end)));
                        if(end-start>0){
                            sliderTable.put((int)((end+start)/2),new JLabel(String.valueOf((end+start)/2)));
                        }
                        slider.setLabelTable(sliderTable);
                        slider.addChangeListener(new ChangeListener() {
                            @Override
                            public void stateChanged(ChangeEvent changeEvent) {
                                float value = (((slider.getValue()*step-start)/step)*step+start);
                                if(value>end){
                                    value=(end/step);
                                }else{
                                    value/=step;
                                }
                                slider.setValue((int)value);
                                field.setText(String.valueOf(value*step));
                            }
                        });
                        field.addKeyListener(new KeyAdapter(){
                            @Override
                            public void keyReleased(KeyEvent ke) {
                                String typed = field.getText();
                                //slider.setValue(0);
                                /*if(!typed.matches("\\d+") || typed.length() > 3) {
                                    return;
                                }*/
                                if(typed.equals("-") ||typed.isBlank() || typed.contains(".")){
                                    return;
                                }
                                double value;
                                try {
                                    value = Float.parseFloat(typed);
                                }catch(NumberFormatException e){
                                    slider.setValue(0);
                                    return;
                                }
                                value = ((float)((((int)((value-start)/step))/step)*step))*step+start;
                                field.setText(String.valueOf(value));
                                slider.setValue((int)value);
                            }
                        });
                        scrollable.add(new JLabel(messageText));
                        scrollable.add(field);
                        scrollable.add(slider);
                    } catch(NumberFormatException e){
                        System.out.println("Wrong params - double");
                        e.printStackTrace();
                        return null;
                    }
                    break;
                }
                case "c":{
                    try {
                        JColorChooser colorChooser = new JColorChooser();
                        AbstractColorChooserPanel[] panels = colorChooser.getChooserPanels();
                        for (AbstractColorChooserPanel accp : panels) {
                            if (!accp.getDisplayName().equals("RGB")) {
                                colorChooser.removeChooserPanel(accp);
                            }
                        }
                        int startR = Integer.parseInt(params[i * 5 + 2]);
                        int startG = Integer.parseInt(params[i * 5 + 3]);
                        int startB = Integer.parseInt(params[i * 5 + 4]);
                        if (startR < 0 || startR > 255 || startG < 0 || startG > 255 || startB < 0 || startB > 255) {
                            return null;
                        }
                        Color oldColor = new Color(0xFF000000 + (startR << 16) + (startG << 8) + startB);
                        colorChooser.setColor(oldColor);

                        JPanel colorComp = new JPanel();
                        JTextField compR = new JTextField();
                        compR.setText(String.valueOf(startR));
                        compR.setPreferredSize(new Dimension(30,15));
                        JTextField compG = new JTextField();
                        compG.setText(String.valueOf(startG));
                        compG.setPreferredSize(new Dimension(30,15));
                        JTextField compB = new JTextField();
                        compB.setText(String.valueOf(startB));
                        compB.setPreferredSize(new Dimension(30,15));

                        Button colorChooserButton = new Button(paramName);
                        colorChooserButton.setBackground(new Color((startR<<16)+(startG<<8)+startB));
                        if((startR+startG+startB)/3<128){
                            colorChooserButton.setForeground(Color.WHITE);
                        }else{
                            colorChooserButton.setForeground(Color.BLACK);
                        }
                        colorComp.add(compR, BorderLayout.WEST);
                        colorComp.add(compG, BorderLayout.CENTER);
                        colorComp.add(compB, BorderLayout.EAST);
                        colorChooserButton.addActionListener(actionEvent -> {
                            int res = JOptionPane.showOptionDialog(component, colorChooser, "Choose" + paramName, JOptionPane.OK_CANCEL_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
                            if (res == JOptionPane.OK_OPTION) {
                                Color color = colorChooser.getColor();
                                compR.setText(String.valueOf(color.getRed()));
                                compG.setText(String.valueOf(color.getGreen()));
                                compB.setText(String.valueOf(color.getBlue()));
                                colorChooserButton.setBackground(color);
                                if((color.getRed()+color.getGreen()+color.getBlue())/3<128){
                                    colorChooserButton.setForeground(Color.WHITE);
                                }else{
                                    colorChooserButton.setForeground(Color.BLACK);
                                }
                                //colorComp.repaint();
                            }
                        });
                        compR.addKeyListener(new KeyAdapter() {
                            @Override
                            public void keyReleased(KeyEvent ke) {
                                String typed = compR.getText();
                                /*if (!typed.matches("\\d+") || typed.length() > 3) {
                                    return;
                                }*/
                                if(typed.isBlank()){
                                    return;
                                }
                                int value,G,B;
                                try {
                                    value = Integer.parseInt(typed);
                                    if (value < 0) {
                                        value=0;
                                        compR.setText("0");
                                    }
                                    if (value > 255) {
                                        value=255;
                                        compR.setText("255");
                                    }
                                    G = Integer.parseInt(compG.getText());
                                    B = Integer.parseInt(compB.getText());
                                }catch (NumberFormatException e){
                                    value=0;
                                    compR.setText("0");
                                    return;
                                }
                                colorChooser.setColor(0xFF000000 + (value << 16) + (G << 8) + B);
                            }
                        });
                        compG.addKeyListener(new KeyAdapter() {
                            @Override
                            public void keyReleased(KeyEvent ke) {
                                String typed = compG.getText();
                                /*if (!typed.matches("\\d+") || typed.length() > 3) {
                                    return;
                                }*/
                                if(typed.isBlank()){
                                    return;
                                }
                                int value, R,B;
                                try {
                                    value = Integer.parseInt(typed);
                                    if (value < 0) {
                                        value=0;
                                        compG.setText("0");
                                    }
                                    if (value > 255) {
                                        value=255;
                                        compG.setText("255");
                                    }
                                    R = Integer.parseInt(compR.getText());
                                    B = Integer.parseInt(compB.getText());
                                }catch (NumberFormatException e){
                                    value=0;
                                    compG.setText("0");
                                    return;
                                }
                                colorChooser.setColor(0xFF000000 + (R << 16) + (value << 8) + B);
                            }
                        });
                        compB.addKeyListener(new KeyAdapter() {
                            @Override
                            public void keyReleased(KeyEvent ke) {
                                String typed = compB.getText();
                                /*if (!typed.matches("\\d+") || typed.length() > 3) {
                                    return;
                                }*/
                                if(typed.isBlank()){
                                    return;
                                }
                                int value, R, G;
                                try {
                                    value = Integer.parseInt(typed);
                                    if (value < 0) {
                                        value=0;
                                        compB.setText("0");
                                    }
                                    if (value > 255) {
                                        value=255;
                                        compB.setText("255");
                                    }
                                    G = Integer.parseInt(compG.getText());
                                    R = Integer.parseInt(compR.getText());
                                }catch (NumberFormatException e){
                                    value=0;
                                    compB.setText("0");
                                    return;
                                }
                                colorChooser.setColor(0xFF000000 + (R << 16) + (G << 8) + value);
                            }
                        });
                        scrollable.add(new JLabel("Choose color " + paramName));
                        scrollable.add(colorChooserButton);
                        scrollable.add(colorComp);
                    }catch (NumberFormatException e){
                        System.out.println("Wrong params - color");
                        e.printStackTrace();
                        return null;
                    }
                    break;
                }
                case "s":{
                    String startString="";
                    if(lastArgs!=null) {
                        try {
                            startString = (String) lastArgs.elementAt(i);
                        } catch (Exception e) {
                            System.out.println("Wrong last args");
                        }
                    }
                    JTextField textField = new JTextField(startString);
                    JLabel label = new JLabel("Set "+paramName);
                    scrollPane.add(label);
                    scrollPane.add(textField);
                    scrollPane.add((Component) null);
                }
                default: System.out.println("Wrong params - undefined type");return null;
            }
        }
        message[0]=scrollPane;
        int res = JOptionPane.showOptionDialog(component,message,name,JOptionPane.OK_OPTION, JOptionPane.PLAIN_MESSAGE, null, null, null);
        if(res != JOptionPane.OK_OPTION){
            return null;
        }
        Vector<Object> param = new Vector<>();
        Component[] components =scrollable.getComponents();
        for(int i=0;i<params.length/5;++i){
            switch (params[i*5+1]){
                case "i":{
                    String text = ((JTextField) components[i * 3 + 1]).getText();
                    param.add(Integer.parseInt(text));
                    break;
                }
                case "d": {
                    String text = ((JTextField) components[i * 3 + 1]).getText();
                    param.add(Double.parseDouble(text));
                    break;
                }
                case "f": {
                    String text = ((JTextField) components[i * 3 + 1]).getText();
                    param.add(Float.parseFloat(text));
                    break;
                }
                case "c":{
                    JPanel p = ((JPanel)components[i*3+2]);
                    Component[] comp = p.getComponents();
                    int colorR = Integer.parseInt(((JTextField)comp[0]).getText());
                    int colorG = Integer.parseInt(((JTextField)comp[1]).getText());
                    int colorB = Integer.parseInt(((JTextField)comp[2]).getText());
                    param.add(0xFF000000+(colorR<<16)+(colorG<<8)+colorB);
                }
                case "s":{
                    String text = ((JTextField) components[i * 3 + 1]).getText();
                }
            }
        }
        return param;
    }
}
