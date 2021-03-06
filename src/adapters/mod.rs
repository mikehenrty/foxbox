/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

mod ip_camera_adapter;

/// An adapter dedicated to the Philips Hue
mod philips_hue;

/// An adapter providing time services.
mod clock;

use controller::Controller;
use self::ip_camera_adapter::IpCameraAdapter;
use self::philips_hue::PhilipsHueAdapter;
use service::ServiceAdapter;

/// Work in progress replacing the AdapterManager by that of foxbox_adapters.
use foxbox_adapters::manager::AdapterManager as AdapterManager2;

use std::sync::Arc;

pub struct AdapterManager<T> {
    controller: T,
    adapters: Vec<Box<ServiceAdapter>>,

    // FIXME: This field isn't used for the time being.
    manager2: Arc<AdapterManager2>,
}

impl<T: Controller> AdapterManager<T> {
    pub fn new(controller: T) -> Self {
        debug!("Creating Adapter Manager");
        AdapterManager {
            controller: controller,
            adapters: Vec::new(),
            manager2: Arc::new(AdapterManager2::new()),
        }
    }

    /// Start all the adapters.
    pub fn start(&mut self) {
        let c = self.controller.clone(); // extracted here to prevent double-borrow of 'self'
        self.start_adapter(Box::new(PhilipsHueAdapter::new(c.clone())));
        self.start_adapter(Box::new(IpCameraAdapter::new(c)));
        clock::Clock::init(&self.manager2).unwrap(); // FIXME: We should have a way to report errors
    }

    fn start_adapter(&mut self, adapter: Box<ServiceAdapter>) {
        adapter.start();
        self.adapters.push(adapter);
    }

    /// Stop all the adapters.
    pub fn stop(&self) {
        for adapter in &self.adapters {
            adapter.stop();
        }
    }
}
