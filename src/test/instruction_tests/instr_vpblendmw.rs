use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 102, 210], OperandSize::Dword)
}

#[test]
fn vpblendmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 102, 36, 64], OperandSize::Dword)
}

#[test]
fn vpblendmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 221, 143, 102, 194], OperandSize::Qword)
}

#[test]
fn vpblendmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1298867299, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 181, 135, 102, 60, 117, 99, 36, 107, 77], OperandSize::Qword)
}

#[test]
fn vpblendmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 102, 253], OperandSize::Dword)
}

#[test]
fn vpblendmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 102, 34], OperandSize::Dword)
}

#[test]
fn vpblendmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 149, 165, 102, 222], OperandSize::Qword)
}

#[test]
fn vpblendmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 380230497, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 149, 163, 102, 132, 83, 97, 219, 169, 22], OperandSize::Qword)
}

#[test]
fn vpblendmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 102, 219], OperandSize::Dword)
}

#[test]
fn vpblendmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 887361155, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 201, 102, 132, 139, 131, 14, 228, 52], OperandSize::Dword)
}

#[test]
fn vpblendmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 237, 198, 102, 197], OperandSize::Qword)
}

#[test]
fn vpblendmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 240417579, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 165, 203, 102, 132, 249, 43, 123, 84, 14], OperandSize::Qword)
}

