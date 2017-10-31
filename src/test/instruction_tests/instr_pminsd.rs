use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 234], OperandSize::Dword)
}

#[test]
fn pminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 142715857, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 44, 93, 209, 171, 129, 8], OperandSize::Dword)
}

#[test]
fn pminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 247], OperandSize::Qword)
}

#[test]
fn pminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1955853054, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 188, 67, 254, 242, 147, 116], OperandSize::Qword)
}

