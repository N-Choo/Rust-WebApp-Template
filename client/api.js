
export let state = {
  currentUser: null
};

export function setCurrentUser(user) {
  state.currentUser = user;
}

// Secure wrapper for all backend calls
export async function apiFetch(url, options = {}) {
  try {
    const response = await fetch(url, options);

    if (response.status === 401) {
      setCurrentUser(null);
      window.location.hash = '/unauthorized';
      throw new Error('Session expired or unauthorized');
    }

    return response;
  } catch (error) {
    console.error(`[API Connection Error] to ${url}:`, error);
    throw error;
  }
}
