import "./styles.css";

function updateReadingProgress() {
  const bar = document.querySelector<HTMLElement>("[data-reading-progress]");
  if (!bar) return;

  const scrollTop = window.scrollY;
  const max = document.documentElement.scrollHeight - window.innerHeight;
  const progress = max <= 0 ? 0 : Math.min(1, scrollTop / max);
  bar.style.transform = `scaleX(${progress})`;
}

function activateCurrentHeading() {
  const links = Array.from(document.querySelectorAll<HTMLAnchorElement>("[data-toc-link]"));
  if (links.length === 0) return;

  const headings = links
    .map((link) => document.getElementById(link.hash.slice(1)))
    .filter((heading): heading is HTMLElement => heading !== null);

  const active = headings
    .filter((heading) => heading.getBoundingClientRect().top < 140)
    .at(-1);

  for (const link of links) {
    link.toggleAttribute("aria-current", active ? link.hash === `#${active.id}` : false);
  }
}

document.addEventListener("click", async (event) => {
  const target = event.target;
  if (!(target instanceof HTMLElement)) return;

  const button = target.closest<HTMLButtonElement>("[data-copy-code]");
  if (!button) return;

  const shell = button.closest<HTMLElement>(".code-shell");
  const code = shell?.querySelector<HTMLElement>("code");
  if (!code) return;

  await navigator.clipboard.writeText(code.innerText);
  button.textContent = "Copied";
  window.setTimeout(() => {
    button.textContent = "Copy";
  }, 1300);
});

window.addEventListener("scroll", () => {
  updateReadingProgress();
  activateCurrentHeading();
}, { passive: true });

updateReadingProgress();
activateCurrentHeading();
