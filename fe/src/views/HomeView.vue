<template>
  <div>
    <div id="firebaseui-auth-container"></div>
    <textarea  v-model="userToken">
    </textarea>
  </div>
</template>

<script lang="ts" setup>
import firebaseConfig from "@/utils/firebaseConfig";
import firebase from 'firebase/compat/app';
import * as firebaseui from 'firebaseui'
import 'firebaseui/dist/firebaseui.css'
import { onMounted, ref } from 'vue';

const userToken = ref<string>()

onMounted(async () => {
  firebase.initializeApp(firebaseConfig);
  var ui = new firebaseui.auth.AuthUI(firebase.auth());
  ui.start('#firebaseui-auth-container', {
    signInSuccessUrl: '/',
    signInOptions: [
      {
        provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
      }
    ],
  });

  firebase.auth().onAuthStateChanged(async (user) => {
    if (user) {
      console.log(user.displayName)
      const token = await user.getIdToken()
      userToken.value = token
    }
  });
})

</script>

<style scoped>
textarea {
  width: 100%;
  height: 500px;
  border: 1px solid black;
  outline: none;
}
</style>