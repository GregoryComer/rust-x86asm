use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 254, 201], OperandSize::Dword)
}

#[test]
fn vpaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 254, 2], OperandSize::Dword)
}

#[test]
fn vpaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 254, 211], OperandSize::Qword)
}

#[test]
fn vpaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 675995468, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 254, 180, 209, 76, 223, 74, 40], OperandSize::Qword)
}

#[test]
fn vpaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 254, 201], OperandSize::Dword)
}

#[test]
fn vpaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 947195398, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 254, 20, 189, 6, 14, 117, 56], OperandSize::Dword)
}

#[test]
fn vpaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 254, 225], OperandSize::Qword)
}

#[test]
fn vpaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 254, 20, 121], OperandSize::Qword)
}

#[test]
fn vpaddd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 254, 208], OperandSize::Dword)
}

#[test]
fn vpaddd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 254, 22], OperandSize::Dword)
}

#[test]
fn vpaddd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1155942694, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 158, 254, 28, 93, 38, 73, 230, 68], OperandSize::Dword)
}

#[test]
fn vpaddd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 134, 254, 252], OperandSize::Qword)
}

#[test]
fn vpaddd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 132, 254, 35], OperandSize::Qword)
}

#[test]
fn vpaddd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 674386512, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 109, 157, 254, 156, 72, 80, 82, 50, 40], OperandSize::Qword)
}

