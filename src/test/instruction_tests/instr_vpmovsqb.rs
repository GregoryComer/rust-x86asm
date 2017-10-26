use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 34, 246], OperandSize::Dword)
}

#[test]
fn vpmovsqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 34, 17], OperandSize::Dword)
}

#[test]
fn vpmovsqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 34, 219], OperandSize::Qword)
}

#[test]
fn vpmovsqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 34, 63], OperandSize::Qword)
}

#[test]
fn vpmovsqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 34, 216], OperandSize::Dword)
}

#[test]
fn vpmovsqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1703987725, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 34, 156, 240, 13, 202, 144, 101], OperandSize::Dword)
}

#[test]
fn vpmovsqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 126, 169, 34, 204], OperandSize::Qword)
}

#[test]
fn vpmovsqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 34, 30], OperandSize::Qword)
}

#[test]
fn vpmovsqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 34, 201], OperandSize::Dword)
}

#[test]
fn vpmovsqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 16], OperandSize::Dword)
}

#[test]
fn vpmovsqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 126, 204, 34, 238], OperandSize::Qword)
}

#[test]
fn vpmovsqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 34, 38], OperandSize::Qword)
}

