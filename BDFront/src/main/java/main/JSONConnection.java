package main;

import javax.swing.*;
import java.awt.*;
import java.io.*;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.util.Arrays;


import generators.StringJSONMessageGenerator;
import main.CancelledConnectionCreation;
import utils.MyEntry;


public class JSONConnection {
    Socket socket;
    InputStream in;
    OutputStream out;

    JSONConnection(Component component, String address, int port) throws CancelledConnectionCreation {
        while(true) {
            try {
                socket = getConnection(address, port);
                in = socket.getInputStream();
                out = socket.getOutputStream();
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

    public byte[] longToBytes(long x) {
        ByteBuffer buffer = ByteBuffer.allocate(Long.BYTES);
        buffer.putLong(x);
        return buffer.array();
    }

    public long bytesToLong(byte[] bytes) {
        ByteBuffer buffer = ByteBuffer.allocate(Long.BYTES);
        buffer.put(bytes);
        buffer.flip();//need flip
        return buffer.getLong();
    }
    //todo

    private String getData()throws IOException{
        int i=0;
        byte[] prelude = new byte[8];
        while (i<8){
            i+=in.read(prelude,i,8-i);
        }
        long length = bytesToLong(prelude);
        byte[] message = new byte[1024];
        StringBuilder builder = new StringBuilder();
        while (length>0){
            int loc_size =in.read(message,0, (int) Math.min(1024,length));
            length-=loc_size;
            //todo
            if(loc_size<1024) {
                builder.append(new String(Arrays.copyOfRange(message,0,loc_size)));
            }
            else {
                builder.append(new String(message));
            }
        }
        return builder.toString();
    }
    private void sendData(String data) throws IOException{
        byte[] b = data.getBytes();
        out.write(longToBytes((long)b.length));
        out.write(b);
    }
    public String getInitData() throws IOException{
        String JSONString = StringJSONMessageGenerator.getInitDataMsg();
        sendData(JSONString);
        return getData();
    }

    public String makeRequest(String JSONRequest){
        String JSONResponse;
        try {
            sendData(JSONRequest);
            JSONResponse = getData();
        }catch (IOException e){
            return null;
        }
        return JSONResponse;
    }
    public boolean makeStatement(String JSONStatement){
        try{
            sendData(JSONStatement);
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



