use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 828494950, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 172, 74, 102, 212, 97, 49], OperandSize::Dword)
}

#[test]
fn lddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 10], OperandSize::Qword)
}

