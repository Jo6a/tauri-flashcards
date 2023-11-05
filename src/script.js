document.getElementById('menu-button').onclick = function() {
    const sidebar = document.getElementById('sidebar');
    const mainContent = document.getElementById('main-content');
    const decksView = document.getElementById('decks-view');
    const addCardView = document.getElementById('add-card-view');
    const optionsView = document.getElementById('options-view');
    
    if (sidebar.style.left === '-250px') {
      sidebar.style.left = '0px';
      mainContent.style.marginLeft = '275px';
      decksView.style.marginLeft = '275px';
      addCardView.style.marginLeft = '275px';
      optionsView.style.marginLeft = '275px';
    } else {
      sidebar.style.left = '-250px';
      mainContent.style.marginLeft = '25px';
      decksView.style.marginLeft = '25px';
      addCardView.style.marginLeft = '25px';
      optionsView.style.marginLeft = '25px';
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
  
  document.getElementById('main-view-button').onclick = function() {
    document.getElementById('main-content').style.display = 'block';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('decks-view').style.display = 'none';
    document.getElementById('decks-button').classList.remove('active');
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('add-card-button').classList.remove('active');
    document.getElementById('main-view-button').classList.add('active');
    document.getElementById('options-button').classList.remove('active');
  };

  document.getElementById('decks-button').onclick = async function() {
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('decks-view').style.display = 'block';
    document.getElementById('decks-button').classList.add('active');
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('main-view-button').classList.remove('active');
    document.getElementById('options-button').classList.remove('active');
    document.getElementById('add-card-button').classList.remove('active');
  
    const { invoke } = window.__TAURI__.tauri;
    const deckNames = await invoke('get_deck_names');
    console.log(deckNames);
    const deckList = document.getElementById('deck-list');
    deckList.innerHTML = '';
    for (const deck of deckNames) {
        const listItem = document.createElement('li');
        console.log(deck.name);
        listItem.textContent = deck.name;
        listItem.onclick = function() {
          // Hier können Sie Code hinzufügen, um das ausgewählte Deck zu laden
        };
        deckList.appendChild(listItem);
      }
      
  };

  document.getElementById('add-card-button').onclick = function() {
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('decks-view').style.display = 'none';
    document.getElementById('decks-button').classList.remove('active');
    document.getElementById('add-card-view').style.display = 'block';
    document.getElementById('add-card-button').classList.add('active');
    document.getElementById('main-view-button').classList.remove('active');
    document.getElementById('options-button').classList.remove('active');
  };
  
  document.getElementById('options-button').onclick = function() {
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'block';
    document.getElementById('decks-view').style.display = 'none';
    document.getElementById('decks-button').classList.remove('active');
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
      document.getElementById('theme-button').innerText = 'Dark Mode';
    } else {
      body.classList.remove('light-mode');
      body.classList.add('dark-mode');
      document.getElementById('theme-button').innerText = 'Light Mode';
    }
  };
  

  