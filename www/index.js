import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 15; // size in pixels
const GRID_COLOR = "#28292c";
const ALIVE_COLOR = "#b0b4b9";
const DEAD_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

function getIndex(row, column) {
  return row * width + column;
}

function drawCells() {
  // Get cells from wasm
  const cellsPtr = universe.cells();
  // Create a copy of universe cells to modify
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  // Draw cells
  ctx.beginPath();
  
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
}

function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }
  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

let animationFrameId = null;

// The animation loop calculates time elapsed since the last loop
// and only draws if your specified fps interval is achieved
function animate() {
  drawGrid();
  drawCells();
  universe.tick();
  animationFrameId = requestAnimationFrame(animate);
}

// Initialize the timer variables and start the animation
function startAnimating() {
  animate();
}

const resetBtn = document.getElementById("reset-btn");
const randomBtn = document.getElementById("random-btn");
const purgeBtn = document.getElementById("purge-btn");
const playPauseBtn = document.getElementById("play-pause");
playPauseBtn.textContent = "Play";

function play() {
  playPauseBtn.textContent = "Pause";
  startAnimating();
}

function pause() {
  playPauseBtn.textContent = "Play";
  cancelAnimationFrame(animationFrameId);
  animationFrameId = null;
}

playPauseBtn.addEventListener("click", (e) => {
  if (animationFrameId === null) {
    play();
  } else {
    pause();
  }
});

randomBtn.addEventListener("click", (e) => {
  if (animationFrameId === null) {
    universe.random();
    drawGrid();
    drawCells();
  } else {
    pause();
    universe.random();
    drawGrid();
    drawCells();
  }
});

resetBtn.addEventListener("click", (e) => {
  if (animationFrameId === null) {
    universe.reset();
    drawGrid();
    drawCells();
  } else {
    pause();
    universe.reset();
    drawGrid();
    drawCells();
  }
});

purgeBtn.addEventListener("click", (e) => {
  if (animationFrameId === null) {
    universe.purge();
    drawGrid();
    drawCells();
  } else {
    universe.purge();
    drawGrid();
    drawCells();
  }
});

canvas.addEventListener("click", (e) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});


// Draw initial grid and cells
drawGrid();
drawCells();
