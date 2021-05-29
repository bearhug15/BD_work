package main;

public class CancelledConnectionCreation extends Error {
    String text;
    CancelledConnectionCreation(String text){
        this.text = text;
    }
    public String getText(){
        return text;
    }
}
