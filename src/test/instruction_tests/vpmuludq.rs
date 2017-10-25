use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 244, 217], OperandSize::Dword)
}

fn vpmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 244, 23], OperandSize::Dword)
}

fn vpmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 244, 192], OperandSize::Qword)
}

fn vpmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 244, 36, 176], OperandSize::Qword)
}

fn vpmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 244, 242], OperandSize::Dword)
}

fn vpmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1373807363, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 244, 28, 117, 3, 163, 226, 81], OperandSize::Dword)
}

fn vpmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 244, 238], OperandSize::Qword)
}

fn vpmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1799702952, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 244, 60, 141, 168, 73, 69, 107], OperandSize::Qword)
}

fn vpmuludq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 244, 196], OperandSize::Dword)
}

fn vpmuludq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 137, 244, 60, 127], OperandSize::Dword)
}

fn vpmuludq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 2079305531, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 153, 244, 4, 125, 59, 175, 239, 123], OperandSize::Dword)
}

fn vpmuludq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 165, 133, 244, 192], OperandSize::Qword)
}

fn vpmuludq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1531949250, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 173, 134, 244, 52, 77, 194, 176, 79, 91], OperandSize::Qword)
}

fn vpmuludq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1326132523, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 173, 159, 244, 148, 195, 43, 45, 11, 79], OperandSize::Qword)
}

fn vpmuludq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 173, 244, 205], OperandSize::Dword)
}

fn vpmuludq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 847879072, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 173, 244, 162, 160, 155, 137, 50], OperandSize::Dword)
}

fn vpmuludq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 187, 244, 52, 72], OperandSize::Dword)
}

fn vpmuludq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 141, 164, 244, 238], OperandSize::Qword)
}

fn vpmuludq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1390806946, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 157, 162, 244, 36, 93, 162, 7, 230, 82], OperandSize::Qword)
}

fn vpmuludq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 205, 180, 244, 51], OperandSize::Qword)
}

fn vpmuludq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 205, 244, 255], OperandSize::Dword)
}

fn vpmuludq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 159797044, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 244, 156, 215, 52, 79, 134, 9], OperandSize::Dword)
}

fn vpmuludq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 223, 244, 34], OperandSize::Dword)
}

fn vpmuludq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 197, 205, 244, 220], OperandSize::Qword)
}

fn vpmuludq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RAX, 1922145393, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 213, 194, 244, 128, 113, 156, 145, 114], OperandSize::Qword)
}

fn vpmuludq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 917448237, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 221, 209, 244, 148, 136, 45, 38, 175, 54], OperandSize::Qword)
}

