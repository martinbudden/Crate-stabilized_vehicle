use crate::AhrsData;

pub trait VehicleControl {
    fn state(&self) -> &VehicleController;
    fn state_mut(&mut self) -> &mut VehicleController;

    fn set_sensor_fusion_filter_is_initializing(&mut self, sensor_fusion_filter_is_initializing: bool) {
        self.state_mut().sensor_fusion_filter_is_initializing = sensor_fusion_filter_is_initializing;
    }

    fn update_outputs_using_pids(
        &mut self,
        ahrs_data: AhrsData,
        ahrs_data: AhrsData,
        motor_mixer_message_queue: MotorMixerMessageQueue,
    );
}

pub struct VehicleController {
    sensor_fusion_filter_is_initializing: bool,
}

pub struct MotorMixerMessageQueue {}
