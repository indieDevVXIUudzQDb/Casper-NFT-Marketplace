### Requirements

- Node.js and npm


### Deploy to production

You can see the results locally in production mode with:

```
cp .env.template .env.local
npm run build
npm run dev
```

The generated HTML and CSS files are minified (built-in feature from Next js). It will also removed unused CSS from [Tailwind CSS](https://tailwindcss.com).

You can create an optimized production build with:

```
npm run build-prod
```

Now, your theme is ready to be deployed. All generated files are located at `out` folder, which you can deploy with any hosting service.
