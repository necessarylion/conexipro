<template>
  <div class="flex items-center justify-center h-screen lg:px-40 md:px-10 px-0 sm:py-0 md:py-12">
    <div class="flex flex-row w-full h-full shadow rounded-lg">
      <!-- left -->
      <div class="bg-background w-1/2 rounded-s-lg px-10 py-10 sm:hidden md:flex flex-col gap-3">
        <div class="flex flex-row items-center gap-3">
          <div>
            <img src="/images/logo.png" class="w-[40px] rounded-full border border-grey">
          </div>
          <div>
            <div class="text-white text-2xl font-semibold mb-3">Conexipro.</div>
            <div class="text-whiteSecondary text-xs	mb-1">Let's create digital profile with us!</div>
          </div>
        </div>
        <div class="flex-1 flex items-center justify-center">
          <img src="/images/card.svg" class="w-full">
        </div>
      </div>

      <!-- right -->
      <div class="lg:w-1/2 md:w-1/2 w-full bg-offWhite sm:rounded-none md:rounded-e-lg px-20 py-10 flex flex-col">

        <div>
          <div class="text-black text-xl font-semibold mb-3">Conexipro.</div>
          <div class="text-grey text-xs mb-7">Welcome back! login with your account to continue with us</div>
        </div>

        <div class="items-center justify-center flex flex-col flex-1">
          <!-- google -->
          <button
            class="bg-white text-grey w-full rounded-md shadow text-xs py-3 mb-4 flex flex-row items-center justify-center gap-2">
            <img src="/images/google.svg" class="w-[20px]">
            <span>Login with Google</span>
          </button>

          <button
            class="bg-white text-grey w-full rounded-md shadow text-xs py-3 mb-3 flex flex-row items-center justify-center gap-2">
            <img src="/images/facebook.svg" class="w-[20px]">
            <span>Login with Facebook</span>
          </button>
        </div>
      </div>
    </div>
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
  if (authStore.user) {
    router.push('/profile/' + authStore.user.username)
  } else {
    setupFirebaseLogin()
  }
})

function setupFirebaseLogin() {
  firebase.initializeApp(firebaseConfig);
  var ui = new firebaseui.auth.AuthUI(firebase.auth());
  ui.start('#firebase', {
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
      // router.push('/profile/' + res.user.username)
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