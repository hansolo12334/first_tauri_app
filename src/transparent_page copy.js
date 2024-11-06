const { invoke } = window.__TAURI__.core;
const { emit, listen } = window.__TAURI__.event;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;

let startX, startY, endX, endY, width, height;

let startX_mouse, //鼠标 起始
  startY_mouse, //鼠标 起始
  startX_selection, // 框起始
  startY_selection, //框起始
  selection_top,
  selection_left,
  selection_offsetX,
  selection_offsetY;
//框选区域是否建立
var isSelectionAreaBuild = false;
var isSelectionAreaListening = false;

const main_overlay = document.querySelector(".main_overlay");
const overlay1_canvas = document.getElementById("overlay1-canvas");
const currentWindow = getCurrentWebviewWindow();
const overlay1_canvas_background = document.getElementById(
  "overlay1-canvas-bottom"
);

overlay1_canvas.height = window.innerHeight;
overlay1_canvas.width = window.innerWidth;
overlay1_canvas_background.height = window.innerHeight;
overlay1_canvas_background.width = window.innerWidth;

let ctx = overlay1_canvas.getContext("2d");
let ctx_background = overlay1_canvas_background.getContext("2d");

const selection = document.createElement("div");
const selection_root = document.createElement("div");

const selection_options = document.createElement("div");
const selection_options_button_apply = document.createElement("img");
const selection_options_button_cancel = document.createElement("img");

function fill_canvas_overlay() {
  ctx.clearRect(0, 0, overlay1_canvas.width, overlay1_canvas.height);
  ctx.fillStyle = "rgba(0, 0, 0, 0.5)"; // 背景颜色
  // console.log(
  //   `画布大小 ${overlay1_canvas.offsetWidth} ${overlay1_canvas.offsetHeight}`
  // );
  ctx.fillRect(0, 0, overlay1_canvas.width, overlay1_canvas.height);
}

// fill_canvas_overlay();

function drawSelectionArea(start_x, start_y) {
  // console.log(`${startX} ${startY} ${width} ${height}`);
  ctx.clearRect(start_x, start_y, width + 4, height + 4); // 清除选择区域
}

// ctx_background.drawImage(overlay1_canvas);
//获取窗口的截图

currentWindow.listen("screenshot-captured", (event) => {
  console.log("js收到截图!");
  const base64Image = event.payload;
  const img = new Image();
  img.src = `data:image/png;base64,${base64Image}`;
  img.onload = () => {
    ctx_background.clearRect(
      0,
      0,
      overlay1_canvas_background.width,
      overlay1_canvas_background.height
    );
    console.log("绘制截图!");
    let scale = Math.min(
      overlay1_canvas_background.width / img.width,
      overlay1_canvas_background.height / img.height
    );
    let new_width = img.width * scale;
    let new_height = img.height * scale;
    ctx_background.drawImage(img, 0, 0, new_width, new_height);
  };
});

emit("get_screenshot");

window.addEventListener("DOMContentLoaded", () => {
  document.addEventListener("keydown", (event) => {
    var keycode = event.key;

    if (keycode == "Escape") {
      console.log(`${keycode} pressed`);

      emit("exit_screenShot");
    }
  });

  selection_root.className = "selection-root";
  selection.className = "selection-area";

  selection_root.className.appendChild(selection);

  main_overlay.appendChild(selection_root);
  // document.appendChild(selection);

  console.log("截屏:1");

  overlay1_canvas.addEventListener("mousedown", (e) => {
    if (isSelectionAreaBuild) {
      console.log("存在框 屏蔽!");
      return;
    }
    console.log(`${e.target}`);
    console.log("截屏:mousedown");
    //清除状态
    width = 0;
    height = 0;
    document.querySelector(".selection-area").textContent = ``;
    selection.style.pointerEvents = "none";
    fill_canvas_overlay();
    selection_offsetX = 0;
    selection_offsetY = 0;
    selection_top = 0;
    selection_left = 0;

    startX = e.clientX;
    startY = e.clientY;
    selection.style.left = `${startX}px`;
    selection.style.top = `${startY}px`;
    selection.style.width = "0px";
    selection.style.height = "0px";

    overlay1_canvas.addEventListener("mousemove", onMouseMove);
  });

  // overlay1.addEventListener("mouseenter", () => {
  //   console.log("鼠标移入overlay");
  //   overlay1.style.cursor = "crosshair";
  // });
  // overlay1.addEventListener("mouseout", () => {
  //   console.log("鼠标移出overlay");
  //   overlay1.style.cursor = "default";
  // });

  overlay1_canvas.addEventListener("mouseup", (e) => {
    if (isSelectionAreaBuild) {
      console.log("存在框 屏蔽!");
      return;
    }
    console.log("截屏:mouseup");

    if (width >= 10 && height >= 10) {
      isSelectionAreaBuild = true;

      selection.style.pointerEvents = "auto";
      console.log(`width: ${width} height:${height}`);

      //添加鼠标移出框选事件
      selection.addEventListener("mouseenter",react_for_cursor_moveIn_selection);
      selection.addEventListener("mouseout", react_for_cursor_moveOut_selection);

      //添加鼠标拖动截图框事件
      selection.addEventListener("mousedown", react_for_selection_mousedown);

      selection.addEventListener("mouseup", react_for_selection_mouseup);

      selection.appendChild(selection_options);

      build_up_selection_options_area();
    } else {
      isSelectionAreaBuild = false;
      selection.style.pointerEvents = "none";
    }

    overlay1_canvas.removeEventListener("mousemove", onMouseMove);
    endX = e.clientX;
    endY = e.clientY;
    // captureScreen();
  });
});

function react_for_cursor_moveIn_selection(e) {
  console.log("鼠标移入selection");
  overlay1_canvas.style.cursor = "move";
}

function react_for_cursor_moveOut_selection(e) {
  console.log("鼠标移出selection");
  overlay1_canvas.style.cursor = "crosshair";
}

function build_up_selection_options_area() {
  console.log("build_up_selection_options_area");
  selection_options.className = "selection-options";

  selection_options_button_apply.className = "selection-options-button-apply";
  selection_options_button_cancel.className = "selection-options-button-cancel";
  selection_options_button_apply.src = "/assets/yes.svg";
  selection_options_button_cancel.src = "/assets/no.svg";

  if (selection_options.contains(selection_options_button_apply)) {
    selection_options.removeChild(selection_options_button_apply);
  }
  if (selection_options.contains(selection_options_button_cancel)) {
    selection_options.removeChild(selection_options_button_cancel);
  }
  selection_options.appendChild(selection_options_button_apply);
  selection_options.appendChild(selection_options_button_cancel);

  selection_options_button_apply.addEventListener("click",react_for_selection_option_accept);

  selection_options_button_cancel.addEventListener("click", react_for_selection_option_refus);
}

function react_for_selection_option_accept(e) {
  console.log("确认截图");
  fill_canvas_overlay();
  isSelectionAreaBuild = false;
  clear_selection_state();

  // if (selection.contains(selection_options)) {
  //   selection.removeChild(selection_options);
  // }
}

function react_for_selection_option_refus(e) { 
  fill_canvas_overlay();
  isSelectionAreaBuild = false;
  console.log("取消截图");

  clear_selection_state();

  // if (selection.contains(selection_options)) {
  //   selection.removeChild(selection_options);
  // }
}

function react_for_selection_mousedown(e) {
  startX_mouse = e.clientX;
  startY_mouse = e.clientY;

  startX_selection = parseInt(selection.style.left, 10);
  startY_selection = parseInt(selection.style.top, 10);

  console.log("selection 鼠标按下");
  // selection_options.classList.toggle("invisable");
  // if (selection.contains(selection_options)) {
  //   selection.removeChild(selection_options);
  // }
  selection.removeEventListener("mouseenter", react_for_cursor_moveIn_selection);
  selection.removeEventListener("mouseout", react_for_cursor_moveOut_selection);

  selection.addEventListener("mousemove", update_selection);
}

function react_for_selection_mouseup(e) {
  console.log("selection 鼠标松开");
  selection_left = selection_left + selection_offsetX;
  selection_top = selection_top + selection_offsetY;
  // selection_options.classList.remove("invisable");
  
  if (!selection.contains(selection_options)) {
    selection.appendChild(selection_options);
  }

  selection_options_button_apply.removeEventListener("click", react_for_selection_option_accept);
  selection_options_button_cancel.removeEventListener("click", react_for_selection_option_refus);

  build_up_selection_options_area();

  selection.addEventListener("mouseenter", react_for_cursor_moveIn_selection);
  selection.addEventListener("mouseout", react_for_cursor_moveOut_selection);

  selection.removeEventListener("mousemove", update_selection);
  console.log("清除截图框移动事件");
}

function clear_selection_state() {
  //清除状态
  width = 0;
  height = 0;
  document.querySelector(".selection-area").textContent = ``;
  document.querySelector(".selection-area").style.height = "0px";
  document.querySelector(".selection-area").style.width = "0px";
  document.querySelector(".selection-area").style.top = "0px";
  document.querySelector(".selection-area").style.left = "0px";
  selection.style.pointerEvents = "none";

  console.log("清除截图框所有事件");
  selection.removeEventListener("mousedown", react_for_selection_mousedown);

  selection.removeEventListener("mouseup", react_for_selection_mouseup);
  selection.removeEventListener("mousemove", update_selection);

  selection_offsetX = 0;
  selection_offsetY = 0;
  selection_top = 0;
  selection_left = 0;
}

function onMouseMove(e) {
  const currentX = e.clientX; //鼠标
  const currentY = e.clientY; //鼠标
  width = Math.abs(currentX - startX);
  height = Math.abs(currentY - startY);
  if (width >= 10 && height >= 10) {
    // isSelectionAreaBuild = true;

    selection.style.width = `${width}px`;
    selection.style.height = `${height}px`;
    var start_x = Math.min(currentX, startX);
    var start_y = Math.min(currentY, startY);
    if (start_x < 0) {
      start_x = 0;
    } else if (start_x > window.offsetWidth) {
      start_x = window.offsetWidth;
    }
    if (start_y < 0) {
      start_y = 0;
    } else if (start_y > window.offsetHeight) {
      start_y = window.offsetHeight;
    }
    selection.style.left = `${start_x}px`;
    selection.style.top = `${start_y}px`;
    selection_left = start_x;
    selection_top = start_y;
    //清下方图层颜色
    fill_canvas_overlay();
    drawSelectionArea(start_x, start_y);
    // selection.tex = `${width}X${height}`;
    document.querySelector(
      ".selection-area"
    ).textContent = `${width} x ${height}`;
    console.log(`鼠标移动: ${currentX} ${currentY}`);
  } else {
    // isSelectionAreaBuild = false;
  }
}
function update_selection(e) {
  console.log(`拖动selection! ${selection.style.left}`);
  var currentX = e.clientX; //当前鼠标的坐标
  var currentY = e.clientY;

  console.log(`currentX: ${currentX}`);
  console.log(`startX_mouse: ${startX_mouse}`);
  var x_offset = currentX - startX_mouse; //鼠标的偏移
  var y_offset = currentY - startY_mouse; //鼠标的偏移

  //清下方图层颜色
  fill_canvas_overlay();

  // 处理边界的特殊情况
  let is_speacific_state = false;
  if (selection_left + x_offset < 0) {
    selection.style.left = `0px`;
    x_offset = parseInt(selection.style.left, 10) - startX_selection;
    is_speacific_state = true;
  }
  if (selection_top + y_offset < 0) {
    selection.style.top = `0px`;
    y_offset = parseInt(selection.style.top, 10) - startY_selection; //到达边界 让鼠标偏移量变为一个最大的固定值
    is_speacific_state = true;
  }

  // selection.style.border.width
  if (
    selection_left + x_offset + parseInt(selection.style.width, 10)+2 >
    overlay1_canvas.width
  ) {
    console.log("left超出右侧");
    selection.style.left = `${
      overlay1_canvas.width - parseInt(selection.style.width, 10) - 4
    }px`;
    x_offset = parseInt(selection.style.left, 10) - startX_selection;
    is_speacific_state = true;
  }

  //更新框选项
  // if (
  //   selection_top + y_offset + parseInt(selection.style.height, 10)+32 >
  //   overlay1_canvas.height
  // ) {
  //   selection_options.classList.toggle("onTop");
  // } else {
  //   selection_options.classList.remove("onTop");
  // }


  if (
    selection_top + y_offset + parseInt(selection.style.height, 10)+2 >
    overlay1_canvas.height
  ) {
    console.log("top超出下侧");
    selection.style.top = `${
      overlay1_canvas.height - parseInt(selection.style.height, 10) - 2
    }px`;
    y_offset = parseInt(selection.style.top, 10) - startY_selection;
    is_speacific_state = true;
  } 

  //正常情况没有到边界 直接使用鼠标偏移更新
  if (!is_speacific_state) {
    selection.style.left = `${selection_left + x_offset}px`;
    selection.style.top = `${selection_top + y_offset}px`;
  }

  // selection.style.left = `${selection_left + x_offset}px`;
  // selection.style.top = `${selection_top + y_offset}px`;
  let top_y = parseInt(selection.style.top, 10);
  let left_x = parseInt(selection.style.left, 10);
  drawSelectionArea(left_x, top_y);

  selection_offsetX = x_offset;
  selection_offsetY = y_offset;

  console.log(`selection_left: ${selection_left}`);
  console.log(`x_offset: ${x_offset}`);
  console.log(
    `当前计算左边坐标 ${
      selection_left + x_offset + parseInt(selection.style.width, 10)
    }`
  );
  console.log(`${overlay1_canvas.width}`);
}
