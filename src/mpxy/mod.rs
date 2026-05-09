use super::{SbiRet, eid, sbi_call_1, sbi_call_2, sbi_call_3, sbi_call_4};

const FID_SET_SHMEM: usize = 0;
const FID_GET_CHANNEL_IDS: usize = 1;
const FID_READ_ATTRS: usize = 2;
const FID_WRITE_ATTRS: usize = 3;
const FID_SEND_WITH_RESPONSE: usize = 4;
const FID_SEND_WITHOUT_RESPONSE: usize = 5;
const FID_GET_NOTIFICATION_EVENTS: usize = 6;

pub fn set_shmem(shmem_phys_lo: usize, shmem_phys_hi: usize, flags: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::MPXY, FID_SET_SHMEM, shmem_phys_lo, shmem_phys_hi, flags) }
}

pub fn get_channel_ids(start_index: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::MPXY, FID_GET_CHANNEL_IDS, start_index) }
}

pub fn read_attrs(channel_id: u32, attr_count: usize, output_phys_lo: usize, output_phys_hi: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::MPXY, FID_READ_ATTRS, channel_id as usize, attr_count, output_phys_lo, output_phys_hi) }
}

pub fn write_attrs(channel_id: u32, attr_count: usize, input_phys_lo: usize, input_phys_hi: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::MPXY, FID_WRITE_ATTRS, channel_id as usize, attr_count, input_phys_lo, input_phys_hi) }
}

pub fn send_with_response(channel_id: u32, message_id: u32, tx_len: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::MPXY, FID_SEND_WITH_RESPONSE, channel_id as usize, message_id as usize, tx_len) }
}

pub fn send_without_response(channel_id: u32, message_id: u32, tx_len: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::MPXY, FID_SEND_WITHOUT_RESPONSE, channel_id as usize, message_id as usize, tx_len) }
}

pub fn get_notification_events(channel_id: u32, start_index: usize) -> SbiRet {
    unsafe { sbi_call_2(eid::MPXY, FID_GET_NOTIFICATION_EVENTS, channel_id as usize, start_index) }
}
