(function() {
  var savedTheme = localStorage.getItem('theme') || 'zen';
  var savedIntensity = localStorage.getItem('themeIntensity') || 'light';

  document.documentElement.setAttribute('data-theme', savedTheme);
  if (savedIntensity === 'dark') {
    document.documentElement.setAttribute('data-theme-intensity', 'dark');
  }

  document.addEventListener('DOMContentLoaded', function() {
    var toggle = document.getElementById('themeToggle');
    var menu = document.getElementById('themeMenu');
    if (!toggle || !menu) return;

    toggle.addEventListener('click', function(e) {
      e.stopPropagation();
      closeOtherMenus('themeSwitcher');
      menu.classList.toggle('show');
    });

    document.querySelectorAll('.theme-option').forEach(function(o) {
      o.addEventListener('click', function() {
        var theme = o.dataset.theme;
        var intensity = o.dataset.intensity || 'light';

        document.documentElement.setAttribute('data-theme', theme);
        localStorage.setItem('theme', theme);

        if (intensity === 'dark') {
          document.documentElement.setAttribute('data-theme-intensity', 'dark');
          localStorage.setItem('themeIntensity', 'dark');
        } else {
          document.documentElement.removeAttribute('data-theme-intensity');
          localStorage.removeItem('themeIntensity');
        }

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

  function closeOtherMenus(except) {
    document.querySelectorAll('.float-widget-menu.show').forEach(function(m) {
      if (!m.parentNode || m.parentNode.id !== except) {
        m.classList.remove('show');
      }
    });
  }

  function updateActiveTheme() {
    var savedTheme = localStorage.getItem('theme') || 'zen';
    var savedIntensity = localStorage.getItem('themeIntensity') || 'light';

    document.querySelectorAll('.theme-option').forEach(function(o) {
      var theme = o.dataset.theme;
      var intensity = o.dataset.intensity || 'light';
      var isActive = (savedTheme === theme && (intensity === 'dark' ? savedIntensity === 'dark' : true));
      o.classList.toggle('active-theme', isActive);
    });
  }
})();
