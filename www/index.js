import { Cpu } from "icpu";

const view = document.getElementById("emulator");

const cpu = Cpu.new();

const renderLoop = () => {
  cpu.tick();

  view.textContent = cpu.render();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
