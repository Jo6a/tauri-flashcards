document.getElementById('menu-button').onclick = function() {
    const sidebar = document.getElementById('sidebar');
    const mainContent = document.getElementById('main-content');
    const addCardView = document.getElementById('add-card-view');
    if (sidebar.style.left === '-250px') {
      sidebar.style.left = '0px';
      mainContent.style.marginLeft = '275px';
      addCardView.style.marginLeft = '275px';
    } else {
      sidebar.style.left = '-250px';
      mainContent.style.marginLeft = '25px';
      addCardView.style.marginLeft = '25px';
    }
  };
  
  document.getElementById('main-view-button').onclick = function() {
    document.getElementById('main-content').style.display = 'block';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('add-card-button').classList.remove('active');
    document.getElementById('main-view-button').classList.add('active');
    document.getElementById('options-button').classList.remove('active');
  };

  document.getElementById('add-card-button').onclick = function() {
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('add-card-view').style.display = 'block';
    document.getElementById('add-card-button').classList.add('active');
    document.getElementById('main-view-button').classList.remove('active');
    document.getElementById('options-button').classList.remove('active');
  };
  
  document.getElementById('options-button').onclick = function() {
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'block';
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('add-card-button').classList.remove('active');
    document.getElementById('options-button').classList.add('active');
    document.getElementById('main-view-button').classList.remove('active');
  };
  
  document.getElementById('theme-button').onclick = function() {
    const body = document.body;
    if (body.classList.contains('dark-mode')) {
      body.classList.remove('dark-mode');
      body.classList.add('light-mode');
      document.getElementById('theme-button').innerText = 'Zum Dunkelmodus wechseln';
    } else {
      body.classList.remove('light-mode');
      body.classList.add('dark-mode');
      document.getElementById('theme-button').innerText = 'Zum Hellmodus wechseln';
    }
  };
  
  window.onload = async function() {
    const { invoke } = window.__TAURI__.tauri;
    const text = await invoke('get_text');
    document.getElementById('text-field').value = text;
    document.getElementById('text-field-2').value = text;
  
    document.getElementById('show-button').onclick = function() {
      document.getElementById('text-field-2').style.display = 'block';
      document.getElementById('wrong-button').style.display = 'block';
    };
  };
  