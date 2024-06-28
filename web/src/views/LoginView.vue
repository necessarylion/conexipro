<template>
  <div class="flex items-center h-screen w-full justify-center">
    <div id="firebaseui-auth-container"></div>
  </div>
</template>

<script lang="ts" setup>
import firebaseConfig from "@/utils/firebaseConfig";
import firebase from 'firebase/compat/app';
import { useAuthStore } from '@/stores/authStore';
import * as firebaseui from 'firebaseui'
import 'firebaseui/dist/firebaseui.css'
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { authService } from '@/services/authService';

const authStore = useAuthStore()
const router = useRouter();

onMounted(async () => {
    if(authStore.user) {
      router.push('/profile/' + authStore.user.username)
    } else {
      setupFirebaseLogin()
    }
})

function setupFirebaseLogin() {
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
      const firebaseToken = await user.getIdToken()
      let res = await authService.login(firebaseToken)
      window.localStorage.setItem("AUTH_TOKEN", res.token.token)
      authStore.setUser(res.user)
      router.push('/profile/' + res.user.username)
    }
  });
}

</script>

<style scoped>
textarea {
  width: 100%;
  height: 500px;
  border: 1px solid black;
  outline: none;
}
</style>