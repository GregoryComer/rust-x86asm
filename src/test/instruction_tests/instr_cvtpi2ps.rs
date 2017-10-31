use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 193], OperandSize::Word)
}

#[test]
fn cvtpi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 172, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 170, 172, 0], OperandSize::Word)
}

#[test]
fn cvtpi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 200], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 44, 138], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 212], OperandSize::Qword)
}

#[test]
fn cvtpi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 4, 136], OperandSize::Qword)
}

