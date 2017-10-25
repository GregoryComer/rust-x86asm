use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 141, 213], OperandSize::Dword)
}

#[test]
fn vpermw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 141, 60, 219], OperandSize::Dword)
}

#[test]
fn vpermw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 221, 137, 141, 241], OperandSize::Qword)
}

#[test]
fn vpermw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 141, 141, 32], OperandSize::Qword)
}

#[test]
fn vpermw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 175, 141, 198], OperandSize::Dword)
}

#[test]
fn vpermw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 2144021963, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 141, 172, 121, 203, 45, 203, 127], OperandSize::Dword)
}

#[test]
fn vpermw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 149, 171, 141, 213], OperandSize::Qword)
}

#[test]
fn vpermw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RAX, 1344854761, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 149, 169, 141, 176, 233, 218, 40, 80], OperandSize::Qword)
}

#[test]
fn vpermw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 141, 210], OperandSize::Dword)
}

#[test]
fn vpermw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1585223708, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 141, 180, 186, 28, 152, 124, 94], OperandSize::Dword)
}

#[test]
fn vpermw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 149, 193, 141, 252], OperandSize::Qword)
}

#[test]
fn vpermw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 710132770, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 141, 132, 136, 34, 196, 83, 42], OperandSize::Qword)
}

