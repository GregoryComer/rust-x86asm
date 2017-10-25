use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 216], OperandSize::Dword)
}

#[test]
fn cvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 10], OperandSize::Dword)
}

#[test]
fn cvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 194], OperandSize::Qword)
}

#[test]
fn cvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 44, 249], OperandSize::Qword)
}

