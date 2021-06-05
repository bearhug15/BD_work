package main;

import java.util.LinkedList;

public class ListableFields {
    static LinkedList<String> equipmentTypes = new LinkedList<>();
    static LinkedList<String> departmentNames = new LinkedList<>();
    static LinkedList<String> workerTypes = new LinkedList<>();
    static LinkedList<String> companies = new LinkedList<>();
    static LinkedList<String> groupTypes = new LinkedList<>();

    public static void addEquipmentType(String newType){
        equipmentTypes.add(newType);
    }
    public static boolean checkEquipmentType(String type){ return equipmentTypes.contains(type);}
    public static void removeEquipmentType(String type){ equipmentTypes.remove(type);}
    public static LinkedList<String> getEquipmentTypes(){return equipmentTypes;}

    public static void addDepartmentName(String newName){
        departmentNames.add(newName);
    }
    public static boolean checkDepartmentName(String name){ return departmentNames.contains(name);}
    public static void removeDepartmentName(String name){ departmentNames.remove(name);}
    public static LinkedList<String> getDepartmentNames() {
        return departmentNames;
    }

    public static void addWorkerType(String newType){
        workerTypes.add(newType);
    }
    public static boolean checkWorkerType(String type){ return workerTypes.contains(type);}
    public static void removeWorkerType(String type){ workerTypes.remove(type);}
    public static LinkedList<String> getWorkerTypes() {
        return workerTypes;
    }

    public static void addCompany(String newCompany){
        companies.add(newCompany);
    }
    public static boolean checkCompany(String company){ return companies.contains(company);}
    public static void removeCompany(String company){ companies.remove(company);}
    public static LinkedList<String> getCompanies() {
        return companies;
    }

    public static void addGroupType(String newType){
        groupTypes.add(newType);
    }
    public static boolean checkGroupType(String type){ return groupTypes.contains(type);}
    public static void removeGroupType(String type){ groupTypes.remove(type);}
    public static LinkedList<String> getGroupTypes() {
        return groupTypes;
    }

    static {
        /*groupTypes.add("default group");
        groupTypes.add("company");
        //departmentNames.add("First department");
        //departmentNames.add("Second department");
        companies.add("First company");
        companies.add("Second company");*/

    }
}
