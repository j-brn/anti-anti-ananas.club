<template>
  <div class="grid h-screen place-items-center bg-pattern">
    <MessageBox />
  </div>
</template>

<script>
import MessageBox from "@/components/MessageBox.vue";

export default {
  name: 'App',
  components: {
    MessageBox
  }
}
</script>
