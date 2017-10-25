use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn orps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 239], OperandSize::Dword)
}

#[test]
fn orps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1621420743, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 4, 117, 199, 234, 164, 96], OperandSize::Dword)
}

#[test]
fn orps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 221], OperandSize::Qword)
}

#[test]
fn orps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 25], OperandSize::Qword)
}

