<template>
  <div id="app">
    <h1>Upload JSON to ICP Canister</h1>
    <input type="file" accept=".json" @change="handleFileChange" />
    <button @click="uploadFile" :disabled="!file">Upload</button>
    <p v-if="message">{{ message }}</p>
  </div>
</template>

<script>
import { ic_burn_backend } from "declarations/ic_burn_backend"; // 导入 canister 接口

export default {
  name: "App",
  data() {
    return {
      file: null,
      message: "",
    };
  },
  methods: {
    handleFileChange(event) {
      this.file = event.target.files[0];
      this.message = this.file ? `Selected file: ${this.file.name}` : "";
    },
    async uploadFile() {
      if (!this.file) {
        this.message = "Please select a file first!";
        return;
      }

      try {
        const arrayBuffer = await this.file.arrayBuffer();
        const bytes = new Uint8Array(arrayBuffer);

        // 调用 canister 的 upload_json_file 函数
        await ic_burn_backend .upload_json_file(Array.from(bytes));
        this.message = "File uploaded successfully!";
      } catch (error) {
        this.message = `Upload failed: ${error.message}`;
      }
    },
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  margin-top: 60px;
}
button {
  margin-top: 10px;
  padding: 5px 10px;
}
</style>