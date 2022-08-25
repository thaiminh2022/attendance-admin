import { getAuth, signOut, setPersistence, signInWithEmailAndPassword, browserSessionPersistence } from "https://www.gstatic.com/firebasejs/9.9.3/firebase-auth.js";

const auth = getAuth();
export async function sign_in(email, password) {
  setPersistence(auth, browserSessionPersistence);
  let userCridentials = await signInWithEmailAndPassword(auth, email, password);

  // Code below will never run if there's an error

  // Sign in OK!
  console.log(userCridentials.user);

  window.location.replace("/admin");
}
export async function sign_out() {
  await signOut(auth);
}

export async function is_logged_in() {
  try {
    await new Promise((resolve, reject) =>
      auth.onAuthStateChanged(
        user => {
          if (user) {
            // User is signed in.
            resolve(user)
          } else {
            // No user is signed in.
            reject('no user logged in')
          }
        },
        // Prevent console error
        error => reject(error)
      )
    )
    return true
  } catch (error) {
    return false
  }
}

