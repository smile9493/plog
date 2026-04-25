(function() {
  var isActive = false;
  var widget = null;
  var toggle = null;
  var menu = null;

  function createWidget() {
    widget = document.createElement('div');
    widget.className = 'float-widget reading-widget';
    widget.id = 'readingWidget';

    toggle = document.createElement('button');
    toggle.className = 'float-widget-toggle';
    toggle.title = '沉浸阅读';
    toggle.innerHTML = '<svg class="reading-icon book" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path></svg><svg class="reading-icon exit" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>';

    menu = document.createElement('div');
    menu.className = 'float-widget-menu';
    menu.id = 'readingMenu';

    var enterBtn = document.createElement('button');
    enterBtn.className = 'float-widget-option';
    enterBtn.setAttribute('data-action', 'enter');
    enterBtn.innerHTML = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path></svg><span>沉浸阅读</span>';

    var exitBtn = document.createElement('button');
    exitBtn.className = 'float-widget-option';
    exitBtn.setAttribute('data-action', 'exit');
    exitBtn.innerHTML = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg><span>退出阅读</span>';

    menu.appendChild(enterBtn);
    menu.appendChild(exitBtn);
    widget.appendChild(toggle);
    widget.appendChild(menu);

    toggle.addEventListener('click', function(e) {
      e.stopPropagation();
      closeAllMenus('readingWidget');
      menu.classList.toggle('show');
    });

    enterBtn.addEventListener('click', function() {
      enterReadingMode();
      menu.classList.remove('show');
    });

    exitBtn.addEventListener('click', function() {
      exitReadingMode();
      menu.classList.remove('show');
    });

    document.addEventListener('click', function(e) {
      if (!e.target.closest('.reading-widget')) {
        menu.classList.remove('show');
      }
    });

    document.body.appendChild(widget);
    updateState();
  }

  function closeAllMenus(except) {
    document.querySelectorAll('.float-widget-menu.show').forEach(function(m) {
      if (!m.parentNode || m.parentNode.id !== except) {
        m.classList.remove('show');
      }
    });
  }

  function enterReadingMode() {
    isActive = true;
    document.body.classList.add('reading-mode');
    localStorage.setItem('readingMode', 'true');
    updateState();
  }

  function exitReadingMode() {
    isActive = false;
    document.body.classList.remove('reading-mode');
    localStorage.removeItem('readingMode');
    updateState();
  }

  function updateState() {
    if (!toggle) return;
    if (isActive) {
      toggle.classList.add('active');
    } else {
      toggle.classList.remove('active');
    }
    var enterOpt = menu ? menu.querySelector('[data-action="enter"]') : null;
    var exitOpt = menu ? menu.querySelector('[data-action="exit"]') : null;
    if (enterOpt) enterOpt.style.display = isActive ? 'none' : 'flex';
    if (exitOpt) exitOpt.style.display = isActive ? 'flex' : 'none';
  }

  document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape' && isActive) {
      exitReadingMode();
    }
  });

  document.addEventListener('DOMContentLoaded', function() {
    createWidget();

    if (localStorage.getItem('readingMode') === 'true') {
      enterReadingMode();
    }
  });
})();
