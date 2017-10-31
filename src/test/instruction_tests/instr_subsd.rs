use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 200], OperandSize::Dword)
}

#[test]
fn subsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1322882642, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 140, 203, 82, 150, 217, 78], OperandSize::Dword)
}

#[test]
fn subsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 253], OperandSize::Qword)
}

#[test]
fn subsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 25], OperandSize::Qword)
}

