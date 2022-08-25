import { getDatabase, ref, child, get } from "https://www.gstatic.com/firebasejs/9.9.3/firebase-database.js";


const dbRef = ref(getDatabase());

export async function get_data_all() {
    let data = await getDataAll();
    let final_data = JSON.stringify(data);
    return final_data;
}

async function getDataAll(){
    let returnData = "NO FUCKING DATA";

    await get(child(dbRef, `/`)).then(async (snapshot) => {
    if (snapshot.exists()) {
        returnData= await snapshot.val();
    } else {
        returnData = "no data";
    }
    }).catch((error) => {
        returnData = error;
    });

    return returnData;
}