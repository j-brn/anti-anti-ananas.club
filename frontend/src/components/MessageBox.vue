<template>
  <div>
    <div
        class="w-screen lg:w-screen-80 h-[260px] lg:h-[450px] overflow-hidden bg-black rounded-2xl px-16 py-8 bg-opacity-95 shadow-xl text-center flex flex-col justify-center items-center"
    >
      <transition name="fadeOne">
        <h3 class="container text-white text-3xl lg:text-5xl mb-4" v-show="show">
          {{ currentPreamble }}
        </h3>
      </transition>
      <transition name="fadeTwo">
        <h1 class="container text-red-500 text-5xl lg:text-8xl" v-show="show">
          {{ currentMessage }}
        </h1>
      </transition>
    </div>
  </div>
</template>

<script>
export default {
  name: "MessageBox",
  data: () => ({
    currentPreamble: '',
    currentMessage: '',
    show: true
  }),
  created() {
    this.fetchMessage();
    setInterval(() => {
      this.fetchMessage();
    }, 4000);
  },
  methods: {
    fetchMessage() {
      fetch('https://anti-anti-ananas.club/api/message')
          .then((response) => response.json())
          .then((data) => {
            this.show = false;
            setTimeout(() => {
              this.currentPreamble = data.preamble;
              this.currentMessage = data.message;
              this.show = true;
            }, 500)
          });
    }
  },
}
</script>

<style scoped>

</style>