package utils;

import java.util.Vector;

public class InitData{
    Vector<Table> tables;
    Vector<FormReq> fromReqs;

    public InitData(Vector<Table> tables, Vector<FormReq> fromReqs) {
        this.tables = tables;
        this.fromReqs = fromReqs;
    }

    public Vector<Table> getTables() {
        return tables;
    }

    public Vector<FormReq> getFromReqs() {
        return fromReqs;
    }
}
