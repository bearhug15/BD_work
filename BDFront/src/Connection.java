import javax.swing.*;
import java.awt.*;
import java.io.IOException;
import java.net.Socket;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;

public class Connection {
    Socket socket;
    Connection(Component component, String address, int port) throws CancelledConnectionCreation{
        while(true){
            try{
                socket = getConnection(address,port);
                JOptionPane.showMessageDialog(component,"Connection established");
                break;
            }catch (IOException e){
                int res = JOptionPane.showOptionDialog(component,"Error in connecting to database. Try again?","Error",JOptionPane.OK_CANCEL_OPTION,JOptionPane.ERROR_MESSAGE,null,null,null);
                if (res!=JOptionPane.OK_OPTION){
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
    public JSONObject getInitData(){
        String jsonString = "";
        JSONObject initDataGetterJson = (JSONObject) new JSONParser().parse(jsonString);

        JSONObject result=null;
        return result;
    }
}
