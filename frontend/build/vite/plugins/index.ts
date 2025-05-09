import type { PluginOption } from "vite"
import type { ViteEnv } from "../../../types/model"

import { quasar, transformAssetUrls } from "@quasar/vite-plugin"
import vue from "@vitejs/plugin-vue"
import { visualizer } from "rollup-plugin-visualizer"
import { createHtmlPlugin } from "vite-plugin-html"
import { viteCompressionPlugin } from "./compression"

export function createVitePlugins(viteEnv: ViteEnv, isBuild: boolean) {
  const {
    VITE_BUILD_COMPRESS: compressType,
    VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: shouldBuildCompressDeleteFile,
  } = viteEnv

  const vitePlugins: (PluginOption | PluginOption[])[] = [
    vue({
      template: { transformAssetUrls },
    }),
    quasar({
      sassVariables: "@/assets/css/quasar-variables.sass",
    }),
    visualizer({
      open: false, // 是否在build完成后自动打开网页查看对应的依赖占比
      gzipSize: true,
      brotliSize: true, // 收集 brotli 大小并将其显示
      filename: "dependency-analysis.html", // 分析图生成的文件名
    }),
    createHtmlPlugin({
      minify: true,
      inject: {
        data: {
          keywords: viteEnv.VITE_KEYWORDS,
          author: viteEnv.VITE_AUTHOR,
          description: viteEnv.VITE_DESCRIPTION,
          noScriptTitle: viteEnv.VITE_TITLE,
          title: viteEnv.VITE_TITLE,
        },
      },
    }),
  ]

  if (isBuild) {
    // 生成模式执行
    vitePlugins.push(
      viteCompressionPlugin(compressType, shouldBuildCompressDeleteFile), // 压缩
    )
  }

  return vitePlugins
}
