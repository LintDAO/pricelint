import fs from "fs";
import path from "path";
import { defineConfig, loadEnv, UserConfig } from "vite";

import AutoImport from "unplugin-auto-import/vite";
import { ElementPlusResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/vite";
import { createVitePlugins } from "./frontend/build/vite/plugins";
import type { ViteEnv } from "./frontend/types/model";

enum ConfigMode {
  development = 1, // 防止 0 情况 if 出错 启用本地后端，只能通过匿名用户进入，并且需要对应local文件夹里的canister_id.json文件
  dev_frontend, //本地前端，线上后端
  production,
}

// 输出配置文件
export default defineConfig(({ command, mode }) => {
  console.log("command ->", command);
  console.log("mode ->", mode);

  const configMode = getConfigMode(mode); // 获取配置模式
  console.log("config mode ->", ConfigMode[configMode]); // 输出查询出来的配置模式

  const readEnv = loadEnv(mode, "./frontend/env");
  // @ts-ignore force transform, not a bit problem for string variable
  const viteEnv: ViteEnv = readEnv; // 导入设置的环境变量，会根据选择的 mode 选择文件
  // but matters other types
  if (readEnv.VITE_DROP_CONSOLE !== undefined)
    viteEnv.VITE_DROP_CONSOLE = readEnv.VITE_DROP_CONSOLE === "true";
  if (readEnv.VITE_DROP_DEBUGGER !== undefined)
    viteEnv.VITE_DROP_DEBUGGER = readEnv.VITE_DROP_DEBUGGER === "true";
  console.log("viteEnv ->", viteEnv); // 输出加载的变量

  const network = getNetwork(viteEnv);
  console.log("network ->", network);

  const canisterIds = getCanisterIds(viteEnv);
  const canisterApis = getCanisterApis(viteEnv);

  const canistersAlias = initAlias(canisterIds, network, canisterApis);

  const location = getLocation(viteEnv);
  console.log("server location ->", location); //

  process.env.configMode = ConfigMode[configMode];
  process.env.network = network;
  process.env.location = location;

  mode = getMode(configMode);
  const isBuild = mode === "production";
  const common: UserConfig = {
    mode, // 运行模式
    define: {
      // "process.env.NODE_ENV": JSON.stringify(getNodeEnv(configMode)), //接口文件里面需要用来判断 process.env.NODE_ENV值为production的时候会导致热更新失效
      "process.env": process.env, // 环境变量
    },
    plugins: [
      ...createVitePlugins(viteEnv, isBuild),
      AutoImport({
        resolvers: [ElementPlusResolver()],
      }),
      Components({
        resolvers: [ElementPlusResolver()],
      }),
    ], // 插件
    resolve: {
      alias: {
        ...canistersAlias, // canister 接口文件位置的映射
        "@": path.resolve(__dirname, "frontend"), // @符号要解析
      },
      extensions: [".js", ".ts", ".jsx", ".tsx", ".vue"], // import 可以省略的拓展名
    },
    publicDir: "frontend/public", //指定新的静态资源目录，免得都堆在根目录，更美观。
    build: {
      minify: isBuild ? "esbuild" : false, // 默认为 Esbuild，它比 terser 快 20-40 倍，压缩率只差 1%-2%
      terserOptions: {
        compress: {
          // 线上环境移除console，注释此段代码则表示线上环境启用console
          // drop_console:
          //     configMode == ConfigMode.production
          //         ? true // 线上部署的生产打包一定不包含
          //         : viteEnv.VITE_DROP_CONSOLE === undefined
          //         ? isBuild
          //         : viteEnv.VITE_DROP_CONSOLE, // 生产环境去除 console
          // drop_debugger:
          //     configMode == ConfigMode.production
          //         ? true // 线上部署的生产打包一定不包含
          //         : viteEnv.VITE_DROP_DEBUGGER === undefined
          //         ? isBuild
          //         : viteEnv.VITE_DROP_DEBUGGER, // 生产环境去除 debugger
        },
      },
      rollupOptions: {
        // external: ["element-plus"],
        output: {
          manualChunks: {
            // 每个 '键' 都表示一个分包块，'值' 包含列出的模块及其所有依赖项
            vue: ["vue", "vue-router", "pinia"],
            framework: ["quasar", "element-plus"],
            echarts: ["echarts"],
            dfinity: [
              "@dfinity/agent",
              "@dfinity/auth-client",
              "@dfinity/candid",
              "@dfinity/identity",
              "@dfinity/ledger-icp",
              "@dfinity/ledger-icrc",
              "@dfinity/nns",
              "@dfinity/nns-proto",
              "@dfinity/principal",
              "@dfinity/sns",
              "@dfinity/utils",
            ],
          },
        },
      },
    },
    css: {
      preprocessorOptions: {
        // 使用现代编译器，好处包括更快的编译速度和更好的错误处理能力
        scss: {
          api: "modern-compiler", // or 'modern'
        },
        sass: {
          api: "modern-compiler", // or 'modern'
        },
      },
    },
    envDir: "env",
    clearScreen: false,
  };
  if (common.define) {
    console.log(
      `process.env.NODE_ENV -> ${common.define["process.env.NODE_ENV"]}`
    );
  } else {
    console.log(`process.env.NODE_ENV -> no env node_env load`);
  }

  if (!isBuild) {
    return {
      // serve 独有配置 开发模式
      ...common,
      server: {
        proxy: {
          "/api": {
            target: location,
            changeOrigin: true,
            rewrite: (path) => path,
          },
        },
        port: 3000,
        cors: true,
        host: "0.0.0.0",
      },
    };
  } else {
    return {
      // build 独有配置 生产模式
      ...common,
    };
  }
});

// 判断配置模式
function getConfigMode(mode: string): ConfigMode {
  if (ConfigMode[mode]) {
    return ConfigMode[mode];
  }
  throw new Error("can not recognize mode: " + mode);
}

// 判断网络
function getNetwork(viteEnv: ViteEnv) {
  if (!viteEnv.VITE_NETWORK) {
    throw new Error(
      "config network is missing. please set config VITE_NETWORK"
    );
  }
  const dfxNetwork = process.env.DFX_NETWORK;
  if (dfxNetwork && dfxNetwork !== viteEnv.VITE_NETWORK) {
    console.log(
      `config process.env.DFX_NETWORK is ${dfxNetwork}. but VITE_NETWORK is ${viteEnv.VITE_NETWORK}`
    );
  }
  return viteEnv.VITE_NETWORK;
}

// 获取后端运行地址
function getLocation(viteEnv: ViteEnv): string {
  const position = viteEnv.VITE_LOCAL_DFX;
  if (position) {
    const dfxJson = require(position);
    return "http://" + dfxJson.networks.local.bind;
  }
  //本地开发端口转发的目标网址，注意400 bad request/ unknow_domain 可能是这出了问题，可能是基金会搬迁了网址。
  return "https://icp-api.io";
}

// 根据环境参数加载 canister 名称 和 id 之间的关系，主要关联需要用到的 canister_ids.json 文件 可以多个
function getCanisterIds(viteEnv: ViteEnv) {
  // 找出 2 段字符串数组的交集
  const intersect = function (a: string[], b: string[]): string[] {
    const result: string[] = [];
    for (const t in a) if (b[t]) result.push(t);
    return result;
  };

  const positions = viteEnv.VITE_CANISTER_IDS
    ? viteEnv.VITE_CANISTER_IDS.split(",")
    : [];
  const canisterIds = {};
  try {
    for (const position of positions) {
      if (fs.existsSync(position)) {
        const read = JSON.parse(fs.readFileSync(position).toString());
        if (!Object.keys(read).length)
          console.log("canister ids file is not empty:", position);
        const inter = intersect(Object.keys(read), Object.keys(canisterIds));
        for (const name in inter)
          console.log(
            `cover canister id ${name} from ${canisterIds[name]} to ${read[name]}`
          );
        Object.assign(canisterIds, read);
      } else {
        console.error("canister ids file is not exist", position);
      }
    }
  } catch (e) {
    console.error("read canister ids failed. the path is", positions);
  }
  return canisterIds;
}

// 根据环境参数加载 canister 名称 和 接口 之间的关系，主要关联需要用到的 canisters 文件夹 可以多个，dfx 编译出的结果
// 调用其他 canister 都要有接口文件才好
function getCanisterApis(viteEnv: ViteEnv) {
  const positions = viteEnv.VITE_CANISTER_APIS
    ? viteEnv.VITE_CANISTER_APIS.split(",")
    : [];

  const canisterApis = {};
  try {
    for (const position of positions) {
      const dirs = fs.readdirSync(position);
      console.log("dirs: " + position + " ->", dirs);
      for (const dir of dirs) {
        if (fs.lstatSync(position + "/" + dir).isDirectory()) {
          const value = position + "/" + dir;
          if (canisterApis[dir]) {
            console.log(
              `cover canister api ${dir} from ${canisterApis[dir]} to ${value}`
            );
          }
          canisterApis[dir] = value;
        }
      }
    }
  } catch (e) {
    console.error("read canister api failed. the path is ", positions);
  }

  return canisterApis;
}

// 总的设置别名 程序里需要用到的地方有 2 种
// 1. 是 canister 的部署 id，要调用方法，有 id 才知道去哪里调用
// 2. 是 canister 的接口文件，有接口文件，才知道哪些方法能用哪些不能用
// 3. 按道理还有个运行网络的问题，但是目前默认，线上网络是 ic ，其他都是 local 本地环境
function initAlias(canisterIds: {}, network: string, apiPositions: {}) {
  const canistersAlias = {};
  for (const canister in canisterIds) {
    // 这里将 id 设置到 process.env 对象里面，标准接口文件中有用到
    const key = "CANISTER_ID_" + canister.toUpperCase();
    process.env[key] = canisterIds[canister][network];
    console.log(key, canisterIds[canister][network]);

    if (apiPositions[canister]) {
      // 接口文件的引入因为是动态配置，这里统一进行映射，代码里写 key ，会在真正运行时候 resolve 成实际的位置
      canistersAlias["canisters/" + canister] = path.join(
        __dirname,
        apiPositions[canister] + "/index.js"
      );
      console.log(
        "initAlias: canisters/" + canister,
        path.join(__dirname, apiPositions[canister] + "/index.js")
      );
    } else {
      //前端容器没有api 自然不需要报警
      if (canister !== "frontend") {
        console.error(`canister ${canister} api position is missing.`);
      }
    }
  }
  return canistersAlias;
}

function getMode(configMode: ConfigMode) {
  let mode = "";
  switch (configMode) {
    case ConfigMode.development:
      mode = "development";
      break;

    case ConfigMode.dev_frontend:
      mode = "development"; // 前端是开发模式
      break;

    case ConfigMode.production:
      mode = "production";
      break;

    default:
      throw new Error(
        `what a config config mode: ${configMode} ${ConfigMode[configMode]}`
      );
  }
  return mode;
}

function getNodeEnv(mode: ConfigMode): string {
  let env = "";
  switch (mode) {
    case ConfigMode.development:
      env = "development";
      break;

    case ConfigMode.dev_frontend:
      env = "production"; // 后端用的是 ic
      break;

    case ConfigMode.production:
      env = "production"; // 后端用的是 ic
      break;

    default:
      throw new Error(`what a config config mode: ${mode} ${ConfigMode[mode]}`);
  }
  return env;
}
