<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { RouteTestClient } from '../test.client';


defineProps<{ msg: string }>()

const count = ref(0)

const query_message = ref("loading")
const stream_message = ref("loading")

onMounted(async () => {
  let transport = new GrpcWebFetchTransport({ baseUrl: "http://localhost:3000" })
  let client = new RouteTestClient(transport)

  // query
  let { response } = await client.checkColorPoint({
    color: { red: 1, blue: 2, green: 3 }, point: { x: 1, y: 2 }, tolerance: 0.5,
    name: 'name',
    children: [],
    info: {}
  })
  query_message.value = JSON.stringify(response)

  // stream
  let stream = client.checkPointStream({
    x: 0,
    y: 100
  });
  for await (let response of stream.responses) {
    stream_message.value = JSON.stringify(response)
  }
})



</script>
<template>

  <h1>{{ msg }}</h1>

  <div class="card">
    <p>query: {{ query_message }}</p>
    <p>stream: {{ stream_message }}</p>
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank">create-vue</a>, the official Vue + Vite
    starter
  </p>
  <p>
    Learn more about IDE Support for Vue in the
    <a href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support" target="_blank">Vue Docs Scaling up Guide</a>.
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
