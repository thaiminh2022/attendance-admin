export function clear_local_storage(){
    localStorage.clear();
}

export function get_local_storage(key){
    const storage = localStorage.getItem(key);

    if (!storage){
        throw Error("M LÀM TRÒ CC J VẬY");
    }
    return storage;
}

export function set_local_storage(key, json){
    localStorage.setItem(key, json);
}