use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 219], OperandSize::Dword)
}

#[test]
fn pminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 36, 241], OperandSize::Dword)
}

#[test]
fn pminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 212], OperandSize::Qword)
}

#[test]
fn pminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 164472341, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 36, 213, 21, 166, 205, 9], OperandSize::Qword)
}

