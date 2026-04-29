import { apiFetch } from './api.js';

export async function fetchAndRenderUsers() {
  const tableBody = document.getElementById("userTableBody");
  if (!tableBody) return;

  tableBody.innerHTML = '<tr><td colspan="6" style="text-align:center; padding:20px;">Fetching records...</td></tr>';

  try {
    const res = await apiFetch("/api/graph/users");
    const body = await res.json();
    const users = body.data || body;

    if (!users?.length) {
      tableBody.innerHTML = '<tr><td colspan="6" style="text-align:center; padding:20px;">No records found.</td></tr>';
      return;
    }

    tableBody.innerHTML = users.map(u => renderUserRow(u)).join("");
  } catch (err) {
    tableBody.innerHTML = `<tr><td colspan="6" style="text-align:center; padding:20px; color:red;">Error connecting to backend or session expired.</td></tr>`;
  }
}

function renderUserRow(u) {
  const cellStyle = `padding: 10px 12px; border-bottom: 1px solid #f0f0f0; white-space: nowrap;`;
  return `
    <tr style="transition: background 0.1s;" onmouseover="this.style.background='#f9fafb'" onmouseout="this.style.background='transparent'">
      <td style="${cellStyle} font-weight: 600; color: #1f2937;">${u.displayName}</td>
      <td style="${cellStyle} color: #4b5563;">${u.jobTitle || '—'}</td>
      <td style="${cellStyle}"><a href="mailto:${u.mail || u.userPrincipalName}" style="color: #2563eb; text-decoration: none;">${u.mail || u.userPrincipalName}</a></td>
      <td style="${cellStyle} color: #6b7280;">${u.officeLocation || 'Remote'}</td>
      <td style="${cellStyle} font-family: monospace; color: #4b5563;">${u.mobilePhone || (u.businessPhones?.[0]) || '—'}</td>
      <td style="${cellStyle} text-align: center; color: #9ca3af; font-size: 0.75rem;">${(u.preferredLanguage || 'en').toUpperCase()}</td>
    </tr>`;
}
