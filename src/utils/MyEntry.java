package utils;

import java.util.Map;

final public class MyEntry<K, V> implements Map.Entry<K, V> {
    private final K key;
    private V value;

    public MyEntry(K key, V value) {
        this.key = key;
        this.value = value;
    }

    @Override
    public K getKey() {
        return key;
    }

    @Override
    public V getValue() {
        return value;
    }

    @Override
    public V setValue(V value) {
        V old = this.value;
        this.value = value;
        return old;
    }

    @Override
    public boolean equals(Object obj) {
        try{
            MyEntry<K,V> newObj = (MyEntry<K,V>) obj;
            return key == newObj.getKey();
        } catch(ClassCastException e){
            return false;
        }
    }

    @Override
    public int hashCode() {
        return key.hashCode();
    }
}