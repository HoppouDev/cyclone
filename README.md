# ꩜ Cyclone ꩜

This is a web application that uses generative AI to create an image given a primary instruction prompt, a style prompt, and reference images. Cyclone is currently made to be deployed on Cloudflare Workers, however it can be deployed on a service like Vercel or Netlify with a few modifications. An OpenRouter key is required due to their superior model schema.

## Project structure

This is a Cargo workspace (see `Cargo.toml`) containing:

- **`cyclone_frontend/`** — the SvelteKit app: UI, auth, and the D1-backed database. This is what gets deployed as the main Cloudflare Worker. See below for its own dev/build/deploy steps.
- **`cyclone_dummy/`** — a small Rust Cloudflare Worker (via [`worker-rs`](https://github.com/cloudflare/workers-rs) + axum) that stands in for the OpenRouter API during local development. Its `/images` route returns a hardcoded sample image as a schema-compliant `ImageGenerationResponse`, so the frontend can be developed without burning real OpenRouter credits.
  - `openrouter/images.schema.json` is a trimmed JSON Schema subset (pulled from OpenRouter's `/images` endpoint) checked into the repo.
  - `build.rs` runs [`typify`](https://github.com/oxidecomputer/typify) against that schema at compile time to generate the Rust request/response types (with `serde`) into `OUT_DIR` — the generated code itself is never committed.
  - `assets/sample.jpg` is the dummy image returned by `/images`.

## Developing

The frontend lives in `cyclone_frontend/`. Once you've installed dependencies with `pnpm install` there, start a development server:

```sh
cd cyclone_frontend
pnpm run dev

# or start the server and open Cyclone in a new browser tab
pnpm run dev -- --open
```

To run the dummy OpenRouter worker locally:

```sh
cd cyclone_dummy
wrangler dev
```

## API Keys

Enter the required API keys and environment variables in a new `.env` file in `cyclone_frontend/`. Refer to `cyclone_frontend/.env.example` for an example.

## Building

To create a production version of the frontend:

```sh
cd cyclone_frontend
pnpm run build
```

## Deploying

Create a D1 database and add its `database_id` (and `database_name`) to the `d1_databases` entry in `cyclone_frontend/wrangler.toml`:

```sh
wrangler d1 create cyclone
```

From `cyclone_frontend/`, generate the auth schema:

```sh
pnpm run auth:schema
```

Push the schema to the database:

```sh
pnpm run db:push
```

You can preview the production build with `pnpm run preview`.

Deploy to Cloudflare Workers:

```sh
wrangler deploy
```
