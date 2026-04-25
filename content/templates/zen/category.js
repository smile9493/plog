(function() {
  var widget = null;
  var toggle = null;
  var menu = null;
  var categories = [];
  var activeCategory = null;

  function createWidget() {
    widget = document.createElement('div');
    widget.className = 'float-widget category-widget';
    widget.id = 'categoryWidget';

    toggle = document.createElement('button');
    toggle.className = 'float-widget-toggle';
    toggle.title = '分类筛选';
    toggle.innerHTML = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>';

    menu = document.createElement('div');
    menu.className = 'float-widget-menu category-menu';
    menu.id = 'categoryMenu';
    menu.innerHTML = '<div class="category-loading">加载中...</div>';

    widget.appendChild(toggle);
    widget.appendChild(menu);

    toggle.addEventListener('click', function(e) {
      e.stopPropagation();
      closeAllMenus('categoryWidget');
      menu.classList.toggle('show');
    });

    document.addEventListener('click', function(e) {
      if (!e.target.closest('.category-widget')) {
        menu.classList.remove('show');
      }
    });

    document.body.appendChild(widget);
    fetchCategories();
  }

  function closeAllMenus(except) {
    document.querySelectorAll('.float-widget-menu.show').forEach(function(m) {
      if (!m.parentNode || m.parentNode.id !== except) {
        m.classList.remove('show');
      }
    });
  }

  function fetchCategories() {
    fetch('/api/categories')
      .then(function(res) { return res.json(); })
      .then(function(data) {
        if (data.success && data.data) {
          categories = data.data;
          renderMenu();
        } else {
          menu.innerHTML = '<div class="category-empty">暂无分类</div>';
        }
      })
      .catch(function() {
        menu.innerHTML = '<div class="category-empty">加载失败</div>';
      });
  }

  function escapeHtml(str) {
    if (!str) return '';
    var div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
  }

  function renderMenu() {
    if (!categories.length) {
      menu.innerHTML = '<div class="category-empty">暂无分类</div>';
      return;
    }

    var html = '<button class="float-widget-option' + (!activeCategory ? ' active' : '') + '" data-category-id="">';
    html += '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect></svg>';
    html += '<span>全部</span></button>';

    categories.forEach(function(cat) {
      html += '<button class="float-widget-option' + (activeCategory === cat.sid ? ' active' : '') + '" data-category-id="' + cat.sid + '">';
      html += '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>';
      html += '<span>' + escapeHtml(cat.sortname) + '</span></button>';
    });

    menu.innerHTML = html;

    menu.querySelectorAll('.float-widget-option').forEach(function(btn) {
      btn.addEventListener('click', function() {
        var catId = btn.getAttribute('data-category-id');
        activeCategory = catId ? parseInt(catId) : null;
        menu.classList.remove('show');
        updateActiveState();
        dispatchCategoryChange();
      });
    });
  }

  function updateActiveState() {
    if (!menu) return;
    menu.querySelectorAll('.float-widget-option').forEach(function(btn) {
      var catId = btn.getAttribute('data-category-id');
      var isActive = catId === '' ? !activeCategory : activeCategory === parseInt(catId);
      btn.classList.toggle('active', isActive);
    });
    if (toggle) {
      toggle.classList.toggle('active', !!activeCategory);
    }
  }

  function dispatchCategoryChange() {
    var event = new CustomEvent('categoryChange', {
      detail: { categoryId: activeCategory }
    });
    document.dispatchEvent(event);
  }

  window.plogCategory = {
    getActive: function() { return activeCategory; },
    setCategory: function(catId) {
      activeCategory = catId;
      updateActiveState();
    }
  };

  document.addEventListener('DOMContentLoaded', function() {
    createWidget();
  });
})();
