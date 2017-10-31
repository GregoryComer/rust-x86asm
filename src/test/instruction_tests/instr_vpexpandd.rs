use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 137, 233], OperandSize::Dword)
}

#[test]
fn vpexpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 137, 10], OperandSize::Dword)
}

#[test]
fn vpexpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 141, 137, 202], OperandSize::Qword)
}

#[test]
fn vpexpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 137, 4, 120], OperandSize::Qword)
}

#[test]
fn vpexpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 137, 245], OperandSize::Dword)
}

#[test]
fn vpexpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 1255338979, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 137, 159, 227, 243, 210, 74], OperandSize::Dword)
}

#[test]
fn vpexpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 125, 169, 137, 226], OperandSize::Qword)
}

#[test]
fn vpexpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM14)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 175, 137, 48], OperandSize::Qword)
}

#[test]
fn vpexpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 137, 249], OperandSize::Dword)
}

#[test]
fn vpexpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1431153468, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 137, 44, 221, 60, 171, 77, 85], OperandSize::Dword)
}

#[test]
fn vpexpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 137, 202], OperandSize::Qword)
}

#[test]
fn vpexpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM10)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 137, 17], OperandSize::Qword)
}

