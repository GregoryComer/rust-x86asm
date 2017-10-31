use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(RDX)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 250, 9], OperandSize::Qword)
}

#[test]
fn pinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 135843107, Some(OperandSize::Qword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 140, 223, 35, 205, 24, 8, 17], OperandSize::Qword)
}

