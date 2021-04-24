import javax.swing.*;
import java.io.IOException;
import java.net.Socket;

public class Main extends JFrame {
    String address="127.0.0.1";
    int port=10000;
    public static void main(String[] args){
        new Main(args);

    }
    public Main(String[] args){
        super("Project company database");
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        Connection connection;
        try {
            connection = new Connection(Main.super.rootPane, address, port);
        }catch (CancelledConnectionCreation e){
            System.exit(1);
        }

        return;
    }

}
