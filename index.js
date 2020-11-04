const rust = import('./pkg');

let target = document.getElementById('target');

rust
  .then(m => {
    let x = new m.RState(target);

    let mainloop = () => {
      x.mainloop();
      setTimeout(mainloop, 100);
    };
    mainloop();
  })
  .catch(console.error);
