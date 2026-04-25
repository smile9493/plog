(function() {
  var savedTheme = localStorage.getItem('theme') || 'zen';
  var savedIntensity = localStorage.getItem('themeIntensity') || 'light';

  var baseTheme = savedTheme;
  if (['zen', 'night', 'sepia'].indexOf(savedTheme) === -1) {
    baseTheme = savedIntensity === 'dark' ? savedTheme : 'zen';
  } else if (savedTheme === 'zen' || savedTheme === 'sepia') {
    baseTheme = savedTheme;
  }

  document.documentElement.setAttribute('data-theme', baseTheme);
  if (savedIntensity === 'dark' && ['zen', 'night', 'sepia'].indexOf(savedTheme) === -1) {
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

        if (['zen', 'night', 'sepia'].indexOf(theme) !== -1) {
          document.documentElement.setAttribute('data-theme', theme);
          document.documentElement.removeAttribute('data-theme-intensity');
          localStorage.setItem('theme', theme);
          localStorage.removeItem('themeIntensity');
        } else {
          document.documentElement.setAttribute('data-theme', theme);
          if (intensity === 'dark') {
            document.documentElement.setAttribute('data-theme-intensity', 'dark');
            localStorage.setItem('theme', theme);
            localStorage.setItem('themeIntensity', 'dark');
          } else {
            document.documentElement.removeAttribute('data-theme-intensity');
            localStorage.setItem('theme', theme);
            localStorage.removeItem('themeIntensity');
          }
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
    var currentTheme = document.documentElement.getAttribute('data-theme') || 'zen';
    var currentIntensity = document.documentElement.getAttribute('data-theme-intensity') || 'light';
    var savedTheme = localStorage.getItem('theme') || 'zen';
    var savedIntensity = localStorage.getItem('themeIntensity') || 'light';

    document.querySelectorAll('.theme-option').forEach(function(o) {
      var theme = o.dataset.theme;
      var intensity = o.dataset.intensity || 'light';

      var isActive = false;
      if (['zen', 'night', 'sepia'].indexOf(theme) !== -1) {
        isActive = (savedTheme === theme);
      } else {
        isActive = (savedTheme === theme && savedIntensity === intensity);
      }

      o.classList.toggle('active-theme', isActive);
    });
  }
})();
