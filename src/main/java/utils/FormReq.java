package utils;

public class FormReq{
    String formName;
    //MyEntry<String,String>fields;

    public FormReq(String formName/*, MyEntry<String, String> fields*/) {
        this.formName = formName;
        //this.fields = fields;
    }

    public String getFormName() {
        return formName;
    }

    /*public MyEntry<String, String> getFields() {
        return fields;
    }*/

    public void setFormName(String formName) {
        this.formName = formName;
    }

    /*public void setFields(MyEntry<String, String> fields) {
        this.fields = fields;
    }*/
}
