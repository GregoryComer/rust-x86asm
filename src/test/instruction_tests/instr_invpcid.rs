use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invpcid_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 12, 198], OperandSize::Dword)
}

#[test]
fn invpcid_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 55], OperandSize::Qword)
}

