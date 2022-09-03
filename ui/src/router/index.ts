import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      components: {
        default: () => import('../views/index.vue'),
        nav_authed: () => import('../components/layouts/nav_authed.vue'),
        sidenav_authed: () => import('../components/layouts/sidenav_authed.vue')
      }
    },
  ]
})

export default router
