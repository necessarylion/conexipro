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
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { Api } from '@/apis/api'

const authStore = useAuthStore()
const router = useRouter();
const userToken = ref<string>()

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

  let api = new Api({
    baseUrl: import.meta.env.VITE_BASE_URL,
  });

  firebase.auth().onAuthStateChanged(async (user) => {
    if (user) {
      const token = await user.getIdToken()
      userToken.value = token
      let res = await api.auth.login({token})
      console.log(res)
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