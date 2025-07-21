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

declare const core: typeof RspackCore & {
  MyBannerPlugin: typeof MyBannerPlugin;
};

export = core;
