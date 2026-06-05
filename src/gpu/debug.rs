use std::sync::Arc;

use tracing::{error, warn, debug, info};
use vulkano::instance::{Instance, debug::{DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger, DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo}};

#[cfg(debug_assertions)]
// PANIC when message severity or message type is unkown
pub fn debug_callback(instance: Arc<Instance>) -> Option<DebugUtilsMessenger> {
    unsafe {
                    let messenger_callback = DebugUtilsMessengerCallback::new(
                        |message_severity, message_type, callback_data| {
                            let ty = if message_type.intersects(DebugUtilsMessageType::GENERAL) {
                                "general"
                            } else if message_type.intersects(DebugUtilsMessageType::VALIDATION) {
                                "validation"
                            } else if message_type.intersects(DebugUtilsMessageType::PERFORMANCE) {
                                "performance"
                            } else {
                                panic!("no-impl");
                            };

                            if message_severity
                                .intersects(DebugUtilsMessageSeverity::ERROR)
                            {
                                error!("{} {}: {}", callback_data.message_id_name.unwrap_or("unkown"), ty, callback_data.message);
                            } else if message_severity.intersects(DebugUtilsMessageSeverity::WARNING) {
                                warn!("{} {}: {}", callback_data.message_id_name.unwrap_or("unkown"), ty, callback_data.message);
                            } else if message_severity.intersects(DebugUtilsMessageSeverity::INFO) {
                                info!("{} {}: {}", callback_data.message_id_name.unwrap_or("unkown"), ty, callback_data.message);
                            } else if message_severity.intersects(DebugUtilsMessageSeverity::VERBOSE) {
                                debug!("{} {}: {}", callback_data.message_id_name.unwrap_or("unkown"), ty, callback_data.message);
                            } else {
                                panic!("no-impl");
                            }
                        },
                    );
                    DebugUtilsMessenger::new(
                        instance,
                        DebugUtilsMessengerCreateInfo {
                            message_severity: DebugUtilsMessageSeverity::ERROR
                                | DebugUtilsMessageSeverity::WARNING
                                | DebugUtilsMessageSeverity::INFO
                                | DebugUtilsMessageSeverity::VERBOSE,
                            message_type: DebugUtilsMessageType::GENERAL
                                | DebugUtilsMessageType::VALIDATION
                                | DebugUtilsMessageType::PERFORMANCE,
                            ..DebugUtilsMessengerCreateInfo::user_callback(messenger_callback)
                        },
                    )
                }
                .ok()
}
