<script setup lang="ts">
import { onMounted, ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
const text = ref("loading..")

// use openapi-ts
import { client, sendCode, sendCode2 } from '../openapi';
client.setConfig({
  baseUrl: "http://localhost:3000"
})

// use openapi-typescript
import createClient from "openapi-fetch";
import { paths } from '../openapi-typescript-schema';

const client2 = createClient<paths>({ baseUrl: "http://localhost:3000" });

; (async () => {
  const response = await sendCode2({
    path: {
      some_path_what: "path1",
    },
    body: {
      mobile: 'body1'
    }
  })

  text.value += "\nfetch with openapi-ts: \n" + JSON.stringify(response.data)

  const response2 = await client2.POST("/send-code2/{some_path_what}", {
    params: {
      path: {
        some_path_what: 'path2'
      },
    },
    body: {
      mobile: 'body2'
    }
  });

  text.value += "\nfetch with openapi-typescript: \n" + JSON.stringify(response2.data)
})()






</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>

    <pre>{{ text }}</pre>


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
