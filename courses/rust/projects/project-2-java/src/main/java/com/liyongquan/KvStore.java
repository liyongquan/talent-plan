package com.liyongquan;

public class KvStore {
    private String path;

    public static KvStore open(String path) {
        KvStore kvStore = new KvStore();
        kvStore.setPath(path);
        return kvStore;
    }


    public String getPath() {
        return path;
    }

    public void setPath(String path) {
        this.path = path;
    }
}
