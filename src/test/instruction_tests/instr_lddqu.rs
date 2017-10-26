use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 52, 223], OperandSize::Dword)
}

#[test]
fn lddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 36, 120], OperandSize::Qword)
}

