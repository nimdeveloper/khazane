<template>
    <main class="flex-1 overflow-auto p-6 bg-white dark:bg-[#0F0F12]">
        <div class="space-y-4">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div
                    class="bg-white dark:bg-[#0F0F12] rounded-xl p-6 flex flex-col border border-gray-200 dark:border-[#1F1F23]"
                >
                    <h2
                        class="text-lg font-bold text-gray-900 dark:text-white mb-4 text-left flex items-center gap-2"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-wallet w-3.5 h-3.5 text-zinc-900 dark:text-zinc-50"
                        >
                            <path
                                d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"
                            ></path>
                            <path
                                d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"
                            ></path></svg
                        >Accounts
                    </h2>
                    <div class="flex-1">
                        <div
                            class="w-full max-w-xl mx-auto bg-white dark:bg-zinc-900/70 border border-zinc-100 dark:border-zinc-800 rounded-xl shadow-sm backdrop-blur-xl h-full"
                        >
                            <div
                                class="p-4 border-b border-zinc-100 dark:border-zinc-800"
                            >
                                <p
                                    class="text-xs text-zinc-600 dark:text-zinc-400"
                                >
                                    Total Balance
                                </p>
                                <h1
                                    class="text-2xl font-semibold text-zinc-900 dark:text-zinc-50"
                                >
                                    $26,540.25
                                </h1>
                            </div>
                            <div class="p-3">
                                <div
                                    class="flex items-center justify-between mb-2"
                                >
                                    <h2
                                        class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                    >
                                        Your Accounts
                                    </h2>
                                </div>
                                <div class="space-y-1">
                                    <div
                                        class="group flex items-center justify-between p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div class="flex items-center gap-2">
                                            <div
                                                class="p-1.5 rounded-lg bg-emerald-100 dark:bg-emerald-900/30"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-wallet w-3.5 h-3.5 text-emerald-600 dark:text-emerald-400"
                                                >
                                                    <path
                                                        d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"
                                                    ></path>
                                                    <path
                                                        d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"
                                                    ></path>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Main Savings
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Personal savings
                                                </p>
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <span
                                                class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >$8,459.45</span
                                            >
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center justify-between p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div class="flex items-center gap-2">
                                            <div
                                                class="p-1.5 rounded-lg bg-blue-100 dark:bg-blue-900/30"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-qr-code w-3.5 h-3.5 text-blue-600 dark:text-blue-400"
                                                >
                                                    <rect
                                                        width="5"
                                                        height="5"
                                                        x="3"
                                                        y="3"
                                                        rx="1"
                                                    ></rect>
                                                    <rect
                                                        width="5"
                                                        height="5"
                                                        x="16"
                                                        y="3"
                                                        rx="1"
                                                    ></rect>
                                                    <rect
                                                        width="5"
                                                        height="5"
                                                        x="3"
                                                        y="16"
                                                        rx="1"
                                                    ></rect>
                                                    <path
                                                        d="M21 16h-3a2 2 0 0 0-2 2v3"
                                                    ></path>
                                                    <path d="M21 21v.01"></path>
                                                    <path
                                                        d="M12 7v3a2 2 0 0 1-2 2H7"
                                                    ></path>
                                                    <path d="M3 12h.01"></path>
                                                    <path d="M12 3h.01"></path>
                                                    <path d="M12 16v.01"></path>
                                                    <path d="M16 12h1"></path>
                                                    <path d="M21 12v.01"></path>
                                                    <path d="M12 21v-1"></path>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Checking Account
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Daily expenses
                                                </p>
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <span
                                                class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >$2,850.00</span
                                            >
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center justify-between p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div class="flex items-center gap-2">
                                            <div
                                                class="p-1.5 rounded-lg bg-purple-100 dark:bg-purple-900/30"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-up-right w-3.5 h-3.5 text-purple-600 dark:text-purple-400"
                                                >
                                                    <path d="M7 7h10v10"></path>
                                                    <path d="M7 17 17 7"></path>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Investment Portfolio
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Stock &amp; ETFs
                                                </p>
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <span
                                                class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >$15,230.80</span
                                            >
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center justify-between p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div class="flex items-center gap-2">
                                            <div class="p-1.5 rounded-lg">
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-credit-card w-3.5 h-3.5 text-red-600 dark:text-red-400"
                                                >
                                                    <rect
                                                        width="20"
                                                        height="14"
                                                        x="2"
                                                        y="5"
                                                        rx="2"
                                                    ></rect>
                                                    <line
                                                        x1="2"
                                                        x2="22"
                                                        y1="10"
                                                        y2="10"
                                                    ></line>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Credit Card
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Pending charges
                                                </p>
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <span
                                                class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >$1,200.00</span
                                            >
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center justify-between p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div class="flex items-center gap-2">
                                            <div
                                                class="p-1.5 rounded-lg bg-emerald-100 dark:bg-emerald-900/30"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-wallet w-3.5 h-3.5 text-emerald-600 dark:text-emerald-400"
                                                >
                                                    <path
                                                        d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"
                                                    ></path>
                                                    <path
                                                        d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"
                                                    ></path>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Savings Account
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Emergency fund
                                                </p>
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <span
                                                class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >$3,000.00</span
                                            >
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div
                                class="p-2 border-t border-zinc-100 dark:border-zinc-800"
                            >
                                <div class="grid grid-cols-4 gap-2">
                                    <button
                                        type="button"
                                        class="flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-medium bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-200 shadow-sm hover:shadow transition-all duration-200"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-plus w-3.5 h-3.5"
                                        >
                                            <path d="M5 12h14"></path>
                                            <path d="M12 5v14"></path></svg
                                        ><span>Add</span></button
                                    ><button
                                        type="button"
                                        class="flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-medium bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-200 shadow-sm hover:shadow transition-all duration-200"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-send-horizontal w-3.5 h-3.5"
                                        >
                                            <path d="m3 3 3 9-3 9 19-9Z"></path>
                                            <path d="M6 12h16"></path></svg
                                        ><span>Send</span></button
                                    ><button
                                        type="button"
                                        class="flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-medium bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-200 shadow-sm hover:shadow transition-all duration-200"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-arrow-down-left w-3.5 h-3.5"
                                        >
                                            <path d="M17 7 7 17"></path>
                                            <path d="M17 17H7V7"></path></svg
                                        ><span>Top-up</span></button
                                    ><button
                                        type="button"
                                        class="flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-medium bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-200 shadow-sm hover:shadow transition-all duration-200"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-arrow-right w-3.5 h-3.5"
                                        >
                                            <path d="M5 12h14"></path>
                                            <path d="m12 5 7 7-7 7"></path></svg
                                        ><span>More</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div
                    class="bg-white dark:bg-[#0F0F12] rounded-xl p-6 flex flex-col border border-gray-200 dark:border-[#1F1F23]"
                >
                    <h2
                        class="text-lg font-bold text-gray-900 dark:text-white mb-4 text-left flex items-center gap-2"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-credit-card w-3.5 h-3.5 text-zinc-900 dark:text-zinc-50"
                        >
                            <rect
                                width="20"
                                height="14"
                                x="2"
                                y="5"
                                rx="2"
                            ></rect>
                            <line x1="2" x2="22" y1="10" y2="10"></line></svg
                        >Recent Transactions
                    </h2>
                    <div class="flex-1">
                        <div
                            class="w-full max-w-xl mx-auto bg-white dark:bg-zinc-900/70 border border-zinc-100 dark:border-zinc-800 rounded-xl shadow-sm backdrop-blur-xl h-full"
                        >
                            <div class="p-4">
                                <div
                                    class="flex items-center justify-between mb-3"
                                >
                                    <h2
                                        class="text-sm font-semibold text-zinc-900 dark:text-zinc-100"
                                    >
                                        Recent Activity<span
                                            class="text-xs font-normal text-zinc-600 dark:text-zinc-400 ml-1"
                                            >(23 transactions)</span
                                        >
                                    </h2>
                                    <span
                                        class="text-xs text-zinc-600 dark:text-zinc-400"
                                        >This Month</span
                                    >
                                </div>
                                <div class="space-y-1">
                                    <div
                                        class="group flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div
                                            class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="lucide lucide-shopping-cart w-4 h-4 text-zinc-900 dark:text-zinc-100"
                                            >
                                                <circle
                                                    cx="8"
                                                    cy="21"
                                                    r="1"
                                                ></circle>
                                                <circle
                                                    cx="19"
                                                    cy="21"
                                                    r="1"
                                                ></circle>
                                                <path
                                                    d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12"
                                                ></path>
                                            </svg>
                                        </div>
                                        <div
                                            class="flex-1 flex items-center justify-between min-w-0"
                                        >
                                            <div class="space-y-0.5">
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Apple Store Purchase
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Today, 2:45 PM
                                                </p>
                                            </div>
                                            <div
                                                class="flex items-center gap-1.5 pl-3"
                                            >
                                                <span
                                                    class="text-xs font-medium text-red-600 dark:text-red-400"
                                                    >-$999.00</span
                                                ><svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-up-right w-3.5 h-3.5 text-red-600 dark:text-red-400"
                                                >
                                                    <path d="M7 7h10v10"></path>
                                                    <path d="M7 17 17 7"></path>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div
                                            class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="lucide lucide-wallet w-4 h-4 text-zinc-900 dark:text-zinc-100"
                                            >
                                                <path
                                                    d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"
                                                ></path>
                                                <path
                                                    d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"
                                                ></path>
                                            </svg>
                                        </div>
                                        <div
                                            class="flex-1 flex items-center justify-between min-w-0"
                                        >
                                            <div class="space-y-0.5">
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Salary Deposit
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Today, 9:00 AM
                                                </p>
                                            </div>
                                            <div
                                                class="flex items-center gap-1.5 pl-3"
                                            >
                                                <span
                                                    class="text-xs font-medium text-emerald-600 dark:text-emerald-400"
                                                    >+$4,500.00</span
                                                ><svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-down-left w-3.5 h-3.5 text-emerald-600 dark:text-emerald-400"
                                                >
                                                    <path d="M17 7 7 17"></path>
                                                    <path d="M17 17H7V7"></path>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div
                                            class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="lucide lucide-credit-card w-4 h-4 text-zinc-900 dark:text-zinc-100"
                                            >
                                                <rect
                                                    width="20"
                                                    height="14"
                                                    x="2"
                                                    y="5"
                                                    rx="2"
                                                ></rect>
                                                <line
                                                    x1="2"
                                                    x2="22"
                                                    y1="10"
                                                    y2="10"
                                                ></line>
                                            </svg>
                                        </div>
                                        <div
                                            class="flex-1 flex items-center justify-between min-w-0"
                                        >
                                            <div class="space-y-0.5">
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Netflix Subscription
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Yesterday
                                                </p>
                                            </div>
                                            <div
                                                class="flex items-center gap-1.5 pl-3"
                                            >
                                                <span
                                                    class="text-xs font-medium text-red-600 dark:text-red-400"
                                                    >-$15.99</span
                                                ><svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-up-right w-3.5 h-3.5 text-red-600 dark:text-red-400"
                                                >
                                                    <path d="M7 7h10v10"></path>
                                                    <path d="M7 17 17 7"></path>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div
                                            class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="lucide lucide-shopping-cart w-4 h-4 text-zinc-900 dark:text-zinc-100"
                                            >
                                                <circle
                                                    cx="8"
                                                    cy="21"
                                                    r="1"
                                                ></circle>
                                                <circle
                                                    cx="19"
                                                    cy="21"
                                                    r="1"
                                                ></circle>
                                                <path
                                                    d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12"
                                                ></path>
                                            </svg>
                                        </div>
                                        <div
                                            class="flex-1 flex items-center justify-between min-w-0"
                                        >
                                            <div class="space-y-0.5">
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Apple Store Purchase
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Today, 2:45 PM
                                                </p>
                                            </div>
                                            <div
                                                class="flex items-center gap-1.5 pl-3"
                                            >
                                                <span
                                                    class="text-xs font-medium text-red-600 dark:text-red-400"
                                                    >-$999.00</span
                                                ><svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-up-right w-3.5 h-3.5 text-red-600 dark:text-red-400"
                                                >
                                                    <path d="M7 7h10v10"></path>
                                                    <path d="M7 17 17 7"></path>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div
                                            class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="lucide lucide-credit-card w-4 h-4 text-zinc-900 dark:text-zinc-100"
                                            >
                                                <rect
                                                    width="20"
                                                    height="14"
                                                    x="2"
                                                    y="5"
                                                    rx="2"
                                                ></rect>
                                                <line
                                                    x1="2"
                                                    x2="22"
                                                    y1="10"
                                                    y2="10"
                                                ></line>
                                            </svg>
                                        </div>
                                        <div
                                            class="flex-1 flex items-center justify-between min-w-0"
                                        >
                                            <div class="space-y-0.5">
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Supabase Subscription
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Yesterday
                                                </p>
                                            </div>
                                            <div
                                                class="flex items-center gap-1.5 pl-3"
                                            >
                                                <span
                                                    class="text-xs font-medium text-red-600 dark:text-red-400"
                                                    >-$15.99</span
                                                ><svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-up-right w-3.5 h-3.5 text-red-600 dark:text-red-400"
                                                >
                                                    <path d="M7 7h10v10"></path>
                                                    <path d="M7 17 17 7"></path>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="group flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-all duration-200"
                                    >
                                        <div
                                            class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700"
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="lucide lucide-credit-card w-4 h-4 text-zinc-900 dark:text-zinc-100"
                                            >
                                                <rect
                                                    width="20"
                                                    height="14"
                                                    x="2"
                                                    y="5"
                                                    rx="2"
                                                ></rect>
                                                <line
                                                    x1="2"
                                                    x2="22"
                                                    y1="10"
                                                    y2="10"
                                                ></line>
                                            </svg>
                                        </div>
                                        <div
                                            class="flex-1 flex items-center justify-between min-w-0"
                                        >
                                            <div class="space-y-0.5">
                                                <h3
                                                    class="text-xs font-medium text-zinc-900 dark:text-zinc-100"
                                                >
                                                    Vercel Subscription
                                                </h3>
                                                <p
                                                    class="text-[11px] text-zinc-600 dark:text-zinc-400"
                                                >
                                                    Yesterday
                                                </p>
                                            </div>
                                            <div
                                                class="flex items-center gap-1.5 pl-3"
                                            >
                                                <span
                                                    class="text-xs font-medium text-red-600 dark:text-red-400"
                                                    >-$15.99</span
                                                ><svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    width="24"
                                                    height="24"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="lucide lucide-arrow-up-right w-3.5 h-3.5 text-red-600 dark:text-red-400"
                                                >
                                                    <path d="M7 7h10v10"></path>
                                                    <path d="M7 17 17 7"></path>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div
                                class="p-2 border-t border-zinc-100 dark:border-zinc-800"
                            >
                                <button
                                    type="button"
                                    class="w-full flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-medium bg-gradient-to-r from-zinc-900 to-zinc-800 dark:from-zinc-50 dark:to-zinc-200 text-zinc-50 dark:text-zinc-900 hover:from-zinc-800 hover:to-zinc-700 dark:hover:from-zinc-200 dark:hover:to-zinc-300 shadow-sm hover:shadow transform transition-all duration-200 hover:-translate-y-0.5 active:translate-y-0 focus:outline-none focus:ring-2 focus:ring-zinc-500 dark:focus:ring-zinc-400 focus:ring-offset-2 dark:focus:ring-offset-zinc-900"
                                >
                                    <span>View All Transactions</span
                                    ><svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-arrow-right w-3.5 h-3.5"
                                    >
                                        <path d="M5 12h14"></path>
                                        <path d="m12 5 7 7-7 7"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div
                class="bg-white dark:bg-[#0F0F12] rounded-xl p-6 flex flex-col items-start justify-start border border-gray-200 dark:border-[#1F1F23]"
            >
                <h2
                    class="text-lg font-bold text-gray-900 dark:text-white mb-4 text-left flex items-center gap-2"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="lucide lucide-calendar w-3.5 h-3.5 text-zinc-900 dark:text-zinc-50"
                    >
                        <path d="M8 2v4"></path>
                        <path d="M16 2v4"></path>
                        <rect width="18" height="18" x="3" y="4" rx="2"></rect>
                        <path d="M3 10h18"></path></svg
                    >Upcoming Events
                </h2>
                <div class="w-full overflow-x-auto scrollbar-none">
                    <div class="flex gap-3 min-w-full p-1">
                        <div
                            class="flex flex-col w-[280px] shrink-0 bg-white dark:bg-zinc-900/70 rounded-xl border border-zinc-100 dark:border-zinc-800 hover:border-zinc-200 dark:hover:border-zinc-700 transition-all duration-200 shadow-sm backdrop-blur-xl"
                        >
                            <div class="p-4 space-y-3">
                                <div class="flex items-start justify-between">
                                    <div
                                        class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-piggy-bank w-4 h-4"
                                        >
                                            <path
                                                d="M19 5c-1.5 0-2.8 1.4-3 2-3.5-1.5-11-.3-11 5 0 1.8 0 3 2 4.5V20h4v-2h3v2h4v-4c1-.5 1.7-1 2-2h2v-4h-2c0-1-.5-1.5-1-2h0V5z"
                                            ></path>
                                            <path
                                                d="M2 9v1c0 1.1.9 2 2 2h1"
                                            ></path>
                                            <path d="M16 11h0"></path>
                                        </svg>
                                    </div>
                                    <div
                                        class="px-2 py-1 rounded-full text-xs font-medium flex items-center gap-1.5 bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-circle-alert w-3.5 h-3.5"
                                        >
                                            <circle
                                                cx="12"
                                                cy="12"
                                                r="10"
                                            ></circle>
                                            <line
                                                x1="12"
                                                x2="12"
                                                y1="8"
                                                y2="12"
                                            ></line>
                                            <line
                                                x1="12"
                                                x2="12.01"
                                                y1="16"
                                                y2="16"
                                            ></line></svg
                                        >In-progress
                                    </div>
                                </div>
                                <div>
                                    <h3
                                        class="text-sm font-medium text-zinc-900 dark:text-zinc-100 mb-1"
                                    >
                                        Emergency Fund
                                    </h3>
                                    <p
                                        class="text-xs text-zinc-600 dark:text-zinc-400 line-clamp-2"
                                    >
                                        3 months of expenses saved
                                    </p>
                                </div>
                                <div class="space-y-1.5">
                                    <div
                                        class="flex items-center justify-between text-xs"
                                    >
                                        <span
                                            class="text-zinc-600 dark:text-zinc-400"
                                            >Progress</span
                                        ><span
                                            class="text-zinc-900 dark:text-zinc-100"
                                            >65%</span
                                        >
                                    </div>
                                    <div
                                        class="h-1.5 bg-zinc-100 dark:bg-zinc-800 rounded-full overflow-hidden"
                                    >
                                        <div
                                            class="h-full bg-zinc-900 dark:bg-zinc-100 rounded-full"
                                            style="width: 65%"
                                        ></div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-1.5">
                                    <span
                                        class="text-sm font-medium text-zinc-900 dark:text-zinc-100"
                                        >$15,000</span
                                    ><span
                                        class="text-xs text-zinc-600 dark:text-zinc-400"
                                        >target</span
                                    >
                                </div>
                                <div
                                    class="flex items-center text-xs text-zinc-600 dark:text-zinc-400"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-calendar w-3.5 h-3.5 mr-1.5"
                                    >
                                        <path d="M8 2v4"></path>
                                        <path d="M16 2v4"></path>
                                        <rect
                                            width="18"
                                            height="18"
                                            x="3"
                                            y="4"
                                            rx="2"
                                        ></rect>
                                        <path d="M3 10h18"></path></svg
                                    ><span>Target: Dec 2024</span>
                                </div>
                            </div>
                            <div
                                class="mt-auto border-t border-zinc-100 dark:border-zinc-800"
                            >
                                <button
                                    class="w-full flex items-center justify-center gap-2 py-2.5 px-3 text-xs font-medium text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100 hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-colors duration-200"
                                >
                                    View Details<svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-arrow-right w-3.5 h-3.5"
                                    >
                                        <path d="M5 12h14"></path>
                                        <path d="m12 5 7 7-7 7"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                        <div
                            class="flex flex-col w-[280px] shrink-0 bg-white dark:bg-zinc-900/70 rounded-xl border border-zinc-100 dark:border-zinc-800 hover:border-zinc-200 dark:hover:border-zinc-700 transition-all duration-200 shadow-sm backdrop-blur-xl"
                        >
                            <div class="p-4 space-y-3">
                                <div class="flex items-start justify-between">
                                    <div
                                        class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-trending-up w-4 h-4"
                                        >
                                            <polyline
                                                points="22 7 13.5 15.5 8.5 10.5 2 17"
                                            ></polyline>
                                            <polyline
                                                points="16 7 22 7 22 13"
                                            ></polyline>
                                        </svg>
                                    </div>
                                    <div
                                        class="px-2 py-1 rounded-full text-xs font-medium flex items-center gap-1.5 bg-amber-100 dark:bg-amber-900/30 text-amber-600 dark:text-amber-400"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-timer w-3.5 h-3.5"
                                        >
                                            <line
                                                x1="10"
                                                x2="14"
                                                y1="2"
                                                y2="2"
                                            ></line>
                                            <line
                                                x1="12"
                                                x2="15"
                                                y1="14"
                                                y2="11"
                                            ></line>
                                            <circle
                                                cx="12"
                                                cy="14"
                                                r="8"
                                            ></circle></svg
                                        >Pending
                                    </div>
                                </div>
                                <div>
                                    <h3
                                        class="text-sm font-medium text-zinc-900 dark:text-zinc-100 mb-1"
                                    >
                                        Stock Portfolio
                                    </h3>
                                    <p
                                        class="text-xs text-zinc-600 dark:text-zinc-400 line-clamp-2"
                                    >
                                        Tech sector investment plan
                                    </p>
                                </div>
                                <div class="space-y-1.5">
                                    <div
                                        class="flex items-center justify-between text-xs"
                                    >
                                        <span
                                            class="text-zinc-600 dark:text-zinc-400"
                                            >Progress</span
                                        ><span
                                            class="text-zinc-900 dark:text-zinc-100"
                                            >30%</span
                                        >
                                    </div>
                                    <div
                                        class="h-1.5 bg-zinc-100 dark:bg-zinc-800 rounded-full overflow-hidden"
                                    >
                                        <div
                                            class="h-full bg-zinc-900 dark:bg-zinc-100 rounded-full"
                                            style="width: 30%"
                                        ></div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-1.5">
                                    <span
                                        class="text-sm font-medium text-zinc-900 dark:text-zinc-100"
                                        >$50,000</span
                                    ><span
                                        class="text-xs text-zinc-600 dark:text-zinc-400"
                                        >target</span
                                    >
                                </div>
                                <div
                                    class="flex items-center text-xs text-zinc-600 dark:text-zinc-400"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-calendar w-3.5 h-3.5 mr-1.5"
                                    >
                                        <path d="M8 2v4"></path>
                                        <path d="M16 2v4"></path>
                                        <rect
                                            width="18"
                                            height="18"
                                            x="3"
                                            y="4"
                                            rx="2"
                                        ></rect>
                                        <path d="M3 10h18"></path></svg
                                    ><span>Target: Jun 2024</span>
                                </div>
                            </div>
                            <div
                                class="mt-auto border-t border-zinc-100 dark:border-zinc-800"
                            >
                                <button
                                    class="w-full flex items-center justify-center gap-2 py-2.5 px-3 text-xs font-medium text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100 hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-colors duration-200"
                                >
                                    View Details<svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-arrow-right w-3.5 h-3.5"
                                    >
                                        <path d="M5 12h14"></path>
                                        <path d="m12 5 7 7-7 7"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                        <div
                            class="flex flex-col w-[280px] shrink-0 bg-white dark:bg-zinc-900/70 rounded-xl border border-zinc-100 dark:border-zinc-800 hover:border-zinc-200 dark:hover:border-zinc-700 transition-all duration-200 shadow-sm backdrop-blur-xl"
                        >
                            <div class="p-4 space-y-3">
                                <div class="flex items-start justify-between">
                                    <div
                                        class="p-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-credit-card w-4 h-4"
                                        >
                                            <rect
                                                width="20"
                                                height="14"
                                                x="2"
                                                y="5"
                                                rx="2"
                                            ></rect>
                                            <line
                                                x1="2"
                                                x2="22"
                                                y1="10"
                                                y2="10"
                                            ></line>
                                        </svg>
                                    </div>
                                    <div
                                        class="px-2 py-1 rounded-full text-xs font-medium flex items-center gap-1.5 bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-circle-alert w-3.5 h-3.5"
                                        >
                                            <circle
                                                cx="12"
                                                cy="12"
                                                r="10"
                                            ></circle>
                                            <line
                                                x1="12"
                                                x2="12"
                                                y1="8"
                                                y2="12"
                                            ></line>
                                            <line
                                                x1="12"
                                                x2="12.01"
                                                y1="16"
                                                y2="16"
                                            ></line></svg
                                        >In-progress
                                    </div>
                                </div>
                                <div>
                                    <h3
                                        class="text-sm font-medium text-zinc-900 dark:text-zinc-100 mb-1"
                                    >
                                        Debt Repayment
                                    </h3>
                                    <p
                                        class="text-xs text-zinc-600 dark:text-zinc-400 line-clamp-2"
                                    >
                                        Student loan payoff plan
                                    </p>
                                </div>
                                <div class="space-y-1.5">
                                    <div
                                        class="flex items-center justify-between text-xs"
                                    >
                                        <span
                                            class="text-zinc-600 dark:text-zinc-400"
                                            >Progress</span
                                        ><span
                                            class="text-zinc-900 dark:text-zinc-100"
                                            >45%</span
                                        >
                                    </div>
                                    <div
                                        class="h-1.5 bg-zinc-100 dark:bg-zinc-800 rounded-full overflow-hidden"
                                    >
                                        <div
                                            class="h-full bg-zinc-900 dark:bg-zinc-100 rounded-full"
                                            style="width: 45%"
                                        ></div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-1.5">
                                    <span
                                        class="text-sm font-medium text-zinc-900 dark:text-zinc-100"
                                        >$25,000</span
                                    ><span
                                        class="text-xs text-zinc-600 dark:text-zinc-400"
                                        >target</span
                                    >
                                </div>
                                <div
                                    class="flex items-center text-xs text-zinc-600 dark:text-zinc-400"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-calendar w-3.5 h-3.5 mr-1.5"
                                    >
                                        <path d="M8 2v4"></path>
                                        <path d="M16 2v4"></path>
                                        <rect
                                            width="18"
                                            height="18"
                                            x="3"
                                            y="4"
                                            rx="2"
                                        ></rect>
                                        <path d="M3 10h18"></path></svg
                                    ><span>Target: Mar 2025</span>
                                </div>
                            </div>
                            <div
                                class="mt-auto border-t border-zinc-100 dark:border-zinc-800"
                            >
                                <button
                                    class="w-full flex items-center justify-center gap-2 py-2.5 px-3 text-xs font-medium text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100 hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-colors duration-200"
                                >
                                    View Details<svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="lucide lucide-arrow-right w-3.5 h-3.5"
                                    >
                                        <path d="M5 12h14"></path>
                                        <path d="m12 5 7 7-7 7"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </main>
</template>

<script lang="ts" setup></script>

<style lang="scss"></style>
