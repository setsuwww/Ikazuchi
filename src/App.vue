<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isExpanded = ref(false)
const isLoading = ref(false)
const answerResult = ref('')

// Simulasi saat file di-drop atau dimasukkan
const handleFileUpload = async (event: DragEvent) => {
  event.preventDefault()
  isExpanded.value = true
  isLoading.value = true
  answerResult.value = ''

  // Ubah ukuran window Tauri jadi lebih besar untuk membaca jawaban
  await appWindow.setSize(new LogicalSize(400, 300))

  // Simulasi proses AI (Nanti bisa diganti fetch API ke backend/OpenAI/Gemini)
  setTimeout(() => {
    isLoading.value = false
    answerResult.value = "Jawaban instan: Faktor utama penyebab... [hasil ringkasan tugas]"
  }, 2000)
}

const toggleWindow = async () => {
  isExpanded.value = !isExpanded.value
  if (isExpanded.value) {
    await appWindow.setSize(new LogicalSize(400, 250))
  } else {
    await appWindow.setSize(new LogicalSize(180, 40))
    answerResult.value = ''
  }
}
</script>

<template>
  <!-- Area luar transparan dan bisa di-drag (gunakan data-tauri-drag-region agar window bisa digeser-geser) -->
  <div
    data-tauri-drag-region
    class="flex justify-center items-start w-screen h-screen select-none overflow-hidden m-0 p-2 bg-transparent"
  >
    <div
      @dragover.prevent
      @drop="handleFileUpload"
      :class="[
        'bg-black/90 backdrop-blur-md text-white transition-all duration-300 ease-in-out flex flex-col shadow-2xl border border-white/10 overflow-hidden',
        isExpanded ? 'w-[380px] h-[240px] rounded-2xl p-4' : 'w-[180px] h-[38px] rounded-full px-3 items-center justify-between cursor-pointer'
      ]"
      @click="!isExpanded && toggleWindow()"
    >
      <!-- STATE 1: Kondisi Kecil (Notch Default) -->
      <div v-if="!isExpanded" class="flex items-center justify-between w-full text-xs pointer-events-none">
        <span class="flex items-center gap-1.5 text-gray-300">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          Drop Tugas Disini
        </span>
        <span class="text-[10px] bg-white/20 px-1.5 py-0.5 rounded text-gray-300">AI</span>
      </div>

      <!-- STATE 2: Kondisi Terbuka (Expanded / Hasil) -->
      <div v-else class="flex flex-col justify-between h-full w-full">
        <!-- Header -->
        <div class="flex justify-between items-center border-b border-white/10 pb-2">
          <span class="text-xs font-semibold text-emerald-400">Zanamy Instant Solver</span>
          <button @click.stop="toggleWindow" class="text-gray-400 hover:text-white text-xs">✕ Tutup</button>
        </div>

        <!-- Body Content -->
        <div class="flex-1 py-2 overflow-y-auto text-xs text-gray-200">
          <div v-if="isLoading" class="flex flex-col items-center justify-center h-full gap-2 text-gray-400">
            <div class="w-5 h-5 border-2 border-emerald-400 border-t-transparent rounded-full animate-spin"></div>
            <span>Menganalisis dokumen & mencari jawaban...</span>
          </div>
          <div v-else class="space-y-2">
            <p class="font-medium text-white">Hasil Analisis:</p>
            <p class="bg-white/5 p-2 rounded text-gray-300 select-text">{{ answerResult }}</p>
          </div>
        </div>

        <!-- Footer / Hint -->
        <div class="text-[10px] text-gray-500 text-center">
          Seret file PDF / Foto soal ke sini kapan saja
        </div>
      </div>
    </div>
  </div>
</template>
