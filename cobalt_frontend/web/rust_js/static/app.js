const API_BASE = '/api';

const nameInput = document.getElementById('nameInput');
const submitBtn = document.getElementById('submitBtn');
const userForm = document.getElementById('userForm');
const feedback = document.getElementById('feedback');
const userList = document.getElementById('userList');
const refreshBtn = document.getElementById('refreshBtn');

async function fetchUsers() {
    try {
        console.log('📡 Fetching users...');
        const response = await fetch(`${API_BASE}/users`);
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        console.log('✅ Received:', data);

        if (data.success) {
            renderUsers(data.data || []);
        } else {
            showError(data.msg || 'Failed to load users');
        }
    } catch (error) {
        console.error('❌ Fetch error:', error);
        showError('Error: ' + error.message);
    }
}

async function createUser(name) {
    try {
        console.log('📤 Creating user:', name);
        
        const response = await fetch(`${API_BASE}/users`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ name: name.trim() })
        });

        console.log('📥 Response status:', response.status);

        if (!response.ok) {
            const text = await response.text();
            console.error('❌ Error response:', text);
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        console.log('✅ Created:', data);

        if (data.success) {
            showSuccess(`✅ User "${name}" created!`);
            nameInput.value = '';
            await fetchUsers();
        } else {
            showError(data.msg || 'Failed to create user');
        }
    } catch (error) {
        console.error('❌ Create error:', error);
        showError('Error: ' + error.message);
    }
}

function renderUsers(users) {
    if (!users || users.length === 0) {
        userList.innerHTML = '<div class="empty">📭 No users yet</div>';
        return;
    }

    let html = '';
    users.forEach(user => {
        html += `
            <div class="user-item">
                <span>👤 ${escapeHtml(user.name)}</span>
                <span class="meta">ID: ${user.id}</span>
            </div>
        `;
    });
    userList.innerHTML = html;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function showSuccess(msg) {
    feedback.textContent = msg;
    feedback.className = 'feedback success';
    feedback.style.display = 'block';
    setTimeout(() => { feedback.style.display = 'none'; }, 4000);
}

function showError(msg) {
    feedback.textContent = '❌ ' + msg;
    feedback.className = 'feedback error';
    feedback.style.display = 'block';
    setTimeout(() => { feedback.style.display = 'none'; }, 5000);
}

// Form submission
userForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const name = nameInput.value.trim();
    if (!name) {
        showError('Please enter a name');
        return;
    }

    submitBtn.disabled = true;
    submitBtn.textContent = '⏳ Adding...';

    await createUser(name);

    submitBtn.disabled = false;
    submitBtn.textContent = '💾 Add User';
});

// Refresh button
refreshBtn.addEventListener('click', fetchUsers);

// Auto-load
document.addEventListener('DOMContentLoaded', () => {
    console.log('🚀 App loaded');
    fetchUsers();
    nameInput.focus();
});