# ꩜ Cyclone ꩜

This is a web application that uses generative AI to create an image given a primary instruction prompt, a style prompt, and reference images. Cyclone is currently made to be deployed on Cloudflare Workers, however it can be deployed on a service like Vercel or Netlify with a few modifications. An OpenRouter key is required due to their superior model schema.

## Developing

Once you've created a project and installed dependencies with `pnpm install`, start a development server:

```sh
pnpm run dev

# or start the server and open Cyclone in a new browser tab
pnpm run dev -- --open
```

## API Keys

Enter the required API keys and environment variables in a new `.env` file in the root directory of the repository. Refer to `.env.example` for an example.

## Building

To create a production version of Cyclone:

```sh
pnpm run build
```

## Deploying

Create a D1 database and add its `database_id` (and `database_name`) to the `d1_databases` entry in `wrangler.jsonc`:

```sh
wrangler d1 create cyclone
```

Generate the auth schema:

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
