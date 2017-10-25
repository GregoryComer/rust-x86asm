use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 203], OperandSize::Word)
}

#[test]
fn cvtpi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Memory(20217, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 14, 249, 78], OperandSize::Word)
}

#[test]
fn cvtpi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 253], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 398844499, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 36, 157, 83, 226, 197, 23], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 203], OperandSize::Qword)
}

#[test]
fn cvtpi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 27], OperandSize::Qword)
}

