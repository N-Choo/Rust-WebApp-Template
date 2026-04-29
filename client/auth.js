import { state, setCurrentUser } from './api.js';

export async function checkAuthApi(forceFetch = false) {
  const displayElement = document.getElementById('userDisplayName');
  const navContainer = document.getElementById('navActions');
  const authBtn = document.getElementById('loginBtn');

  try {
    if (!state.currentUser || forceFetch) {
      const res = await fetch("/api/graph/me");
      if (res.ok) {
        const body = await res.json();
        if (body.success) setCurrentUser(body.data);
      } else if (res.status === 401) {
        setCurrentUser(null);
      }
    }

    if (state.currentUser) {
      if (displayElement) displayElement.textContent = state.currentUser.displayName || state.currentUser.email;
      if (authBtn) {
        authBtn.textContent = 'Logout';
        authBtn.setAttribute('data-action', 'logout');
        authBtn.style.display = 'inline-block';
      }
      if (navContainer) {
        navContainer.innerHTML = `
          <button onclick="window.location.hash='/users'" class="btn">Users</button>
          <button onclick="window.location.hash='/update'" class="btn">Profile</button>
        `;
        navContainer.style.display = 'inline-block';
      }
    } else {
      if (displayElement) displayElement.textContent = 'Guest';
      if (authBtn) {
        authBtn.textContent = 'Login';
        authBtn.setAttribute('data-action', 'login');
        authBtn.style.display = 'inline-block';
      }
      if (navContainer) navContainer.style.display = 'none';
    }
  } catch (e) {
    console.error("Auth API Error:", e);
  }
}
