import { apiFetch, state, setCurrentUser } from './api.js';

export async function initProfile() {
  const status = document.getElementById("status");
  try {
    const res = await apiFetch("/api/graph/me");
    const body = await res.json();
    const data = body.data || body;

    document.getElementById("id").textContent = data.id || "N/A";
    document.getElementById("displayName").value = data.displayName || "";
    document.getElementById("mail").value = data.mail || data.userPrincipalName || "";
    document.getElementById("givenName").value = data.givenName || "";
    document.getElementById("surname").value = data.surname || "";
    document.getElementById("jobTitle").value = data.jobTitle || "";
    document.getElementById("mobilePhone").value = data.mobilePhone || "";
    document.getElementById("officeLocation").value = data.officeLocation || "";
    document.getElementById("preferredLanguage").value = data.preferredLanguage || "";
    document.getElementById("businessPhone").value = (data.businessPhones && data.businessPhones[0]) || "";

    document.getElementById("profileForm").onsubmit = handleProfileSubmit;
  } catch (err) {
    if (status) {
      status.style.color = "red";
      status.textContent = "Error loading profile data. You may need to log in again.";
    }
  }
}

async function handleProfileSubmit(e) {
  e.preventDefault();
  const status = document.getElementById("status");
  const saveBtn = document.getElementById("saveBtn");

  status.style.display = "block";
  status.style.background = "#e7f3ff";
  status.style.color = "#004085";
  status.textContent = "⏳ Communicating with Microsoft Graph...";
  saveBtn.disabled = true;

  const currentFormState = {
    displayName: document.getElementById("displayName").value,
    givenName: document.getElementById("givenName").value,
    surname: document.getElementById("surname").value,
    jobTitle: document.getElementById("jobTitle").value,
    mobilePhone: document.getElementById("mobilePhone").value,
    officeLocation: document.getElementById("officeLocation").value,
    preferredLanguage: document.getElementById("preferredLanguage").value,
    businessPhones: [document.getElementById("businessPhone").value].filter(p => p !== ""),
  };

  const patchData = {};
  Object.keys(currentFormState).forEach(key => {
    let originalValue = state.currentUser ? state.currentUser[key] : null;
    let newValue = currentFormState[key];

    if (Array.isArray(newValue)) {
      if (JSON.stringify(newValue) !== JSON.stringify(originalValue || [])) patchData[key] = newValue;
    } else if (newValue !== (originalValue || "")) {
      patchData[key] = newValue;
    }
  });

  if (Object.keys(patchData).length === 0) {
    status.style.background = "#fff3cd";
    status.style.color = "#856404";
    status.textContent = "ℹ️ No changes were made to your profile.";
    saveBtn.disabled = false;
    return;
  }

  try {
    const res = await apiFetch("/api/graph/update", {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(patchData),
    });

    const result = await res.json();

    if (res.ok) {
      status.style.background = "#d4edda";
      status.style.color = "#155724";
      status.textContent = "✅ Success! Profile updated in Entra ID.";
      setCurrentUser(null); // Force refetch on navigation
      setTimeout(() => window.location.hash = "/", 1500);
    } else {
      status.style.background = "#f8d7da";
      status.style.color = "#721c24";
      status.innerHTML = res.status === 403
        ? `<strong>Access Denied (403):</strong> Your IT policy prevents changing these specific fields.`
        : `❌ Update failed: ${result.data || 'Unknown Error'}`;
      saveBtn.disabled = false;
    }
  } catch (err) {
    status.style.background = "#f8d7da";
    status.style.color = "#721c24";
    status.textContent = "❌ Connection error. Check your internet or session.";
    saveBtn.disabled = false;
  }
}
