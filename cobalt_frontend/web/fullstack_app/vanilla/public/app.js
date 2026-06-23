const API_BASE = 'http://localhost:8081/api';

const nameInput = document.getElementById('nameInput');
const submitBtn = document.getElementById('submitBtn');
const userForm = document.getElementById('userForm');
const formFeedback = document.getElementById('formFeedback');
const userList = document.getElementById('userList');
const dbButtons = document.querySelectorAll('.db-btn');

let currentDb = 'sqlite';

// Load users on page load
document.addEventListener('DOMContentLoaded', () => {
    loadUsers('sqlite');
});

// DB selector buttons
dbButtons.forEach(btn => {
    btn.addEventListener('click', () => {
        dbButtons.forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        currentDb = btn.dataset.db;
        loadUsers(currentDb);
    });
});

// Form submission
userForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const name = nameInput.value.trim();
    if (!name) return;

    submitBtn.disabled = true;
    submitBtn.textContent = '⏳ Adding...';

    try {
        const response = await fetch(`${API_BASE}/users`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name })
        });

        const data = await response.json();

        if (data.success) {
            showFeedback(`✅ User "${name}" added to all databases!`, 'success');
            nameInput.value = '';
            // Reload users
            loadUsers(currentDb);
        } else {
            showFeedback(`❌ Error: ${data.message}`, 'error');
        }
    } catch (error) {
        showFeedback(`❌ Network error: ${error.message}`, 'error');
    } finally {
        submitBtn.disabled = false;
        submitBtn.textContent = '💾 Add User';
    }
});

async function loadUsers(db) {
    userList.innerHTML = '<div class="loading">Loading users...</div>';

    try {
        let url = `${API_BASE}/users`;
        if (db !== 'all') {
            url += `?db=${db}`;
        } else {
            url = `${API_BASE}/users/all`;
        }

        const response = await fetch(url);
        const data = await response.json();

        if (data.success) {
            renderUsers(data, db);
        } else {
            userList.innerHTML = `<div class="empty">❌ ${data.error || 'Failed to load users'}</div>`;
        }
    } catch (error) {
        userList.innerHTML = `<div class="empty">❌ Network error: ${error.message}</div>`;
    }
}

function renderUsers(data, db) {
    if (db === 'all') {
        // Show users from all databases
        let html = '';
        data.databases.forEach(dbData => {
            if (dbData.users && dbData.users.length > 0) {
                html += `<div style="margin-bottom: 1rem; color: #8888aa; font-size: 0.8rem; font-weight: 600;">📁 ${dbData.database.toUpperCase()} (${dbData.count})</div>`;
                dbData.users.forEach(user => {
                    html += renderUserItem(user);
                });
            } else if (dbData.error) {
                html += `<div style="margin-bottom: 1rem; color: #ff4757; font-size: 0.8rem;">❌ ${dbData.database}: ${dbData.error}</div>`;
            } else {
                html += `<div style="margin-bottom: 1rem; color: #666688; font-size: 0.8rem;">📁 ${dbData.database.toUpperCase()}: No users</div>`;
            }
        });
        userList.innerHTML = html || '<div class="empty">No users found</div>';
        return;
    }

    // Single database view
    if (data.users && data.users.length > 0) {
        let html = `<div style="margin-bottom: 0.8rem; color: #8888aa; font-size: 0.8rem; font-weight: 600;">${data.count} user${data.count > 1 ? 's' : ''}</div>`;
        data.users.forEach(user => {
            html += renderUserItem(user);
        });
        userList.innerHTML = html;
    } else {
        userList.innerHTML = '<div class="empty">📭 No users yet. Add one above!</div>';
    }
}

function renderUserItem(user) {
    const date = new Date(user.created_at);
    const timeStr = date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
    return `
        <div class="user-item">
            <span class="name">👤 ${user.name}</span>
            <span class="meta">ID: ${user.id} · ${timeStr}</span>
        </div>
    `;
}

function showFeedback(message, type) {
    formFeedback.textContent = message;
    formFeedback.className = `feedback ${type}`;
    formFeedback.style.display = 'block';

    setTimeout(() => {
        formFeedback.style.display = 'none';
    }, 5000);
}