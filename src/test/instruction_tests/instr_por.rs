use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn por_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 219], OperandSize::Dword)
}

#[test]
fn por_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDX, 389414313, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 186, 169, 253, 53, 23], OperandSize::Dword)
}

#[test]
fn por_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 192], OperandSize::Qword)
}

#[test]
fn por_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 36, 131], OperandSize::Qword)
}

#[test]
fn por_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 251], OperandSize::Dword)
}

#[test]
fn por_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1363578530, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 28, 93, 162, 142, 70, 81], OperandSize::Dword)
}

#[test]
fn por_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 197], OperandSize::Qword)
}

#[test]
fn por_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 497038708, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 180, 255, 116, 53, 160, 29], OperandSize::Qword)
}

