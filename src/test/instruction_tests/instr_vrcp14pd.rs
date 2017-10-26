use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 76, 209], OperandSize::Dword)
}

#[test]
fn vrcp14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1336577004, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 76, 188, 89, 236, 139, 170, 79], OperandSize::Dword)
}

#[test]
fn vrcp14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1665463164, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 156, 76, 158, 124, 243, 68, 99], OperandSize::Dword)
}

#[test]
fn vrcp14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 253, 143, 76, 240], OperandSize::Qword)
}

#[test]
fn vrcp14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1167166211, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 142, 76, 132, 127, 3, 139, 145, 69], OperandSize::Qword)
}

#[test]
fn vrcp14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 156, 76, 28, 246], OperandSize::Qword)
}

#[test]
fn vrcp14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 76, 233], OperandSize::Dword)
}

#[test]
fn vrcp14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EAX, 848683395, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 76, 152, 131, 225, 149, 50], OperandSize::Dword)
}

#[test]
fn vrcp14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 76, 35], OperandSize::Dword)
}

#[test]
fn vrcp14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 253, 171, 76, 195], OperandSize::Qword)
}

#[test]
fn vrcp14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM29)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 172, 76, 42], OperandSize::Qword)
}

#[test]
fn vrcp14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM30)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 185, 76, 51], OperandSize::Qword)
}

#[test]
fn vrcp14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 76, 219], OperandSize::Dword)
}

#[test]
fn vrcp14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1320326398, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 76, 12, 205, 254, 148, 178, 78], OperandSize::Dword)
}

#[test]
fn vrcp14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 46160409, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 76, 4, 133, 25, 90, 192, 2], OperandSize::Dword)
}

#[test]
fn vrcp14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 205, 76, 212], OperandSize::Qword)
}

#[test]
fn vrcp14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM8)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 203, 76, 2], OperandSize::Qword)
}

#[test]
fn vrcp14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1174827071, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 223, 76, 180, 118, 63, 112, 6, 70], OperandSize::Qword)
}

