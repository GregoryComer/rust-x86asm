use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 229], OperandSize::Dword)
}

#[test]
fn pmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 1500740227, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 182, 131, 122, 115, 89], OperandSize::Dword)
}

#[test]
fn pmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 208], OperandSize::Qword)
}

#[test]
fn pmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1291568088, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 140, 222, 216, 195, 251, 76], OperandSize::Qword)
}

