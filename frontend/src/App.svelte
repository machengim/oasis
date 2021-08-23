<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Tailwind from "./components/Tailwind.svelte";
  import Header from "./sections/Header.svelte";
  import Notification from "./sections/Notification.svelte";
  import UploadTasks from "./sections/UploadTasks.svelte";
  import Home from "./pages/Home.svelte";
  import FileList from "./pages/FileList.svelte";
  import FileDetail from "./pages/FileDetail.svelte";
  import Login from "./pages/Login.svelte";
  import Setup from "./pages/Setup.svelte";
  import Error404 from "./pages/Error404.svelte";
  import { clickEventStore } from "./utils/store";

  export let path: string;

  const clicked = () => {
    clickEventStore.set(+new Date());
  };
</script>

<main class="relative min-h-screen" on:click={clicked}>
  <Router url={path}>
    <Header {path} />

    <Notification />
    <UploadTasks />

    <Route path="login" component={Login} />
    <Route path="setup" component={Setup} />
    <Route path="files" component={FileList} />
    <Route path="files/:file_id" component={FileList} />
    <Route path="detail/:file_id" component={FileDetail} />
    <Route path="/"><Home /></Route>
  </Router>
</main>
