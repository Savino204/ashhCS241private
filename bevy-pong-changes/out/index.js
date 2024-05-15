import { initializeApp } from 'firebase/app';

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseApp = initializeApp({
    apiKey: "AIzaSyBSeY2lSozrUSZse44EUCQzKUK2e6Nm8fc",
    authDomain: "pong-leaderboard-cf5bd.firebaseapp.com",
    projectId: "pong-leaderboard-cf5bd",
    storageBucket: "pong-leaderboard-cf5bd.appspot.com",
    messagingSenderId: "772792511052",
    appId: "1:772792511052:web:cbcd766e08e0b0216aa2e9",
    measurementId: "G-CY96CW6SCJ"
})

// Initialize Firebase
const app = initializeApp(firebaseApp);