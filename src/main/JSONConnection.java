package main;

import javax.swing.*;
import java.awt.*;
import java.io.*;
import java.net.Socket;


import generators.StringJSONMessageGenerator;
import main.CancelledConnectionCreation;


public class JSONConnection {
    Socket socket;
    BufferedReader in;
    BufferedWriter out;

    JSONConnection(Component component, String address, int port) throws CancelledConnectionCreation {
        while(true) {
            try {
                socket = getConnection(address, port);
                in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
                out = new BufferedWriter(new OutputStreamWriter(socket.getOutputStream()));
                JOptionPane.showMessageDialog(component, "Connection established");
                break;
            } catch (IOException e) {
                int res = JOptionPane.showOptionDialog(component, "Error in connecting to database. Try again?", "Error", JOptionPane.OK_CANCEL_OPTION, JOptionPane.ERROR_MESSAGE, null, null, null);
                if (res != JOptionPane.OK_OPTION) {
                    throw new CancelledConnectionCreation("Connection was cancelled after error");
                }
            }
        }

    }

    public Socket getConnection(String address, int port)throws IOException {
        Socket connection = new Socket(address,port);
        return connection;
    }
    //todo
    public String getInitData() throws IOException{
        String jsonString = StringJSONMessageGenerator.getInitDataMsg();
        out.write(jsonString);
        String initString =in.readLine();
        return initString;
    }

    public String makeRequest(String JSONRequest){
        String JSONResponse;
        try {
            out.write(JSONRequest);
            JSONResponse = in.readLine();
        }catch (IOException e){
            return null;
        }
        return JSONResponse;
    }
    public boolean makeStatement(String JSONStatement){
        try{
            out.write(JSONStatement);
        }catch (IOException e){
            return false;
        }
        return true;
    }

    public void close(){
        try {
            out.close();
            in.close();
            socket.close();
        }catch (IOException e) {
            System.out.println("Error at connection closing");
            //e.printStackTrace();
        }
    }
}


