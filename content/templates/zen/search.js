(function() {
  var overlay = null;
  var input = null;
  var resultsContainer = null;
  var debounceTimer = null;
  var isOpen = false;

  function createSearchUI() {
    overlay = document.createElement('div');
    overlay.className = 'search-overlay';
    overlay.id = 'searchOverlay';

    var dialog = document.createElement('div');
    dialog.className = 'search-dialog';

    var header = document.createElement('div');
    header.className = 'search-header';

    var searchIcon = document.createElement('span');
    searchIcon.className = 'search-input-icon';
    searchIcon.innerHTML = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>';

    input = document.createElement('input');
    input.type = 'text';
    input.className = 'search-input';
    input.id = 'searchInput';
    input.placeholder = '搜索文章...';
    input.autocomplete = 'off';

    var shortcut = document.createElement('kbd');
    shortcut.className = 'search-shortcut';
    shortcut.textContent = 'ESC';

    header.appendChild(searchIcon);
    header.appendChild(input);
    header.appendChild(shortcut);

    resultsContainer = document.createElement('div');
    resultsContainer.className = 'search-results';
    resultsContainer.id = 'searchResults';

    var hint = document.createElement('div');
    hint.className = 'search-hint';
    hint.innerHTML = '<span>输入关键词搜索文章</span><span class="search-hint-keys"><kbd>↑↓</kbd> 导航 <kbd>Enter</kbd> 打开</span>';

    dialog.appendChild(header);
    dialog.appendChild(resultsContainer);
    dialog.appendChild(hint);
    overlay.appendChild(dialog);

    overlay.addEventListener('click', function(e) {
      if (e.target === overlay) closeSearch();
    });

    input.addEventListener('input', function() {
      clearTimeout(debounceTimer);
      var query = input.value.trim();
      if (!query) {
        resultsContainer.innerHTML = '';
        return;
      }
      debounceTimer = setTimeout(function() {
        performSearch(query);
      }, 250);
    });

    input.addEventListener('keydown', function(e) {
      if (e.key === 'Escape') {
        closeSearch();
      } else if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
        e.preventDefault();
        navigateResults(e.key === 'ArrowDown' ? 1 : -1);
      } else if (e.key === 'Enter') {
        var active = resultsContainer.querySelector('.search-result-item.active');
        if (active) {
          var link = active.getAttribute('data-url');
          if (link) window.location.href = link;
        }
      }
    });

    document.body.appendChild(overlay);
  }

  function performSearch(query) {
    fetch('/api/posts?keyword=' + encodeURIComponent(query) + '&status=n&per_page=10')
      .then(function(res) { return res.json(); })
      .then(function(data) {
        if (data.success && data.data && data.data.items && data.data.items.length > 0) {
          renderResults(data.data.items, query);
        } else {
          resultsContainer.innerHTML = '<div class="search-empty">未找到相关文章</div>';
        }
      })
      .catch(function() {
        resultsContainer.innerHTML = '<div class="search-empty">搜索失败，请重试</div>';
      });
  }

  function escapeHtml(str) {
    if (!str) return '';
    var div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
  }

  function highlightText(text, query) {
    if (!query) return escapeHtml(text);
    var escaped = escapeHtml(text);
    var regex = new RegExp('(' + query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + ')', 'gi');
    return escaped.replace(regex, '<mark>$1</mark>');
  }

  function formatTimestamp(ts) {
    if (!ts) return '';
    var d = new Date(ts * 1000);
    return d.getFullYear() + '-' + String(d.getMonth() + 1).padStart(2, '0') + '-' + String(d.getDate()).padStart(2, '0');
  }

  function renderResults(items, query) {
    var html = '';
    items.forEach(function(item, index) {
      var excerpt = item.excerpt || '';
      if (!excerpt && item.content) {
        excerpt = item.content.replace(/[#*`\n]/g, ' ').substring(0, 120);
      }
      html += '<div class="search-result-item' + (index === 0 ? ' active' : '') + '" data-url="/post.html?id=' + item.gid + '">';
      html += '<div class="search-result-title">' + highlightText(item.title, query) + '</div>';
      html += '<div class="search-result-excerpt">' + highlightText(excerpt, query) + '</div>';
      html += '<div class="search-result-meta"><span>' + formatTimestamp(item.date) + '</span></div>';
      html += '</div>';
    });
    resultsContainer.innerHTML = html;

    resultsContainer.querySelectorAll('.search-result-item').forEach(function(el) {
      el.addEventListener('click', function() {
        var link = el.getAttribute('data-url');
        if (link) window.location.href = link;
      });
      el.addEventListener('mouseenter', function() {
        resultsContainer.querySelectorAll('.search-result-item').forEach(function(item) {
          item.classList.remove('active');
        });
        el.classList.add('active');
      });
    });
  }

  function navigateResults(direction) {
    var items = resultsContainer.querySelectorAll('.search-result-item');
    if (!items.length) return;
    var current = resultsContainer.querySelector('.search-result-item.active');
    var index = -1;
    items.forEach(function(item, i) {
      if (item === current) index = i;
    });
    items.forEach(function(item) { item.classList.remove('active'); });
    var next = index + direction;
    if (next < 0) next = items.length - 1;
    if (next >= items.length) next = 0;
    items[next].classList.add('active');
    items[next].scrollIntoView({ block: 'nearest' });
  }

  function openSearch() {
    if (!overlay) createSearchUI();
    isOpen = true;
    overlay.classList.add('show');
    document.body.style.overflow = 'hidden';
    setTimeout(function() { input.focus(); }, 50);
  }

  function closeSearch() {
    if (!overlay) return;
    isOpen = false;
    overlay.classList.remove('show');
    document.body.style.overflow = '';
    input.value = '';
    resultsContainer.innerHTML = '';
  }

  document.addEventListener('keydown', function(e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      if (isOpen) closeSearch(); else openSearch();
    }
    if (e.key === '/' && !e.ctrlKey && !e.metaKey) {
      var tag = document.activeElement.tagName.toLowerCase();
      if (tag === 'input' || tag === 'textarea') return;
      e.preventDefault();
      if (isOpen) closeSearch(); else openSearch();
    }
  });

  document.addEventListener('DOMContentLoaded', function() {
    var searchBtn = document.getElementById('searchBtn');
    if (searchBtn) {
      searchBtn.addEventListener('click', function(e) {
        e.preventDefault();
        e.stopPropagation();
        if (isOpen) closeSearch(); else openSearch();
      });
    }
  });
})();
