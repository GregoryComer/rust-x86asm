use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 76, 221], OperandSize::Dword)
}

#[test]
fn vrcp14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2140680966, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 76, 20, 197, 6, 51, 152, 127], OperandSize::Dword)
}

#[test]
fn vrcp14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 76, 24], OperandSize::Dword)
}

#[test]
fn vrcp14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 76, 212], OperandSize::Qword)
}

#[test]
fn vrcp14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 76, 4, 201], OperandSize::Qword)
}

#[test]
fn vrcp14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1632208950, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 76, 140, 142, 54, 136, 73, 97], OperandSize::Qword)
}

#[test]
fn vrcp14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 76, 222], OperandSize::Dword)
}

#[test]
fn vrcp14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 493681274, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 76, 44, 149, 122, 250, 108, 29], OperandSize::Dword)
}

#[test]
fn vrcp14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 76, 49], OperandSize::Dword)
}

#[test]
fn vrcp14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 76, 211], OperandSize::Qword)
}

#[test]
fn vrcp14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectDisplaced(RBX, 1049024375, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 173, 76, 187, 119, 215, 134, 62], OperandSize::Qword)
}

#[test]
fn vrcp14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 190, 76, 44, 145], OperandSize::Qword)
}

#[test]
fn vrcp14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 76, 206], OperandSize::Dword)
}

#[test]
fn vrcp14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 828172751, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 76, 52, 197, 207, 233, 92, 49], OperandSize::Dword)
}

#[test]
fn vrcp14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1051420863, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 76, 148, 122, 191, 104, 171, 62], OperandSize::Dword)
}

#[test]
fn vrcp14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 125, 205, 76, 224], OperandSize::Qword)
}

#[test]
fn vrcp14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 201, 76, 28, 178], OperandSize::Qword)
}

#[test]
fn vrcp14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(RCX, 1961398463, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 76, 177, 191, 144, 232, 116], OperandSize::Qword)
}

