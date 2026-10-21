/* Farouk Pandor — minimal progressive enhancement.
   No dependencies. Everything degrades to a readable page without JS. */
(function () {
  "use strict";

  // Current year in the footer
  var year = document.getElementById("year");
  if (year) year.textContent = String(new Date().getFullYear());

  // Work filter — only activates once we know JS is running
  var bar = document.querySelector(".filters");
  var list = document.getElementById("worklist");
  var status = document.getElementById("filter-status");
  if (!bar || !list || !status) return;

  bar.hidden = false;
  var buttons = Array.prototype.slice.call(bar.querySelectorAll(".chip"));
  var items = Array.prototype.slice.call(list.querySelectorAll(".work"));

  function apply(filter) {
    var shown = 0;
    items.forEach(function (item) {
      var caps = (item.getAttribute("data-caps") || "").split(/\s+/);
      var match = filter === "all" || caps.indexOf(filter) !== -1;
      item.hidden = !match;
      if (match) shown++;
    });

    buttons.forEach(function (btn) {
      btn.setAttribute("aria-pressed", String(btn.dataset.filter === filter));
    });

    var label = filter === "all" ? "all disciplines" : filter;
    status.textContent = shown + " project" + (shown === 1 ? "" : "s") + " shown for " + label + ".";
  }

  bar.addEventListener("click", function (event) {
    var btn = event.target.closest(".chip");
    if (!btn || !bar.contains(btn)) return;
    apply(btn.dataset.filter);
  });
})();
