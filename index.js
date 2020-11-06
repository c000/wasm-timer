import 'bulma/css/bulma.css';

const rust = import('./pkg');

let target = document.getElementById('target');
let execButton = document.getElementById('exec');
let timespan = document.getElementById('timespan');

rust
  .then(m => {
    let r = new m.RState(target);

    let mainloop = () => {
      r.mainloop();
      setTimeout(mainloop, 100);
    };
    mainloop();

    execButton.addEventListener('click', e => {
      e.preventDefault();
      r.exec(timespan.value);
    });
  })
  .catch(console.error);
