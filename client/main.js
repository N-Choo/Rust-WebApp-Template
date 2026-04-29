import { checkAuthApi } from './auth.js';
import { fetchAndRenderUsers } from './users.js';
import { initProfile } from './profile.js';

const routes = {
  '/': { file: 'pages/welcome.html', title: 'Welcome' },
  '/users': { file: 'pages/users.html', title: 'Tenant Users' },
  '/update': { file: 'pages/profile.html', title: 'Update Profile' },
  '/unauthorized': { file: 'pages/unauthorized.html', title: 'Unauthorized' }
};

async function router() {
  const path = window.location.hash.slice(1) || '/';
  const route = routes[path] || routes['/unauthorized'];

  try {
    const response = await fetch(`${route.file}`);
    const html = await response.text();
    const appDiv = document.getElementById('app');

    if (appDiv) {
      appDiv.innerHTML = html;
      document.title = route.title;
    }

    // Trigger module functions based on the current page
    if (path === '/users') {
      await fetchAndRenderUsers();
    } else if (path === '/update') {
      await initProfile();
    } else {
      await checkAuthApi(path === '/');
    }
  } catch (err) {
    console.error("Router error:", err);
  }
}

// Global Click Handler for navigation and authentication buttons
document.addEventListener('click', (event) => {
  const targetPath = event.target.getAttribute('data-nav');

  if (targetPath) {
    window.location.hash = targetPath;
    return;
  }

  if (event.target.id === 'loginBtn' || event.target.id === 'logoutBtn') {
    const action = event.target.getAttribute('data-action') || event.target.id.replace('Btn', '').toLowerCase();
    window.location.href = action === 'logout' ? "/api/auth/logout" : "/api/auth/login";
  }
});

// Boot up the router
window.addEventListener('hashchange', router);
router();
