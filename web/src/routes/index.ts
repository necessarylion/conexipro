import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '@/views/LoginView.vue'
import ProfileView from '@/views/ProfileVue.vue'

const routes = [
  { path: '/', component: LoginView },
  { path: '/profile', component: ProfileView }
]

export default createRouter({
  history: createWebHistory(),
  routes
})
