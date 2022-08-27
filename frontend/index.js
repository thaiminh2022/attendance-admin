import { initializeApp } from "https://www.gstatic.com/firebasejs/9.9.3/firebase-app.js";

// TODO: Add SDKs for Firebase products that you want to use

// https://firebase.google.com/docs/web/setup#available-libraries


// Your web app's Firebase configuration

const firebaseConfig = {

  apiKey: "AIzaSyCeomOGJUF7BFKoCiytg5xIjqu-gBl73P4",

  authDomain: "attendance-admin-9d68c.firebaseapp.com",

  databaseURL: "https://attendance-admin-9d68c-default-rtdb.asia-southeast1.firebasedatabase.app",

  projectId: "attendance-admin-9d68c",

  storageBucket: "attendance-admin-9d68c.appspot.com",

  messagingSenderId: "157261787620",

  appId: "1:157261787620:web:a86bcbe8c0fd21cdc317f1"

};


// Initialize Firebase
export const app = initializeApp(firebaseConfig);
