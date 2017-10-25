use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 220], OperandSize::Dword)
}

#[test]
fn subsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 28, 143], OperandSize::Dword)
}

#[test]
fn subsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 204], OperandSize::Qword)
}

#[test]
fn subsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1187529349, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 148, 194, 133, 66, 200, 70], OperandSize::Qword)
}

