use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 192], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 167082144, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 60, 93, 160, 120, 245, 9], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 213], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 583313607, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 12, 149, 199, 168, 196, 34], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 248], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 2005582350, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 132, 223, 14, 194, 138, 119], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 249], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 62], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 37, 240], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 37, 28, 135], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 125, 137, 37, 251], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM22)), operand2: Some(IndirectDisplaced(RDX, 788666446, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 37, 178, 78, 24, 2, 47], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 37, 200], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(ECX, 1526907460, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 37, 169, 68, 194, 2, 91], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 174, 37, 197], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1210389055, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 37, 60, 245, 63, 18, 37, 72], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 37, 231], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 37, 30], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 125, 204, 37, 213], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 227301815, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 37, 140, 66, 183, 89, 140, 13], OperandSize::Qword)
}

