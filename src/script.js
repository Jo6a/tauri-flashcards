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
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('decks-view').style.display = 'block';
    document.getElementById('decks-button').classList.add('active');
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('main-view-button').classList.remove('active');
    document.getElementById('options-button').classList.remove('active');
    document.getElementById('add-card-button').classList.remove('active');  

    await loadDecksFromBackend();
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
    
    await loadDecksFromBackend();
  };

  document.getElementById('main-view-button').onclick = async function() {
    document.getElementById('main-content').style.display = 'block';
    document.getElementById('options-view').style.display = 'none';
    document.getElementById('decks-view').style.display = 'none';
    document.getElementById('decks-button').classList.remove('active');
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('add-card-button').classList.remove('active');
    document.getElementById('main-view-button').classList.add('active');
    document.getElementById('options-button').classList.remove('active');

    const { invoke } = window.__TAURI__.tauri;
    const [question, answer] = await invoke('get_card', { deckName: document.getElementById('selected-deck').textContent });
    document.getElementById('text-field').value = question;
    document.getElementById('text-field-2').value = answer;
  
    document.getElementById('show-button').onclick = function() {
      const textfield2 = document.getElementById('text-field-2');
      if (textfield2.style.display === 'block') {
        textfield2.style.display = 'none';
      } else {
        textfield2.style.display = 'block';
      }
    };
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
  
  document.getElementById('options-button').onclick = async function() {
    document.getElementById('main-content').style.display = 'none';
    document.getElementById('options-view').style.display = 'block';
    document.getElementById('decks-view').style.display = 'none';
    document.getElementById('decks-button').classList.remove('active');
    document.getElementById('add-card-view').style.display = 'none';
    document.getElementById('add-card-button').classList.remove('active');
    document.getElementById('options-button').classList.add('active');
    document.getElementById('main-view-button').classList.remove('active');

    let deckName = document.getElementById('selected-deck').textContent;
    var deckOptionsHeading = document.getElementById("deck-options");
    deckOptionsHeading.textContent = "Deck Options (" + deckName + ")";

    await loadDeckParams();
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

  document.getElementById('add-deck-button').onclick = async function() {
    deck_name = document.getElementById('new-deck').value;
    let initial_interval = parseInt(document.getElementById('initial-interval').value);
    let ease_factor = parseFloat(document.getElementById('ease-factor').value);
    console.log(initial_interval);
    console.log(ease_factor);
    if (deck_name != '') {
        const { invoke } = window.__TAURI__.tauri;
        await invoke('add_deck', { deckName: deck_name, initialInterval: initial_interval, initialEaseFactor: ease_factor });
        await loadDecksFromBackend();
        document.getElementById('new-deck').value = "";
    }
  };

  document.getElementById('delete-deck-button').onclick = async function() {
    deck_name = document.getElementById('selected-deck').textContent;
    if (deck_name != '') {
        const { invoke } = window.__TAURI__.tauri;
        await invoke('delete_deck', { deckName: deck_name });
        await loadDecksFromBackend();
    }
  };

  document.getElementById('add-button').onclick = async function() {
    deck_text = document.getElementById('selected-deck').textContent;
    question_text = document.getElementById('front-field').value;
    answer_text = document.getElementById('back-field').value;
    let initial_interval = parseInt(document.getElementById('initial-interval').value);
    let ease_factor = parseFloat(document.getElementById('ease-factor').value);
    console.log(deck_text);
    console.log(question_text);
    if (question_text != '') {
        const { invoke } = window.__TAURI__.tauri;
        await invoke('add_card', { deckName: deck_text, question: question_text, answer: answer_text,
          initialInterval: initial_interval, initialEaseFactor: ease_factor });
    }
  };

  document.getElementById('apply-button').onclick = async function() {
    deck_name = document.getElementById('selected-deck').textContent;
    let initial_interval = parseInt(document.getElementById('initial-interval').value) * 3600;
    let ease_factor = parseFloat(document.getElementById('ease-factor').value);
    const { invoke } = window.__TAURI__.tauri;
    await invoke('set_deckoptions', { deckName: deck_name, initialInterval: initial_interval, initialEaseFactor: ease_factor });
  };

  async function reviewCard(difficulty) {
    const deck_text = document.getElementById('selected-deck').textContent;
    const question_text = document.getElementById('text-field').value;
    console.log(deck_text);
    console.log(question_text);
    if (deck_text !== '') {
      const { invoke } = window.__TAURI__.tauri;
      await invoke('review_card', { deckName: deck_text, cardQuestion: question_text, difficulty: difficulty });
      const [question, answer] = await invoke('get_card', { deckName: document.getElementById('selected-deck').textContent });
      document.getElementById('text-field').value = question;
      document.getElementById('text-field-2').value = answer;
      document.getElementById('text-field-2').style.display = 'none';
    }
  }
  
  const buttons = [
    { id: 'wrong-button', difficulty: 'wrong' },
    { id: 'hard-button', difficulty: 'hard' },
    { id: 'good-button', difficulty: 'good' },
    { id: 'easy-button', difficulty: 'easy' }
  ];
  
  buttons.forEach(button => {
    document.getElementById(button.id).onclick = function () {
      reviewCard(button.difficulty);
    };
  });

async function loadDecksFromBackend() {
    const { invoke } = window.__TAURI__.tauri;
    const decks = await invoke('get_deck_names');
    console.log(decks);
    const deckList = document.getElementById('deck-list');
    deckList.innerHTML = '';
    for (const deck of decks) {
        const listItem = document.createElement('li');
        listItem.textContent = deck.name;
        listItem.onclick = function () {
            document.getElementById('selected-deck').textContent = deck.name;
        };
        deckList.appendChild(listItem);
    }
}

async function loadDeckParams() {
  const { invoke } = window.__TAURI__.tauri;
  const params = await invoke('get_deckoptions', { deckName: document.getElementById('selected-deck').textContent });
  if (params) {
    console.log(params);

    const initialIntervalSelect = document.getElementById('initial-interval');
    const easeFactorSelect = document.getElementById('ease-factor');
    initialIntervalSelect.value = params[0].toString(); 
    easeFactorSelect.value = params[1].toFixed(1);
  } else {
    console.error("Deck parameters could not be loaded.");
  }
}
  