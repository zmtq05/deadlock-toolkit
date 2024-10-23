<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { appCacheDir } from "@tauri-apps/api/path";
  import { download } from "@tauri-apps/plugin-upload";

  export let obj: string;
  const urlMap: Record<string, string> = {
    translation:
      "https://drive.google.com/uc?id=1eYAZiLb6xmNQZw-sxh1mJWshTC6xHLJz",
    builtin_font:
      "https://drive.google.com/uc?id=1kEHlqJ58PE5lSaSr_Hmmgclij1tSjR17",
    external_font:
      "https://drive.google.com/uc?id=1t2lh6KPnTkBoM_-PPFmx5CRBum-gLb31",
  };
  const description: Record<string, string> = {
    translation: "번역",
    builtin_font: "내장 폰트",
    external_font: "맞춤 폰트",
  };

  let max = 100;
  let value = 0;

  const download0 = async () => {
    value = 0;
    const url = urlMap[obj];
    const dir = await appCacheDir();
    await download(url, `${dir}/${obj}.zip`, ({ progress, total }) => {
      max = total;
      value += progress;
    });
    await invoke("record_download_time", { obj });
  };

  let error = "";
  let result = "";
  const apply = async () => {
    try {
      await invoke(`extract_${obj}`);
      result = "완료";
    } catch (e: any) {
      error = e;
    }
  };
</script>

<div>
  <div>
    <span>{description[obj]}</span>
    <progress {max} {value}></progress>
    <button on:click={download0}>다운로드</button>
    <button on:click={apply}>적용</button>
    <span style="color: blue;">{result}</span>
  </div>
  <p style="color: red;">{error}</p>
</div>
