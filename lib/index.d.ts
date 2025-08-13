import * as RspackCore from '@rspack/core';

/**
 * MyBannerPlugin class that adds a banner to the output main.js file.
 */
declare class MyBannerPlugin {
  /**
   * The banner text to be added to the output file.
   */
  constructor(banner: string);
}

/**
 * MyBannerLoaderPlugin class that adds a custom banner loader (builtin:my-banner-loader) to Rspack.
 */
declare class MyBannerLoaderPlugin {
  constructor();
}

declare const core: typeof RspackCore & {
  MyBannerPlugin: typeof MyBannerPlugin;
  MyBannerLoaderPlugin: typeof MyBannerLoaderPlugin;
};

export = core;
