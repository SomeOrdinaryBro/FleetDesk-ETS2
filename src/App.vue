<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

type Profile = { name:string; path:string }
type PlayerState = { level:number; skills:string[]; discovered_cities:string[]; owned_trailers:string[]; save_format_ok:boolean }
type DlcFlags = {
  base:boolean; heavy_cargo:boolean; special_transport:boolean;
  east:boolean; north:boolean; fr:boolean; it:boolean;
  balt:boolean; rbs:boolean; iberia:boolean; wb:boolean; greece:boolean;
}
type JobSpec = {
  source_city:string; source_company:string;
  dest_city:string; dest_company:string;
  cargo:string; trailer_type:string;
}

const profiles = ref<Profile[]>([])
const chosen = ref<Profile | null>(null)
const state = ref<PlayerState | null>(null)
const dlc = ref<DlcFlags | null>(null)

const job = ref<JobSpec>({
  source_city:'', source_company:'',
  dest_city:'', dest_company:'',
  cargo:'', trailer_type:''
})

onMounted(async () => {
  profiles.value = await invoke('find_profiles')
  dlc.value = await invoke('detect_dlcs', { explicitPath: null })
})

async function loadState() {
  if (!chosen.value) return
  state.value = await invoke('read_player_state', { profilePath: chosen.value.path })
}

async function exportJob() {
  if (!chosen.value) return
  const outPath = await invoke<string>('export_job_spec', { profilePath: chosen.value.path, job: job.value })
  alert(`Job spec saved: ${outPath}`)
}

// simple DLC gate example
const hasSpecial = computed(() => !!dlc.value?.special_transport)
const hasHeavy = computed(() => !!dlc.value?.heavy_cargo)
</script>

<template>
  <div class="p-6 space-y-4">
    <h1 class="text-2xl font-bold">Fleet Desk for ETS2</h1>

    <section class="space-y-2">
      <label class="block font-semibold">Profile</label>
      <select v-model="chosen" @change="loadState">
        <option :value="null">Select profile…</option>
        <option v-for="p in profiles" :key="p.path" :value="p">{{ p.name }}</option>
      </select>
    </section>

    <section v-if="state" class="space-y-1">
      <div>Level: <b>{{ state.level }}</b></div>
      <div>Discovered cities: <b>{{ state.discovered_cities.length }}</b></div>
      <div>Owned trailers: <b>{{ state.owned_trailers.join(', ') || 'None' }}</b></div>
      <div v-if="!state.save_format_ok" style="color:#b00;">
        Your game isn't in text save mode. Set <code>uset g_save_format "2"</code> in config.cfg and quick-save once.
      </div>
    </section>

    <section v-if="dlc" class="space-y-1">
      <div class="font-semibold">DLCs detected:</div>
      <ul style="columns:2; margin:0; padding-left:1rem;">
        <li>Heavy Cargo: <b>{{ dlc.heavy_cargo ? 'Yes' : 'No' }}</b></li>
        <li>Special Transport: <b>{{ dlc.special_transport ? 'Yes' : 'No' }}</b></li>
        <li>Going East: <b>{{ dlc.east ? 'Yes' : 'No' }}</b></li>
        <li>Scandinavia: <b>{{ dlc.north ? 'Yes' : 'No' }}</b></li>
        <li>Vive la France: <b>{{ dlc.fr ? 'Yes' : 'No' }}</b></li>
        <li>Italia: <b>{{ dlc.it ? 'Yes' : 'No' }}</b></li>
        <li>Baltic: <b>{{ dlc.balt ? 'Yes' : 'No' }}</b></li>
        <li>Black Sea: <b>{{ dlc.rbs ? 'Yes' : 'No' }}</b></li>
        <li>Iberia: <b>{{ dlc.iberia ? 'Yes' : 'No' }}</b></li>
        <li>West Balkans: <b>{{ dlc.wb ? 'Yes' : 'No' }}</b></li>
        <li>Greece: <b>{{ dlc.greece ? 'Yes' : 'No' }}</b></li>
      </ul>
    </section>

    <section class="space-y-2">
      <h2 class="text-lg font-semibold">Create Job (MVP stub)</h2>
      <div class="grid gap-2" style="max-width:480px;">
        <input v-model="job.source_city" placeholder="Source City id (e.g., berlin)"/>
        <input v-model="job.source_company" placeholder="Source Company id"/>
        <input v-model="job.dest_city" placeholder="Dest City id"/>
        <input v-model="job.dest_company" placeholder="Dest Company id"/>
        <input v-model="job.cargo" placeholder="Cargo id"/>
        <input v-model="job.trailer_type" placeholder="Trailer type id"/>
        <div class="text-sm">
          Special Transport allowed: <b>{{ hasSpecial ? 'Yes' : 'No' }}</b> |
          Heavy Cargo allowed: <b>{{ hasHeavy ? 'Yes' : 'No' }}</b>
        </div>
        <button @click="exportJob">Export Job JSON</button>
      </div>
      <p class="text-sm opacity-80">This writes a JSON spec next to your profile. Next step is writing the save block.</p>
    </section>
  </div>
</template>

<style>
body { font-family: ui-sans-serif, system-ui; }
input, select, button { padding:.5rem; border:1px solid #ddd; border-radius:.375rem; }
button { cursor:pointer }
</style>
