import { createRouter, createWebHashHistory } from 'vue-router'
import { usePageStore } from '@/stores/pageStore'

const router = createRouter({
  history: createWebHashHistory(), // Hash mode for Tauri
  routes: [
    {
      path: '/',
      redirect: () => {
        // Redirect to first visible page if available, otherwise to home (legacy)
        const pageStore = usePageStore()
        const firstPage = pageStore.visiblePages[0]
        if (firstPage && firstPage.route) {
          // Use the page's route field (stable, user-defined) instead of SurrealDB ID
          // Strip leading slash if present to avoid double slashes
          const routeSlug = firstPage.route.startsWith('/')
            ? firstPage.route.substring(1)
            : firstPage.route
          return `/page/${routeSlug}`
        }
        return '/home'
      },
    },
    {
      path: '/home',
      name: 'home',
      component: () => import('@/views/HomePage.vue'),
    },
    {
      path: '/dashboard/:id',
      name: 'dashboard',
      component: () => import('@/views/DashboardView.vue'),
      props: true,
    },
    {
      path: '/offline-browser',
      name: 'offline-browser',
      component: () => import('@/views/OfflineDataBrowser.vue'),
    },
    {
      path: '/plugin-management',
      name: 'plugin-management',
      component: () => import('@/views/PluginManagement.vue'),
    },
    {
      path: '/database-management',
      name: 'database-management',
      component: () => import('@/components/DatabaseManager.vue'),
    },
    {
      path: '/page/:id',
      name: 'page',
      component: () => import('@/views/PageView.vue'),
      props: true,
    },
    {
      path: '/page/:id/dashboard/:dashboardId',
      name: 'page-dashboard',
      component: () => import('@/views/PageView.vue'),
      props: true,
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('@/views/NotFound.vue'),
    },
  ],
})

export default router
