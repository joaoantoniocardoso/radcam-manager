// vite.config.mjs
import Components from "file:///home/joaoantoniocardoso/BlueRobotics/radcam-manager/frontend/node_modules/unplugin-vue-components/dist/vite.js";
import Vue from "file:///home/joaoantoniocardoso/BlueRobotics/radcam-manager/frontend/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import Vuetify, {
  transformAssetUrls,
} from "file:///home/joaoantoniocardoso/BlueRobotics/radcam-manager/frontend/node_modules/vite-plugin-vuetify/dist/index.mjs";
import ViteFonts from "file:///home/joaoantoniocardoso/BlueRobotics/radcam-manager/frontend/node_modules/unplugin-fonts/dist/vite.mjs";
import { defineConfig } from "file:///home/joaoantoniocardoso/BlueRobotics/radcam-manager/frontend/node_modules/vite/dist/node/index.js";
import { fileURLToPath, URL } from "node:url";
var __vite_injected_original_import_meta_url =
  "file:///home/joaoantoniocardoso/BlueRobotics/radcam-manager/frontend/vite.config.mjs";
var vite_config_default = defineConfig({
  plugins: [
    Vue({
      template: { transformAssetUrls },
    }),
    // https://github.com/vuetifyjs/vuetify-loader/tree/master/packages/vite-plugin#readme
    Vuetify({
      autoImport: true,
      styles: {
        configFile: "src/styles/settings.scss",
      },
    }),
    Components(),
    ViteFonts({
      google: {
        families: [
          {
            name: "Roboto",
            styles: "wght@100;300;400;500;700;900",
          },
        ],
      },
    }),
  ],
  define: { "process.env": {} },
  resolve: {
    alias: {
      "@": fileURLToPath(
        new URL("./src", __vite_injected_original_import_meta_url)
      ),
    },
    extensions: [".js", ".json", ".jsx", ".mjs", ".ts", ".tsx", ".vue"],
  },
  server: {
    port: 3e3,
  },
  css: {
    preprocessorOptions: {
      sass: {
        api: "modern-compiler",
      },
    },
  },
});
export { vite_config_default as default };
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcubWpzIl0sCiAgInNvdXJjZXNDb250ZW50IjogWyJjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZGlybmFtZSA9IFwiL2hvbWUvam9hb2FudG9uaW9jYXJkb3NvL0JsdWVSb2JvdGljcy9yYWRjYW0tbWFuYWdlci9mcm9udGVuZFwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL2hvbWUvam9hb2FudG9uaW9jYXJkb3NvL0JsdWVSb2JvdGljcy9yYWRjYW0tbWFuYWdlci9mcm9udGVuZC92aXRlLmNvbmZpZy5tanNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL2hvbWUvam9hb2FudG9uaW9jYXJkb3NvL0JsdWVSb2JvdGljcy9yYWRjYW0tbWFuYWdlci9mcm9udGVuZC92aXRlLmNvbmZpZy5tanNcIjsvLyBQbHVnaW5zXG5pbXBvcnQgQ29tcG9uZW50cyBmcm9tICd1bnBsdWdpbi12dWUtY29tcG9uZW50cy92aXRlJztcbmltcG9ydCBWdWUgZnJvbSAnQHZpdGVqcy9wbHVnaW4tdnVlJztcbmltcG9ydCBWdWV0aWZ5LCB7IHRyYW5zZm9ybUFzc2V0VXJscyB9IGZyb20gJ3ZpdGUtcGx1Z2luLXZ1ZXRpZnknO1xuaW1wb3J0IFZpdGVGb250cyBmcm9tICd1bnBsdWdpbi1mb250cy92aXRlJztcbi8vIFV0aWxpdGllc1xuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSc7XG5pbXBvcnQgeyBmaWxlVVJMVG9QYXRoLCBVUkwgfSBmcm9tICdub2RlOnVybCc7XG4vLyBodHRwczovL3ZpdGVqcy5kZXYvY29uZmlnL1xuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcbiAgICBwbHVnaW5zOiBbXG4gICAgICAgIFZ1ZSh7XG4gICAgICAgICAgICB0ZW1wbGF0ZTogeyB0cmFuc2Zvcm1Bc3NldFVybHMgfSxcbiAgICAgICAgfSksXG4gICAgICAgIC8vIGh0dHBzOi8vZ2l0aHViLmNvbS92dWV0aWZ5anMvdnVldGlmeS1sb2FkZXIvdHJlZS9tYXN0ZXIvcGFja2FnZXMvdml0ZS1wbHVnaW4jcmVhZG1lXG4gICAgICAgIFZ1ZXRpZnkoe1xuICAgICAgICAgICAgYXV0b0ltcG9ydDogdHJ1ZSxcbiAgICAgICAgICAgIHN0eWxlczoge1xuICAgICAgICAgICAgICAgIGNvbmZpZ0ZpbGU6ICdzcmMvc3R5bGVzL3NldHRpbmdzLnNjc3MnLFxuICAgICAgICAgICAgfSxcbiAgICAgICAgfSksXG4gICAgICAgIENvbXBvbmVudHMoKSxcbiAgICAgICAgVml0ZUZvbnRzKHtcbiAgICAgICAgICAgIGdvb2dsZToge1xuICAgICAgICAgICAgICAgIGZhbWlsaWVzOiBbe1xuICAgICAgICAgICAgICAgICAgICAgICAgbmFtZTogJ1JvYm90bycsXG4gICAgICAgICAgICAgICAgICAgICAgICBzdHlsZXM6ICd3Z2h0QDEwMDszMDA7NDAwOzUwMDs3MDA7OTAwJyxcbiAgICAgICAgICAgICAgICAgICAgfV0sXG4gICAgICAgICAgICB9LFxuICAgICAgICB9KSxcbiAgICBdLFxuICAgIGRlZmluZTogeyAncHJvY2Vzcy5lbnYnOiB7fSB9LFxuICAgIHJlc29sdmU6IHtcbiAgICAgICAgYWxpYXM6IHtcbiAgICAgICAgICAgICdAJzogZmlsZVVSTFRvUGF0aChuZXcgVVJMKCcuL3NyYycsIGltcG9ydC5tZXRhLnVybCkpLFxuICAgICAgICB9LFxuICAgICAgICBleHRlbnNpb25zOiBbXG4gICAgICAgICAgICAnLmpzJyxcbiAgICAgICAgICAgICcuanNvbicsXG4gICAgICAgICAgICAnLmpzeCcsXG4gICAgICAgICAgICAnLm1qcycsXG4gICAgICAgICAgICAnLnRzJyxcbiAgICAgICAgICAgICcudHN4JyxcbiAgICAgICAgICAgICcudnVlJyxcbiAgICAgICAgXSxcbiAgICB9LFxuICAgIHNlcnZlcjoge1xuICAgICAgICBwb3J0OiAzMDAwLFxuICAgIH0sXG4gICAgY3NzOiB7XG4gICAgICAgIHByZXByb2Nlc3Nvck9wdGlvbnM6IHtcbiAgICAgICAgICAgIHNhc3M6IHtcbiAgICAgICAgICAgICAgICBhcGk6ICdtb2Rlcm4tY29tcGlsZXInLFxuICAgICAgICAgICAgfSxcbiAgICAgICAgfSxcbiAgICB9LFxufSk7XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQ0EsT0FBTyxnQkFBZ0I7QUFDdkIsT0FBTyxTQUFTO0FBQ2hCLE9BQU8sV0FBVywwQkFBMEI7QUFDNUMsT0FBTyxlQUFlO0FBRXRCLFNBQVMsb0JBQW9CO0FBQzdCLFNBQVMsZUFBZSxXQUFXO0FBUGdNLElBQU0sMkNBQTJDO0FBU3BSLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQ3hCLFNBQVM7QUFBQSxJQUNMLElBQUk7QUFBQSxNQUNBLFVBQVUsRUFBRSxtQkFBbUI7QUFBQSxJQUNuQyxDQUFDO0FBQUE7QUFBQSxJQUVELFFBQVE7QUFBQSxNQUNKLFlBQVk7QUFBQSxNQUNaLFFBQVE7QUFBQSxRQUNKLFlBQVk7QUFBQSxNQUNoQjtBQUFBLElBQ0osQ0FBQztBQUFBLElBQ0QsV0FBVztBQUFBLElBQ1gsVUFBVTtBQUFBLE1BQ04sUUFBUTtBQUFBLFFBQ0osVUFBVSxDQUFDO0FBQUEsVUFDSCxNQUFNO0FBQUEsVUFDTixRQUFRO0FBQUEsUUFDWixDQUFDO0FBQUEsTUFDVDtBQUFBLElBQ0osQ0FBQztBQUFBLEVBQ0w7QUFBQSxFQUNBLFFBQVEsRUFBRSxlQUFlLENBQUMsRUFBRTtBQUFBLEVBQzVCLFNBQVM7QUFBQSxJQUNMLE9BQU87QUFBQSxNQUNILEtBQUssY0FBYyxJQUFJLElBQUksU0FBUyx3Q0FBZSxDQUFDO0FBQUEsSUFDeEQ7QUFBQSxJQUNBLFlBQVk7QUFBQSxNQUNSO0FBQUEsTUFDQTtBQUFBLE1BQ0E7QUFBQSxNQUNBO0FBQUEsTUFDQTtBQUFBLE1BQ0E7QUFBQSxNQUNBO0FBQUEsSUFDSjtBQUFBLEVBQ0o7QUFBQSxFQUNBLFFBQVE7QUFBQSxJQUNKLE1BQU07QUFBQSxFQUNWO0FBQUEsRUFDQSxLQUFLO0FBQUEsSUFDRCxxQkFBcUI7QUFBQSxNQUNqQixNQUFNO0FBQUEsUUFDRixLQUFLO0FBQUEsTUFDVDtBQUFBLElBQ0o7QUFBQSxFQUNKO0FBQ0osQ0FBQzsiLAogICJuYW1lcyI6IFtdCn0K
