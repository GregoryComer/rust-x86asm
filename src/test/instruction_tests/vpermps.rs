use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 22, 244], OperandSize::Dword)
}

fn vpermps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 489450888, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 22, 136, 136, 109, 44, 29], OperandSize::Dword)
}

fn vpermps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 22, 238], OperandSize::Qword)
}

fn vpermps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 600573962, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 22, 156, 143, 10, 8, 204, 35], OperandSize::Qword)
}

fn vpermps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 169, 22, 223], OperandSize::Dword)
}

fn vpermps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 22, 44, 130], OperandSize::Dword)
}

fn vpermps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 130315453, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 188, 22, 28, 117, 189, 116, 196, 7], OperandSize::Dword)
}

fn vpermps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 117, 170, 22, 248], OperandSize::Qword)
}

fn vpermps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RDI, 1710324614, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 13, 169, 22, 135, 134, 123, 241, 101], OperandSize::Qword)
}

fn vpermps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 880162973, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 21, 185, 22, 172, 66, 157, 56, 118, 52], OperandSize::Qword)
}

fn vpermps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 22, 233], OperandSize::Dword)
}

fn vpermps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 314383272, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 22, 52, 125, 168, 27, 189, 18], OperandSize::Dword)
}

fn vpermps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 325390480, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 22, 52, 125, 144, 16, 101, 19], OperandSize::Dword)
}

fn vpermps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 125, 206, 22, 245], OperandSize::Qword)
}

fn vpermps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RAX, 1413395922, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 117, 196, 22, 128, 210, 181, 62, 84], OperandSize::Qword)
}

fn vpermps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 118392026, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 210, 22, 44, 125, 218, 132, 14, 7], OperandSize::Qword)
}

