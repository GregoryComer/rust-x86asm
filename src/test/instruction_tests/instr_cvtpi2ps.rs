use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 220], OperandSize::Word)
}

#[test]
fn cvtpi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 49], OperandSize::Word)
}

#[test]
fn cvtpi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 195], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 44, 153], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 226], OperandSize::Qword)
}

#[test]
fn cvtpi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1717535164, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 188, 147, 188, 129, 95, 102], OperandSize::Qword)
}

