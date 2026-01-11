// Vue Router lazy loading pattern - should NOT be flagged as side-effect import
import { createRouter } from 'vue-router';

export const router = createRouter({
    routes: [
        {
            path: '/',
            component: () => import('./pages/Home.vue'),
        },
        {
            path: '/about',
            component: () => import('./pages/About.vue'),
        },
    ],
});
