.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── app
│   ├── backend
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── build
│   │   │   ├── README.md
│   │   │   ├── index.d.ts
│   │   │   ├── index.js
│   │   │   ├── index_bg.wasm.d.ts
│   │   │   ├── package.json
│   │   │   └── worker
│   │   │       ├── index.wasm
│   │   │       └── shim.mjs
│   │   ├── deno.json
│   │   ├── migrations
│   │   │   ├── 0001_users.sql
│   │   │   └── 0002_memos.sql
│   │   ├── src
│   │   │   ├── api
│   │   │   │   ├── auth
│   │   │   │   │   ├── login
│   │   │   │   │   │   ├── error.rs
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── logout
│   │   │   │   │   │   ├── error.rs
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── session
│   │   │   │   │       └── mod.rs
│   │   │   │   ├── memo
│   │   │   │   │   ├── create
│   │   │   │   │   │   ├── error.rs
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── delete
│   │   │   │   │   │   ├── error.rs
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── edit
│   │   │   │   │   │   ├── error.rs
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── get
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── to_public
│   │   │   │   │       ├── error.rs
│   │   │   │   │       └── mod.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── user
│   │   │   │       ├── mod.rs
│   │   │   │       └── register
│   │   │   │           ├── error.rs
│   │   │   │           └── mod.rs
│   │   │   ├── internal
│   │   │   │   ├── authorization
│   │   │   │   │   ├── error.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── components
│   │   │   │   │   ├── cors.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── di
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── domain
│   │   │   │   │   ├── entity
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── repository
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   ├── service
│   │   │   │   │   │   └── mod.rs
│   │   │   │   │   └── value_object
│   │   │   │   │       └── mod.rs
│   │   │   │   ├── logging
│   │   │   │   │   ├── error.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── middleware
│   │   │   │   │   ├── context.rs
│   │   │   │   │   ├── error.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── mod.rs
│   │   │   ├── lib.rs
│   │   │   └── route.rs
│   │   └── wrangler.toml
│   ├── frontend
│   │   ├── README.md
│   │   ├── eslint.config.js
│   │   ├── package.json
│   │   ├── pnpm-lock.yaml
│   │   ├── src
│   │   │   ├── app.css
│   │   │   ├── app.d.ts
│   │   │   ├── app.html
│   │   │   ├── demo.spec.ts
│   │   │   ├── lib
│   │   │   │   ├── auth
│   │   │   │   │   ├── components
│   │   │   │   │   │   └── AuthGuard.svelte
│   │   │   │   │   └── store
│   │   │   │   │       ├── auth.ts
│   │   │   │   │       ├── check-session.ts
│   │   │   │   │       ├── error.ts
│   │   │   │   │       └── index.ts
│   │   │   │   ├── components
│   │   │   │   │   ├── BaseCard.svelte
│   │   │   │   │   ├── BaseForm.svelte
│   │   │   │   │   ├── CreateLifeLogButton.svelte
│   │   │   │   │   ├── CreateNote.svelte
│   │   │   │   │   ├── EmptyAwareList.svelte
│   │   │   │   │   ├── GoBackButton.svelte
│   │   │   │   │   ├── LifeLogForm.svelte
│   │   │   │   │   ├── MarkdownPreview.svelte
│   │   │   │   │   └── Navigation.svelte
│   │   │   │   ├── index.ts
│   │   │   │   ├── mock-data.ts
│   │   │   │   ├── routes.ts
│   │   │   │   ├── store
│   │   │   │   │   ├── index.ts
│   │   │   │   │   └── memo
│   │   │   │   │       └── api.ts
│   │   │   │   ├── types
│   │   │   │   │   └── src
│   │   │   │   │       └── api
│   │   │   │   │           ├── auth
│   │   │   │   │           │   └── login
│   │   │   │   │           │       └── login.ts
│   │   │   │   │           └── memo
│   │   │   │   │               └── get_count_by_date.ts
│   │   │   │   └── utils
│   │   │   │       ├── date.ts
│   │   │   │       └── local-storage.ts
│   │   │   ├── routes
│   │   │   │   ├── +layout.svelte
│   │   │   │   ├── +page.svelte
│   │   │   │   ├── create
│   │   │   │   │   └── life-log
│   │   │   │   │       ├── +page.svelte
│   │   │   │   │       └── const.ts
│   │   │   │   ├── login
│   │   │   │   │   ├── +page.svelte
│   │   │   │   │   └── components
│   │   │   │   │       ├── LoginForm.svelte
│   │   │   │   │       ├── LoginInput.svelte
│   │   │   │   │       └── LoginLavel.svelte
│   │   │   │   ├── memomesh
│   │   │   │   │   ├── +page.svelte
│   │   │   │   │   ├── [date]
│   │   │   │   │   │   ├── +page.svelte
│   │   │   │   │   │   └── store.ts
│   │   │   │   │   ├── components
│   │   │   │   │   │   ├── DateCard.svelte
│   │   │   │   │   │   ├── Section.svelte
│   │   │   │   │   │   └── SectionHeader.svelte
│   │   │   │   │   └── type.ts
│   │   │   │   └── page.svelte.test.ts
│   │   │   └── worker-configuration.d.ts
│   │   ├── static
│   │   │   └── favicon.png
│   │   ├── svelte.config.js
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts
│   │   ├── vitest-setup-client.ts
│   │   └── wrangler.toml
│   └── proto
│       ├── Cargo.toml
│       ├── build.rs
│       ├── package-lock.json
│       ├── package.json
│       └── src
│           ├── api
│           │   ├── auth
│           │   │   └── login
│           │   │       └── login.proto
│           │   └── memo
│           │       └── get_count_by_date.proto
│           ├── generated
│           │   └── api
│           │       ├── auth
│           │       │   └── login.rs
│           │       └── memo
│           │           └── get_count_by_date.rs
│           └── lib.rs
├── develop
│   ├── entrypoint.sh
│   ├── nix-shell-hook.sh
│   └── updateProtoc.js
├── docker-compose.yaml
├── dockerfile
├── docs
│   ├── curl.md
│   ├── prmpt.md
│   └── project-tree.md
├── flake.lock
├── flake.nix
├── makefile
├── package.json
├── pnpm-lock.yaml
└── pnpm-workspace.yaml

68 directories, 131 files
