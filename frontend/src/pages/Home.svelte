<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    import { resetTitle } from "../utils/store";

    const navigate = useNavigate();
    resetTitle();
    navigate("/files");
</script>
