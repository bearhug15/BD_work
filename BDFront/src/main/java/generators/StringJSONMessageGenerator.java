package generators;

import utils.MyEntry;

import java.util.Vector;

public class StringJSONMessageGenerator {
    public static String getInitDataMsg(){
        return "{ \"messageType\" : \"initAsk\" }";
    }
    public static String getTableMsg(String tableName){
        String stringJSON=null;
        stringJSON = "{ \"messageType\" : \"fullDataAsk\", \"tableName\" : \""+ tableName + "\"  }";
        return stringJSON;
    }
    public static String getTablePartMsg(String tableName, long partNumber){
        String stringJSON=null;
        stringJSON = "{ \"messageType\" : \"partDataAsk\", \"tableName\" : \""+ tableName + "\", \"partNumber\" : \""+String.valueOf(partNumber)+"\"}";
        return stringJSON;
    }
    public static String updateTableMsg(String tableName, String key, Vector<MyEntry<String,String>> args){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"updateTable\", \"tableName\" : \""+ tableName + "\", \"key\" : \""+key+"\"");
        stringJSONBuild.append(", values: {");
        args.forEach(entry->{
            stringJSONBuild.append(" \""+entry.getKey()+"\" : \"" + entry.getValue()+"\",");
        });
        if(args.size()!=0){
            stringJSONBuild.deleteCharAt(stringJSONBuild.length()-1);
        }
        stringJSONBuild.append("}");
        stringJSONBuild.append(" }");
        return stringJSONBuild.toString();
    }
    public static String replaceEntriesTableMsg(String tableName,String keyName,String key,Vector<MyEntry<String,Vector<String>>> args){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"replaceEntriesTable\", \"tableName\" : \"");
        stringJSONBuild.append(tableName);
        stringJSONBuild.append("\", \"keyName\" : \"");
        stringJSONBuild.append(keyName);
        stringJSONBuild.append("\", \"key\" : \"");
        stringJSONBuild.append(key);
        stringJSONBuild.append("\", values: {");
        args.forEach(field->{
            stringJSONBuild.append("\"" +field.getKey());
            stringJSONBuild.append("\": [");
            field.getValue().forEach(value->{
                stringJSONBuild.append("\""+value+"\",");
            });
            if(field.getValue().size()!=0) {
                stringJSONBuild.deleteCharAt(stringJSONBuild.length() - 1);
            }
            stringJSONBuild.append("],");
        });
        if(args.size()!=0){
            stringJSONBuild.deleteCharAt(stringJSONBuild.length()-1);
        }
        stringJSONBuild.append("}");
        stringJSONBuild.append(" }");
        return stringJSONBuild.toString();
    }
    public static String addTableEntryMsg(String tableName,Vector<MyEntry<String,String>> args){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"addEntry\", \"tableName\" : \""+ tableName + "\"");
        if(args!=null) {
            stringJSONBuild.append(", values: {");
            args.forEach(entry -> {
                stringJSONBuild.append(" \"" + entry.getKey() + "\" : \"" + entry.getValue() + "\",");
            });
            if(args.size()!=0){
                stringJSONBuild.deleteCharAt(stringJSONBuild.length()-1);
            }
            stringJSONBuild.append("}");
        }
        stringJSONBuild.append(" }");
        return stringJSONBuild.toString();
    }
    public static String addSerialTableEntryMsg(String tableName,Vector<MyEntry<String,Vector<String>>> args){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"serialAddTable\", \"tableName\" : \"");
        stringJSONBuild.append(tableName);
        stringJSONBuild.append("\", values: {");
        args.forEach(field->{
            stringJSONBuild.append("\"" +field.getKey());
            stringJSONBuild.append("\": [");
            field.getValue().forEach(value->{
                stringJSONBuild.append("\""+value+"\",");
            });
            if(field.getValue().size()!=0) {
                stringJSONBuild.deleteCharAt(stringJSONBuild.length() - 1);
            }
            stringJSONBuild.append("],");
        });
        if(args.size()!=0){
            stringJSONBuild.deleteCharAt(stringJSONBuild.length()-1);
        }
        stringJSONBuild.append("}");
        stringJSONBuild.append(" }");
        return stringJSONBuild.toString();
    }
    public static String deleteTableEntryMsg(String tableName, String keyName,String key){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"deleteEntry\", \"tableName\" : \""+ tableName + "\", \"keyName\" : \""+keyName+"\", \"key\" : \""+key+"\"");
        return stringJSONBuild.toString();
    }
    public static String getContractsAtTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"contractsAtTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getContractsByProjectForm(String projectId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"contractsByProject\", \"projectId\" : \"");
        stringJSONBuild.append(projectId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getContractsCostByTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"contractsCostByTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getContractsEfForm(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"contractsEf\" }");
        return stringJSONBuild.toString();
    }
    public static String getDepartmentHeadsForm(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"departmentHeads\" }");
        return stringJSONBuild.toString();
    }
    public static String getDepartmentStuffByAgeForm(String departmentName, String ageStart,String ageEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"departmentStuffByAge\", \"departmentName\" :  \"");
        stringJSONBuild.append(departmentName);
        stringJSONBuild.append("\" , \"ageStart\" : \"");
        stringJSONBuild.append(ageStart);
        stringJSONBuild.append("\", \"ageEnd\" : \"");
        stringJSONBuild.append(ageEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getDepartmentStuffByTypeForm(String departmentName, String workerType){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"departmentStuffByType\", \"departmentName\" :  \"");
        stringJSONBuild.append(departmentName);
        stringJSONBuild.append("\" , \"workerType\" : \"");
        stringJSONBuild.append(workerType);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getEquipmentByContractForm(String contractId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"equipmentByContract\", \"contractId\" :  \"");
        stringJSONBuild.append(contractId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getEquipmentByProjectForm(String projectId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"equipmentByProject\", \"projectId\" :  \"");
        stringJSONBuild.append(projectId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getEquipmentPlaceByTimeForm(String time){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"equipmentPlaceByTime\", \"time\" :  \"");
        stringJSONBuild.append(time);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getFullStuffByAgeForm(String ageStart, String ageEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"fullStuffByAge\", \"ageStart\" : \"");
        stringJSONBuild.append(ageStart);
        stringJSONBuild.append("\" , \"ageEnd\" : \"");
        stringJSONBuild.append(ageEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getFullStuffByTypeForm(String workerType){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"fullStuffByType\", \"workerType\" :  \"");
        stringJSONBuild.append(workerType);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getPCByEqForm(String eqId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"PCByEq\", \"eqId\" :  \"");
        stringJSONBuild.append(eqId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsAtTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"projectAtTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsByContractForm(String contractId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"projectsByContract\", \"contractId\" :  \"");
        stringJSONBuild.append(contractId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsCostByTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"projectsCostByTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsWorkersByTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"projectsWorkersByTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getWorkDoneByCompaniesForm(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"workDoneByCompanies\" }");
        return stringJSONBuild.toString();
    }
    public static String getWorkersContractsByTimeForm(String workerId, String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"workersContractsByTime\", \"workerId\" : \"");
        stringJSONBuild.append(workerId);
        stringJSONBuild.append("\" , \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getWorkersInContractForm(String contractId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"workersInContract\", \"contractId\" :  \"");
        stringJSONBuild.append(contractId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
}
