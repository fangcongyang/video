<template>
  <!--    全屏容器    -->
  <div
    ref="pageDivRef"
    @mousemove="demo_move"
    @mouseup="demo_up"
    :class="{ zlevelTop: status.mouseDownState }"
    style="position: absolute; top: 0; height: 100%; width: 100%"
  >
    <!--  点击蒙版  -->
    <div
      v-if="status.menuOpen"
      @click.stop="closeOpenModal"
      style="
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 998;
      "
    ></div>
    <!--  多功能菜单 -->
    <div
      :class="{
        'six-more-modal-btn': status.menuOpen,
        moreModal: !status.menuOpen,
        'more-tran-animate': !status.mouseDownState,
      }"
      ref="actionMgrRef"
      @mousedown="demo_down"
      v-on:contextmenu.prevent="demo_click($event)"
    >
      <!--  触发器 -->
      <div v-if="!status.menuOpen" @click="demo_click" class="imgMore">
        <img class="more-img" src="" alt="" title="多功能菜单" />
      </div>
      <!--  菜单  -->
      <div v-else>1111</div>
    </div>
  </div>
</template>

<script>
import { defineComponent, reactive, ref, onMounted } from "vue";

export default defineComponent({
  name: "homeDragbtn",
  props: {
    // 通过position来设置初始定位
    position: {
      type: Object,
      default: function () {
        return {
          top: "12.25rem",
          left: "52.25rem",
        }
      },
    },
  },
  setup(props) {
    const status = reactive({
      menuOpen: false, //  菜单展开状态
      mouseDownState: false, //  鼠标点击状态
      iX: 0,
      iY: 0,
      dX: 0,
      dY: 500, //  初始定位
      lastMoveIndex: 0, //  拖拽计数
      curMoveIndex: 0, //  历史计数
    });
    
    const pageDivRef = ref();
    const actionMgrRef = ref();

    //  鼠标按下
    const demo_down = (event) => {
      //  如果打开了菜单，则不做响应
      if (status.menuOpen) {
        status.mouseDownState = false;
        return;
      }
      /* 此处判断  pc 或 移动端 得到 event 事件 */
      var touch;
      if (event.touches) {
        touch = event.touches[0];
      } else {
        touch = event;
      }
      // 鼠标点击 面向页面 的 x坐标 y坐标
      let { clientX, clientY } = touch;
      // 鼠标x坐标 - 拖拽按钮x坐标  得到鼠标 距离 拖拽按钮 的间距
      status.iX = clientX - actionMgrRef.value.offsetLeft;
      // 鼠标y坐标 - 拖拽按钮y坐标  得到鼠标 距离 拖拽按钮 的间距
      status.iY = clientY - actionMgrRef.value.offsetTop;
      // 设置当前 状态为 鼠标按下
      status.mouseDownState = true;
    };

    //  鼠标拖拽
    const demo_move = (event) => {
      //鼠标按下 切移动中
      if (status.mouseDownState) {
        /* 此处判断  pc 或 移动端 得到 event 事件 */
        var touch;
        if (event.touches) {
          touch = event.touches[0];
        } else {
          touch = event;
        }
        // 鼠标移动时 面向页面 的 x坐标 y坐标
        let { clientX, clientY } = touch;
        //当前页面全局容器 dom 元素  获取容器 宽高
        let { clientHeight: pageDivY, clientWidth: pageDivX } = pageDivRef.value;
        /* 鼠标坐标 - 鼠标与拖拽按钮的 间距坐标  得到 拖拽按钮的 左上角 x轴y轴坐标 */
        let [x, y] = [clientX - status.iX, clientY - status.iY];

        //拖拽按钮 dom 元素  获取 宽高 style 对象
        let {
          clientHeight: actionMgrY,
          clientWidth: actionMgrX,
          style: actionMgrStyle,
        } = actionMgrRef.value;
        /* 此处判断 拖拽按钮 如果超出 屏幕宽高 或者 小于
                 设置 屏幕最大 x=全局容器x y=全局容器y 否则 设置 为 x=0 y=0
              */
        if (x > pageDivX - actionMgrX) x = pageDivX - actionMgrX;
        else if (x < 0) x = 0;
        if (y > pageDivY - actionMgrY) y = pageDivY - actionMgrY;
        else if (y < 0) y = 0;
        status.dX = x;
        status.dY = y;
        // 计算后坐标  设置 按钮位置
        actionMgrStyle.left = `${x}px`;
        actionMgrStyle.top = `${y}px`;
        actionMgrStyle.bottom = "auto";
        actionMgrStyle.right = "auto";
        //  move Index
        status.lastMoveIndex++;
        //  当按下键滑动时， 阻止屏幕滑动事件
        event.preventDefault();
      }
    };

    //    鼠标抬起
    const demo_up = (event) => {
      if (!status.mouseDownState) {
        return;
      }
      //  当前页面全局容器 dom 元素  获取容器 宽高
      let { clientHeight: windowHeight, clientWidth: windowWidth } = pageDivRef.value;
      //  拖拽按钮 dom 元素  获取 宽高 style 对象
      let {
        style: actionMgrStyle,
      } = actionMgrRef.value;

      // 计算后坐标  设置 按钮位置
      if (status.dY > 0 && status.dY < windowHeight - 50) {
        if (status.dY >= windowHeight / 2) {
          actionMgrStyle.top = "auto";
          actionMgrStyle.bottom = windowHeight - status.dY + "px";
        }
      } else {
        if (status.dY === 0) {
          //  在顶部
          actionMgrStyle.top = 0;
          actionMgrStyle.bottom = "auto";
        } else if (status.dY === windowHeight - 50) {
          actionMgrStyle.bottom = 0;
          actionMgrStyle.top = "auto";
        }
        if (status.dX >= windowWidth / 2) {
          //  右侧是将left改为auto，调整right
          actionMgrStyle.left = "auto";
          actionMgrStyle.right = windowWidth - status.dX + 50 + "px";
        }
      }
      status.mouseDownState = false;
    };

    //    单击事件
    const demo_click = () => {
      //  mouseup 后会激活click事件
      //  如果上一次down事件到下一次click事件中经历10次以下move，则视为纯点击事件
      if (status.lastMoveIndex - status.curMoveIndex <= 10) {
        //  点击事件
        status.menuOpen = !status.menuOpen;
        if (status.menuOpen) {
          //  打开菜单
        }
      }
      status.curMoveIndex = status.lastMoveIndex;
    };
    //    点击空白关闭菜单
    const closeOpenModal = () => {};

    onMounted(() => {
      actionMgrRef.value.style.top = props.position.top;
      actionMgrRef.value.style.left = props.position.left;
    })

    return {
      status,
      demo_down,
      demo_move,
      demo_up,
      demo_click,
      closeOpenModal,
      pageDivRef,
      actionMgrRef,
    };
  },
});
</script>

<style type="text/css">
.zlevelTop {
  z-index: 9999;
}
.more-tran-animate {
  transition: 0.5s;
}
.moreModal {
  /* 如果碰到滑动问题，1.3 请检查 z-index。z-index需比web大一级*/
  z-index: 9999;
  position: fixed;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background-color: #337ab7;
  line-height: 40px;
  text-align: center;
  color: #fff;
  opacity: 0.6;
}
.moreModal:hover {
  opacity: 1;
}
.six-more-modal-btn {
  position: fixed;
  z-index: 9999;
  width: 14rem;
  height: 14rem;
  border-radius: 5px;
  background: #fff;
  color: #fff;
}
.imgMore {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}
</style>
