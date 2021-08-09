<script lang="ts">
    import * as api from "../utils/api";

    const selectFile = async (event: Event) => {
        let target = <HTMLInputElement>event.target;

        const data = new FormData();
        for (const file of target.files) {
            data.append("file", file);
        }

        // Use XHR here for upload progress info.
        let xhr = new XMLHttpRequest();
        xhr.open("POST", "/api/upload");
        xhr.send(data);
    };
</script>

<div
    class="flex justify-center align-middle fixed bottom-36 right-36 z-10 text-lg"
>
    <input type="checkbox" id="toggle" />
    <label class="button bg-blue-400" for="toggle" />
    <div class="menu">
        <div class="mb-2 hover:bg-gray-200 cursor-pointer px-4 rounded">
            Create folder
        </div>
        <div class="hover:bg-gray-200 px-4 rounded">
            <input
                id="file"
                type="file"
                multiple
                class="w-0 h-0"
                on:change={selectFile}
            />
            <label for="file" class="cursor-pointer">Upload file(s)</label>
        </div>
    </div>
</div>

<style>
    #toggle {
        -webkit-appearance: none;
    }

    .button {
        position: absolute;
        width: 3rem;
        height: 3rem;
        border-radius: 100%;
        cursor: pointer;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .button:before {
        position: absolute;
        content: "";
        width: 1.5rem;
        height: 2px;
        background: #fff;
        transform: rotate(90deg);
        transition: all 0.4s ease;
    }
    .button:after {
        position: absolute;
        content: "";
        width: 1.5rem;
        height: 2px;
        background: #fff;
        transition: all 0.4s ease;
    }

    .menu {
        opacity: 0;
        transition: all 0.4s ease-in-out;
        background: #fff;
        width: 100%;
        border-radius: 5px;
        transform: translateY(0%);
        box-shadow: 2px 3px 10px 0 rgba(81, 81, 81, 0.1);
        border: 1px solid #e4e4e4;
        padding: 0.5rem;
        position: relative;
        top: -6rem;
    }

    #toggle:checked ~ .menu {
        opacity: 1;
        transform: translateY(10%);
    }

    #toggle:checked ~ .button:before {
        transform: rotate(225deg);
    }

    #toggle:checked ~ .button:after {
        transform: rotate(135deg);
    }
</style>
