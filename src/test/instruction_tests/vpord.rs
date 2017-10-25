use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 235, 222], OperandSize::Dword)
}

fn vpord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 235, 15], OperandSize::Dword)
}

fn vpord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 155, 235, 27], OperandSize::Dword)
}

fn vpord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 85, 135, 235, 248], OperandSize::Qword)
}

fn vpord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 509434912, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 93, 138, 235, 164, 143, 32, 92, 93, 30], OperandSize::Qword)
}

fn vpord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RAX, 1788588014, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 93, 151, 235, 152, 238, 175, 155, 106], OperandSize::Qword)
}

fn vpord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 235, 247], OperandSize::Dword)
}

fn vpord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 235, 33], OperandSize::Dword)
}

fn vpord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 191, 235, 4, 73], OperandSize::Dword)
}

fn vpord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 77, 173, 235, 197], OperandSize::Qword)
}

fn vpord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 85, 173, 235, 40], OperandSize::Qword)
}

fn vpord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 661268859, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 191, 235, 164, 143, 123, 41, 106, 39], OperandSize::Qword)
}

fn vpord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 201, 235, 205], OperandSize::Dword)
}

fn vpord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1286494879, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 235, 148, 218, 159, 90, 174, 76], OperandSize::Dword)
}

fn vpord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EAX, 1977276542, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 218, 235, 160, 126, 216, 218, 117], OperandSize::Dword)
}

fn vpord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 37, 197, 235, 196], OperandSize::Qword)
}

fn vpord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 419583794, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 69, 207, 235, 60, 125, 50, 87, 2, 25], OperandSize::Qword)
}

fn vpord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 13, 223, 235, 25], OperandSize::Qword)
}

