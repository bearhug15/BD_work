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
    public static String getTablePartMsg(String tableName, int partNumber){
        String stringJSON=null;
        stringJSON = "{ \"messageType\" : \"partDataAsk\", \"tableName\" : \""+ tableName + "\", \"partNumber\" : \""+String.valueOf(partNumber)+"\"}";
        return stringJSON;
    }
    public static String changeTableMsg(String tableName,String key,Vector<MyEntry<String,String>> args){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"changeTable\", \"tableName\" : \""+ tableName + "\", \"key\" : \""+key+"\"");
        args.forEach(entry->{
            stringJSONBuild.append(" , \""+entry.getKey()+"\" : \"" + entry.getValue()+"\"");
        });
        stringJSONBuild.append(" }");
        return stringJSONBuild.toString();
    }
    public static String addTableEntryMsg(String tableName,Vector<MyEntry<String,String>> args){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"addEntry\", \"tableName\" : \""+ tableName + "\"");
        args.forEach(entry->{
            stringJSONBuild.append(" , \""+entry.getKey()+"\" : \"" + entry.getValue()+"\"");
        });
        stringJSONBuild.append(" }");
        return stringJSONBuild.toString();
    }
    public static String deleteTableEntryMsg(String tableName, String key){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"deleteEntry\", \"tableName\" : \""+ tableName + "\", \"key\" : \""+key+"\"");
        return stringJSONBuild.toString();
    }
    public static String getContractsAtTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getContractsAtTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getContractsByProjectForm(String projectId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getContractsByProject\", \"projectId\" : \"");
        stringJSONBuild.append(projectId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getContractsCostByTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getContractsCostByTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getContractsEfForm(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getContractsEf\" }");
        return stringJSONBuild.toString();
    }
    public static String getDepartmentHeadsForm(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getDepartmentHeads\" }");
        return stringJSONBuild.toString();
    }
    public static String getDepartmentStuffByAgeForm(String departmentName, String ageStart,String ageEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getDepartmentStuffByAge\", \"departmentName\" :  \"");
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
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getDepartmentStuffByType\", \"departmentName\" :  \"");
        stringJSONBuild.append(departmentName);
        stringJSONBuild.append("\" , \"workerType\" : \"");
        stringJSONBuild.append(workerType);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getEquipmentByContractForm(String contractId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getEquipmentByContract\", \"contractId\" :  \"");
        stringJSONBuild.append(contractId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getEquipmentByProjectForm(String projectId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getEquipmentByProject\", \"projectId\" :  \"");
        stringJSONBuild.append(projectId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getEquipmentPlaceByTimeForm(String time){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getEquipmentPlaceByTime\", \"time\" :  \"");
        stringJSONBuild.append(time);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getFullStuffByAgeForm(String ageStart, String ageEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getFullStuffByAge\", \"ageStart\" : \"");
        stringJSONBuild.append(ageStart);
        stringJSONBuild.append("\" , \"ageEnd\" : \"");
        stringJSONBuild.append(ageEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getFullStuffByTypeForm(String workerType){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getFullStuffByType\", \"workerType\" :  \"");
        stringJSONBuild.append(workerType);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getPCByEqForm(String eqId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getPCByEq\", \"eqId\" :  \"");
        stringJSONBuild.append(eqId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsAtTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getProjectAtTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsByContractForm(String contractId){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getProjectsByContract\", \"contractId\" :  \"");
        stringJSONBuild.append(contractId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsCostByTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getProjectsCostByTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getProjectsWorkersByTimeForm(String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getProjectsWorkersByTime\", \"timeStart\" : \"");
        stringJSONBuild.append(timeStart);
        stringJSONBuild.append("\" , \"timeEnd\" : \"");
        stringJSONBuild.append(timeEnd);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
    public static String getWorkDoneByCompaniesForm(){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getWorkDoneByCompanies\" }");
        return stringJSONBuild.toString();
    }
    public static String getWorkersContractsByTimeForm(String workerId, String timeStart, String timeEnd){
        StringBuilder stringJSONBuild = new StringBuilder();
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getWorkersContractsByTime\", \"workerId\" : \"");
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
        stringJSONBuild.append("{ \"messageType\" : \"form\" , \"type\" : \"getWorkersInContract\", \"contractId\" :  \"");
        stringJSONBuild.append(contractId);
        stringJSONBuild.append("\" }");
        return stringJSONBuild.toString();
    }
}
