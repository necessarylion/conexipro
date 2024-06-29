<template>
  <div class="flex items-center justify-center h-screen lg:px-40 md:px-10 px-0 xs:py-0 md:py-12">
    <div class="flex flex-row w-full h-full shadow rounded-lg">
      <!-- left -->
      <div class="bg-background w-1/2 rounded-s-lg px-10 py-10 md:flex hidden flex-col gap-3">
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
          <img src="/images/card.svg" class="w-5/6">
        </div>
      </div>

      <!-- right -->
      <div class="bg-white lg:w-1/2 md:w-1/2 w-full sm:rounded-none md:rounded-e-lg lg:px-20 md:px-10 px-8 py-10 flex flex-col">

        <div class="right-header">
          <div class="text-black text-xl font-semibold mb-3">Conexipro.</div>
          <div class="text-grey text-xs mb-7">Welcome back! login with your account to continue with us</div>
        </div>

        <div class="items-center justify-center flex flex-col flex-1" id="firebase">
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
    signInSuccessUrl: window.location.href,
    signInOptions: [
      {
        provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
        fullLabel: 'Continue with Google'
      },
      {
        provider: firebase.auth.PhoneAuthProvider.PROVIDER_ID,
        defaultCountry: 'THB',
        fullLabel: 'Continue with Phone'
      },
      {
        provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
        signInMethod: firebase.auth.EmailAuthProvider.EMAIL_LINK_SIGN_IN_METHOD,
        fullLabel: 'Continue with Email'
      }
    ],
  });

  firebase.auth().onAuthStateChanged(async (user) => {
    if (user) {
      const firebaseToken = await user.getIdToken()
      let res = await authService.login(firebaseToken)
      window.localStorage.setItem("AUTH_TOKEN", res.token.token)
      // authStore.setUser(res.user)
      // router.push('/profile/' + res.user.username)
    }
  });
}

</script>

<style scoped>
.right-header {
  max-width: 315px;
  margin: 0 auto;
}
</style>