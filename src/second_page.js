const { invoke } = window.__TAURI__.core;
const { emit, listen } = window.__TAURI__.event;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;

// (async () => {
//   await listen('window-resized2x', (event) => {
//     console.log(`New window resized to: ${event.payload.width} x ${event.payload.height}`);
//   });
// })();

var width_over = false;

function updateLayout(width) {
  const main_container = document.querySelector(".main_container");
  const right_container = document.querySelector(".right_container");
  if (width > 960) {
    console.log("窗口宽度大于960!");
    if (!width_over) {
      width_over = true;
      // if (!newLayout) {
        // main_container.style.flexDirection = "column";
        // const newDiv = doocument.createElement("div");
        // newDiv.id = "right-container";
        // newDiv.className = "right-container";
        // newDiv.textContent = "new-right-layout";
        // main_container.appendChild(newDiv);
        right_container.classList.toggle("visible");

        console.log("增加监听!");
      // }
    }
  } else {
    // main_container.style.flexDirection = "row";
    if (width_over) {
      console.log("删除布局");
      width_over = false;
      // if (newLayout) {
        // main_container.removeChild(newLayout);
        right_container.classList.remove("visible");
        // console.log("移除监听!");
      // }
    }
  }
}



async function openScreenShutWindowClicked() {
  console.log("openScreenShutWindowClicked!");
  
  
  try {
    await invoke("open_screen_shot_page_derict");
  } catch (error) {
    console.error("打开截图窗口失败", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector(".open-setting-button")
    .addEventListener("click", () => {
      console.log("click!");
      //改变按钮颜色
      document
        .querySelector(".open-setting-button")
        .classList.toggle("opening_color");
      //显示侧边栏
      document.getElementById("sidebar").classList.toggle("visible");
    });

  document
    .querySelector(".left_container")
    .addEventListener("click", (event) => {
      // 检查点击的目标元素是否是 open-setting-button
      if (!event.target.closest(".open-setting-button")) {
        //如果侧边栏状态为打开 则关闭 否则无事发时
        if (document.getElementById("sidebar").classList.contains("visible")) {
          document.getElementById("sidebar").classList.remove("visible");
          //改变按钮颜色 保持和周围背景一个颜色
          document
            .querySelector(".open-setting-button")
            .classList.remove("opening_color");
          console.log("close side click!");
          event.stopPropagation(); //阻止事件冒泡 传递到子类内
        }
      }
    });

  document
    .querySelector(".right_container")
    .addEventListener("click", (event) => {
      // 检查点击的目标元素是否是 open-setting-button
      if (!event.target.closest(".open-setting-button")) {
        if (document.getElementById("sidebar").classList.contains("visible")) {
          document.getElementById("sidebar").classList.remove("visible");
          console.log("close side click!");
          event.stopPropagation(); //阻止事件冒泡 传递到子类内
        }
      }
    });

  const function_button = document.querySelectorAll(".function-button");

  function_button.forEach((button) => {
    button.addEventListener("click", () => {
      console.log(`${button.textContent} clicked!`);
      // console.log(`${document.querySelector('.button-text').textContent} to!`);
      document.querySelector(".button-text").textContent = button.textContent;
    });
  });

  const currentWindow = getCurrentWebviewWindow();

  currentWindow.listen("window-resized2x", (event) => {
    // console.log(`New window resized to: ${event.payload.width} x ${event.payload.height}`);
    updateLayout(event.payload.width);
  });


  //截图按钮点击
  document.querySelector('.screenshot-image').addEventListener('click', openScreenShutWindowClicked);
});
