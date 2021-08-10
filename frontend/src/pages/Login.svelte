<script lang="ts">
  import Button from "../components/Button.svelte";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/store";
  import type { ILoginRequest } from "../utils/types";
  import { navigate } from "svelte-navigator";

  let username = "";
  let password = "";
  let isLoading = false;
  let form: HTMLFormElement;

  const onConfirm = async (e: Event) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    isLoading = true;
    const result = await sendLoginRequest();
    isLoading = false;

    if (result) {
      navigate("/");
    }
  };

  const validateForm = (): boolean => {
    if (!form.checkValidity()) {
      form.reportValidity();
      return false;
    }

    return true;
  };

  const sendLoginRequest = async () => {
    const payload: ILoginRequest = {
      username,
      password,
    };

    try {
      await api.post("/api/login", payload, false);
      setNotification("success", "Login successfully,");
    } catch (e) {
      console.log(e);
      setNotification("error", "Login failed.");
      return false;
    }

    return true;
  };
</script>

<div class="absolute w-full">
  <form on:submit={onConfirm} bind:this={form}>
    <div
      class="w-80 mx-auto mt-28 bg-gray-50 shadow rounded-lg flex flex-col items-center p-8"
    >
      <div class="text-xl font-bold mb-8 text-gray-700">Login</div>
      <div class="w-full grid grid-cols-4 mb-4">
        <div>username:</div>
        <div class="col-span-3 ml-2">
          <input
            required
            minLength={1}
            maxLength={16}
            class="ml-2 w-40 border rounded focus:outline-none px-2"
            bind:value={username}
          />
        </div>
      </div>
      <div class="w-full grid grid-cols-4 mb-8">
        <div>password:</div>
        <div class="col-span-3 ml-2">
          <input
            required
            type="password"
            minLength={6}
            maxLength={16}
            class="ml-2 w-40 border rounded focus:outline-none px-2"
            bind:value={password}
          />
        </div>
      </div>

      <div class="mb-2">
        <Button
          value={isLoading ? "Logging in..." : "Login"}
          onClick={onConfirm}
          disabled={isLoading}
          spec="important"
          type="submit"
        />
      </div>
    </div>
  </form>
</div>
