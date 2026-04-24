(function() {
  var t = localStorage.getItem('theme') || 'light';
  document.documentElement.setAttribute('data-theme', t);

  document.addEventListener('DOMContentLoaded', function() {
    var toggle = document.getElementById('themeToggle');
    var menu = document.getElementById('themeMenu');
    if (!toggle || !menu) return;

    toggle.addEventListener('click', function(e) {
      e.stopPropagation();
      menu.classList.toggle('show');
    });

    document.querySelectorAll('.theme-option').forEach(function(o) {
      o.addEventListener('click', function() {
        var name = o.dataset.theme;
        document.documentElement.setAttribute('data-theme', name);
        localStorage.setItem('theme', name);
        menu.classList.remove('show');
        updateActiveTheme();
      });
    });

    document.addEventListener('click', function(e) {
      if (!e.target.closest('.theme-switcher')) {
        menu.classList.remove('show');
      }
    });

    updateActiveTheme();
  });

  function updateActiveTheme() {
    var current = document.documentElement.getAttribute('data-theme') || 'light';
    document.querySelectorAll('.theme-option').forEach(function(o) {
      o.classList.toggle('active-theme', o.dataset.theme === current);
    });
  }
})();
